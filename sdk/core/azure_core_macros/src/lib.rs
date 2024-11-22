// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod recorded;

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
/// use azure_core_macros::recorded;
/// use azure_core_test::TestContext;
/// use azure_core::Result;
///
/// #[recorded(live)]
/// async fn get_secret(ctx: TestContext) -> Result<()> {
///     todo!()
/// }
/// ```
///
/// To execute tests only when `AZURE_TEST_MODE` is "live", you can optionally pass `live` to the `#[recorded(live)]` attribute.

#[proc_macro_attribute]
pub fn recorded(attr: TokenStream, item: TokenStream) -> TokenStream {
    recorded::parse_recorded(attr.into(), item.into())
        .map_or_else(|e| e.into_compile_error().into(), |v| v.into())
}
