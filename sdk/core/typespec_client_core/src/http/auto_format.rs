// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! ## Prototype Notes
//!
//! ### Options tried (in order)
//!
//! **Option C: `SelectFormat` trait on model types** — tried first.
//!
//! Defined [`SelectFormat`] with `fn select_format(headers: &Headers) -> FormatChoice`.
//! A blanket *default* impl (all types → JSON unless overridden) requires the
//! `specialization` nightly feature. Without specialization, a blanket
//! `impl<T> SelectFormat for T` is a *seal* — downstream crates cannot override it for
//! their own types (orphan rules prevent both crates from providing impls for the same
//! foreign type, and the blanket impl in this crate would conflict with any downstream
//! impl).
//!
//! **Result**: blanket default impl is not viable on stable Rust.
//! `SelectFormat` is kept as an *explicit opt-in* trait; types that want header-driven
//! dispatch implement it themselves.
//!
//! **Option B: callback passed at call-site** — implemented alongside Option C.
//!
//! [`AutoResponse<T>`] wraps a [`RawResponse`] and exposes:
//! - [`AutoResponse::into_model_with`] — caller provides a format-selector closure.
//! - [`AutoResponse::into_model_auto`] — detects format from the `content-type` header.
//! - [`AutoResponse::into_model`] — available when `T: SelectFormat`; uses the type's
//!   own selector.
//!
//! **Result**: all three methods compile on stable Rust with no nightly features.
//!
//! **Option A: `AutoFormat` implementing `Format`** — explored as a bonus.
//!
//! [`Format::deserialize`](crate::http::Format::deserialize) receives only bytes —
//! headers are not in scope. Therefore [`AutoFormat::deserialize`] must fall back to
//! JSON; it cannot inspect the `content-type` header. A blanket
//! `DeserializeWith<AutoFormat>` impl (also JSON) means
//! `Response<T, AutoFormat>::into_model()` compiles and works, but **always uses JSON**
//! — defeating the purpose of a mixed-format type.
//!
//! Skipping the `DeserializeWith<AutoFormat>` impl would prevent
//! `Response<T, AutoFormat>` from calling `into_model()` at all, which is a confusing
//! API surface.
//!
//! **Result**: [`AutoFormat`] is included as a convenience alias for JSON when used
//! inside [`crate::http::Response`], but crate authors that need real header-based
//! dispatch should use [`AutoResponse<T>`] instead.
//!
//! ---
//!
//! ### What worked
//! - `AutoResponse<T>` + `into_model_with` (Option B) compiles on stable Rust.
//! - Explicit `SelectFormat` opt-in + `AutoResponse::into_model` (Option C) also works.
//! - [`detect_format_from_headers`] inspects the `content-type` header with no extra deps.
//! - `AutoFormat: Format` + `DeserializeWith<AutoFormat>` compiles; `into_model()` on
//!   `Response<T, AutoFormat>` works but always uses JSON.
//!
//! ### What didn't work
//! - Blanket default impl for `SelectFormat` (requires `#![feature(specialization)]`).
//! - Making `AutoFormat: Format` header-aware (`Format::deserialize` has no headers).
//!
//! ### Viability without nightly features
//! Yes — `AutoResponse<T>` + closures + explicit `SelectFormat` impls is 100% stable
//! Rust.
//!
//! ### Ergonomic cost for crate authors
//! - **Closure path** (`into_model_with` / `into_model_auto`): zero extra impl; callers
//!   pass [`detect_format_from_headers`] or their own closure.
//! - **SelectFormat path** (`into_model`): one explicit `impl SelectFormat for MyType`
//!   per model type (~3 lines). A `#[derive(SelectFormat)]` proc-macro would eliminate
//!   this boilerplate entirely.
//!
//! ### Minimal changes to existing public APIs that would make this cleaner
//! - A new `DeserializeWithHeaders<F>` trait with
//!   `fn deserialize_with_headers(headers: &Headers, body: ResponseBody) -> Result<Self>`
//!   would make header-aware dispatch first-class in `Response<T, F>` without breaking
//!   the existing `DeserializeWith` impls.
//! - A `#[derive(SelectFormat)]` proc-macro accepting a `content_type = "application/xml"`
//!   attribute would eliminate per-type boilerplate when XML is the intended format.

use crate::http::{
    headers::{Headers, CONTENT_TYPE},
    response::ResponseBody,
    DeserializeWith, Format, RawResponse, StatusCode,
};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

/// Indicates whether a response body should be deserialized as JSON or XML.
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
/// `specialization` nightly feature, which is unavailable on stable Rust. Types that
/// do not need custom format selection can call
/// [`AutoResponse::into_model_auto`] (automatic content-type detection) or
/// [`AutoResponse::into_model_with`] (caller-provided closure) instead.
pub trait SelectFormat {
    /// Return the deserialization format to use for a response with these headers.
    fn select_format(headers: &Headers) -> FormatChoice;
}

/// A [`Format`] that falls back to JSON deserialization.
///
/// This type implements [`Format`] so that `Response<T, AutoFormat>` is a valid type
/// and `into_model()` compiles; however, because [`Format::deserialize`] does not
/// receive response headers, this implementation **always uses JSON** regardless of
/// the `content-type` header.
///
/// For header-based dispatch, use [`AutoResponse<T>`] instead.
#[derive(Debug, Clone)]
pub struct AutoFormat;

impl Format for AutoFormat {
    fn deserialize<T: DeserializeOwned, S: AsRef<[u8]>>(body: S) -> crate::Result<T> {
        crate::json::from_json(body)
    }
}

impl<D: DeserializeOwned> DeserializeWith<AutoFormat> for D {
    fn deserialize_with(body: ResponseBody) -> typespec::Result<Self> {
        body.json()
    }
}

/// A typed fully-buffered HTTP response that selects JSON or XML deserialization
/// based on the response headers **at runtime**.
///
/// Unlike [`crate::http::Response<T, F>`], where the format is fixed at compile time,
/// `AutoResponse<T>` defers the format decision to one of three mechanisms:
///
/// 1. [`into_model`](AutoResponse::into_model) — requires `T: SelectFormat`; the type
///    itself declares which format to use.
/// 2. [`into_model_auto`](AutoResponse::into_model_auto) — inspects the `content-type`
///    header automatically via [`detect_format_from_headers`].
/// 3. [`into_model_with`](AutoResponse::into_model_with) — caller provides a closure.
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
    /// Deserialize the body using the format selected by `selector`.
    ///
    /// # Arguments
    /// * `selector` — a closure that receives the response headers and returns a
    ///   [`FormatChoice`].
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::Deserialize;
    /// # use typespec_client_core::http::{RawResponse, StatusCode, headers::Headers};
    /// # use typespec_client_core::http::auto_format::{AutoResponse, FormatChoice};
    /// # #[derive(Debug, Deserialize)] struct MyModel { name: String }
    /// let raw = RawResponse::from_bytes(
    ///     StatusCode::Ok, Headers::new(), r#"{"name":"test"}"#,
    /// );
    /// let resp: AutoResponse<MyModel> = raw.into();
    /// let model = resp.into_model_with(|_headers| FormatChoice::Json).unwrap();
    /// assert_eq!(model.name, "test");
    /// ```
    pub fn into_model_with<F>(self, selector: F) -> crate::Result<T>
    where
        F: Fn(&Headers) -> FormatChoice,
    {
        let (_, headers, body) = self.raw.deconstruct();
        match selector(&headers) {
            FormatChoice::Json => body.json(),
            FormatChoice::Xml => body.xml(),
        }
    }

    /// Deserialize the body by automatically detecting the format from the
    /// `content-type` response header.
    ///
    /// Delegates to [`detect_format_from_headers`] for format selection.
    pub fn into_model_auto(self) -> crate::Result<T> {
        self.into_model_with(detect_format_from_headers)
    }
}

impl<T: DeserializeOwned + SelectFormat> AutoResponse<T> {
    /// Deserialize the body using the format returned by
    /// [`T::select_format`](SelectFormat::select_format).
    pub fn into_model(self) -> crate::Result<T> {
        self.into_model_with(T::select_format)
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
    use crate::http::{headers::Headers, RawResponse, Response, StatusCode};
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
        let model = resp
            .into_model_with(|_| FormatChoice::Json)
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
            .into_model_with(|_| FormatChoice::Xml)
            .expect("deserializes with XML closure");
        assert_eq!(
            model,
            Widget {
                id: 6,
                name: "widget-f".into()
            }
        );
    }

    // --- AutoFormat on Response<T, AutoFormat> ---

    #[test]
    fn auto_format_response_always_uses_json() {
        // AutoFormat: Format falls back to JSON since Format::deserialize has no headers.
        let raw = RawResponse::from_bytes(
            StatusCode::Ok,
            Headers::new(),
            r#"{"id":7,"name":"widget-g"}"#,
        );
        let resp: Response<Widget, AutoFormat> = raw.into();
        let model = resp.into_model().expect("AutoFormat always uses JSON");
        assert_eq!(
            model,
            Widget {
                id: 7,
                name: "widget-g".into()
            }
        );
    }
}
