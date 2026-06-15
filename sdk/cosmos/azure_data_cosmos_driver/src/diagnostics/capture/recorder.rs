// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The per-operation **capture recorder** — the entire hot-path cost of diagnostics.
//!
//! A [`DiagnosticsRecorder`] rents one buffer from a [`LogPool`](super::pool::LogPool) at
//! operation start and appends a compact, tag-length-value (TLV) record stream as attempts and
//! fan-out children complete. Appends go through `&mut self`, so there is **no lock on the
//! per-attempt hot path**; the pool's brief lock is touched only at operation boundaries
//! (rent / return).
//!
//! At operation end the recorder is handed to the gate ([`super::gate`]). On a fast success the
//! buffer is returned to the pool for ~free; on a slow or errored operation the log is parsed and
//! built into a summary (and, opt-in, an `AZD1` blob).
//!
//! ## Cancellation & panic safety
//!
//! The recorder owns its buffer in an [`Option`]. [`DiagnosticsRecorder::finish`] takes the
//! buffer out and returns it to the pool. If the operation future is dropped before `finish`
//! runs (cancellation, timeout, `select!`) or a panic unwinds through it, the [`Drop`] impl
//! returns the still-owned buffer to the pool — so a cancelled or panicking operation never
//! leaks a pooled buffer, and a partially written buffer is `clear()`-ed before reuse, never
//! poisoning the next operation.

use super::pool::LogPool;
use super::wire;
use super::Outcome;
use std::time::Instant;

/// Record tags in the append-only capture log.
#[repr(u8)]
enum Tag {
    Op = 1,
    Attempt = 2,
    Child = 3,
    End = 4,
}

/// A fan-out child captured by a concurrent task, merged at the operation layer on join.
///
/// Carrying plain values (not a shared recorder) keeps per-task capture lock-free for concurrent
/// cross-partition children. See [`DiagnosticsRecorder::merge_child`].
#[derive(Clone, Debug)]
pub struct ChildRecord {
    /// Query-plan node id for this child.
    pub plan_node_id: String,
    /// Feed range this child addresses.
    pub feed_range: String,
    /// Start (nanoseconds) relative to the operation start.
    pub start_ns: u64,
    /// Child duration (nanoseconds).
    pub duration_ns: u64,
}

/// A per-operation append-only capture recorder.
///
/// Construct one with [`DiagnosticsRecorder::start`], append per-attempt/child records as the
/// operation runs, then call [`DiagnosticsRecorder::finish`] with the effective
/// [`DiagnosticsPolicy`](super::DiagnosticsPolicy) to gate and (maybe) build the diagnostics.
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
    /// `operation` is the operation name (e.g. `read_item`), `endpoint` the target URL, and
    /// `client_request_id` the client-generated correlation id. The single preamble id byte
    /// stands in for the full SDK/driver version + User-Agent provenance, rehydrated only if the
    /// gate later builds the diagnostics.
    pub fn start(pool: &LogPool, operation: &str, endpoint: &str, client_request_id: &str) -> Self {
        let mut buf = pool.rent();
        buf.push(Tag::Op as u8);
        buf.push(super::preamble::PREAMBLE_ID);
        wire::write_str(&mut buf, operation);
        wire::write_str(&mut buf, endpoint);
        wire::write_str(&mut buf, client_request_id);
        Self {
            pool: pool.clone(),
            buf: Some(buf),
            start: Instant::now(),
            outcome: Outcome::Success,
            attempt_count: 0,
            total_ns: 0,
        }
    }

    /// The [`Instant`] the operation started, for computing relative attempt timings.
    pub fn started_at(&self) -> Instant {
        self.start
    }

    /// Nanoseconds elapsed since the operation started (monotonic, from [`Instant`]).
    pub fn elapsed_ns(&self) -> u64 {
        self.start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64
    }

    /// Records one HTTP attempt. `start_ns` is relative to the operation start.
    ///
    /// `service_request_id` is read from the **response** (Cosmos `x-ms-request-id` /
    /// activity id); `request_charge` is the per-attempt RU. Both are `None` when unavailable
    /// (e.g. a transport failure before a response). For sub-status / request-sent detail use
    /// [`DiagnosticsRecorder::record_attempt_ext`].
    pub fn record_attempt(
        &mut self,
        attempt_index: u32,
        status: u16,
        service_request_id: Option<&str>,
        request_charge: Option<f64>,
        start_ns: u64,
        duration_ns: u64,
    ) {
        self.record_attempt_ext(
            attempt_index,
            status,
            service_request_id,
            request_charge,
            None,
            None,
            start_ns,
            duration_ns,
        );
    }

    /// Records one HTTP attempt with Cosmos detail (`sub_status`, `request_sent`).
    ///
    /// `sub_status` is the Cosmos `x-ms-substatus` (finer error classification, available on the
    /// response path); `request_sent` is the retry-safety signal on a transport failure
    /// (`"sent"` / `"not_sent"` / `"unknown"`). Both are `None`/absent when not applicable.
    #[allow(clippy::too_many_arguments)]
    pub fn record_attempt_ext(
        &mut self,
        attempt_index: u32,
        status: u16,
        service_request_id: Option<&str>,
        request_charge: Option<f64>,
        sub_status: Option<u16>,
        request_sent: Option<&str>,
        start_ns: u64,
        duration_ns: u64,
    ) {
        let Some(buf) = self.buf.as_mut() else {
            return;
        };
        buf.push(Tag::Attempt as u8);
        wire::write_varint(buf, u64::from(attempt_index));
        wire::write_varint(buf, u64::from(status));
        wire::write_str(buf, service_request_id.unwrap_or_default());
        buf.extend_from_slice(&(request_charge.unwrap_or(0.0) as f32).to_le_bytes());
        wire::write_varint(buf, start_ns);
        wire::write_varint(buf, duration_ns);
        // sub_status: 0 == none, else value + 1 (sub-status 0 is a valid Cosmos code).
        wire::write_varint(buf, sub_status.map_or(0, |s| u64::from(s) + 1));
        wire::write_str(buf, request_sent.unwrap_or_default());
    }

    /// Records one fan-out child (per partition / feed range). `start_ns` is relative to op start.
    pub fn record_child(
        &mut self,
        plan_node_id: &str,
        feed_range: &str,
        start_ns: u64,
        duration_ns: u64,
    ) {
        let Some(buf) = self.buf.as_mut() else {
            return;
        };
        buf.push(Tag::Child as u8);
        wire::write_str(buf, plan_node_id);
        wire::write_str(buf, feed_range);
        wire::write_varint(buf, start_ns);
        wire::write_varint(buf, duration_ns);
    }

    /// Merges a [`ChildRecord`] captured by a concurrent fan-out task at join time.
    ///
    /// Per the fan-out design, concurrent cross-partition children do **not** share the
    /// single-owner `&mut` recorder. Each child task captures its own values into a plain
    /// [`ChildRecord`] (a `Send` value, no shared state, no lock) and the operation layer merges
    /// them here once the children join — keeping per-task capture lock-free.
    pub fn merge_child(&mut self, child: &ChildRecord) {
        self.record_child(
            &child.plan_node_id,
            &child.feed_range,
            child.start_ns,
            child.duration_ns,
        );
    }

    /// Records the operation outcome and finalizes the log header.
    ///
    /// If `total_ns` is `None`, the recorder's own monotonic elapsed time is used.
    pub fn record_end(&mut self, outcome: Outcome, attempt_count: u32, total_ns: Option<u64>) {
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
            wire::write_varint(buf, u64::from(attempt_count));
            wire::write_varint(buf, total);
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

    /// Borrows the raw capture bytes (for parsing/building past the gate).
    pub(crate) fn bytes(&self) -> &[u8] {
        self.buf.as_deref().unwrap_or(&[])
    }

    /// Returns the backing buffer to the pool, consuming the recorder.
    ///
    /// Called by the gate after a build/drop decision. After this, [`Drop`] is a no-op.
    pub(crate) fn return_buffer(mut self) {
        if let Some(buf) = self.buf.take() {
            self.pool.give_back(buf);
        }
    }
}

impl Drop for DiagnosticsRecorder {
    fn drop(&mut self) {
        // Cancellation / panic safety: if the buffer was not consumed by `return_buffer`
        // (e.g. the operation future was dropped before the gate ran), return it to the pool.
        // `give_back` clears it, so a partially written buffer never poisons the next rent.
        if let Some(buf) = self.buf.take() {
            self.pool.give_back(buf);
        }
    }
}

/// A parsed attempt record.
pub(crate) struct ParsedAttempt {
    pub index: u32,
    pub status: u16,
    pub service_request_id: String,
    pub request_charge: f32,
    pub sub_status: Option<u16>,
    pub request_sent: String,
    pub start_ns: u64,
    pub duration_ns: u64,
}

/// A parsed fan-out child record.
pub(crate) struct ParsedChild {
    pub plan_node_id: String,
    pub feed_range: String,
    pub start_ns: u64,
    pub duration_ns: u64,
}

/// A fully parsed capture log, ready to reduce to a summary or project to a wire tree.
pub(crate) struct Parsed {
    pub operation: String,
    pub endpoint: String,
    pub client_request_id: String,
    pub attempts: Vec<ParsedAttempt>,
    pub children: Vec<ParsedChild>,
    pub outcome: Outcome,
    pub attempt_count: u32,
    pub total_ns: u64,
}

/// Parses a raw capture log buffer. Tolerant of truncation (a cancelled op may lack the `End`).
pub(crate) fn parse(buf: &[u8]) -> Parsed {
    let mut p = Parsed {
        operation: String::new(),
        endpoint: String::new(),
        client_request_id: String::new(),
        attempts: Vec::new(),
        children: Vec::new(),
        outcome: Outcome::Success,
        attempt_count: 0,
        total_ns: 0,
    };
    let mut pos = 0usize;
    while pos < buf.len() {
        let tag = buf[pos];
        pos += 1;
        match tag {
            t if t == Tag::Op as u8 => {
                pos += 1; // preamble id (single global)
                p.operation = wire::read_str(buf, &mut pos).unwrap_or_default();
                p.endpoint = wire::read_str(buf, &mut pos).unwrap_or_default();
                p.client_request_id = wire::read_str(buf, &mut pos).unwrap_or_default();
            }
            t if t == Tag::Attempt as u8 => {
                let index = wire::read_varint(buf, &mut pos)
                    .unwrap_or(0)
                    .min(u64::from(u32::MAX)) as u32;
                let status = wire::read_varint(buf, &mut pos)
                    .unwrap_or(0)
                    .min(u64::from(u16::MAX)) as u16;
                let service_request_id = wire::read_str(buf, &mut pos).unwrap_or_default();
                let mut ru_bytes = [0u8; 4];
                if let Some(slice) = buf.get(pos..pos + 4) {
                    ru_bytes.copy_from_slice(slice);
                }
                pos += 4;
                let request_charge = f32::from_le_bytes(ru_bytes);
                let start_ns = wire::read_varint(buf, &mut pos).unwrap_or(0);
                let duration_ns = wire::read_varint(buf, &mut pos).unwrap_or(0);
                let raw_sub = wire::read_varint(buf, &mut pos).unwrap_or(0);
                let sub_status =
                    (raw_sub != 0).then(|| (raw_sub - 1).min(u64::from(u16::MAX)) as u16);
                let request_sent = wire::read_str(buf, &mut pos).unwrap_or_default();
                p.attempts.push(ParsedAttempt {
                    index,
                    status,
                    service_request_id,
                    request_charge,
                    sub_status,
                    request_sent,
                    start_ns,
                    duration_ns,
                });
            }
            t if t == Tag::Child as u8 => {
                let plan_node_id = wire::read_str(buf, &mut pos).unwrap_or_default();
                let feed_range = wire::read_str(buf, &mut pos).unwrap_or_default();
                let start_ns = wire::read_varint(buf, &mut pos).unwrap_or(0);
                let duration_ns = wire::read_varint(buf, &mut pos).unwrap_or(0);
                p.children.push(ParsedChild {
                    plan_node_id,
                    feed_range,
                    start_ns,
                    duration_ns,
                });
            }
            t if t == Tag::End as u8 => {
                p.outcome = if buf.get(pos).copied() == Some(1) {
                    Outcome::Error
                } else {
                    Outcome::Success
                };
                pos += 1;
                p.attempt_count = wire::read_varint(buf, &mut pos)
                    .unwrap_or(0)
                    .min(u64::from(u32::MAX)) as u32;
                p.total_ns = wire::read_varint(buf, &mut pos).unwrap_or(0);
            }
            _ => break,
        }
    }
    p
}
