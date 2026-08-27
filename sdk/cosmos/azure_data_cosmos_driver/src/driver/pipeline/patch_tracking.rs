// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Persisted idempotency markers for client-side PATCH operations.

pub(crate) use crate::models::PATCH_TRACKING_PROPERTY;
use crate::models::PATCH_TRACKING_RETENTION;
use azure_core::http::StatusCode;
use serde_json::{Map, Value};
use std::num::NonZeroU16;
use uuid::Uuid;

pub(crate) const PATCH_TRACKING_POINTER: &str = "/_azsdkPatchTracking";

const TRACKING_ID_FIELD: &str = "trackingId";
const ATTEMPTED_AT_FIELD: &str = "attemptedAt";

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
/// [`PATCH_TRACKING_RETENTION`] old are removed before enforcing `capacity`,
/// but only when an authoritative service response time is available.
pub(crate) fn prepare_tracking_marker(
    document: &mut Value,
    tracking_id: Uuid,
    service_time_unix_seconds: Option<i64>,
    capacity: NonZeroU16,
    allow_insert: bool,
) -> crate::error::Result<TrackingMarkerOutcome> {
    let document_timestamp = document
        .get("_ts")
        .and_then(Value::as_i64)
        .filter(|timestamp| *timestamp >= 0);
    let marker_timestamp = service_time_unix_seconds
        .into_iter()
        .chain(document_timestamp)
        .max()
        .unwrap_or_else(|| time::OffsetDateTime::now_utc().unix_timestamp().max(0));
    let object = document.as_object_mut().ok_or_else(|| {
        invalid_property_error("PATCH tracking requires the item body to be a JSON object")
    })?;

    let Some(property) = object.get_mut(PATCH_TRACKING_PROPERTY) else {
        if !allow_insert {
            return Ok(TrackingMarkerOutcome::Missing);
        }
        object.insert(
            PATCH_TRACKING_PROPERTY.to_owned(),
            Value::Array(vec![new_entry(tracking_id, marker_timestamp)]),
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

    if let Some(document_timestamp) = document_timestamp {
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
    }

    if let Some(now_unix_seconds) = service_time_unix_seconds {
        let retention_seconds = i64::try_from(PATCH_TRACKING_RETENTION.as_secs())
            .expect("PATCH tracking retention fits in i64 seconds");
        let cutoff = now_unix_seconds.saturating_sub(retention_seconds);
        for index in (0..entries.len()).rev() {
            if parsed[index].attempted_at <= cutoff {
                entries.remove(index);
            }
        }
    }

    if entries.len() >= usize::from(capacity.get()) {
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::new(StatusCode::Conflict))
            .with_message(format!(
                "PATCH tracking capacity {} is exhausted by unexpired entries in reserved property '{PATCH_TRACKING_PROPERTY}'; increase the tracking capacity or retry after the retention window",
                capacity.get()
            ))
            .build());
    }

    entries.push(new_entry(tracking_id, marker_timestamp));
    Ok(TrackingMarkerOutcome::Added)
}

struct ParsedEntry {
    tracking_id: Uuid,
    attempted_at: i64,
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

    Ok(ParsedEntry {
        tracking_id,
        attempted_at,
    })
}

fn new_entry(tracking_id: Uuid, attempted_at: i64) -> Value {
    let mut entry = Map::new();
    entry.insert(
        TRACKING_ID_FIELD.to_owned(),
        Value::String(tracking_id.to_string()),
    );
    entry.insert(ATTEMPTED_AT_FIELD.to_owned(), Value::from(attempted_at));
    Value::Object(entry)
}

fn set_attempted_at(entry: &mut Value, attempted_at: i64) {
    entry
        .as_object_mut()
        .expect("tracking entry was validated as an object")
        .insert(ATTEMPTED_AT_FIELD.to_owned(), Value::from(attempted_at));
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
        })
    }

    #[test]
    fn inserts_marker_and_detects_it_without_duplication() {
        let tracking_id = id(1);
        let mut document = json!({"id": "item"});

        assert_eq!(
            prepare_tracking_marker(
                &mut document,
                tracking_id,
                Some(NOW),
                DEFAULT_PATCH_TRACKING_CAPACITY,
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
                Some(NOW + 1),
                DEFAULT_PATCH_TRACKING_CAPACITY,
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
                Some(NOW),
                DEFAULT_PATCH_TRACKING_CAPACITY,
                false,
            )
            .unwrap(),
            TrackingMarkerOutcome::Missing
        );
        assert_eq!(document, before);
    }

    #[test]
    fn prunes_only_expired_entries_and_preserves_unknown_fields() {
        let retention = PATCH_TRACKING_RETENTION.as_secs() as i64;
        let young = entry(id(2), NOW - retention + 1);
        let mut future = entry(id(3), NOW + 60);
        future
            .as_object_mut()
            .unwrap()
            .insert("futureField".to_owned(), json!(true));
        let mut document = json!({
            PATCH_TRACKING_PROPERTY: [
                entry(id(1), NOW - retention),
                young.clone(),
                future.clone(),
            ]
        });

        prepare_tracking_marker(
            &mut document,
            id(4),
            Some(NOW),
            NonZeroU16::new(3).unwrap(),
            true,
        )
        .unwrap();

        let entries = document[PATCH_TRACKING_PROPERTY].as_array().unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0], young);
        assert_eq!(entries[1], future);
        assert_eq!(entries[2][TRACKING_ID_FIELD], id(4).to_string());
    }

    #[test]
    fn full_unexpired_list_fails_without_evicting_an_entry() {
        let mut document = json!({
            PATCH_TRACKING_PROPERTY: [entry(id(1), NOW), entry(id(2), NOW)]
        });
        let before = document.clone();

        let error = prepare_tracking_marker(
            &mut document,
            id(3),
            Some(NOW),
            NonZeroU16::new(2).unwrap(),
            true,
        )
        .unwrap_err();

        assert_eq!(error.status().status_code(), StatusCode::Conflict);
        assert!(error.to_string().contains("capacity 2 is exhausted"));
        assert_eq!(document, before);
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
                Some(NOW),
                NonZeroU16::new(1).unwrap(),
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
        ] {
            let mut document = json!({PATCH_TRACKING_PROPERTY: property});
            let error = prepare_tracking_marker(
                &mut document,
                id(2),
                Some(NOW),
                DEFAULT_PATCH_TRACKING_CAPACITY,
                true,
            )
            .unwrap_err();
            assert_eq!(error.status().status_code(), StatusCode::BadRequest);
        }
    }

    #[test]
    fn future_timestamp_is_clamped_to_item_time_before_pruning() {
        let retention = PATCH_TRACKING_RETENTION.as_secs() as i64;
        let mut document = json!({
            "_ts": NOW,
            PATCH_TRACKING_PROPERTY: [entry(id(1), NOW + 365 * 24 * 60 * 60)]
        });

        prepare_tracking_marker(
            &mut document,
            id(2),
            Some(NOW + retention + 1),
            NonZeroU16::new(1).unwrap(),
            true,
        )
        .unwrap();

        let entries = document[PATCH_TRACKING_PROPERTY].as_array().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0][TRACKING_ID_FIELD], id(2).to_string());
    }

    #[test]
    fn newest_timestamp_is_promoted_to_item_commit_time() {
        let retention = PATCH_TRACKING_RETENTION.as_secs() as i64;
        let mut document = json!({
            "_ts": NOW,
            PATCH_TRACKING_PROPERTY: [entry(id(1), 0)]
        });

        prepare_tracking_marker(
            &mut document,
            id(2),
            Some(NOW + retention - 1),
            NonZeroU16::new(2).unwrap(),
            true,
        )
        .unwrap();

        let entries = document[PATCH_TRACKING_PROPERTY].as_array().unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0][ATTEMPTED_AT_FIELD], NOW);
    }

    #[test]
    fn missing_service_time_disables_pruning() {
        let mut document = json!({
            "_ts": NOW,
            PATCH_TRACKING_PROPERTY: [entry(id(1), 0)]
        });

        let error = prepare_tracking_marker(
            &mut document,
            id(2),
            None,
            NonZeroU16::new(1).unwrap(),
            true,
        )
        .unwrap_err();

        assert_eq!(error.status().status_code(), StatusCode::Conflict);
        assert_eq!(
            document[PATCH_TRACKING_PROPERTY][0][ATTEMPTED_AT_FIELD],
            NOW
        );
    }
}
