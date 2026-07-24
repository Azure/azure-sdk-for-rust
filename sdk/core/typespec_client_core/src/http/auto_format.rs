// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! ## Prototype Notes
//!
//! ### Design goal
//!
//! Let `Response<T, AutoFormat>::into_model()` dispatch to JSON or XML at runtime
//! based on the response `content-type` header, with **zero caller boilerplate** and
//! **no breaking changes** to the existing `Format` / `Response` public APIs.
//! MSRV is rustc 1.88 (stable); no nightly features are permissible.
//!
//! ### Options tried (in order)
//!
//! **Option C: blanket `SelectFormat` default** — explored first, rejected.
//!
//! A blanket `impl<T> SelectFormat for T` with a JSON default would give every
//! `DeserializeOwned` type automatic format selection that they could opt out of.
//! This requires `#![feature(specialization)]`, which is nightly-only.  Since the
//! MSRV is rustc 1.88 (stable), this option is **not viable**.
//! `SelectFormat` is therefore kept as an *explicit opt-in* trait for types that want
//! custom format selection inside `AutoResponse<T>`.
//!
//! **Option B: callback passed at call-site** — adopted for `AutoResponse<T>`.
//!
//! [`AutoResponse<T>`] wraps a [`RawResponse`] and exposes:
//! - [`AutoResponse::into_model_with`] — caller provides an arbitrary closure
//!   `Fn(&RawResponse) -> crate::Result<T>`.  The closure is responsible for
//!   calling whichever format's `deserialize` (or `deserialize_from`) it needs.
//!   This is **fully extensible**: the closure is not tied to any sealed enum of
//!   known formats, so service crates can introduce their own formats.
//! - [`AutoResponse::into_model_auto`] — header inspection via
//!   [`AutoFormat::deserialize_from`]; no boilerplate needed.
//! - [`AutoResponse::into_model`] (requires `T: SelectFormat`) — the type itself
//!   declares which [`FormatChoice`] to use.  `FormatChoice` is a convenience helper
//!   for the common JSON / XML case; for additional formats, use a custom [`Format`]
//!   type with [`Format::deserialize_from`] or the `into_model_with` closure path.
//!
//! **Option A: `AutoFormat` implementing `Format` with `deserialize_from`**
//! — primary recommended path.
//!
//! [`Format`] now exposes a `deserialize_from(response: &RawResponse)` method that
//! receives the complete response (headers + body).  The default impl ignores headers
//! and delegates to [`Format::deserialize`], preserving behaviour for all existing
//! format types (`JsonFormat`, `XmlFormat`, `NoFormat`).
//!
//! [`AutoFormat`] **overrides** `deserialize_from` to inspect the `content-type`
//! header and dispatch to JSON or XML accordingly.  Because `Response<T, F>::into_model`
//! now calls `F::deserialize_from::<T>(&self.raw)`, callers of
//! `Response<T, AutoFormat>::into_model()` get correct JSON/XML dispatch **without
//! any additional boilerplate**.  Service crates can define their own format type and
//! override `deserialize_from` for any new format.
//!
//! ---
//!
//! ### What worked
//! - `Format::deserialize_from` addition (stable Rust, default impl preserves backward
//!   compatibility, zero boilerplate for callers).
//! - `AutoFormat::deserialize_from` override — `Response<T, AutoFormat>::into_model()`
//!   now correctly dispatches to JSON or XML based on `content-type`.
//! - `AutoResponse<T>` + `into_model_with(Fn(&RawResponse) -> Result<T>)` gives
//!   fully extensible closure-based dispatch.
//!
//! ### What didn't work / trade-offs
//! - Blanket `SelectFormat` default requires `specialization` (nightly) — not viable
//!   at MSRV rustc 1.88.
//! - `FormatChoice { Json, Xml }` is a sealed enum; service crates that need additional
//!   formats should define a custom [`Format`] type and override `deserialize_from`
//!   rather than extending `FormatChoice`.
//!
//! ### Viability without nightly features
//! Yes — the `Format::deserialize_from` + `AutoFormat` approach is 100% stable Rust.
//!
//! ### Ergonomic cost for crate authors
//! - **Primary path** (`Response<T, AutoFormat>`): zero boilerplate.  `into_model()`
//!   automatically detects JSON vs XML from `content-type`.
//! - **Custom format** (e.g. `Response<T, MyCborFormat>`): define a struct, implement
//!   `Format` (2–3 methods), optionally override `deserialize_from`.
//! - **`AutoResponse` closure** (`into_model_with`): zero boilerplate; the closure
//!   is entirely responsible for picking and calling the right format.
//! - **Explicit `SelectFormat` opt-in**: one `impl SelectFormat for MyType` per model
//!   type (~3 lines).  Only needed for `AutoResponse::into_model`.
//!
//! ### Minimal changes to existing public APIs
//! - Adding `Format::deserialize_from` (done) is the key unlock.
//! - `Response::into_model` now calls `F::deserialize_from` (done).
//! - No other existing public APIs are changed.

use crate::http::{
    headers::{Headers, CONTENT_TYPE},
    response::ResponseBody,
    DeserializeWith, Format, JsonFormat, RawResponse, StatusCode, XmlFormat,
};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

/// Indicates whether a response body should be deserialized as JSON or XML.
///
/// Used as a convenience return type by [`detect_format_from_headers`] and
/// [`SelectFormat`].  For formats beyond JSON and XML, define a custom [`Format`]
/// type and override [`Format::deserialize_from`] instead of extending this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatChoice {
    /// Deserialize as JSON.
    Json,
    /// Deserialize as XML.
    Xml,
}

/// Inspect `headers` and return the appropriate [`FormatChoice`].
///
/// Returns [`FormatChoice::Xml`] when the `content-type` header value contains `"xml"`
/// (e.g. `application/xml`, `text/xml`). Returns [`FormatChoice::Json`] otherwise,
/// including when the header is absent.
pub fn detect_format_from_headers(headers: &Headers) -> FormatChoice {
    if let Some(ct) = headers.get_optional_str(&CONTENT_TYPE) {
        if ct.contains("xml") {
            return FormatChoice::Xml;
        }
    }
    FormatChoice::Json
}

/// A trait that model types can implement to choose their deserialization format at
/// runtime based on the response headers.
///
/// # Note on blanket defaults
///
/// A blanket `impl<T> SelectFormat for T` that defaults to JSON would require the
/// `specialization` nightly feature, which is unavailable on stable Rust (MSRV 1.88).
/// Types that do not need custom format selection can call
/// [`AutoResponse::into_model_auto`] (automatic content-type detection) or
/// [`AutoResponse::into_model_with`] (caller-provided closure) instead.
///
/// For service crates that need custom format selection beyond JSON and XML, the
/// recommended approach is to define a custom [`Format`] type and override
/// [`Format::deserialize_from`].
pub trait SelectFormat {
    /// Return the deserialization format to use for a response with these headers.
    fn select_format(headers: &Headers) -> FormatChoice;
}

/// A [`Format`] that selects JSON or XML deserialization based on the
/// `content-type` response header.
///
/// This type implements [`Format`] and overrides [`Format::deserialize_from`] to
/// inspect the `content-type` header. When used as the format parameter in
/// [`crate::http::Response<T, AutoFormat>`], `into_model()` automatically dispatches
/// to JSON or XML — **no caller boilerplate required**.
///
/// If the `content-type` header is absent or does not contain `"xml"`, JSON is used.
///
/// # Limitations
///
/// `AutoFormat` only supports JSON and XML. For service-specific formats, define a
/// custom type that implements [`Format`] and overrides [`Format::deserialize_from`].
///
/// # Example
///
/// ```no_run
/// # use serde::Deserialize;
/// # use typespec_client_core::http::{Response, StatusCode, headers::Headers};
/// # use typespec_client_core::http::auto_format::AutoFormat;
/// # #[derive(Deserialize)] struct MyModel { value: String }
/// # fn get_raw_response() -> typespec_client_core::http::RawResponse { unimplemented!() }
/// // Service client returns Response<MyModel, AutoFormat>.
/// // Content-type header is inspected automatically in into_model().
/// let response: Response<MyModel, AutoFormat> = get_raw_response().into();
/// let model = response.into_model().expect("deserialized");
/// ```
#[derive(Debug, Clone)]
pub struct AutoFormat;

impl Format for AutoFormat {
    fn deserialize<T: DeserializeOwned, S: AsRef<[u8]>>(body: S) -> crate::Result<T> {
        // `Format::deserialize` receives only bytes; fall back to JSON.
        // Real header-based dispatch happens in `deserialize_from` below.
        crate::json::from_json(body)
    }

    fn deserialize_from<T: DeserializeOwned>(response: &RawResponse) -> crate::Result<T> {
        match detect_format_from_headers(response.headers()) {
            FormatChoice::Json => JsonFormat::deserialize(response.body()),
            FormatChoice::Xml => XmlFormat::deserialize(response.body()),
        }
    }
}

impl<D: DeserializeOwned> DeserializeWith<AutoFormat> for D {
    fn deserialize_with(body: ResponseBody) -> typespec::Result<Self> {
        // Blanket impl so Response<T, AutoFormat> is a valid type.
        // Real dispatch uses Format::deserialize_from via Response::into_model.
        body.json()
    }
}

/// A typed fully-buffered HTTP response that selects its deserialization format
/// based on the response headers **at runtime**.
///
/// Unlike [`crate::http::Response<T, F>`], where the format is fixed at compile time,
/// `AutoResponse<T>` defers the format decision to one of three mechanisms:
///
/// 1. [`into_model`](AutoResponse::into_model) — requires `T: SelectFormat`; the type
///    itself declares which format to use.
/// 2. [`into_model_auto`](AutoResponse::into_model_auto) — inspects the `content-type`
///    header automatically via [`AutoFormat::deserialize_from`].
/// 3. [`into_model_with`](AutoResponse::into_model_with) — caller provides a closure
///    `Fn(&RawResponse) -> crate::Result<T>` that is **not** tied to any sealed enum
///    of known formats, allowing full extensibility.
pub struct AutoResponse<T> {
    raw: RawResponse,
    phantom: PhantomData<T>,
}

impl<T> AutoResponse<T> {
    /// Get the HTTP status code.
    pub fn status(&self) -> StatusCode {
        self.raw.status()
    }

    /// Get the response headers.
    pub fn headers(&self) -> &Headers {
        self.raw.headers()
    }

    /// Get the response body.
    pub fn body(&self) -> &ResponseBody {
        self.raw.body()
    }

    /// Deconstruct the response into its components.
    pub fn deconstruct(self) -> (StatusCode, Headers, ResponseBody) {
        self.raw.deconstruct()
    }

    /// Consume the response, returning the underlying [`RawResponse`].
    pub fn into_raw_response(self) -> RawResponse {
        self.raw
    }
}

impl<T: DeserializeOwned> AutoResponse<T> {
    /// Deserialize the body using the provided `selector` closure.
    ///
    /// The closure receives the full [`RawResponse`] (headers + body) and is
    /// responsible for calling the appropriate format's deserialization.  This path
    /// is **not** constrained to a sealed set of known formats, so service crates can
    /// use any [`Format`] type — including ones defined outside `azure_core`.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::Deserialize;
    /// # use typespec_client_core::http::{RawResponse, StatusCode, headers::Headers, JsonFormat, Format};
    /// # use typespec_client_core::http::auto_format::AutoResponse;
    /// # #[derive(Debug, Deserialize)] struct MyModel { name: String }
    /// let raw = RawResponse::from_bytes(
    ///     StatusCode::Ok, Headers::new(), r#"{"name":"test"}"#,
    /// );
    /// let resp: AutoResponse<MyModel> = raw.into();
    /// let model = resp.into_model_with(|raw| JsonFormat::deserialize(raw.body())).unwrap();
    /// assert_eq!(model.name, "test");
    /// ```
    pub fn into_model_with<Sel>(self, selector: Sel) -> crate::Result<T>
    where
        Sel: Fn(&RawResponse) -> crate::Result<T>,
    {
        selector(&self.raw)
    }

    /// Deserialize the body by automatically detecting the format from the
    /// `content-type` response header.
    ///
    /// Delegates to [`AutoFormat::deserialize_from`] for format selection.
    pub fn into_model_auto(self) -> crate::Result<T> {
        AutoFormat::deserialize_from::<T>(&self.raw)
    }
}

impl<T: DeserializeOwned + SelectFormat> AutoResponse<T> {
    /// Deserialize the body using the format returned by
    /// [`T::select_format`](SelectFormat::select_format).
    ///
    /// Note that [`SelectFormat`] returns [`FormatChoice`], which only covers JSON and
    /// XML.  For additional formats, use [`AutoResponse::into_model_with`] or define a
    /// custom [`Format`] type and use `Response<T, MyFormat>` directly.
    pub fn into_model(self) -> crate::Result<T> {
        match T::select_format(self.raw.headers()) {
            FormatChoice::Json => JsonFormat::deserialize(self.raw.body()),
            FormatChoice::Xml => XmlFormat::deserialize(self.raw.body()),
        }
    }
}

impl<T> From<RawResponse> for AutoResponse<T> {
    fn from(raw: RawResponse) -> Self {
        Self {
            raw,
            phantom: PhantomData,
        }
    }
}

impl<T> std::fmt::Debug for AutoResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutoResponse")
            .field("status", &self.raw.status())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{headers::Headers, Format, JsonFormat, RawResponse, Response, StatusCode};
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Widget {
        id: u32,
        name: String,
    }

    /// A model that opts into `SelectFormat` and routes based on content-type.
    #[derive(Debug, Deserialize, PartialEq)]
    struct XmlWidget {
        id: u32,
        name: String,
    }

    impl SelectFormat for XmlWidget {
        fn select_format(headers: &Headers) -> FormatChoice {
            detect_format_from_headers(headers)
        }
    }

    fn json_headers() -> Headers {
        let mut h = Headers::new();
        h.insert("content-type", "application/json");
        h
    }

    fn xml_headers() -> Headers {
        let mut h = Headers::new();
        h.insert("content-type", "application/xml");
        h
    }

    // --- detect_format_from_headers ---

    #[test]
    fn detect_format_returns_xml_for_application_xml() {
        assert_eq!(
            detect_format_from_headers(&xml_headers()),
            FormatChoice::Xml
        );
    }

    #[test]
    fn detect_format_returns_xml_for_text_xml() {
        let mut h = Headers::new();
        h.insert("content-type", "text/xml; charset=utf-8");
        assert_eq!(detect_format_from_headers(&h), FormatChoice::Xml);
    }

    #[test]
    fn detect_format_returns_json_for_application_json() {
        assert_eq!(
            detect_format_from_headers(&json_headers()),
            FormatChoice::Json
        );
    }

    #[test]
    fn detect_format_returns_json_for_empty_headers() {
        assert_eq!(
            detect_format_from_headers(&Headers::new()),
            FormatChoice::Json
        );
    }

    // --- AutoResponse::into_model_auto ---

    #[test]
    fn into_model_auto_deserializes_json() {
        let raw = RawResponse::from_bytes(
            StatusCode::Ok,
            json_headers(),
            r#"{"id":1,"name":"widget-a"}"#,
        );
        let resp: AutoResponse<Widget> = raw.into();
        let model = resp.into_model_auto().expect("deserializes JSON");
        assert_eq!(
            model,
            Widget {
                id: 1,
                name: "widget-a".into()
            }
        );
    }

    #[test]
    fn into_model_auto_deserializes_xml() {
        let raw = RawResponse::from_bytes(
            StatusCode::Ok,
            xml_headers(),
            r#"<XmlWidget><id>2</id><name>widget-b</name></XmlWidget>"#,
        );
        let resp: AutoResponse<XmlWidget> = raw.into();
        let model = resp.into_model_auto().expect("deserializes XML");
        assert_eq!(
            model,
            XmlWidget {
                id: 2,
                name: "widget-b".into()
            }
        );
    }

    // --- AutoResponse::into_model (SelectFormat) ---

    #[test]
    fn select_format_drives_xml_deserialization() {
        // XmlWidget::select_format returns Xml when content-type is application/xml.
        let raw = RawResponse::from_bytes(
            StatusCode::Ok,
            xml_headers(),
            r#"<XmlWidget><id>3</id><name>widget-c</name></XmlWidget>"#,
        );
        let resp: AutoResponse<XmlWidget> = raw.into();
        let model = resp
            .into_model()
            .expect("deserializes XML via SelectFormat");
        assert_eq!(
            model,
            XmlWidget {
                id: 3,
                name: "widget-c".into()
            }
        );
    }

    #[test]
    fn select_format_drives_json_deserialization() {
        // XmlWidget::select_format returns Json when content-type is application/json.
        let raw = RawResponse::from_bytes(
            StatusCode::Ok,
            json_headers(),
            r#"{"id":4,"name":"widget-d"}"#,
        );
        let resp: AutoResponse<XmlWidget> = raw.into();
        let model = resp
            .into_model()
            .expect("deserializes JSON via SelectFormat");
        assert_eq!(
            model,
            XmlWidget {
                id: 4,
                name: "widget-d".into()
            }
        );
    }

    // --- AutoResponse::into_model_with (closure) ---

    #[test]
    fn into_model_with_closure_selects_json() {
        let raw = RawResponse::from_bytes(
            StatusCode::Ok,
            Headers::new(),
            r#"{"id":5,"name":"widget-e"}"#,
        );
        let resp: AutoResponse<Widget> = raw.into();
        // The closure is not tied to FormatChoice; it can call any Format::deserialize.
        let model = resp
            .into_model_with(|r| JsonFormat::deserialize(r.body()))
            .expect("deserializes with JSON closure");
        assert_eq!(
            model,
            Widget {
                id: 5,
                name: "widget-e".into()
            }
        );
    }

    #[test]
    fn into_model_with_closure_selects_xml() {
        let raw = RawResponse::from_bytes(
            StatusCode::Ok,
            Headers::new(),
            r#"<Widget><id>6</id><name>widget-f</name></Widget>"#,
        );
        let resp: AutoResponse<Widget> = raw.into();
        let model = resp
            .into_model_with(|r| crate::http::XmlFormat::deserialize(r.body()))
            .expect("deserializes with XML closure");
        assert_eq!(
            model,
            Widget {
                id: 6,
                name: "widget-f".into()
            }
        );
    }

    #[test]
    fn into_model_with_closure_delegates_to_auto_format() {
        // into_model_with can delegate to AutoFormat::deserialize_from for header dispatch.
        let raw = RawResponse::from_bytes(
            StatusCode::Ok,
            xml_headers(),
            r#"<Widget><id>9</id><name>widget-i</name></Widget>"#,
        );
        let resp: AutoResponse<Widget> = raw.into();
        let model = resp
            .into_model_with(AutoFormat::deserialize_from)
            .expect("dispatches to XML via AutoFormat");
        assert_eq!(
            model,
            Widget {
                id: 9,
                name: "widget-i".into()
            }
        );
    }

    // --- AutoFormat on Response<T, AutoFormat> ---

    #[test]
    fn auto_format_response_dispatches_json_by_content_type() {
        // Response<T, AutoFormat>::into_model() calls AutoFormat::deserialize_from,
        // which inspects the content-type header and selects JSON.
        let raw = RawResponse::from_bytes(
            StatusCode::Ok,
            json_headers(),
            r#"{"id":7,"name":"widget-g"}"#,
        );
        let resp: Response<Widget, AutoFormat> = raw.into();
        let model = resp.into_model().expect("AutoFormat dispatches to JSON");
        assert_eq!(
            model,
            Widget {
                id: 7,
                name: "widget-g".into()
            }
        );
    }

    #[test]
    fn auto_format_response_dispatches_xml_by_content_type() {
        // Response<T, AutoFormat>::into_model() dispatches to XML when content-type
        // is application/xml — no boilerplate on T required.
        let raw = RawResponse::from_bytes(
            StatusCode::Ok,
            xml_headers(),
            r#"<Widget><id>8</id><name>widget-h</name></Widget>"#,
        );
        let resp: Response<Widget, AutoFormat> = raw.into();
        let model = resp.into_model().expect("AutoFormat dispatches to XML");
        assert_eq!(
            model,
            Widget {
                id: 8,
                name: "widget-h".into()
            }
        );
    }
}
