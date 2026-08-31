//! Temporary probe — deleted before commit.

/// Is a >128-deep document reachable through the real ORDER BY binary path?
#[test]
fn probe_deep_document_through_real_path() {
    // 150 levels: above serde_json's 128 recursion limit, below the binary
    // reader's MAX_DEPTH of 256.
    let deep = format!("{}1{}", "[".repeat(150), "]".repeat(150));
    let envelope = format!(
        r#"{{"_rid":"abc","Documents":[{{"_rid":"r1","orderByItems":[{{"item":1}}],"payload":{{"id":"d1","deep":{deep}}}}}],"_count":1}}"#
    );

    // 1. Can a binary page carrying it be decoded to text? (normalize_page_body)
    //    Encode via the reader-side path the service would use.
    let as_binary = super::transcode_to_binary(envelope.as_bytes());
    println!(
        "1. text->binary encode of 150-deep envelope: {:?}",
        as_binary.as_ref().map(|_| "ok").map_err(|e| e.to_string())
    );
    if let Ok(bin) = &as_binary {
        let back = super::transcode_to_text(bin);
        println!(
            "2. binary->text decode: {:?}",
            back.as_ref().map(|_| "ok").map_err(|e| e.to_string())
        );
    }

    // 3. Does the envelope parse with RawValue borrows? (parse_envelope_page)
    #[derive(serde::Deserialize)]
    struct Feed {
        #[serde(alias = "Documents")]
        documents: Vec<Box<serde_json::value::RawValue>>,
    }
    let parsed: Result<Feed, _> = serde_json::from_slice(envelope.as_bytes());
    println!(
        "3. envelope RawValue parse: {:?}",
        parsed
            .as_ref()
            .map(|f| f.documents.len())
            .map_err(|e| e.to_string())
    );

    // 4. Does build_page's per-item re-encode fail? (the finding)
    if let Ok(feed) = parsed {
        let payload = feed.documents[0].get();
        let reencode = super::transcode_to_binary(payload.as_bytes());
        println!(
            "4. build_page per-item text->binary re-encode: {:?}",
            reencode.as_ref().map(|_| "ok").map_err(|e| e.to_string())
        );
    }

    panic!("probe");
}

/// The binary route: the service encodes a deep document (its encoder is not
/// `serde_json`), so build the equivalent programmatically and walk the real
/// driver path.
#[test]
fn probe_deep_document_binary_route() {
    // Build 150-deep programmatically — no parser, so no recursion limit.
    let mut deep = serde_json::Value::Number(1.into());
    for _ in 0..150 {
        deep = serde_json::Value::Array(vec![deep]);
    }
    let doc = serde_json::json!({ "id": "d1", "deep": deep });
    let envelope = serde_json::json!({
        "_rid": "abc",
        "Documents": [{ "_rid": "r1", "orderByItems": [{"item": 1}], "payload": doc }],
        "_count": 1,
    });

    // 1. Service-side binary encode (writer, not serde_json parse).
    let binary = super::encode(&envelope);
    println!("1. writer encoded binary page: {} bytes", binary.len());
    println!("   is_binary: {}", super::is_binary(&binary));

    // 2. normalize_page_body: binary -> text via the driver's reader.
    let text = super::transcode_to_text(&binary);
    println!(
        "2. normalize_page_body binary->text: {:?}",
        text.as_ref().map(|t| t.len()).map_err(|e| e.to_string())
    );

    // 3. parse_envelope_page: RawValue borrows.
    #[derive(serde::Deserialize)]
    struct Feed {
        #[serde(alias = "Documents")]
        documents: Vec<Box<serde_json::value::RawValue>>,
    }
    let Ok(text) = text else {
        panic!("step 2 failed")
    };
    let parsed: Result<Feed, _> = serde_json::from_slice(&text);
    println!(
        "3. envelope RawValue parse: {:?}",
        parsed
            .as_ref()
            .map(|f| f.documents.len())
            .map_err(|e| e.to_string())
    );

    // 4. build_page per-item re-encode — the finding.
    if let Ok(feed) = parsed {
        let reencode = super::transcode_to_binary(feed.documents[0].get().as_bytes());
        println!(
            "4. build_page per-item re-encode: {:?}",
            reencode.as_ref().map(|_| "ok").map_err(|e| e.to_string())
        );
    }

    panic!("probe");
}
