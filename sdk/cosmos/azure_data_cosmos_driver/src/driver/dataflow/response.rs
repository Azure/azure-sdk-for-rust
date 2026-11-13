// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Private page ownership, materialized only after the root pipeline emits.

#[cfg(test)]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{ops::Range, sync::Arc};

use azure_core::{fmt::SafeDebug, Bytes};
#[cfg(test)]
use serde::de::DeserializeOwned;

use crate::{
    diagnostics::DiagnosticsContext,
    error::{CosmosError, Result},
    models::{CosmosResponse, CosmosResponseHeaders, CosmosStatus, ResponseBody},
    options::Utf8DecodingMode,
};

#[derive(Clone, SafeDebug)]
pub(crate) struct ItemView {
    source_page: Bytes,
    value_range: Range<usize>,
    // A per-item probe follows clones to catch conversion before root selection.
    // Correct output alone cannot reveal wasted conversions of discarded items.
    #[cfg(test)]
    materializations: Option<Arc<MaterializationCounts>>,
}

#[cfg(test)]
#[derive(Default, SafeDebug)]
struct MaterializationCounts {
    total: AtomicUsize,
    binary: AtomicUsize,
}

impl ItemView {
    pub(crate) fn standalone(bytes: Bytes) -> Self {
        Self {
            value_range: 0..bytes.len(),
            source_page: bytes,
            #[cfg(test)]
            materializations: None,
        }
    }

    pub(crate) fn from_binary_page(page: Bytes, value_range: Range<usize>) -> Result<Self> {
        if !crate::binary_json::is_binary(&page)
            || value_range.start == 0
            || value_range.start > value_range.end
            || page.get(value_range.clone()).is_none()
        {
            return Err(CosmosError::builder()
                .with_status(crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("binary item range lies outside its source page")
                .build());
        }
        Ok(Self {
            source_page: page,
            value_range,
            #[cfg(test)]
            materializations: None,
        })
    }

    #[cfg(test)]
    pub(crate) fn source_page(&self) -> &[u8] {
        &self.source_page
    }

    #[cfg(test)]
    pub(crate) fn value_range(&self) -> Range<usize> {
        self.value_range.clone()
    }

    #[cfg(test)]
    pub(crate) fn raw_value(&self) -> &[u8] {
        &self.source_page[self.value_range.clone()]
    }

    pub(crate) fn is_page_backed(&self) -> bool {
        self.value_range.start != 0
    }

    pub(crate) fn to_standalone(&self) -> Result<Bytes> {
        self.materialize(true)
    }

    fn materialize(&self, binary: bool) -> Result<Bytes> {
        if !self.is_page_backed() {
            return Ok(self.source_page.clone());
        }
        #[cfg(test)]
        if let Some(count) = &self.materializations {
            count.total.fetch_add(1, Ordering::Relaxed);
            if binary {
                count.binary.fetch_add(1, Ordering::Relaxed);
            }
        }
        let value = self.decoded_value()?;
        if binary {
            Ok(Bytes::from(crate::binary_json::encode(&value)))
        } else {
            serde_json::to_vec(&value)
                .map(Bytes::from)
                .map_err(|source| invalid_body_error("failed to transcode feed item", source))
        }
    }

    pub(crate) fn decoded_value(&self) -> Result<serde_json::Value> {
        if !self.is_page_backed() {
            return ResponseBody::from_bytes(self.source_page.clone()).into_single();
        }
        let mut value =
            crate::binary_json::reader::decode_at(&self.source_page, self.value_range.clone())
                .map_err(|source| invalid_body_error("failed to decode feed item", source))?;
        crate::binary_json::normalize_integral_floats(&mut value);
        Ok(value)
    }

    #[cfg(test)]
    pub(crate) fn deserialize<T: DeserializeOwned>(&self) -> Result<T> {
        ResponseBody::from_bytes(self.to_standalone()?).into_single()
    }
}

#[derive(Clone, Default, SafeDebug)]
pub(crate) enum PipelineBody {
    #[default]
    NoPayload,
    Bytes(Bytes),
    Items(Vec<Bytes>),
    ContextualItems(Vec<ItemView>),
}

impl PipelineBody {
    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        match self {
            Self::NoPayload => true,
            Self::Bytes(bytes) => bytes.is_empty(),
            Self::Items(items) => items.is_empty(),
            Self::ContextualItems(items) => items.is_empty(),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_bytes(bytes: impl Into<Bytes>) -> Self {
        ResponseBody::from_bytes(bytes).into()
    }

    pub(crate) fn from_items(items: Vec<Bytes>) -> Self {
        Self::Items(items)
    }

    pub(crate) fn from_item_views(items: Vec<ItemView>) -> Self {
        if items.iter().any(ItemView::is_page_backed) {
            Self::ContextualItems(items)
        } else {
            Self::Items(items.into_iter().map(|item| item.source_page).collect())
        }
    }

    pub(crate) fn single(self) -> Result<Bytes> {
        let count = match self {
            Self::NoPayload => return Ok(Bytes::new()),
            Self::Bytes(bytes) => return Ok(bytes),
            Self::Items(items) => items.len(),
            Self::ContextualItems(items) => items.len(),
        };
        Err(CosmosError::builder()
            .with_status(CosmosStatus::new(azure_core::http::StatusCode::BadRequest))
            .with_message(format!(
                "expected single response body, found feed response with {count} item(s)"
            ))
            .build())
    }

    #[cfg(test)]
    pub(crate) fn items(self) -> Result<Vec<Bytes>> {
        self.into_public(false, true)?.items()
    }

    #[cfg(test)]
    pub(crate) fn into_items<T: DeserializeOwned>(self) -> Result<Vec<T>> {
        self.into_public(false, true)?.into_items()
    }

    fn into_public(self, text_response: bool, emit_binary: bool) -> Result<ResponseBody> {
        let body = match self {
            Self::NoPayload => ResponseBody::NoPayload,
            Self::Bytes(bytes) => ResponseBody::Bytes(bytes),
            Self::Items(items) => ResponseBody::Items(items),
            Self::ContextualItems(items) => {
                let items = items
                    .iter()
                    .map(|item| {
                        if (text_response || !emit_binary) && item.is_page_backed() {
                            item.materialize(false)
                        } else {
                            item.to_standalone()
                        }
                    })
                    .collect::<Result<Vec<_>>>()?;
                ResponseBody::Items(items)
            }
        };
        Ok(body)
    }
}

impl From<ResponseBody> for PipelineBody {
    fn from(body: ResponseBody) -> Self {
        match body {
            ResponseBody::NoPayload => Self::NoPayload,
            ResponseBody::Bytes(bytes) => Self::Bytes(bytes),
            ResponseBody::Items(items) => Self::Items(items),
        }
    }
}

impl From<Bytes> for PipelineBody {
    fn from(bytes: Bytes) -> Self {
        ResponseBody::from_bytes(bytes).into()
    }
}

impl From<Vec<u8>> for PipelineBody {
    fn from(bytes: Vec<u8>) -> Self {
        Bytes::from(bytes).into()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PipelineResponse {
    metadata: CosmosResponse,
    body: PipelineBody,
}

impl PipelineResponse {
    pub(crate) fn new(
        body: impl Into<PipelineBody>,
        headers: CosmosResponseHeaders,
        status: CosmosStatus,
        diagnostics: Arc<DiagnosticsContext>,
    ) -> Self {
        Self {
            metadata: CosmosResponse::new(ResponseBody::NoPayload, headers, status, diagnostics),
            body: body.into(),
        }
    }

    pub(crate) fn body(&self) -> &PipelineBody {
        &self.body
    }

    #[cfg(test)]
    pub(crate) fn body_bytes(&self) -> &[u8] {
        match &self.body {
            PipelineBody::Bytes(bytes) => bytes,
            _ => panic!("expected PipelineBody::Bytes"),
        }
    }

    pub(crate) fn into_body(self) -> PipelineBody {
        self.body
    }

    pub(crate) fn headers(&self) -> &CosmosResponseHeaders {
        self.metadata.headers()
    }

    pub(crate) fn status(&self) -> CosmosStatus {
        self.metadata.status()
    }

    pub(crate) fn diagnostics(&self) -> Arc<DiagnosticsContext> {
        self.metadata.diagnostics()
    }

    pub(crate) fn with_aggregated_prior_diagnostics(
        mut self,
        prior: &[Arc<DiagnosticsContext>],
    ) -> Self {
        self.metadata = self.metadata.with_aggregated_prior_diagnostics(prior);
        self
    }

    pub(crate) fn into_response(
        self,
        text_response: bool,
        emit_binary: bool,
        utf8_decoding_mode: Utf8DecodingMode,
    ) -> Result<CosmosResponse> {
        let diagnostics = self.metadata.diagnostics();
        let body = self
            .body
            .into_public(text_response, emit_binary)
            .map_err(|error| {
                let diagnostics = Arc::new(diagnostics.clone_with_status(error.status()));
                crate::error::CosmosErrorBuilder::from_error(error)
                    .with_diagnostics(diagnostics)
                    .build()
            })?;
        let mut response = self.metadata.with_body(body);
        if text_response {
            response.transcode_body_to_text_with_utf8_mode(utf8_decoding_mode)?;
        }
        Ok(response)
    }
}

impl From<CosmosResponse> for PipelineResponse {
    fn from(mut response: CosmosResponse) -> Self {
        let body = response.take_body().into();
        Self {
            metadata: response,
            body,
        }
    }
}

fn invalid_body_error(
    message: &'static str,
    source: impl std::error::Error + Send + Sync + 'static,
) -> CosmosError {
    CosmosError::builder()
        .with_status(crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
        .with_message(message)
        .with_source(source)
        .build()
}

#[cfg(test)]
mod tests {
    use super::{ItemView, MaterializationCounts, PipelineBody, PipelineResponse};
    use crate::{
        binary_json::markers,
        driver::dataflow::{
            mocks::{MockLeaf, NoopRequestExecutor},
            query_plan::DistinctType,
            Distinct, PageResult, PipelineContext, PipelineNode, SkipTake,
        },
        models::{CosmosResponseHeaders, CosmosStatus, ResponseBody},
        options::Utf8DecodingMode,
    };
    use azure_core::Bytes;
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

    struct OwnedPage {
        bytes: Vec<u8>,
        released: Arc<AtomicBool>,
    }

    impl AsRef<[u8]> for OwnedPage {
        fn as_ref(&self) -> &[u8] {
            &self.bytes
        }
    }

    impl Drop for OwnedPage {
        fn drop(&mut self) {
            self.released.store(true, Ordering::Relaxed);
        }
    }

    #[tokio::test]
    async fn only_root_survivors_are_materialized_and_public_items_release_source() {
        for (distinct, skip, take) in [(false, 0, 2), (false, 1, 1), (true, 0, 1)] {
            for (emit_binary, text_response) in
                [(false, false), (false, true), (true, false), (true, true)]
            {
                let count = Arc::new(MaterializationCounts::default());
                let released = Arc::new(AtomicBool::new(false));
                let page = Bytes::from_owner(OwnedPage {
                    bytes: vec![
                        0x80,
                        crate::binary_json::markers::ARR_L1,
                        8,
                        0x83,
                        b'f',
                        b'o',
                        b'o',
                        crate::binary_json::markers::STR_R1,
                        3,
                        crate::binary_json::markers::STR_R1,
                        3,
                    ],
                    released: released.clone(),
                });
                let items = [7..9, 9..11]
                    .into_iter()
                    .map(|range| {
                        let mut item = ItemView::from_binary_page(page.clone(), range).unwrap();
                        item.materializations = Some(count.clone());
                        item
                    })
                    .collect();
                let leaf: Box<dyn PipelineNode> = Box::new(MockLeaf::with_responses([(
                    PipelineResponse::new(
                        PipelineBody::from_item_views(items),
                        CosmosResponseHeaders::default(),
                        CosmosStatus::new(azure_core::http::StatusCode::Ok),
                        crate::driver::dataflow::mocks::response(b"").diagnostics(),
                    ),
                    true,
                )]));
                drop(page);
                let child = if distinct {
                    Box::new(Distinct::with_last_hash(
                        leaf,
                        DistinctType::Ordered,
                        None,
                        emit_binary,
                    )) as Box<dyn PipelineNode>
                } else {
                    leaf
                };
                let mut root = SkipTake::new(child, skip, Some(take), emit_binary);
                let mut executor = NoopRequestExecutor;
                let mut context = PipelineContext::new(&mut executor, None);
                let PageResult::Page { response, .. } = root.next_page(&mut context).await.unwrap()
                else {
                    panic!("expected selected private page");
                };
                assert_eq!(count.total.load(Ordering::Relaxed), 0);
                assert!(
                    matches!(response.body(), PipelineBody::ContextualItems(items) if items.len() == take as usize)
                );
                let public = response
                    .into_response(text_response, emit_binary, Default::default())
                    .unwrap();
                drop(root);
                assert_eq!(count.total.load(Ordering::Relaxed), take as usize);
                let binary = emit_binary && !text_response;
                assert_eq!(
                    count.binary.load(Ordering::Relaxed),
                    if binary { take as usize } else { 0 }
                );
                assert!(released.load(Ordering::Relaxed));
                let ResponseBody::Items(items) = public.into_body() else {
                    panic!("expected public standalone items");
                };
                assert_eq!(items.len(), take as usize);
                for item in &items {
                    assert_eq!(crate::binary_json::is_binary(item), binary);
                }
                assert_eq!(
                    ResponseBody::Items(items).into_items::<String>().unwrap(),
                    vec!["foo"; take as usize]
                );
            }
        }
    }

    #[test]
    fn public_body_remains_exhaustively_matchable() {
        fn item_count(body: ResponseBody) -> usize {
            match body {
                ResponseBody::NoPayload => 0,
                ResponseBody::Bytes(_) => 1,
                ResponseBody::Items(items) => items.len(),
            }
        }
        assert_eq!(item_count(ResponseBody::Items(Vec::new())), 0);
    }

    #[test]
    fn single_payload_rejects_feed_shapes_without_materializing_items() {
        let bytes = Bytes::from_static(&[0x80, markers::STR_R1, 255]);
        let view = ItemView::from_binary_page(bytes.clone(), 1..3).unwrap();
        let expected = ResponseBody::Items(vec![bytes.clone()])
            .single()
            .unwrap_err();
        for body in [
            PipelineBody::Items(vec![bytes.clone()]),
            PipelineBody::from_item_views(vec![view]),
        ] {
            let error = body.single().unwrap_err();
            assert_eq!(error.status(), expected.status());
            assert_eq!(error.to_string(), expected.to_string());
        }
        assert!(PipelineBody::NoPayload.single().unwrap().is_empty());
        assert_eq!(PipelineBody::Bytes(bytes.clone()).single().unwrap(), bytes);
    }

    #[test]
    fn final_items_preserve_utf8_recovery_and_diagnostics() {
        let body = PipelineBody::Items(vec![
            Bytes::from_static(&[0x80, markers::STR_L1, 1, 0xFF]),
            Bytes::from_static(&[b'"', 0xFF, b'"']),
        ]);
        let response = PipelineResponse::new(
            body,
            CosmosResponseHeaders::default(),
            CosmosStatus::new(azure_core::http::StatusCode::Ok),
            crate::driver::dataflow::mocks::response(b"").diagnostics(),
        );
        let activity_id = response.diagnostics().activity_id().clone();
        let error = response
            .clone()
            .into_response(true, false, Utf8DecodingMode::Strict)
            .unwrap_err();
        let diagnostics = error.diagnostics().unwrap();
        assert_eq!(diagnostics.activity_id(), &activity_id);
        assert_eq!(diagnostics.status().copied(), Some(error.status()));

        let response = response
            .into_response(true, false, Utf8DecodingMode::Lossy)
            .unwrap();
        assert_eq!(response.diagnostics().activity_id(), &activity_id);
        assert!(response.diagnostics().invalid_utf8_was_recovered());
        assert_eq!(
            response.into_body().into_items::<String>().unwrap(),
            ["\u{FFFD}", "\u{FFFD}"]
        );
    }
}
