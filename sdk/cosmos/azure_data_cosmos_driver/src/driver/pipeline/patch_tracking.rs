// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Persisted idempotency markers for client-side PATCH operations.

pub(crate) use crate::models::PATCH_TRACKING_PROPERTY;
#[cfg(test)]
use crate::models::PATCH_TRACKING_RETENTION;
use azure_core::http::StatusCode;
use serde_json::{Map, Value};
use std::num::{NonZeroU16, NonZeroU32};
use uuid::Uuid;

pub(crate) const PATCH_TRACKING_POINTER: &str = "/_azsdkPatchTracking";

const TRACKING_ID_FIELD: &str = "trackingId";
const ATTEMPTED_AT_FIELD: &str = "attemptedAt";
const RETENTION_SECONDS_FIELD: &str = "retentionSeconds";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TrackingMarkerOutcome {
    AlreadyApplied,
    Added,
    Missing,
}

/// Checks for `tracking_id` and optionally appends a marker to `document`.
///
/// Existing entries are validated before any mutation. A matching ID wins even
/// when its timestamp is old: expiration only makes an entry eligible for
/// pruning by a later operation; it never weakens a marker that is still
/// present. When insertion is allowed, entries at least
/// `retention_seconds` old are removed before enforcing `capacity`, using the
/// item's service-managed `_ts` as the clock.
pub(crate) fn prepare_tracking_marker(
    document: &mut Value,
    tracking_id: Uuid,
    capacity: NonZeroU16,
    retention_seconds: NonZeroU32,
    allow_insert: bool,
) -> crate::error::Result<TrackingMarkerOutcome> {
    let document_timestamp = document
        .get("_ts")
        .and_then(Value::as_i64)
        .filter(|timestamp| *timestamp >= 0);
    let require_document_timestamp =
        || document_timestamp.ok_or_else(missing_document_timestamp_error);
    let object = document.as_object_mut().ok_or_else(|| {
        invalid_property_error("PATCH tracking requires the item body to be a JSON object")
    })?;

    let Some(property) = object.get_mut(PATCH_TRACKING_PROPERTY) else {
        if !allow_insert {
            return Ok(TrackingMarkerOutcome::Missing);
        }
        let marker_timestamp = require_document_timestamp()?;
        object.insert(
            PATCH_TRACKING_PROPERTY.to_owned(),
            Value::Array(vec![new_entry(
                tracking_id,
                marker_timestamp,
                retention_seconds,
            )]),
        );
        return Ok(TrackingMarkerOutcome::Added);
    };

    let entries = property.as_array_mut().ok_or_else(|| {
        invalid_property_error(format!(
            "reserved PATCH tracking property '{PATCH_TRACKING_PROPERTY}' must be an array"
        ))
    })?;
    let mut parsed = entries
        .iter()
        .enumerate()
        .map(parse_entry)
        .collect::<crate::error::Result<Vec<_>>>()?;

    if parsed.iter().any(|entry| entry.tracking_id == tracking_id) {
        return Ok(TrackingMarkerOutcome::AlreadyApplied);
    }
    if !allow_insert {
        return Ok(TrackingMarkerOutcome::Missing);
    }

    let document_timestamp = require_document_timestamp()?;
    for (entry, parsed_entry) in entries.iter_mut().zip(&mut parsed) {
        if parsed_entry.attempted_at > document_timestamp {
            set_attempted_at(entry, document_timestamp);
            parsed_entry.attempted_at = document_timestamp;
        }
    }
    if let (Some(entry), Some(parsed_entry)) = (entries.last_mut(), parsed.last_mut()) {
        if parsed_entry.attempted_at < document_timestamp {
            set_attempted_at(entry, document_timestamp);
            parsed_entry.attempted_at = document_timestamp;
        }
    }

    for index in (0..entries.len()).rev() {
        // `_ts` has second-level precision. A strict comparison guarantees
        // the full retention interval elapsed even when the marker committed
        // near the end of its timestamp second.
        let cutoff =
            document_timestamp.saturating_sub(i64::from(parsed[index].retention_seconds.get()));
        if parsed[index].attempted_at < cutoff {
            entries.remove(index);
        }
    }

    let retained_before_insert = usize::from(capacity.get()) - 1;
    let eviction_count = entries.len().saturating_sub(retained_before_insert);
    entries.drain(..eviction_count);

    entries.push(new_entry(
        tracking_id,
        document_timestamp,
        retention_seconds,
    ));
    Ok(TrackingMarkerOutcome::Added)
}

struct ParsedEntry {
    tracking_id: Uuid,
    attempted_at: i64,
    retention_seconds: NonZeroU32,
}

fn parse_entry((index, value): (usize, &Value)) -> crate::error::Result<ParsedEntry> {
    let object = value.as_object().ok_or_else(|| {
        invalid_property_error(format!(
            "entry {index} in reserved PATCH tracking property '{PATCH_TRACKING_PROPERTY}' must be an object"
        ))
    })?;
    let tracking_id = object
        .get(TRACKING_ID_FIELD)
        .and_then(Value::as_str)
        .ok_or_else(|| {
            invalid_property_error(format!(
                "entry {index} in reserved PATCH tracking property '{PATCH_TRACKING_PROPERTY}' must contain a string '{TRACKING_ID_FIELD}'"
            ))
        })?
        .parse::<Uuid>()
        .map_err(|error| {
            invalid_property_error(format!(
                "entry {index} in reserved PATCH tracking property '{PATCH_TRACKING_PROPERTY}' has an invalid UUID in '{TRACKING_ID_FIELD}': {error}"
            ))
        })?;
    let attempted_at = object
        .get(ATTEMPTED_AT_FIELD)
        .and_then(Value::as_i64)
        .filter(|timestamp| *timestamp >= 0)
        .ok_or_else(|| {
            invalid_property_error(format!(
                "entry {index} in reserved PATCH tracking property '{PATCH_TRACKING_PROPERTY}' must contain a non-negative integer '{ATTEMPTED_AT_FIELD}'"
            ))
        })?;
    let retention_seconds = object
        .get(RETENTION_SECONDS_FIELD)
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .and_then(NonZeroU32::new)
        .ok_or_else(|| {
            invalid_property_error(format!(
                "entry {index} in reserved PATCH tracking property '{PATCH_TRACKING_PROPERTY}' must contain a positive integer '{RETENTION_SECONDS_FIELD}' no greater than {}",
                u32::MAX
            ))
        })?;

    Ok(ParsedEntry {
        tracking_id,
        attempted_at,
        retention_seconds,
    })
}

fn new_entry(tracking_id: Uuid, attempted_at: i64, retention_seconds: NonZeroU32) -> Value {
    let mut entry = Map::new();
    entry.insert(
        TRACKING_ID_FIELD.to_owned(),
        Value::String(tracking_id.to_string()),
    );
    entry.insert(ATTEMPTED_AT_FIELD.to_owned(), Value::from(attempted_at));
    entry.insert(
        RETENTION_SECONDS_FIELD.to_owned(),
        Value::from(retention_seconds.get()),
    );
    Value::Object(entry)
}

#[cfg(test)]
fn default_retention_seconds() -> NonZeroU32 {
    let seconds = u32::try_from(PATCH_TRACKING_RETENTION.as_secs())
        .expect("default PATCH tracking retention fits in u32 seconds");
    NonZeroU32::new(seconds).expect("default PATCH tracking retention is non-zero")
}

fn set_attempted_at(entry: &mut Value, attempted_at: i64) {
    entry
        .as_object_mut()
        .expect("tracking entry was validated as an object")
        .insert(ATTEMPTED_AT_FIELD.to_owned(), Value::from(attempted_at));
}

fn missing_document_timestamp_error() -> crate::error::CosmosError {
    invalid_property_error("PATCH tracking requires a non-negative integer item '_ts'")
}

fn invalid_property_error(message: impl Into<String>) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::new(StatusCode::BadRequest))
        .with_message(message.into())
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::DEFAULT_PATCH_TRACKING_CAPACITY;
    use serde_json::json;

    const NOW: i64 = 10_000;

    fn id(value: u128) -> Uuid {
        Uuid::from_u128(value)
    }

    fn entry(tracking_id: Uuid, attempted_at: i64) -> Value {
        json!({
            TRACKING_ID_FIELD: tracking_id.to_string(),
            ATTEMPTED_AT_FIELD: attempted_at,
            RETENTION_SECONDS_FIELD: default_retention_seconds().get(),
        })
    }

    fn entry_with_retention(tracking_id: Uuid, attempted_at: i64, retention_seconds: u32) -> Value {
        json!({
            TRACKING_ID_FIELD: tracking_id.to_string(),
            ATTEMPTED_AT_FIELD: attempted_at,
            RETENTION_SECONDS_FIELD: retention_seconds,
        })
    }

    #[test]
    fn inserts_marker_and_detects_it_without_duplication() {
        let tracking_id = id(1);
        let mut document = json!({"id": "item", "_ts": NOW});

        assert_eq!(
            prepare_tracking_marker(
                &mut document,
                tracking_id,
                DEFAULT_PATCH_TRACKING_CAPACITY,
                default_retention_seconds(),
                true,
            )
            .unwrap(),
            TrackingMarkerOutcome::Added
        );
        let after_insert = document.clone();
        assert_eq!(
            prepare_tracking_marker(
                &mut document,
                tracking_id,
                DEFAULT_PATCH_TRACKING_CAPACITY,
                default_retention_seconds(),
                true,
            )
            .unwrap(),
            TrackingMarkerOutcome::AlreadyApplied
        );
        assert_eq!(document, after_insert);
    }

    #[test]
    fn verification_only_reports_missing_without_mutating_document() {
        let mut document = json!({"id": "item"});
        let before = document.clone();

        assert_eq!(
            prepare_tracking_marker(
                &mut document,
                id(1),
                DEFAULT_PATCH_TRACKING_CAPACITY,
                default_retention_seconds(),
                false,
            )
            .unwrap(),
            TrackingMarkerOutcome::Missing
        );
        assert_eq!(document, before);
    }

    #[test]
    fn second_granularity_prunes_only_after_full_retention() {
        let retention = PATCH_TRACKING_RETENTION.as_secs() as i64;
        let boundary = entry(id(2), NOW - retention);
        let young = entry(id(2), NOW - retention + 1);
        let mut future = entry(id(3), NOW + 60);
        future
            .as_object_mut()
            .unwrap()
            .insert("futureField".to_owned(), json!(true));
        let mut document = json!({
            "_ts": NOW,
            PATCH_TRACKING_PROPERTY: [
                entry(id(1), NOW - retention - 1),
                boundary.clone(),
                young.clone(),
                future.clone(),
            ]
        });

        prepare_tracking_marker(
            &mut document,
            id(4),
            NonZeroU16::new(4).unwrap(),
            default_retention_seconds(),
            true,
        )
        .unwrap();

        let entries = document[PATCH_TRACKING_PROPERTY].as_array().unwrap();
        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0], boundary);
        assert_eq!(entries[1], young);
        assert_eq!(entries[2][TRACKING_ID_FIELD], id(3).to_string());
        assert_eq!(entries[2][ATTEMPTED_AT_FIELD], NOW);
        assert_eq!(entries[2]["futureField"], true);
        assert_eq!(entries[3][TRACKING_ID_FIELD], id(4).to_string());
    }

    #[test]
    fn configurable_retention_uses_whole_seconds() {
        let mut document = json!({
            "_ts": NOW,
            PATCH_TRACKING_PROPERTY: [
                entry_with_retention(id(1), NOW - 15, 10),
                entry_with_retention(id(2), NOW - 15, 20),
                entry(id(3), NOW),
            ]
        });
        prepare_tracking_marker(
            &mut document,
            id(4),
            NonZeroU16::new(4).unwrap(),
            NonZeroU32::new(30).unwrap(),
            true,
        )
        .unwrap();

        let entries = document[PATCH_TRACKING_PROPERTY].as_array().unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0][TRACKING_ID_FIELD], id(2).to_string());
        assert_eq!(entries[1][TRACKING_ID_FIELD], id(3).to_string());
        assert_eq!(entries[2][TRACKING_ID_FIELD], id(4).to_string());
        assert_eq!(entries[2][RETENTION_SECONDS_FIELD], 30);
    }

    #[test]
    fn over_capacity_list_evicts_oldest_entries_in_one_prefix() {
        let mut document = json!({
            "_ts": NOW,
            PATCH_TRACKING_PROPERTY: [
                entry(id(1), NOW),
                entry(id(2), NOW),
                entry(id(3), NOW),
                entry(id(4), NOW),
            ]
        });
        let outcome = prepare_tracking_marker(
            &mut document,
            id(5),
            NonZeroU16::new(2).unwrap(),
            default_retention_seconds(),
            true,
        )
        .unwrap();

        assert_eq!(outcome, TrackingMarkerOutcome::Added);
        let entries = document[PATCH_TRACKING_PROPERTY].as_array().unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0][TRACKING_ID_FIELD], id(4).to_string());
        assert_eq!(entries[1][TRACKING_ID_FIELD], id(5).to_string());
    }

    #[test]
    fn tracking_pointer_matches_reserved_property() {
        assert_eq!(
            PATCH_TRACKING_POINTER,
            format!("/{PATCH_TRACKING_PROPERTY}")
        );
    }

    #[test]
    fn matching_id_wins_even_when_entry_is_expired_or_list_exceeds_cap() {
        let tracking_id = id(1);
        let mut document = json!({
            PATCH_TRACKING_PROPERTY: [entry(tracking_id, 0), entry(id(2), NOW)]
        });

        assert_eq!(
            prepare_tracking_marker(
                &mut document,
                tracking_id,
                NonZeroU16::new(1).unwrap(),
                default_retention_seconds(),
                true,
            )
            .unwrap(),
            TrackingMarkerOutcome::AlreadyApplied
        );
    }

    #[test]
    fn malformed_reserved_property_fails_instead_of_overwriting_evidence() {
        for property in [
            json!("not-an-array"),
            json!(["not-an-object"]),
            json!([{TRACKING_ID_FIELD: "not-a-uuid", ATTEMPTED_AT_FIELD: NOW}]),
            json!([{TRACKING_ID_FIELD: id(1).to_string(), ATTEMPTED_AT_FIELD: -1}]),
            json!([{
                TRACKING_ID_FIELD: id(1).to_string(),
                ATTEMPTED_AT_FIELD: NOW
            }]),
            json!([{
                TRACKING_ID_FIELD: id(1).to_string(),
                ATTEMPTED_AT_FIELD: NOW,
                RETENTION_SECONDS_FIELD: 0
            }]),
        ] {
            let mut document = json!({PATCH_TRACKING_PROPERTY: property});
            let error = prepare_tracking_marker(
                &mut document,
                id(2),
                DEFAULT_PATCH_TRACKING_CAPACITY,
                default_retention_seconds(),
                true,
            )
            .unwrap_err();
            assert_eq!(error.status().status_code(), StatusCode::BadRequest);
        }
    }

    #[test]
    fn future_timestamp_is_clamped_to_item_time_before_pruning() {
        let mut document = json!({
            "_ts": NOW,
            PATCH_TRACKING_PROPERTY: [entry(id(1), NOW + 365 * 24 * 60 * 60)]
        });

        prepare_tracking_marker(
            &mut document,
            id(2),
            NonZeroU16::new(2).unwrap(),
            default_retention_seconds(),
            true,
        )
        .unwrap();

        let entries = document[PATCH_TRACKING_PROPERTY].as_array().unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0][TRACKING_ID_FIELD], id(1).to_string());
        assert_eq!(entries[0][ATTEMPTED_AT_FIELD], NOW);
        assert_eq!(entries[1][TRACKING_ID_FIELD], id(2).to_string());
    }

    #[test]
    fn newest_timestamp_is_promoted_to_item_commit_time() {
        let mut document = json!({
            "_ts": NOW,
            PATCH_TRACKING_PROPERTY: [entry(id(1), 0)]
        });

        prepare_tracking_marker(
            &mut document,
            id(2),
            NonZeroU16::new(2).unwrap(),
            default_retention_seconds(),
            true,
        )
        .unwrap();

        let entries = document[PATCH_TRACKING_PROPERTY].as_array().unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0][ATTEMPTED_AT_FIELD], NOW);
    }

    #[test]
    fn later_item_timestamp_does_not_refresh_older_markers() {
        let older_timestamp = NOW - 100;
        let newest_timestamp = NOW - 50;
        let mut document = json!({
            "_ts": NOW,
            PATCH_TRACKING_PROPERTY: [
                entry(id(1), older_timestamp),
                entry(id(2), newest_timestamp),
            ]
        });

        prepare_tracking_marker(
            &mut document,
            id(3),
            NonZeroU16::new(3).unwrap(),
            default_retention_seconds(),
            true,
        )
        .unwrap();

        let entries = document[PATCH_TRACKING_PROPERTY].as_array().unwrap();
        assert_eq!(entries[0][ATTEMPTED_AT_FIELD], older_timestamp);
        assert_eq!(entries[1][ATTEMPTED_AT_FIELD], NOW);
        assert_eq!(entries[2][ATTEMPTED_AT_FIELD], NOW);
    }

    #[test]
    fn document_timestamp_prunes_without_response_date() {
        let mut document = json!({
            "_ts": NOW,
            PATCH_TRACKING_PROPERTY: [entry(id(1), 0), entry(id(2), NOW)]
        });

        prepare_tracking_marker(
            &mut document,
            id(3),
            NonZeroU16::new(2).unwrap(),
            default_retention_seconds(),
            true,
        )
        .unwrap();

        let entries = document[PATCH_TRACKING_PROPERTY].as_array().unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0][TRACKING_ID_FIELD], id(2).to_string());
        assert_eq!(entries[1][TRACKING_ID_FIELD], id(3).to_string());
    }

    #[test]
    fn insertion_requires_document_timestamp() {
        let mut document = json!({"id": "item"});

        let error = prepare_tracking_marker(
            &mut document,
            id(1),
            DEFAULT_PATCH_TRACKING_CAPACITY,
            default_retention_seconds(),
            true,
        )
        .unwrap_err();

        assert_eq!(error.status().status_code(), StatusCode::BadRequest);
        assert!(error.to_string().contains("'_ts'"));
    }
}
