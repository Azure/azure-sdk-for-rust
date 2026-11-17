// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    epk, parse_wrapped_request, request_header_names, signed_request, wrap_inputs,
    wrap_request_for_gateway_v2, AuthorizationContext, Cow, ErrorKind, HeaderName, HeaderValue,
    Method, OperationType, ParsedTokenValue, PartitionKey, PartitionKeyDefinition, ResourceType,
    ACCEPT, ACTIVITY_ID, AUTHORIZATION, CLIENT_ID, CONTENT_TYPE, USER_AGENT, X_MS_ACTIVITY_ID,
    X_MS_DATE,
};

#[test]
fn wrap_preserves_batch_controls_and_operation_payload() {
    let body = br#"[{"operationType":"Read","id":"batch"},{"operationType":"Upsert","resourceBody":{"id":"batch","pk":"A","value":2}}]"#;
    let context =
        AuthorizationContext::new(Method::Post, ResourceType::Document, "dbs/db1/colls/coll1");
    let key = epk(
        &PartitionKey::from("A"),
        &PartitionKeyDefinition::new(vec![Cow::from("/pk")]),
    );
    for atomic in [None, Some("True"), Some("false")] {
        for continue_on_error in [None, Some("true"), Some("False")] {
            for formats in ["JsonText", "JsonText,CosmosBinary"] {
                let mut request = signed_request(Some(body));
                request.method = Method::Post;
                request.url.set_path("/dbs/db1/colls/coll1/docs");
                request
                    .headers
                    .insert(request_header_names::PARTITION_KEY, r#"["A"]"#);
                request.headers.insert(
                    request_header_names::SUPPORTED_SERIALIZATION_FORMATS,
                    formats,
                );
                for (name, value) in [
                    (request_header_names::BATCH_ATOMIC, atomic),
                    (
                        request_header_names::BATCH_CONTINUE_ON_ERROR,
                        continue_on_error,
                    ),
                ] {
                    if let Some(value) = value {
                        request.headers.insert(HeaderName::from_static(name), value);
                    }
                }
                let wrapped = wrap_request_for_gateway_v2(
                    request,
                    &wrap_inputs(&context, OperationType::Batch, Some(&key)),
                )
                .unwrap();
                let parsed = parse_wrapped_request(&wrapped, 0);
                assert_eq!(parsed.operation_type, 0x0025);
                for (id, value) in [(0x0073, atomic), (0x0074, continue_on_error)] {
                    let expected = value.map(|value| {
                        ParsedTokenValue::Byte(u8::from(value.eq_ignore_ascii_case("true")))
                    });
                    assert_eq!(
                        parsed.tokens.get(&id),
                        expected.as_ref(),
                        "token {id:#06x}, {formats}"
                    );
                }
                assert_eq!(parsed.body.as_deref(), Some(body.as_slice()));
                assert!(wrapped
                    .headers
                    .get_optional_str(&HeaderName::from_static(request_header_names::BATCH_ATOMIC))
                    .is_none());
            }
        }
    }
}

#[cfg(feature = "fault_injection")]
#[test]
fn wrap_forwards_fault_injection_operation_header() {
    // Regression: the G2 wrap builds a fresh `Headers` map and used to
    // drop `x-ms-fault-injection-operation`, causing
    // `FaultInjectingHttpClient` to report `OperationMismatch` for any
    // operation-typed rule on G2 traffic even when the V1 path matched.
    use crate::models::cosmos_headers::fault_injection_header_names::FAULT_INJECTION_OPERATION;
    let mut request = signed_request(None);
    request.headers.insert(
        HeaderName::from_static(FAULT_INJECTION_OPERATION),
        "ReadItem",
    );
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::Read, None),
    )
    .unwrap();
    assert_eq!(
        wrapped
            .headers
            .get_optional_str(&HeaderName::from_static(FAULT_INJECTION_OPERATION))
            .map(|s| s.to_owned()),
        Some("ReadItem".to_owned()),
        "G2 wrap must forward the fault-injection operation header so FaultInjectingHttpClient can match operation-typed rules"
    );
}

/// End-to-end regression: the full G2 wrap pipeline produces a frame
/// whose first 4 bytes (`LengthInBytes`) cover only the header section
/// and exclude the body length prefix + body bytes. This is the exact
/// property the Gateway 2.0 proxy reads — getting it wrong is what
/// caused HTTP 400 sub-status 13007 ("error routing the request") on
/// every G2 Create request in CI. Pin it at the wrap-function boundary
/// in addition to the lower-level `RntbdRequestFrame::serialize` tests
/// so a future change to the wrap function that adds bytes between the
/// header and the body cannot regress this.
#[test]
fn wrap_writes_length_in_bytes_covering_header_only_for_create_with_payload() {
    let payload = br#"{"id":"doc1","pk":"abc","data":"hello"}"#;
    let request = signed_request(Some(payload));
    let auth_context =
        AuthorizationContext::new(Method::Post, ResourceType::Document, "dbs/db1/colls/coll1");

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::Create, None),
    )
    .unwrap();

    let bytes = wrapped.body.as_ref().unwrap().as_ref();
    let length_in_bytes = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;

    // The `LengthInBytes` field must be strictly less than the frame size
    // (body prefix + body bytes follow). If it ever equals `bytes.len()`,
    // the proxy will treat the body as additional metadata tokens and
    // fail with sub-status 13007.
    assert!(
        length_in_bytes < bytes.len(),
        "LengthInBytes ({length_in_bytes}) must exclude body section but equals total frame size ({})",
        bytes.len()
    );
    // And the body-length prefix immediately at offset = header_len must
    // match the actual payload length the proxy receives.
    let body_len = u32::from_le_bytes([
        bytes[length_in_bytes],
        bytes[length_in_bytes + 1],
        bytes[length_in_bytes + 2],
        bytes[length_in_bytes + 3],
    ]) as usize;
    assert_eq!(body_len, payload.len());
    assert_eq!(bytes.len(), length_in_bytes + 4 + payload.len());
}

#[test]
fn wrap_preserves_payload_and_sets_payload_present() {
    let request = signed_request(Some(br#"{"id":"doc1"}"#));
    let auth_context =
        AuthorizationContext::new(Method::Post, ResourceType::Document, "dbs/db1/colls/coll1");

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::Create, None),
    )
    .unwrap();
    let parsed = parse_wrapped_request(&wrapped, 9);

    assert_eq!(parsed.tokens[&0x0002], ParsedTokenValue::Byte(1));
    assert_eq!(parsed.body, Some(br#"{"id":"doc1"}"#.to_vec()));
}

#[test]
fn wrap_preserves_client_id_proxy_header_and_drops_inner_headers() {
    let mut request = signed_request(None);
    request.headers.insert(
        CLIENT_ID,
        HeaderValue::from_static("00000000-0000-4000-8000-000000000000"),
    );
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::Read, None),
    )
    .unwrap();

    assert_eq!(
        wrapped.headers.get_optional_str(&USER_AGENT),
        Some("test-agent")
    );
    assert_eq!(
        wrapped.headers.get_optional_str(&X_MS_ACTIVITY_ID),
        Some(ACTIVITY_ID)
    );
    assert_eq!(
        wrapped.headers.get_optional_str(&CLIENT_ID),
        Some("00000000-0000-4000-8000-000000000000")
    );
    assert!(wrapped.headers.get_optional_str(&AUTHORIZATION).is_none());
    assert!(wrapped.headers.get_optional_str(&X_MS_DATE).is_none());
    assert!(wrapped.headers.get_optional_str(&CONTENT_TYPE).is_none());
    assert!(wrapped.headers.get_optional_str(&ACCEPT).is_none());
}

#[test]
fn wrap_rejects_missing_authorization_header() {
    let mut request = signed_request(None);
    request.headers.remove(AUTHORIZATION);
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );

    let error = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::Read, None),
    )
    .unwrap_err();

    assert_eq!(error.kind(), &ErrorKind::DataConversion);
}

#[test]
fn wrap_rejects_missing_date_header() {
    let mut request = signed_request(None);
    request.headers.remove(X_MS_DATE);
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );

    let error = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::Read, None),
    )
    .unwrap_err();

    assert_eq!(error.kind(), &ErrorKind::DataConversion);
}

#[test]
fn wrap_rejects_invalid_activity_id() {
    let mut request = signed_request(None);
    request.headers.insert(X_MS_ACTIVITY_ID, "not-a-guid");
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );

    let error = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::Read, None),
    )
    .unwrap_err();

    assert_eq!(error.kind(), &ErrorKind::DataConversion);
}
