// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

mod test;

use proc_macro::TokenStream;

/// Attribute client library tests to play back recordings, record sessions, or execute tests without recording.
///
/// # Examples
///
/// For live or recorded tests, you must be async and accept a `TestContext`.
/// You may return a `Result<T, E>`.
///
/// ```
/// use azure_core_test::{recorded, TestContext};
///
/// #[recorded::test]
/// async fn test(ctx: TestContext) {
///     todo!()
/// }
/// ```
///
/// For live-only tests, you must be async and may accept a `TestContext`.
/// You may return a `Result<T, E>`.
///
/// ```
/// use azure_core_test::recorded;
///
/// #[recorded::test(live)]
/// async fn test() -> Result<(), Box<dyn std::error::Error>> {
///     todo!()
/// }
/// ```
///
/// Read documentation for `azure_core_test` for more information and examples.
#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    test::parse_test(attr.into(), item.into())
        .map_or_else(|e| e.into_compile_error().into(), |v| v.into())
}
