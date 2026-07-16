// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(unused_attributes)]

use azure_core_test::{recorded, TestContext};

#[ignore = "compile-only macro coverage"]
#[recorded::test(live)]
async fn recorded_macro_supports_live_only_without_context(
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[ignore = "compile-only macro coverage"]
#[recorded::test(live)]
async fn recorded_macro_supports_context(
    _ctx: TestContext,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
