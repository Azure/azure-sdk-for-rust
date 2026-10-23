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

    #[cfg(test)]
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
        for distinct in [false, true] {
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
                let leaf: Box<dyn PipelineNode> =
                    Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Page {
                        response: PipelineResponse::new(
                            PipelineBody::from_item_views(items),
                            CosmosResponseHeaders::default(),
                            CosmosStatus::new(azure_core::http::StatusCode::Ok),
                            crate::driver::dataflow::mocks::response(b"").diagnostics(),
                        ),
                        is_terminal: true,
                    })]));
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
                let mut root = SkipTake::new(child, u64::from(!distinct), Some(1), emit_binary);
                let mut executor = NoopRequestExecutor;
                let mut context = PipelineContext::new(&mut executor, None);
                let PageResult::Page { response, .. } = root.next_page(&mut context).await.unwrap()
                else {
                    panic!("expected selected private page");
                };
                assert_eq!(count.total.load(Ordering::Relaxed), 0);
                assert!(
                    matches!(response.body(), PipelineBody::ContextualItems(items) if items.len() == 1)
                );
                let public = response
                    .into_response(text_response, emit_binary, Default::default())
                    .unwrap();
                drop(root);
                assert_eq!(count.total.load(Ordering::Relaxed), 1);
                let binary = emit_binary && !text_response;
                assert_eq!(count.binary.load(Ordering::Relaxed), usize::from(binary));
                assert!(released.load(Ordering::Relaxed));
                let ResponseBody::Items(items) = public.into_body() else {
                    panic!("expected public standalone items");
                };
                assert_eq!(crate::binary_json::is_binary(&items[0]), binary);
                assert_eq!(
                    ResponseBody::Items(items).into_items::<String>().unwrap(),
                    ["foo"]
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

    #[test]
    fn materialization_preserves_absolute_references_and_releases_source_page() {
        let page = Bytes::from(vec![
            0x80,
            markers::ARR_L1,
            8,
            0x83,
            b'f',
            b'o',
            b'o',
            markers::STR_R1,
            3,
            markers::STR_R1,
            3,
        ]);
        let views = vec![
            ItemView::from_binary_page(page.clone(), 7..9).unwrap(),
            ItemView::from_binary_page(page.clone(), 9..11).unwrap(),
        ];
        for text_response in [false, true] {
            let PipelineBody::ContextualItems(_) = PipelineBody::from_item_views(views.clone())
            else {
                panic!("expected private page views");
            };
            let ResponseBody::Items(items) = PipelineBody::from_item_views(views.clone())
                .into_public(text_response, !text_response)
                .unwrap()
            else {
                panic!("expected public standalone items");
            };
            for item in &items {
                assert_ne!(item.as_ptr(), page.as_ptr());
                assert_eq!(crate::binary_json::is_binary(item), !text_response);
            }
            drop(items);
        }
        let body = PipelineBody::from_item_views(views)
            .into_public(false, true)
            .unwrap();
        drop(page);
        assert_eq!(
            body.into_items::<String>().unwrap(),
            vec!["foo".to_owned(), "foo".to_owned()]
        );
    }
}
