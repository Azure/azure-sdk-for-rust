// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    response_frame, response_header_names, unwrap_response_for_gateway_v2, write_byte_token,
    write_double_token, write_i64_token, write_small_string_token, write_string_token,
    write_u32_token, write_u64_token, ErrorKind, HeaderName, Headers, HttpResponse, Uuid,
    ACTIVITY_ID, X_MS_ACTIVITY_ID, X_MS_GLOBAL_COMMITTED_LSN, X_MS_LSN,
};

#[test]
fn unwrap_maps_response_status_headers_and_body() {
    let activity_id = Uuid::parse_str(ACTIVITY_ID).unwrap();
    let response = HttpResponse {
        status: 200,
        headers: Headers::new(),
        body: response_frame(
            404,
            activity_id,
            |tokens| {
                write_small_string_token(tokens, 0x0002, "2026-07-21T00:00:00Z");
                write_string_token(tokens, 0x000E, "documentSize=10240;");
                write_string_token(tokens, 0x000F, "documentSize=1;");
                write_small_string_token(tokens, 0x0010, "1.0");
                write_u32_token(tokens, 0x001C, 1002);
                write_u32_token(tokens, 0x0014, 1);
                write_double_token(tokens, 0x0015, 3.5);
                write_string_token(tokens, 0x0018, "owner-rid");
                write_i64_token(tokens, 0x001A, 40);
                write_u32_token(tokens, 0x001E, 3);
                write_u32_token(tokens, 0x001F, 4);
                write_u32_token(tokens, 0x0026, 1);
                write_u32_token(tokens, 0x0030, 2);
                write_i64_token(tokens, 0x003A, 41);
                write_i64_token(tokens, 0x003B, 40);
                write_i64_token(tokens, 0x003C, 39);
                write_string_token(tokens, 0x0045, "{\"reverseRidEnabled\":false}");
                write_double_token(tokens, 0x0051, 12.75);
                write_byte_token(tokens, 0x0055, 0);
                write_string_token(tokens, 0x0063, "physical-0");
                write_u64_token(tokens, 0x0087, 1_234_567);
                write_u32_token(tokens, 0x0035, 45);
                write_string_token(tokens, 0x0021, "1");
                write_string_token(tokens, 0x003E, "1:2#3");
                write_string_token(tokens, 0x0004, "\"etag\"");
                write_string_token(tokens, 0x0003, "continuation");
                write_i64_token(tokens, 0x0013, 42);
                write_i64_token(tokens, 0x0032, 43);
                write_i64_token(tokens, 0x0029, 44);
                write_string_token(tokens, 0x0017, "dbs/db1/colls/coll1/docs/doc1");
                write_string_token(tokens, 0x0028, "metrics-blob");
                write_string_token(tokens, 0x0044, "index-blob");
            },
            b"{}",
        ),
    };

    let unwrapped = unwrap_response_for_gateway_v2(response).unwrap();

    assert_eq!(unwrapped.status, 404);
    assert_eq!(unwrapped.body, b"{}".to_vec());
    assert_eq!(
        unwrapped.headers.get_optional_str(&X_MS_ACTIVITY_ID),
        Some(ACTIVITY_ID)
    );
    assert_eq!(
        unwrapped
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-substatus")),
        Some("1002")
    );
    assert_eq!(
        unwrapped
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-request-charge")),
        Some("3.5")
    );
    assert_eq!(
        unwrapped.headers.get_optional_str(&HeaderName::from_static(
            response_header_names::SERVER_DURATION_MS
        )),
        Some("12.75")
    );
    let parsed_headers = crate::models::CosmosResponseHeaders::from_headers(&unwrapped.headers);
    assert_eq!(parsed_headers.server_duration_ms, Some(12.75));
    assert_eq!(
        parsed_headers.last_state_change_utc.as_deref(),
        Some("2026-07-21T00:00:00Z")
    );
    assert_eq!(
        parsed_headers.resource_quota.as_deref(),
        Some("documentSize=10240;")
    );
    assert_eq!(
        parsed_headers.resource_usage.as_deref(),
        Some("documentSize=1;")
    );
    assert_eq!(parsed_headers.schema_version.as_deref(), Some("1.0"));
    assert_eq!(parsed_headers.item_count, Some(1));
    assert_eq!(parsed_headers.owner_id.as_deref(), Some("owner-rid"));
    assert_eq!(parsed_headers.quorum_acked_lsn, Some(40));
    assert_eq!(parsed_headers.current_write_quorum, Some(3));
    assert_eq!(parsed_headers.current_replica_set_size, Some(4));
    assert_eq!(parsed_headers.xp_role, Some(1));
    assert_eq!(parsed_headers.number_of_read_regions, Some(2));
    assert_eq!(parsed_headers.local_lsn, Some(41));
    assert_eq!(parsed_headers.quorum_acked_local_lsn, Some(40));
    assert_eq!(parsed_headers.item_local_lsn, Some(39));
    assert_eq!(
        parsed_headers.query_execution_info.as_deref(),
        Some("{\"reverseRidEnabled\":false}")
    );
    assert_eq!(parsed_headers.pending_pk_delete, Some(false));
    assert_eq!(
        parsed_headers.physical_partition_id.as_deref(),
        Some("physical-0")
    );
    assert_eq!(parsed_headers.conflict_resolved_timestamp, Some(1_234_567));
    assert_eq!(parsed_headers.transport_request_id, Some(45));
    assert_eq!(parsed_headers.partition_key_range_id.as_deref(), Some("1"));
    assert_eq!(
        unwrapped
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-session-token")),
        Some("1:2#3")
    );
    assert_eq!(
        unwrapped
            .headers
            .get_optional_str(&HeaderName::from_static("etag")),
        Some("\"etag\"")
    );
    assert_eq!(
        unwrapped
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-continuation")),
        Some("continuation")
    );
    assert_eq!(
        unwrapped
            .headers
            .get_optional_str(&HeaderName::from_static("lsn")),
        Some("42")
    );
    assert_eq!(unwrapped.headers.get_optional_str(&X_MS_LSN), Some("42"));
    assert_eq!(
        unwrapped
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-item-lsn")),
        Some("43")
    );
    assert_eq!(
        unwrapped
            .headers
            .get_optional_str(&X_MS_GLOBAL_COMMITTED_LSN),
        Some("44")
    );
    assert_eq!(
        unwrapped
            .headers
            .get_optional_str(&HeaderName::from_static(response_header_names::ITEM_COUNT)),
        Some("1")
    );
    assert_eq!(
        unwrapped.headers.get_optional_str(&HeaderName::from_static(
            response_header_names::QUERY_METRICS
        )),
        Some("metrics-blob")
    );
    assert_eq!(
        unwrapped.headers.get_optional_str(&HeaderName::from_static(
            response_header_names::INDEX_METRICS
        )),
        Some("index-blob")
    );
}

#[test]
fn unwrap_prefixes_session_token_with_partition_key_range_id() {
    let response = HttpResponse {
        status: 200,
        headers: Headers::new(),
        body: response_frame(
            200,
            Uuid::parse_str(ACTIVITY_ID).unwrap(),
            |tokens| {
                // GW2 surfaces the bare vector token and the pk range id separately.
                write_string_token(tokens, 0x0021, "0");
                write_string_token(tokens, 0x003E, "0#2#2=-1");
            },
            b"",
        ),
    };

    let unwrapped = unwrap_response_for_gateway_v2(response).unwrap();

    assert_eq!(
        unwrapped
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-session-token")),
        Some("0:0#2#2=-1")
    );
}

#[test]
fn unwrap_keeps_session_token_when_already_prefixed() {
    let response = HttpResponse {
        status: 200,
        headers: Headers::new(),
        body: response_frame(
            200,
            Uuid::parse_str(ACTIVITY_ID).unwrap(),
            |tokens| {
                write_string_token(tokens, 0x0021, "0");
                write_string_token(tokens, 0x003E, "0:0#2#2=-1");
            },
            b"",
        ),
    };

    let unwrapped = unwrap_response_for_gateway_v2(response).unwrap();

    assert_eq!(
        unwrapped
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-session-token")),
        Some("0:0#2#2=-1")
    );
}

#[test]
fn unwrap_preserves_retry_after_for_throttle() {
    let response = HttpResponse {
        status: 200,
        headers: Headers::new(),
        body: response_frame(
            429,
            Uuid::parse_str(ACTIVITY_ID).unwrap(),
            |tokens| write_u32_token(tokens, 0x000C, 125),
            b"",
        ),
    };

    let unwrapped = unwrap_response_for_gateway_v2(response).unwrap();

    assert_eq!(unwrapped.status, 429);
    assert_eq!(
        unwrapped
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-retry-after-ms")),
        Some("125")
    );
}

#[test]
fn unwrap_rejects_malformed_rntbd_body() {
    let response = HttpResponse {
        status: 200,
        headers: Headers::new(),
        body: vec![1, 2, 3],
    };

    let error = unwrap_response_for_gateway_v2(response).unwrap_err();

    assert_eq!(error.kind(), &ErrorKind::DataConversion);
}

#[test]
fn unwrap_rejects_out_of_range_inner_status() {
    let response = HttpResponse {
        status: 200,
        headers: Headers::new(),
        body: response_frame(70_000, Uuid::parse_str(ACTIVITY_ID).unwrap(), |_| {}, b""),
    };

    let error = unwrap_response_for_gateway_v2(response).unwrap_err();

    assert_eq!(error.kind(), &ErrorKind::DataConversion);
}
