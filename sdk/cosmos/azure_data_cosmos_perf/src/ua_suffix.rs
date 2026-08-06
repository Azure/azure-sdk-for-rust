// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Composes the User-Agent suffix sent on every Cosmos request.
//!
//! The suffix is `{configured}-{first-8-chars-of-workload_id}` so a
//! single perf run's traffic is correlatable in server-side telemetry
//! with that run's client-side logs (which already carry `workload_id`).
//!
//! If the composite would exceed [`UserAgentSuffix::MAX_LENGTH`] the
//! configured-suffix portion is truncated at a UTF-8 char boundary so
//! the `-<id>` tail is always preserved — the per-run identifier wins,
//! the vanity text loses.

use azure_data_cosmos::options::UserAgentSuffix;

/// Number of leading characters of `workload_id` to keep in the composite.
const WORKLOAD_ID_CHARS: usize = 8;

/// Build a composite suffix from a user-supplied base and a workload id.
///
/// Returns `None` when:
/// * `configured` is empty (user explicitly opted out of any suffix), or
/// * `workload_id` is empty (nothing to append; the bare configured
///   suffix is returned via the second arm of [`resolve_user_agent_suffix`]).
///
/// The returned string is guaranteed to be at most
/// [`UserAgentSuffix::MAX_LENGTH`] characters long and never split on a
/// multi-byte UTF-8 codepoint.
pub fn compose_suffix(configured: &str, workload_id: &str) -> Option<String> {
    if configured.is_empty() || workload_id.is_empty() {
        return None;
    }

    let max = UserAgentSuffix::MAX_LENGTH;

    let id_part: String = workload_id.chars().take(WORKLOAD_ID_CHARS).collect();
    // `-<id_part>` — the tail that always survives.
    let tail_len_chars = 1 + id_part.chars().count();
    if tail_len_chars > max {
        // Pathological: workload_id alone (plus the separator) doesn't fit.
        // Drop the configured portion and truncate the id.
        let id_room = max.saturating_sub(1);
        let truncated_id: String = id_part.chars().take(id_room).collect();
        return Some(format!("-{truncated_id}"));
    }

    let room_for_configured = max - tail_len_chars;
    let truncated_configured: String = configured.chars().take(room_for_configured).collect();
    Some(format!("{truncated_configured}-{id_part}"))
}

/// Resolve a validated `UserAgentSuffix` from the configured and workload-id
/// inputs.
///
/// The raw `configured` value is validated up front via
/// [`UserAgentSuffix::try_new`], so operator input errors (over-length or
/// HTTP-unsafe) surface immediately — before any network activity — rather
/// than being silently truncated during composition. Only an
/// *already-valid* configured value is then composed with the workload-id
/// tail via [`compose_suffix`], which truncates the configured portion as
/// needed so the `-<id>` tail always fits.
///
/// Returns `Ok(None)` when the user opted out (empty configured suffix).
pub fn resolve_user_agent_suffix(
    configured: &str,
    workload_id: &str,
) -> Result<Option<UserAgentSuffix>, String> {
    if configured.is_empty() {
        return Ok(None);
    }

    // Reject invalid operator input before composing (and before any network
    // call in `main`). Silently truncating an over-length raw suffix would
    // hide a mistake the operator almost certainly wants to know about.
    let base = UserAgentSuffix::try_new(configured.to_string()).ok_or_else(|| {
        format!(
            "--user-agent-suffix {configured:?} is invalid (must be \u{2264} {} ASCII \
             characters and HTTP-header-safe)",
            UserAgentSuffix::MAX_LENGTH
        )
    })?;

    if let Some(composite) = compose_suffix(configured, workload_id) {
        if let Some(s) = UserAgentSuffix::try_new(composite.clone()) {
            return Ok(Some(s));
        }
        // Composite failed validation (e.g. configured contained an HTTP-unsafe
        // byte we couldn't see at compose time). Fall back to the base so the
        // run isn't blocked, but report the issue.
        eprintln!(
            "warning: composite user-agent suffix {composite:?} failed validation; \
             falling back to base {configured:?}"
        );
    }

    Ok(Some(base))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX: usize = UserAgentSuffix::MAX_LENGTH;

    #[test]
    fn happy_path_default_suffix_and_uuid() {
        // "rust-perf" (9) + "-" + 8 = 18 chars, comfortably under MAX (25).
        let composite =
            compose_suffix("rust-perf", "9e107d9d-372b-b6cd-1234-567890abcdef").unwrap();
        assert_eq!(composite, "rust-perf-9e107d9d");
        assert!(composite.chars().count() <= MAX);
    }

    #[test]
    fn empty_configured_returns_none() {
        assert!(compose_suffix("", "9e107d9d").is_none());
    }

    #[test]
    fn empty_workload_id_returns_none() {
        assert!(compose_suffix("rust-perf", "").is_none());
    }

    #[test]
    fn oversized_configured_truncates_so_tail_survives() {
        // 30-char configured base; MAX is 25; -<8 char id> = 9 char tail.
        let composite = compose_suffix("a".repeat(30).as_str(), "deadbeef12").unwrap();
        // Total length must respect the cap.
        assert!(composite.chars().count() <= MAX);
        // Tail must be preserved verbatim.
        assert!(composite.ends_with("-deadbeef"));
        // What remains of the configured portion is just 'a's.
        assert!(composite.starts_with("aaaa"));
    }

    #[test]
    fn multi_byte_configured_never_splits_codepoint() {
        // 12 × 3-byte emoji = 36 bytes, 12 chars; will be truncated to
        // fit MAX - tail. Composite must remain valid UTF-8 and never
        // end on a partial codepoint.
        let configured: String = "\u{1F600}".repeat(12);
        let composite = compose_suffix(&configured, "deadbeef").unwrap();
        // Round-trip validates UTF-8 well-formed bytes.
        let _ = std::str::from_utf8(composite.as_bytes()).expect("valid UTF-8");
        assert!(composite.ends_with("-deadbeef"));
    }

    #[test]
    fn resolve_empty_configured_returns_ok_none() {
        let out = resolve_user_agent_suffix("", "any-workload-id").unwrap();
        assert!(out.is_none());
    }

    #[test]
    fn resolve_happy_path() {
        let out = resolve_user_agent_suffix("rust-perf", "deadbeef-cafe").unwrap();
        let suffix = out.unwrap();
        assert_eq!(suffix.as_str(), "rust-perf-deadbeef");
    }

    #[test]
    fn resolve_rejects_oversized_configured_before_compose() {
        // A raw suffix longer than MAX must be rejected outright rather than
        // silently truncated during composition — even with a non-empty
        // workload_id that would otherwise let compose_suffix salvage it.
        let too_long = "a".repeat(MAX + 1);
        let err = resolve_user_agent_suffix(&too_long, "deadbeef-cafe")
            .expect_err("oversized configured suffix must be rejected");
        assert!(err.contains("--user-agent-suffix"), "got: {err}");
        assert!(err.contains("invalid"), "got: {err}");
    }

    #[test]
    fn resolve_rejects_http_unsafe_configured() {
        // HTTP-unsafe characters are rejected up front as well.
        let err = resolve_user_agent_suffix("bad suffix!", "deadbeef")
            .expect_err("unsafe configured suffix must be rejected");
        assert!(err.contains("--user-agent-suffix"), "got: {err}");
        assert!(err.contains("invalid"), "got: {err}");
    }
}
