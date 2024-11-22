// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

mod test;

#[cfg(doc)]
use azure_core_test::TestContext;
use proc_macro::TokenStream;

/// Attribute client library tests to play back recordings, record sessions, or execute tests without recording.
///
/// # Examples
///
/// Recorded tests can be synchronous or asynchronous (recommended), can return a `Result`,
/// and must take a single [`TestContext`] parameter used to set up to record or play back the HTTP client.
///
/// ```
/// use azure_core_test::{recorded, TestContext};
/// use azure_core::Result;
///
/// #[recorded::test(live)]
/// async fn get_secret(ctx: TestContext) -> Result<()> {
///     todo!()
/// }
/// ```
///
/// To execute tests only when `AZURE_TEST_MODE` is "live", you can optionally pass `live` to the `#[recorded::test(live)]` attribute.
#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    test::parse_test(attr.into(), item.into())
        .map_or_else(|e| e.into_compile_error().into(), |v| v.into())
}
