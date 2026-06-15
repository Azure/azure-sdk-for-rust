// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The per-operation **capture recorder** — the entire hot-path cost of diagnostics.
//!
//! A [`DiagnosticsRecorder`] rents one buffer from a [`LogPool`](super::pool::LogPool) at
//! operation start and appends a compact, tag-length-value (TLV) record stream as attempts and
//! hedge legs complete. Appends go through `&mut self`, so there is **no lock on the per-attempt
//! hot path**; the pool's brief lock is touched only at operation boundaries (rent / return).
//!
//! At operation end the recorder is handed to the gate ([`super::gate`]). On a fast success the
//! buffer is returned to the pool for ~free; on a slow or errored operation the log is parsed and
//! materialized into the canonical [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext)
//! (see [`super::context`]).
//!
//! ## Cancellation & panic safety
//!
//! The recorder owns its buffer in an [`Option`]. [`DiagnosticsRecorder::return_buffer`] takes the
//! buffer out so the gate can return it to the pool. If the operation future is dropped before
//! the gate runs (cancellation, timeout, `select!`) or a panic unwinds through it, the [`Drop`]
//! impl returns the still-owned buffer to the pool — so a cancelled or panicking operation never
//! leaks a pooled buffer, and a partially written buffer is `clear()`-ed before reuse.

use super::pool::LogPool;
use super::Outcome;
use crate::diagnostics::ExecutionContext;
use std::time::Instant;

/// Record tags in the append-only capture log.
#[repr(u8)]
enum Tag {
    Op = 1,
    Attempt = 2,
    Hedge = 3,
    End = 4,
}

// --- compact varint + string TLV helpers (LEB128) ---------------------------

fn write_varint(out: &mut Vec<u8>, mut value: u64) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

fn read_varint(input: &[u8], pos: &mut usize) -> Option<u64> {
    let mut result: u64 = 0;
    let mut shift = 0;
    loop {
        let byte = *input.get(*pos)?;
        *pos += 1;
        result |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            return None;
        }
    }
    Some(result)
}

fn write_str(out: &mut Vec<u8>, value: &str) {
    write_varint(out, value.len() as u64);
    out.extend_from_slice(value.as_bytes());
}

fn read_str(input: &[u8], pos: &mut usize) -> Option<String> {
    let len = usize::try_from(read_varint(input, pos)?).ok()?;
    let end = pos.checked_add(len)?;
    let bytes = input.get(*pos..end)?;
    let s = std::str::from_utf8(bytes).ok()?.to_string();
    *pos = end;
    Some(s)
}

/// Maps an [`ExecutionContext`] to its wire byte.
fn exec_context_to_u8(ctx: ExecutionContext) -> u8 {
    match ctx {
        ExecutionContext::Initial => 0,
        ExecutionContext::Retry => 1,
        ExecutionContext::TransportRetry => 2,
        ExecutionContext::Hedging => 3,
        ExecutionContext::RegionFailover => 4,
        ExecutionContext::CircuitBreakerProbe => 5,
    }
}

/// Maps a wire byte back to an [`ExecutionContext`], defaulting unknown values to `Initial`.
fn exec_context_from_u8(value: u8) -> ExecutionContext {
    match value {
        1 => ExecutionContext::Retry,
        2 => ExecutionContext::TransportRetry,
        3 => ExecutionContext::Hedging,
        4 => ExecutionContext::RegionFailover,
        5 => ExecutionContext::CircuitBreakerProbe,
        _ => ExecutionContext::Initial,
    }
}

/// The terminal outcome of a hedging race, captured for the built `HedgeDiagnostics`.
///
/// [`HedgeDiagnostics`]: crate::diagnostics::HedgeDiagnostics
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HedgeOutcome {
    /// The primary returned before the hedge threshold fired; no alternate spawned.
    PrimaryWonPreThreshold,
    /// The primary returned after the threshold, beating the spawned alternate.
    PrimaryWonAfterHedge,
    /// The alternate (hedge) leg won the race.
    AlternateWon,
    /// Both legs returned retriable failures; `deadline_elapsed` carries through.
    BothTransient {
        /// Whether the operation deadline had elapsed when the race concluded.
        deadline_elapsed: bool,
    },
}

impl HedgeOutcome {
    fn to_u8(self) -> u8 {
        match self {
            HedgeOutcome::PrimaryWonPreThreshold => 0,
            HedgeOutcome::PrimaryWonAfterHedge => 1,
            HedgeOutcome::AlternateWon => 2,
            HedgeOutcome::BothTransient {
                deadline_elapsed: false,
            } => 3,
            HedgeOutcome::BothTransient {
                deadline_elapsed: true,
            } => 4,
        }
    }

    fn from_u8(value: u8) -> Self {
        match value {
            1 => HedgeOutcome::PrimaryWonAfterHedge,
            2 => HedgeOutcome::AlternateWon,
            4 => HedgeOutcome::BothTransient {
                deadline_elapsed: true,
            },
            3 => HedgeOutcome::BothTransient {
                deadline_elapsed: false,
            },
            _ => HedgeOutcome::PrimaryWonPreThreshold,
        }
    }
}

/// Per-attempt data captured on the hot path. `start_ns`/`duration_ns` are relative to op start.
///
/// This is a plain value type so a concurrent fan-out / hedge task can capture its own attempt
/// and the operation layer merges it later — keeping per-task capture lock-free.
#[derive(Clone, Debug)]
pub struct AttemptRecord {
    /// Which execution context this attempt ran under (Initial / Retry / Hedging / ...).
    pub execution_context: ExecutionContext,
    /// The region this attempt targeted (empty when unknown / global endpoint).
    pub region: String,
    /// The endpoint URL this attempt targeted.
    pub endpoint: String,
    /// HTTP status code (`0` for a transport failure with no response).
    pub status: u16,
    /// Cosmos sub-status, when present.
    pub sub_status: Option<u16>,
    /// Service request id / activity id from the response.
    pub service_request_id: Option<String>,
    /// Request charge (RU).
    pub request_charge: Option<f64>,
    /// Retry-safety signal on a transport failure (`sent` / `not_sent` / `unknown`).
    pub request_sent: Option<String>,
    /// Start relative to the operation start (nanoseconds); filled from the clock when `0`.
    pub start_ns: u64,
    /// Attempt duration (nanoseconds).
    pub duration_ns: u64,
}

impl AttemptRecord {
    /// Creates an attempt record for the common success/response case.
    pub fn new(
        execution_context: ExecutionContext,
        region: impl Into<String>,
        endpoint: impl Into<String>,
        status: u16,
    ) -> Self {
        Self {
            execution_context,
            region: region.into(),
            endpoint: endpoint.into(),
            status,
            sub_status: None,
            service_request_id: None,
            request_charge: None,
            request_sent: None,
            start_ns: 0,
            duration_ns: 0,
        }
    }

    /// Sets the service request id / activity id (builder-style).
    pub fn with_service_request_id(mut self, id: impl Into<String>) -> Self {
        self.service_request_id = Some(id.into());
        self
    }

    /// Sets the request charge (builder-style).
    pub fn with_request_charge(mut self, ru: f64) -> Self {
        self.request_charge = Some(ru);
        self
    }

    /// Sets the Cosmos sub-status (builder-style).
    pub fn with_sub_status(mut self, sub_status: u16) -> Self {
        self.sub_status = Some(sub_status);
        self
    }

    /// Sets the retry-safety signal for a transport failure (builder-style).
    pub fn with_request_sent(mut self, request_sent: impl Into<String>) -> Self {
        self.request_sent = Some(request_sent.into());
        self
    }

    /// Sets the attempt duration in nanoseconds (builder-style).
    pub fn with_duration_ns(mut self, duration_ns: u64) -> Self {
        self.duration_ns = duration_ns;
        self
    }
}

/// A per-operation append-only capture recorder.
#[derive(Debug)]
pub struct DiagnosticsRecorder {
    pool: LogPool,
    buf: Option<Vec<u8>>,
    start: Instant,
    outcome: Outcome,
    attempt_count: u32,
    total_ns: u64,
}

impl DiagnosticsRecorder {
    /// Begins capture for an operation, renting a buffer and writing the operation header.
    ///
    /// `operation` is the operation name, `endpoint` the initial target URL, and `activity_id`
    /// the operation's activity id (used as the top-level id of the built `DiagnosticsContext`).
    pub fn start(pool: &LogPool, operation: &str, endpoint: &str, activity_id: &str) -> Self {
        let mut buf = pool.rent();
        buf.push(Tag::Op as u8);
        write_str(&mut buf, operation);
        write_str(&mut buf, endpoint);
        write_str(&mut buf, activity_id);
        Self {
            pool: pool.clone(),
            buf: Some(buf),
            start: Instant::now(),
            outcome: Outcome::Success,
            attempt_count: 0,
            total_ns: 0,
        }
    }

    /// Nanoseconds elapsed since the operation started (monotonic, from [`Instant`]).
    pub fn elapsed_ns(&self) -> u64 {
        self.start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64
    }

    /// Records one attempt (or hedge leg). `start_ns` is set from the recorder's clock if `0`.
    pub fn record_attempt(&mut self, mut attempt: AttemptRecord) {
        let Some(buf) = self.buf.as_mut() else {
            return;
        };
        if attempt.start_ns == 0 {
            attempt.start_ns = self.start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
        }
        buf.push(Tag::Attempt as u8);
        buf.push(exec_context_to_u8(attempt.execution_context));
        write_str(buf, &attempt.region);
        write_str(buf, &attempt.endpoint);
        write_varint(buf, u64::from(attempt.status));
        write_varint(buf, attempt.sub_status.map_or(0, |s| u64::from(s) + 1));
        write_str(buf, attempt.service_request_id.as_deref().unwrap_or(""));
        buf.extend_from_slice(&(attempt.request_charge.unwrap_or(0.0) as f32).to_le_bytes());
        write_str(buf, attempt.request_sent.as_deref().unwrap_or(""));
        write_varint(buf, attempt.start_ns);
        write_varint(buf, attempt.duration_ns);
        self.attempt_count += 1;
    }

    /// Records the hedge race outcome (regions + terminal state) for the built `HedgeDiagnostics`.
    pub fn record_hedge_outcome(
        &mut self,
        outcome: HedgeOutcome,
        threshold: std::time::Duration,
        primary_region: &str,
        alternate_region: Option<&str>,
        response_region: Option<&str>,
    ) {
        let Some(buf) = self.buf.as_mut() else {
            return;
        };
        buf.push(Tag::Hedge as u8);
        buf.push(outcome.to_u8());
        write_varint(buf, threshold.as_nanos().min(u128::from(u64::MAX)) as u64);
        write_str(buf, primary_region);
        write_str(buf, alternate_region.unwrap_or(""));
        write_str(buf, response_region.unwrap_or(""));
    }

    /// Records the operation outcome and finalizes the log header.
    ///
    /// `final_status` / `final_sub_status` set the top-level status of the built context. If
    /// `total_ns` is `None`, the recorder's own monotonic elapsed time is used.
    pub fn record_end(
        &mut self,
        outcome: Outcome,
        attempt_count: u32,
        final_status: u16,
        final_sub_status: Option<u16>,
        total_ns: Option<u64>,
    ) {
        let total = total_ns.unwrap_or_else(|| self.elapsed_ns());
        self.outcome = outcome;
        self.attempt_count = attempt_count;
        self.total_ns = total;
        if let Some(buf) = self.buf.as_mut() {
            buf.push(Tag::End as u8);
            buf.push(match outcome {
                Outcome::Success => 0,
                Outcome::Error => 1,
            });
            write_varint(buf, u64::from(attempt_count));
            write_varint(buf, u64::from(final_status));
            write_varint(buf, final_sub_status.map_or(0, |s| u64::from(s) + 1));
            write_varint(buf, total);
        }
    }

    /// The recorded operation outcome.
    pub fn outcome(&self) -> Outcome {
        self.outcome
    }

    /// The recorded total elapsed time (nanoseconds).
    pub fn total_ns(&self) -> u64 {
        self.total_ns
    }

    /// The raw, compact size of what was appended on the hot path.
    pub fn raw_len(&self) -> usize {
        self.buf.as_ref().map_or(0, Vec::len)
    }

    /// Borrows the raw capture bytes (for parsing past the gate).
    pub(crate) fn bytes(&self) -> &[u8] {
        self.buf.as_deref().unwrap_or(&[])
    }

    /// Returns the backing buffer to the pool, consuming the recorder.
    pub(crate) fn return_buffer(mut self) {
        if let Some(buf) = self.buf.take() {
            self.pool.give_back(buf);
        }
    }
}

impl Drop for DiagnosticsRecorder {
    fn drop(&mut self) {
        // Cancellation / panic safety: return the buffer to the pool if the gate didn't consume
        // it. `give_back` clears it, so a partially written buffer never poisons the next rent.
        if let Some(buf) = self.buf.take() {
            self.pool.give_back(buf);
        }
    }
}

/// A fully parsed attempt.
pub(crate) struct ParsedAttempt {
    pub execution_context: ExecutionContext,
    pub region: String,
    pub endpoint: String,
    pub status: u16,
    pub sub_status: Option<u16>,
    pub service_request_id: String,
    pub request_charge: f32,
    pub request_sent: String,
    pub duration_ns: u64,
}

/// The parsed hedge outcome.
pub(crate) struct ParsedHedge {
    pub outcome: HedgeOutcome,
    pub threshold_ns: u64,
    pub primary_region: String,
    pub alternate_region: Option<String>,
}

/// A fully parsed capture log, ready to materialize into a `DiagnosticsContext`.
pub(crate) struct Parsed {
    pub operation: String,
    pub activity_id: String,
    pub attempts: Vec<ParsedAttempt>,
    pub hedge: Option<ParsedHedge>,
    pub outcome: Outcome,
    pub attempt_count: u32,
    pub final_status: u16,
    pub final_sub_status: Option<u16>,
    pub total_ns: u64,
}

/// Parses a raw capture log buffer. Tolerant of truncation (a cancelled op may lack the `End`).
pub(crate) fn parse(buf: &[u8]) -> Parsed {
    let mut p = Parsed {
        operation: String::new(),
        activity_id: String::new(),
        attempts: Vec::new(),
        hedge: None,
        outcome: Outcome::Success,
        attempt_count: 0,
        final_status: 0,
        final_sub_status: None,
        total_ns: 0,
    };
    let mut pos = 0usize;
    while pos < buf.len() {
        let tag = buf[pos];
        pos += 1;
        match tag {
            t if t == Tag::Op as u8 => {
                p.operation = read_str(buf, &mut pos).unwrap_or_default();
                let _endpoint = read_str(buf, &mut pos).unwrap_or_default();
                p.activity_id = read_str(buf, &mut pos).unwrap_or_default();
            }
            t if t == Tag::Attempt as u8 => {
                let Some(exec) = buf.get(pos).copied() else {
                    break;
                };
                pos += 1;
                let region = read_str(buf, &mut pos).unwrap_or_default();
                let endpoint = read_str(buf, &mut pos).unwrap_or_default();
                let status = read_varint(buf, &mut pos)
                    .unwrap_or(0)
                    .min(u64::from(u16::MAX)) as u16;
                let raw_sub = read_varint(buf, &mut pos).unwrap_or(0);
                let sub_status =
                    (raw_sub != 0).then(|| (raw_sub - 1).min(u64::from(u16::MAX)) as u16);
                let service_request_id = read_str(buf, &mut pos).unwrap_or_default();
                let mut ru_bytes = [0u8; 4];
                if let Some(slice) = buf.get(pos..pos + 4) {
                    ru_bytes.copy_from_slice(slice);
                }
                pos += 4;
                let request_charge = f32::from_le_bytes(ru_bytes);
                let request_sent = read_str(buf, &mut pos).unwrap_or_default();
                let _start_ns = read_varint(buf, &mut pos).unwrap_or(0);
                let duration_ns = read_varint(buf, &mut pos).unwrap_or(0);
                p.attempts.push(ParsedAttempt {
                    execution_context: exec_context_from_u8(exec),
                    region,
                    endpoint,
                    status,
                    sub_status,
                    service_request_id,
                    request_charge,
                    request_sent,
                    duration_ns,
                });
            }
            t if t == Tag::Hedge as u8 => {
                let Some(outcome_byte) = buf.get(pos).copied() else {
                    break;
                };
                pos += 1;
                let threshold_ns = read_varint(buf, &mut pos).unwrap_or(0);
                let primary_region = read_str(buf, &mut pos).unwrap_or_default();
                let alternate = read_str(buf, &mut pos).unwrap_or_default();
                let _response = read_str(buf, &mut pos).unwrap_or_default();
                p.hedge = Some(ParsedHedge {
                    outcome: HedgeOutcome::from_u8(outcome_byte),
                    threshold_ns,
                    primary_region,
                    alternate_region: (!alternate.is_empty()).then_some(alternate),
                });
            }
            t if t == Tag::End as u8 => {
                p.outcome = if buf.get(pos).copied() == Some(1) {
                    Outcome::Error
                } else {
                    Outcome::Success
                };
                pos += 1;
                p.attempt_count = read_varint(buf, &mut pos)
                    .unwrap_or(0)
                    .min(u64::from(u32::MAX)) as u32;
                p.final_status = read_varint(buf, &mut pos)
                    .unwrap_or(0)
                    .min(u64::from(u16::MAX)) as u16;
                let raw_sub = read_varint(buf, &mut pos).unwrap_or(0);
                p.final_sub_status =
                    (raw_sub != 0).then(|| (raw_sub - 1).min(u64::from(u16::MAX)) as u16);
                p.total_ns = read_varint(buf, &mut pos).unwrap_or(0);
            }
            _ => break,
        }
    }
    p
}
