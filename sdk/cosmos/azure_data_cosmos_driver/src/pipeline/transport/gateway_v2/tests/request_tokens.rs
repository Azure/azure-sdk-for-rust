// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    effective_partition_key_multi_hash_v2_binary, effective_partition_key_v1_binary,
    effective_partition_key_v2_binary, epk, parse_supported_serialization_formats,
    parse_token_value, parse_wrapped_request, request_header_names, signed_request, take_u16,
    take_u32, take_u8, take_uuid, wrap_inputs, wrap_request_for_gateway_v2, AuthorizationContext,
    Cow, DefaultConsistencyLevel, EffectivePartitionKey, HeaderName, Method, OperationType,
    ParsedTokenValue, PartitionKey, PartitionKeyDefinition, PartitionKeyValue, PartitionKeyVersion,
    ResourceType, RntbdRequestFrame, Uuid, WrapInputs, ACTIVITY_ID, GATEWAY_V2_RANGE_MAX,
    GATEWAY_V2_RANGE_MIN, SUPPORTED_CAPABILITIES_BITS, X_MS_CONTINUATION,
};

#[test]
fn wrap_rejects_invalid_batch_control_values() {
    let context =
        AuthorizationContext::new(Method::Post, ResourceType::Document, "dbs/db1/colls/coll1");
    for name in [
        request_header_names::BATCH_ATOMIC,
        request_header_names::BATCH_CONTINUE_ON_ERROR,
    ] {
        for value in ["", "1", "invalid"] {
            let mut request = signed_request(Some(b"[]"));
            request.headers.insert(HeaderName::from_static(name), value);
            let error = wrap_request_for_gateway_v2(
                request,
                &wrap_inputs(&context, OperationType::Batch, None),
            )
            .expect_err("invalid controls must not silently weaken batch semantics");
            assert!(error.to_string().contains(name));
        }
    }
}

#[test]
fn wrap_omits_batch_controls_for_non_batch_operations() {
    let context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let mut request = signed_request(None);
    request
        .headers
        .insert(request_header_names::BATCH_ATOMIC, "True");
    request
        .headers
        .insert(request_header_names::BATCH_CONTINUE_ON_ERROR, "False");
    let wrapped =
        wrap_request_for_gateway_v2(request, &wrap_inputs(&context, OperationType::Read, None))
            .unwrap();
    let parsed = parse_wrapped_request(&wrapped, 0);
    assert!(!parsed.tokens.contains_key(&0x0073));
    assert!(!parsed.tokens.contains_key(&0x0074));
}

#[test]
fn wrap_emits_tokens_in_thin_client_required_order() {
    // The thin-client contract mandates the relative ordering
    // EffectivePartitionKey -> GlobalDatabaseAccountName -> PayloadPresent
    // on every thin-client request (CorrelatedActivityId is not emitted by
    // the Rust driver). The emission order is hard-coded in
    // `wrap_request_for_gateway_v2`; pin the contract here so a
    // future refactor that shuffles the token order is a compile-time
    // failure rather than a silent wire-compat break.
    let request = signed_request(None);
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let partition_key = PartitionKey::from("pk-value");
    let partition_key_definition = PartitionKeyDefinition::new(vec![Cow::from("/pk")]);

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(
            &auth_context,
            OperationType::Read,
            Some(&epk(&partition_key, &partition_key_definition)),
        ),
    )
    .unwrap();

    // Read the metadata token IDs in stream order (don't go through
    // `parse_wrapped_request` which puts tokens into a HashMap and
    // loses ordering).
    let mut src = wrapped.body.as_ref().unwrap().as_ref();
    let _total_len = take_u32(&mut src);
    let _resource_type = take_u16(&mut src);
    let _operation_type = take_u16(&mut src);
    let _activity_id = take_uuid(&mut src);

    let mut emitted_ids = Vec::new();
    // Stop when the remaining bytes can't form a token header; the
    // body (if any) comes last and is too short to be confused with a
    // token frame here (a Read has no body).
    while src.len() >= 3 {
        let id = take_u16(&mut src);
        emitted_ids.push(id);
        let token_type = take_u8(&mut src);
        // Consume the value bytes so the next iteration starts at the
        // next token header. Mirrors `parse_token_value` for the
        // token types this test fixture actually produces.
        let _ = parse_token_value(token_type, &mut src);
    }
    assert!(
        src.is_empty(),
        "Read should have no inner body; leftover bytes {:?}",
        src
    );

    let pos = |id: u16| -> usize {
        emitted_ids
            .iter()
            .position(|&x| x == id)
            .unwrap_or_else(|| panic!("token 0x{id:04X} not emitted; got {emitted_ids:?}"))
    };
    let epk = pos(0x005A); // EffectivePartitionKey
    let global_account = pos(0x00CE); // GlobalDatabaseAccountName
    let payload_present = pos(0x0002); // PayloadPresent

    assert!(
        epk < global_account,
        "EffectivePartitionKey (0x005A) must precede GlobalDatabaseAccountName (0x00CE) per thin-client contract; got {emitted_ids:?}"
    );
    assert!(
        global_account < payload_present,
        "GlobalDatabaseAccountName (0x00CE) must precede PayloadPresent (0x0002) per thin-client contract; got {emitted_ids:?}"
    );
}

#[test]
fn wrap_builds_required_request_tokens_for_read() {
    let request = signed_request(None);
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
    let parsed = parse_wrapped_request(&wrapped, 10);

    assert_eq!(wrapped.method, Method::Post);
    assert_eq!(parsed.resource_type, 0x0003);
    assert_eq!(parsed.operation_type, 0x0003);
    assert_eq!(parsed.activity_id, Uuid::parse_str(ACTIVITY_ID).unwrap());
    assert_eq!(
        parsed.tokens[&0x0001],
        ParsedTokenValue::String("auth-token".into())
    );
    assert_eq!(parsed.tokens[&0x0002], ParsedTokenValue::Byte(0));
    assert_eq!(
        parsed.tokens[&0x0003],
        ParsedTokenValue::SmallString("Wed, 21 Oct 2015 07:28:00 GMT".into())
    );
    assert_eq!(parsed.tokens[&0x0010], ParsedTokenValue::Byte(0x02));
    assert_eq!(
        parsed.tokens[&0x0015],
        ParsedTokenValue::String("db1".into())
    );
    assert_eq!(
        parsed.tokens[&0x0016],
        ParsedTokenValue::String("coll1".into())
    );
    assert_eq!(
        parsed.tokens[&0x0017],
        ParsedTokenValue::String("doc1".into())
    );
    // TransportRequestId (0x004D) is intentionally NOT emitted on the G2
    // path — it is in the thin-client exclusion list and the proxy assigns
    // its own request id.
    assert!(!parsed.tokens.contains_key(&0x004D));
    assert_eq!(
        parsed.tokens[&0x00A2],
        ParsedTokenValue::ULong(SUPPORTED_CAPABILITIES_BITS)
    );
    assert_eq!(
        parsed.tokens[&0x00CE],
        ParsedTokenValue::String("account".into())
    );
}

#[test]
fn wrap_omits_allow_tentative_writes_and_return_preference_by_default() {
    // Regression: `ReturnPreference=1` is only set
    // when the caller passes `Prefer: return=minimal`; otherwise the token
    // is absent and the server returns the full body. Likewise
    // `AllowTentativeWrites` is only set when the caller passes the
    // `x-ms-cosmos-allow-tentative-writes` header.
    //
    // Pinning the conditional path here: emitting these tokens
    // unconditionally tells the server to drop response bodies (and on
    // Create silently discards the request body server-side, persisting
    // a phantom 0-byte document).
    let request = signed_request(None);
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
    let parsed = parse_wrapped_request(&wrapped, 0);

    assert!(
        !parsed.tokens.contains_key(&0x0066),
        "AllowTentativeWrites (0x0066) must NOT be emitted by default"
    );
    assert!(
        !parsed.tokens.contains_key(&0x0082),
        "ReturnPreference (0x0082) must NOT be emitted by default"
    );
}

#[test]
fn wrap_emits_return_preference_when_prefer_return_minimal_header_is_set() {
    // Emit `ReturnPreference=1`
    // when the request carries `Prefer: return=minimal` and only then.
    let mut request = signed_request(None);
    request
        .headers
        .insert(HeaderName::from_static("prefer"), "return=minimal");
    let auth_context = AuthorizationContext::new(
        Method::Post,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs",
    );

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::Create, None),
    )
    .unwrap();
    let parsed = parse_wrapped_request(&wrapped, 0);

    assert_eq!(parsed.tokens[&0x0082], ParsedTokenValue::Byte(1));
    assert!(
        !parsed.tokens.contains_key(&0x0066),
        "AllowTentativeWrites must remain absent when only Prefer is set"
    );
}

#[test]
fn wrap_emits_allow_tentative_writes_when_header_is_set_to_true() {
    // Emit `AllowTentativeWrites=1` only when the caller forwards the
    // `x-ms-cosmos-allow-tentative-writes` header.
    let mut request = signed_request(None);
    request.headers.insert(
        HeaderName::from_static("x-ms-cosmos-allow-tentative-writes"),
        "true",
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
    let parsed = parse_wrapped_request(&wrapped, 0);

    assert_eq!(parsed.tokens[&0x0066], ParsedTokenValue::Byte(1));
    assert!(
        !parsed.tokens.contains_key(&0x0082),
        "ReturnPreference must remain absent when only allow-tentative-writes is set"
    );
}

#[test]
fn wrap_emits_collection_rid_and_decoded_resource_id_when_supplied() {
    let request = signed_request(None);
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );

    for (rid, expected_resource_id) in [
        (
            "-NY+AJoR+cc=",
            [0xfc, 0xd6, 0x3e, 0x00, 0x9a, 0x11, 0xf9, 0xc7],
        ),
        (
            "YpNuAIVuY-0=",
            [0x62, 0x93, 0x6e, 0x00, 0x85, 0x6e, 0x63, 0xfd],
        ),
    ] {
        let mut inputs = wrap_inputs(&auth_context, OperationType::Read, None);
        inputs.collection_rid = Some(rid);

        let wrapped = wrap_request_for_gateway_v2(request.clone(), &inputs).unwrap();
        let parsed = parse_wrapped_request(&wrapped, 0);

        assert_eq!(
            parsed.tokens[&0x0035],
            ParsedTokenValue::String(rid.into()),
            "CollectionRid (0x0035) must preserve the wire value"
        );
        match &parsed.tokens[&0x0000] {
            ParsedTokenValue::Bytes(bytes) => assert_eq!(bytes, &expected_resource_id),
            other => panic!("expected ResourceId bytes, got {other:?}"),
        }
    }
}

#[test]
fn wrap_rejects_invalid_collection_rid() {
    let request = signed_request(None);
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let mut inputs = wrap_inputs(&auth_context, OperationType::Read, None);
    inputs.collection_rid = Some("invalid!");

    let err = wrap_request_for_gateway_v2(request, &inputs).unwrap_err();
    assert!(err.to_string().contains("invalid collection RID"));
}

#[test]
fn wrap_omits_collection_rid_and_resource_id_when_not_supplied() {
    // The collection rid is optional from the operation pipeline; pin
    // that absence means neither token is emitted (rather than e.g. an
    // empty string token which the proxy would reject).
    let request = signed_request(None);
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
    let parsed = parse_wrapped_request(&wrapped, 0);

    assert!(!parsed.tokens.contains_key(&0x0035));
    assert!(!parsed.tokens.contains_key(&0x0000));
}

#[test]
fn wrap_preserves_range_filters_separately_from_routing_hashes() {
    for operation in [OperationType::Query, OperationType::ReadFeed] {
        let mut request = signed_request(None);
        for (name, value) in [
            (request_header_names::START_EPK, "10AB"),
            (request_header_names::END_EPK, "20CD"),
            (
                request_header_names::READ_FEED_KEY_TYPE,
                "EffectivePartitionKeyRange",
            ),
            (request_header_names::THINCLIENT_PKRANGE_MIN, ""),
            (request_header_names::THINCLIENT_PKRANGE_MAX, "FF"),
        ] {
            request.headers.insert(HeaderName::from_static(name), value);
        }
        let auth_context = AuthorizationContext::new(
            Method::Post,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs",
        );
        let wrapped =
            wrap_request_for_gateway_v2(request, &wrap_inputs(&auth_context, operation, None))
                .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 0);
        assert_eq!(parsed.tokens[&0x0064], ParsedTokenValue::Byte(3));
        assert_eq!(
            parsed.tokens[&0x0062],
            ParsedTokenValue::Bytes(b"10AB".to_vec())
        );
        assert_eq!(
            parsed.tokens[&0x0063],
            ParsedTokenValue::Bytes(b"20CD".to_vec())
        );
        assert_eq!(
            parsed.tokens[&0x00D2],
            ParsedTokenValue::Bytes(vec![0x10, 0xAB])
        );
        assert_eq!(
            parsed.tokens[&0x00D3],
            ParsedTokenValue::Bytes(vec![0x20, 0xCD])
        );
        for name in [
            request_header_names::START_EPK,
            request_header_names::END_EPK,
            request_header_names::READ_FEED_KEY_TYPE,
        ] {
            assert!(wrapped
                .headers
                .get_optional_str(&HeaderName::from_static(name))
                .is_none());
        }
        let frame = RntbdRequestFrame::read(wrapped.body.as_ref().unwrap()).unwrap();
        let ids: Vec<_> = frame
            .metadata
            .iter()
            .map(|token| token.id.value())
            .collect();
        assert_eq!(&ids[..2], &[0x00D2, 0x00D3]);
        let date = ids.iter().position(|id| *id == 0x0003).unwrap();
        for id in [0x0062, 0x0063, 0x0064] {
            assert!(ids.iter().position(|value| *value == id).unwrap() > date);
        }
    }
}

#[test]
fn wrap_preserves_open_minimum_filter_with_point_or_range_routing() {
    let epk = EffectivePartitionKey::try_from("10").unwrap();
    for routing_epk in [None, Some(&epk)] {
        let mut request = signed_request(None);
        for (name, value) in [
            (request_header_names::START_EPK, ""),
            (request_header_names::END_EPK, "20"),
            (
                request_header_names::READ_FEED_KEY_TYPE,
                "EffectivePartitionKeyRange",
            ),
        ] {
            request.headers.insert(HeaderName::from_static(name), value);
        }
        let auth_context = AuthorizationContext::new(
            Method::Post,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs",
        );
        let wrapped = wrap_request_for_gateway_v2(
            request,
            &wrap_inputs(&auth_context, OperationType::Query, routing_epk),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 0);
        assert!(!parsed.tokens.contains_key(&0x0062));
        assert_eq!(
            parsed.tokens[&0x0063],
            ParsedTokenValue::Bytes(b"20".to_vec())
        );
        assert_eq!(parsed.tokens[&0x0064], ParsedTokenValue::Byte(3));
        if routing_epk.is_some() {
            assert_eq!(parsed.tokens[&0x005A], ParsedTokenValue::Bytes(vec![0x10]));
            assert!(!parsed.tokens.contains_key(&0x00D2));
            assert!(!parsed.tokens.contains_key(&0x00D3));
        } else {
            assert_eq!(parsed.tokens[&0x00D2], ParsedTokenValue::Bytes(Vec::new()));
        }
    }
}

#[test]
fn wrap_does_not_invent_filters_from_physical_range_bounds() {
    let mut request = signed_request(None);
    request
        .headers
        .insert(request_header_names::THINCLIENT_PKRANGE_MIN, "");
    request
        .headers
        .insert(request_header_names::THINCLIENT_PKRANGE_MAX, "FF");
    let auth_context = AuthorizationContext::new(
        Method::Post,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs",
    );
    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::Query, None),
    )
    .unwrap();
    let parsed = parse_wrapped_request(&wrapped, 0);
    assert_eq!(parsed.tokens[&0x00D2], ParsedTokenValue::Bytes(Vec::new()));
    assert_eq!(parsed.tokens[&0x00D3], ParsedTokenValue::Bytes(vec![0xFF]));
    for id in [0x0062, 0x0063, 0x0064] {
        assert!(!parsed.tokens.contains_key(&id));
    }
}

#[test]
fn wrap_validates_read_feed_key_type() {
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs",
    );
    for (header, expected) in [
        ("ResourceId", Some(1)),
        ("EffectivePartitionKey", Some(2)),
        ("EffectivePartitionKeyRange", Some(3)),
        ("", None),
    ] {
        let mut request = signed_request(None);
        request.headers.insert(
            request_header_names::READ_FEED_KEY_TYPE,
            header.to_ascii_lowercase(),
        );
        let wrapped = wrap_request_for_gateway_v2(
            request,
            &wrap_inputs(&auth_context, OperationType::ReadFeed, None),
        )
        .unwrap();
        let parsed = parse_wrapped_request(&wrapped, 0);
        assert_eq!(
            parsed.tokens.get(&0x0064),
            expected.map(ParsedTokenValue::Byte).as_ref()
        );
        assert!(!parsed.tokens.contains_key(&0x0062));
        assert!(!parsed.tokens.contains_key(&0x0063));
    }
    let mut request = signed_request(None);
    request
        .headers
        .insert(request_header_names::READ_FEED_KEY_TYPE, "unsupported");
    let error = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::ReadFeed, None),
    )
    .unwrap_err();
    assert!(error.to_string().contains("x-ms-read-key-type"));
}

#[test]
fn wrap_emits_partition_key_range_id_token_when_http_header_present() {
    // Regression: cross-partition Query routes via StartEpkHash/EndEpkHash
    // bytes, but the thin-client proxy ALSO requires the
    // PartitionKeyRangeId (0x002C, String) RNTBD token to disambiguate
    // the target physical partition. This is filled from the
    // `x-ms-documentdb-partitionkeyrangeid` HTTP header. Without it, the proxy
    // misinterprets an empty StartEpkHash as a `[]` partition-key value
    // and rejects with "PartitionKey supplied in x-ms-partitionkey header
    // has fewer components than defined in the the collection."
    let mut request = signed_request(None);
    request.headers.insert(
        HeaderName::from_static(request_header_names::PARTITION_KEY_RANGE_ID),
        "0",
    );
    let auth_context = AuthorizationContext::new(
        Method::Post,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs",
    );

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::Query, None),
    )
    .unwrap();
    let parsed = parse_wrapped_request(&wrapped, 0);

    assert_eq!(
        parsed.tokens[&0x002C],
        ParsedTokenValue::String("0".into()),
        "PartitionKeyRangeId (0x002C) must be emitted from the HTTP header"
    );
}

#[test]
fn wrap_omits_partition_key_range_id_token_when_http_header_absent() {
    // Pin the conditional emission path: no header => no token (rather
    // than an empty-string token, which the proxy would treat as a real
    // routing key for partition id "").
    let request = signed_request(None);
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
    let parsed = parse_wrapped_request(&wrapped, 0);

    assert!(!parsed.tokens.contains_key(&0x002C));
}

#[test]
fn wrap_emits_session_token_when_http_header_present() {
    // Session-consistent reads require the client's per-partition LSN
    // progress so the backend can pick a caught-up replica. Java's
    // thin-client encoder forwards it as the SessionToken (0x0005, String)
    // RNTBD token, filled from the `x-ms-session-token` header.
    let mut request = signed_request(None);
    request.headers.insert(
        HeaderName::from_static(request_header_names::SESSION_TOKEN),
        "0:1#42",
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
    let parsed = parse_wrapped_request(&wrapped, 0);

    assert_eq!(
        parsed.tokens[&0x0005],
        ParsedTokenValue::String("0:1#42".into()),
        "SessionToken (0x0005) must be emitted from the HTTP header"
    );
}

#[test]
fn wrap_omits_session_token_when_http_header_absent_or_empty() {
    // No header => no token. An empty value is also skipped so the proxy
    // never receives a zero-length session token it would have to reject.
    let request = signed_request(None);
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
    assert!(!parse_wrapped_request(&wrapped, 0)
        .tokens
        .contains_key(&0x0005));

    let mut empty = signed_request(None);
    empty.headers.insert(
        HeaderName::from_static(request_header_names::SESSION_TOKEN),
        "",
    );
    let wrapped_empty = wrap_request_for_gateway_v2(
        empty,
        &wrap_inputs(&auth_context, OperationType::Read, None),
    )
    .unwrap();
    assert!(!parse_wrapped_request(&wrapped_empty, 0)
        .tokens
        .contains_key(&0x0005));
}

#[test]
fn wrap_emits_supported_serialization_formats_token_from_header() {
    // `JsonText,CosmosBinary` header => 0x01 | 0x02 token.
    let mut request = signed_request(None);
    request.headers.insert(
        HeaderName::from_static(request_header_names::SUPPORTED_SERIALIZATION_FORMATS),
        "JsonText,CosmosBinary",
    );
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::Query, None),
    )
    .unwrap();
    assert_eq!(
        parse_wrapped_request(&wrapped, 0).tokens[&0x00C4],
        ParsedTokenValue::Byte(0x03),
        "SupportedSerializationFormats (0x00C4) must be the OR of the accepted format flags"
    );

    // Point-op sends `CosmosBinary` alone => 0x02.
    let mut point = signed_request(None);
    point.headers.insert(
        HeaderName::from_static(request_header_names::SUPPORTED_SERIALIZATION_FORMATS),
        "CosmosBinary",
    );
    let wrapped_point = wrap_request_for_gateway_v2(
        point,
        &wrap_inputs(&auth_context, OperationType::Read, None),
    )
    .unwrap();
    assert_eq!(
        parse_wrapped_request(&wrapped_point, 0).tokens[&0x00C4],
        ParsedTokenValue::Byte(0x02),
    );
}

#[test]
fn wrap_omits_supported_serialization_formats_token_when_absent_or_unknown() {
    // No header => no token.
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let wrapped = wrap_request_for_gateway_v2(
        signed_request(None),
        &wrap_inputs(&auth_context, OperationType::Read, None),
    )
    .unwrap();
    assert!(!parse_wrapped_request(&wrapped, 0)
        .tokens
        .contains_key(&0x00C4));

    // An unrecognized value emits nothing, not a zero-flag token.
    let mut unknown = signed_request(None);
    unknown.headers.insert(
        HeaderName::from_static(request_header_names::SUPPORTED_SERIALIZATION_FORMATS),
        "SomethingElse",
    );
    let wrapped_unknown = wrap_request_for_gateway_v2(
        unknown,
        &wrap_inputs(&auth_context, OperationType::Read, None),
    )
    .unwrap();
    assert!(!parse_wrapped_request(&wrapped_unknown, 0)
        .tokens
        .contains_key(&0x00C4));
}

#[test]
fn parse_supported_serialization_formats_maps_names_to_flags() {
    assert_eq!(parse_supported_serialization_formats("JsonText"), 0x01);
    assert_eq!(parse_supported_serialization_formats("CosmosBinary"), 0x02);
    assert_eq!(
        parse_supported_serialization_formats("JsonText,CosmosBinary"),
        0x03
    );
    assert_eq!(parse_supported_serialization_formats("HybridRow"), 0x04);
    // Case-insensitive, whitespace-tolerant, `+feature` stripped.
    assert_eq!(
        parse_supported_serialization_formats(" cosmosbinary+UInt64 , jsontext "),
        0x03
    );
    assert_eq!(parse_supported_serialization_formats("nonsense"), 0x00);
}

#[test]
fn wrap_emits_page_size_from_max_item_count_header() {
    // Query/read-feed page size must reach the proxy as PageSize (0x0004,
    // ULong), filled from `x-ms-max-item-count`. A negative (unbounded)
    // request is encoded as 0xFFFFFFFF.
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );

    let mut request = signed_request(None);
    request.headers.insert(
        HeaderName::from_static(request_header_names::MAX_ITEM_COUNT),
        "100",
    );
    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::ReadFeed, None),
    )
    .unwrap();
    assert_eq!(
        parse_wrapped_request(&wrapped, 0).tokens[&0x0004],
        ParsedTokenValue::ULong(100),
        "PageSize (0x0004) must be emitted from x-ms-max-item-count"
    );

    let mut unbounded = signed_request(None);
    unbounded.headers.insert(
        HeaderName::from_static(request_header_names::MAX_ITEM_COUNT),
        "-1",
    );
    let wrapped_unbounded = wrap_request_for_gateway_v2(
        unbounded,
        &wrap_inputs(&auth_context, OperationType::ReadFeed, None),
    )
    .unwrap();
    assert_eq!(
        parse_wrapped_request(&wrapped_unbounded, 0).tokens[&0x0004],
        ParsedTokenValue::ULong(0xFFFF_FFFF),
        "a negative max item count must encode as 0xFFFFFFFF"
    );
}

#[test]
fn wrap_omits_page_size_when_header_absent() {
    let request = signed_request(None);
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::ReadFeed, None),
    )
    .unwrap();
    assert!(!parse_wrapped_request(&wrapped, 0)
        .tokens
        .contains_key(&0x0004));
}

#[test]
fn wrap_emits_incremental_feed_a_im_token() {
    let mut request = signed_request(None);
    request.headers.insert(
        HeaderName::from_static(request_header_names::A_IM),
        request_header_names::INCREMENTAL_FEED,
    );
    request.headers.insert(
        HeaderName::from_static(request_header_names::CHANGEFEED_WIRE_FORMAT_VERSION),
        request_header_names::CHANGEFEED_WIRE_FORMAT_VERSION_2021_09_15,
    );
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs",
    );

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::ReadFeed, None),
    )
    .unwrap();

    assert_eq!(
        parse_wrapped_request(&wrapped, 0).tokens[&0x003F],
        ParsedTokenValue::String(request_header_names::INCREMENTAL_FEED.into())
    );
    assert_eq!(
        parse_wrapped_request(&wrapped, 0).tokens[&0x00B2],
        ParsedTokenValue::String(
            request_header_names::CHANGEFEED_WIRE_FORMAT_VERSION_2021_09_15.into()
        )
    );
}

#[test]
fn wrap_omits_a_im_token_for_ordinary_read_feed() {
    let request = signed_request(None);
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs",
    );

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::ReadFeed, None),
    )
    .unwrap();

    assert!(!parse_wrapped_request(&wrapped, 0)
        .tokens
        .contains_key(&0x003F));
    assert!(!parse_wrapped_request(&wrapped, 0)
        .tokens
        .contains_key(&0x00B2));
}

#[test]
fn wrap_emits_match_from_if_match_on_writes() {
    // Writes carry the ETag precondition as Match (0x0008, String) filled
    // from `If-Match`; `If-None-Match` is ignored on a write.
    let auth_context = AuthorizationContext::new(
        Method::Put,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let mut request = signed_request(Some(b"{}"));
    request.headers.insert(
        HeaderName::from_static(request_header_names::IF_MATCH),
        "\"etag-42\"",
    );
    request.headers.insert(
        HeaderName::from_static(request_header_names::IF_NONE_MATCH),
        "\"ignored\"",
    );
    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::Replace, None),
    )
    .unwrap();
    assert_eq!(
        parse_wrapped_request(&wrapped, 0).tokens[&0x0008],
        ParsedTokenValue::String("\"etag-42\"".into()),
        "Match (0x0008) on a write must come from If-Match"
    );
}

#[test]
fn wrap_emits_match_from_if_none_match_on_reads() {
    // Reads/read-feeds carry the conditional-GET precondition as Match
    // (0x0008) filled from `If-None-Match`; `If-Match` is ignored on reads.
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let mut request = signed_request(None);
    request.headers.insert(
        HeaderName::from_static(request_header_names::IF_NONE_MATCH),
        "\"etag-7\"",
    );
    request.headers.insert(
        HeaderName::from_static(request_header_names::IF_MATCH),
        "\"ignored\"",
    );
    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::Read, None),
    )
    .unwrap();
    assert_eq!(
        parse_wrapped_request(&wrapped, 0).tokens[&0x0008],
        ParsedTokenValue::String("\"etag-7\"".into()),
        "Match (0x0008) on a read must come from If-None-Match"
    );
}

#[test]
fn wrap_emits_point_in_time_with_continuation_on_read_feed() {
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs",
    );
    let mut request = signed_request(None);
    request.headers.insert(
        HeaderName::from_static(request_header_names::IF_NONE_MATCH),
        "\"etag-7\"",
    );
    request.headers.insert(
        HeaderName::from_static(request_header_names::IF_MODIFIED_SINCE),
        "Mon, 01 Jan 2024 00:00:00 GMT",
    );
    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::ReadFeed, None),
    )
    .unwrap();
    let parsed = parse_wrapped_request(&wrapped, 0);
    assert_eq!(
        parsed.tokens[&0x0008],
        ParsedTokenValue::String("\"etag-7\"".into())
    );
    assert_eq!(
        parsed.tokens[&0x0047],
        ParsedTokenValue::String("Mon, 01 Jan 2024 00:00:00 GMT".into())
    );
}

#[test]
fn wrap_omits_document_name_for_create() {
    let request = signed_request(Some(b"{}"));
    let auth_context =
        AuthorizationContext::new(Method::Post, ResourceType::Document, "dbs/db1/colls/coll1");

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(&auth_context, OperationType::Create, None),
    )
    .unwrap();
    let parsed = parse_wrapped_request(&wrapped, 9);

    assert!(!parsed.tokens.contains_key(&0x0017));
}

#[test]
fn wrap_uses_resolved_consistency_token() {
    let request = signed_request(None);
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let mut inputs = wrap_inputs(&auth_context, OperationType::Read, None);
    inputs.effective_consistency = DefaultConsistencyLevel::Eventual;

    let wrapped = wrap_request_for_gateway_v2(request, &inputs).unwrap();
    let parsed = parse_wrapped_request(&wrapped, 10);

    assert_eq!(parsed.tokens[&0x0010], ParsedTokenValue::Byte(0x03));
}

/// Rule 1+5 (RCS resolution):
/// non-`Default` RCS on the wrap path emits token `0x00FE` and SUPPRESSES
/// the legacy `ConsistencyLevel` token `0x0010`.
#[test]
fn wrap_emits_read_consistency_strategy_token_and_drops_consistency_level() {
    use crate::options::ReadConsistencyStrategy;

    let cases = [
        (ReadConsistencyStrategy::Eventual, 0x01u8),
        (ReadConsistencyStrategy::Session, 0x02u8),
        (ReadConsistencyStrategy::LatestCommitted, 0x03u8),
        (ReadConsistencyStrategy::GlobalStrong, 0x04u8),
    ];

    for (strategy, expected_byte) in cases {
        let request = signed_request(None);
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        );
        let mut inputs = wrap_inputs(&auth_context, OperationType::Read, None);
        inputs.read_consistency_strategy = strategy;

        let wrapped = wrap_request_for_gateway_v2(request.clone(), &inputs).unwrap();
        let parsed = parse_wrapped_request(&wrapped, 10);

        assert_eq!(
            parsed.tokens[&0x00FE],
            ParsedTokenValue::Byte(expected_byte),
            "ReadConsistencyStrategy {strategy:?} should serialize to byte {expected_byte:#x}"
        );
        assert!(
            !parsed.tokens.contains_key(&0x0010),
            "ConsistencyLevel token (0x0010) must be omitted when RCS={strategy:?} is non-Default"
        );
    }
}

/// Rule 3: `Default` RCS is transparent — wrap behaves identically to the
/// pre-RCS world, emitting only `ConsistencyLevel`.
#[test]
fn wrap_with_default_rcs_emits_only_consistency_level_token() {
    use crate::options::ReadConsistencyStrategy;

    let request = signed_request(None);
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let mut inputs = wrap_inputs(&auth_context, OperationType::Read, None);
    inputs.read_consistency_strategy = ReadConsistencyStrategy::Default;
    inputs.effective_consistency = DefaultConsistencyLevel::Session;

    let wrapped = wrap_request_for_gateway_v2(request, &inputs).unwrap();
    let parsed = parse_wrapped_request(&wrapped, 10);

    assert_eq!(parsed.tokens[&0x0010], ParsedTokenValue::Byte(0x02));
    assert!(!parsed.tokens.contains_key(&0x00FE));
}

#[test]
fn wrap_emits_v2_binary_effective_partition_key_for_v2_collections() {
    // For collections with PartitionKey version=V2 (the default), the RNTBD
    // EffectivePartitionKey token (0x005A) must carry the V2 binary form:
    // the raw 16-byte Murmur3_128 hash with the top 2 bits of byte 0
    // cleared. Sending V1 binary against a V2 collection causes the proxy
    // to mis-route (or silently drop the body), persisting a phantom 0-byte
    // document.
    let request = signed_request(None);
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let partition_key = PartitionKey::from("tenant1");
    let partition_key_definition = PartitionKeyDefinition::new(vec![Cow::from("/tenantId")]);
    assert_eq!(
        partition_key_definition.version(),
        PartitionKeyVersion::V2,
        "sanity check: PartitionKeyDefinition::new defaults to V2"
    );
    let expected = effective_partition_key_v2_binary(partition_key.values());

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(
            &auth_context,
            OperationType::Read,
            Some(&epk(&partition_key, &partition_key_definition)),
        ),
    )
    .unwrap();
    let parsed = parse_wrapped_request(&wrapped, 11);

    assert_eq!(parsed.tokens[&0x005A], ParsedTokenValue::Bytes(expected));
}

#[test]
fn wrap_emits_v1_binary_effective_partition_key_for_v1_collections() {
    // For legacy V1 collections, EPK must remain the V1 binary tuple
    // (`Number(hash) + truncated_components`). Pin the version-aware
    // dispatch so legacy accounts still route correctly.
    let request = signed_request(None);
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let partition_key = PartitionKey::from("tenant1");
    let partition_key_definition = PartitionKeyDefinition::new(vec![Cow::from("/tenantId")])
        .with_version(PartitionKeyVersion::V1);
    let expected = effective_partition_key_v1_binary(partition_key.values());

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(
            &auth_context,
            OperationType::Read,
            Some(&epk(&partition_key, &partition_key_definition)),
        ),
    )
    .unwrap();
    let parsed = parse_wrapped_request(&wrapped, 11);

    assert_eq!(parsed.tokens[&0x005A], ParsedTokenValue::Bytes(expected));
}

#[test]
fn wrap_emits_v1_binary_effective_partition_key_for_version_less_collections() {
    // A container definition read back from the service with no `version`
    // field is a legacy V1 container (the service omits `version` only for
    // V1). Gateway 2.0 routes point ops on the client-computed EPK, so a
    // version-less container MUST emit the V1 binary EPK token (0x005A),
    // NOT V2 — mis-detecting it as V2 was the root-cause routing bug this
    // guards against.
    let request = signed_request(None);
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let partition_key = PartitionKey::from("tenant1");
    let partition_key_definition: PartitionKeyDefinition =
        serde_json::from_str(r#"{"paths":["/tenantId"]}"#).unwrap();
    assert_eq!(
        partition_key_definition.version(),
        PartitionKeyVersion::V1,
        "sanity check: a version-less definition deserializes to V1"
    );
    let expected = effective_partition_key_v1_binary(partition_key.values());
    assert_ne!(
        expected,
        effective_partition_key_v2_binary(partition_key.values()),
        "sanity check: V1 and V2 EPK encodings differ for this pk value"
    );

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(
            &auth_context,
            OperationType::Read,
            Some(&epk(&partition_key, &partition_key_definition)),
        ),
    )
    .unwrap();
    let parsed = parse_wrapped_request(&wrapped, 11);

    assert_eq!(parsed.tokens[&0x005A], ParsedTokenValue::Bytes(expected));
}

/// HPK partial-PK (prefix on a MultiHash container) emits an
/// `EffectivePartitionKey` RNTBD token containing the per-component
/// MultiHash V2 binary for the supplied prefix components (16 * N bytes
/// for N supplied components). The same token is emitted for point AND
/// prefix keys whenever a partition key is present on the request.
#[test]
fn wrap_emits_prefix_epk_token_for_hpk_prefix_partition_key() {
    let request = signed_request(None);
    let auth_context =
        AuthorizationContext::new(Method::Get, ResourceType::Document, "dbs/db1/colls/coll1");
    let partition_key =
        PartitionKey::try_from(vec![PartitionKeyValue::from("tenant1".to_string())]).unwrap();
    let partition_key_definition =
        PartitionKeyDefinition::from(("/tenantId", "/userId", "/sessionId"));
    let expected = effective_partition_key_multi_hash_v2_binary(partition_key.values());
    assert_eq!(
        expected.len(),
        16,
        "1-of-3 component prefix must produce a single 16-byte MultiHash V2 chunk"
    );

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(
            &auth_context,
            OperationType::Query,
            Some(&epk(&partition_key, &partition_key_definition)),
        ),
    )
    .unwrap();

    // Token layout for a partial-HPK Query: 9 base tokens + EPK = 10.
    let parsed = parse_wrapped_request(&wrapped, 10);
    assert_eq!(parsed.tokens[&0x005A], ParsedTokenValue::Bytes(expected));
    // Range headers must NOT be emitted in the EPK-token path — the
    // emission is mutually exclusive: partition key present → EPK token,
    // otherwise → StartEpkHash/EndEpkHash range tokens.
    assert!(
        wrapped
            .headers
            .get_optional_str(&GATEWAY_V2_RANGE_MIN)
            .is_none(),
        "range-min header must not be emitted alongside EPK token"
    );
    assert!(
        wrapped
            .headers
            .get_optional_str(&GATEWAY_V2_RANGE_MAX)
            .is_none(),
        "range-max header must not be emitted alongside EPK token"
    );
}

/// Full HPK key (component count == definition path count) collapses to a
/// point op: emit the EPK token, no range headers.
#[test]
fn wrap_emits_token_only_for_full_hpk_partition_key() {
    let request = signed_request(None);
    let auth_context = AuthorizationContext::new(
        Method::Get,
        ResourceType::Document,
        "dbs/db1/colls/coll1/docs/doc1",
    );
    let partition_key = PartitionKey::try_from(vec![
        PartitionKeyValue::from("tenant1".to_string()),
        PartitionKeyValue::from("user1".to_string()),
        PartitionKeyValue::from("session1".to_string()),
    ])
    .unwrap();
    let partition_key_definition =
        PartitionKeyDefinition::from(("/tenantId", "/userId", "/sessionId"));

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &wrap_inputs(
            &auth_context,
            OperationType::Read,
            Some(&epk(&partition_key, &partition_key_definition)),
        ),
    )
    .unwrap();

    // Range headers must NOT be present on the point path.
    assert!(wrapped
        .headers
        .get_optional_str(&GATEWAY_V2_RANGE_MIN)
        .is_none());
    assert!(wrapped
        .headers
        .get_optional_str(&GATEWAY_V2_RANGE_MAX)
        .is_none());

    // EPK token present in the inner RNTBD frame, and bytes must be the
    // per-component MultiHash V2 concatenation (16 * N bytes for N PK
    // paths). Sending a single 16-byte V2 hash for a MultiHash container
    // causes the proxy to silently drop the body.
    let parsed = parse_wrapped_request(&wrapped, 11);
    let expected = effective_partition_key_multi_hash_v2_binary(partition_key.values());
    assert_eq!(expected.len(), 16 * 3, "sanity check: 3 PK paths * 16B");
    assert_eq!(
        parsed.tokens[&0x005A],
        ParsedTokenValue::Bytes(expected),
        "EffectivePartitionKey token must be per-component MultiHash V2 bytes"
    );
}

#[test]
fn wrap_propagates_continuation_token_into_rntbd_metadata() {
    let mut request = signed_request(None);
    request.headers.insert(X_MS_CONTINUATION, "page-token-1");
    let auth_context =
        AuthorizationContext::new(Method::Get, ResourceType::Document, "dbs/db1/colls/coll1");

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &WrapInputs {
            auth_context: &auth_context,
            operation_type: OperationType::Query,
            resource_type: ResourceType::Document,
            effective_partition_key: None,
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            account_name: Some("account"),
            collection_rid: None,
        },
    )
    .unwrap();
    let parsed = parse_wrapped_request(&wrapped, 10);

    assert_eq!(
        parsed.tokens[&0x0006],
        ParsedTokenValue::String("page-token-1".into()),
        "continuation token should be encoded as string token 0x0006",
    );
    assert!(
        wrapped
            .headers
            .get_optional_str(&X_MS_CONTINUATION)
            .is_none(),
        "x-ms-continuation header should not be forwarded on the outer HTTP request",
    );
}

#[test]
fn wrap_omits_continuation_token_when_header_absent() {
    let request = signed_request(None);
    let auth_context =
        AuthorizationContext::new(Method::Get, ResourceType::Document, "dbs/db1/colls/coll1");

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &WrapInputs {
            auth_context: &auth_context,
            operation_type: OperationType::Query,
            resource_type: ResourceType::Document,
            effective_partition_key: None,
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            account_name: Some("account"),
            collection_rid: None,
        },
    )
    .unwrap();
    let parsed = parse_wrapped_request(&wrapped, 9);

    assert!(
        !parsed.tokens.contains_key(&0x0006),
        "continuation token should be absent when no x-ms-continuation header is present",
    );
}

#[test]
fn wrap_emits_empty_continuation_token_when_header_value_empty() {
    // The wrap side forwards empty continuation strings verbatim, matching
    // the unwrap side. Continuation is opaque on the wire — the wrap path
    // does not infer intent from emptiness.
    let mut request = signed_request(None);
    request.headers.insert(X_MS_CONTINUATION, "");
    let auth_context =
        AuthorizationContext::new(Method::Get, ResourceType::Document, "dbs/db1/colls/coll1");

    let wrapped = wrap_request_for_gateway_v2(
        request,
        &WrapInputs {
            auth_context: &auth_context,
            operation_type: OperationType::Query,
            resource_type: ResourceType::Document,
            effective_partition_key: None,
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            account_name: Some("account"),
            collection_rid: None,
        },
    )
    .unwrap();
    let parsed = parse_wrapped_request(&wrapped, 10);

    assert_eq!(
        parsed.tokens[&0x0006],
        ParsedTokenValue::String(String::new()),
        "empty continuation header should be emitted as a zero-length string token",
    );
}
