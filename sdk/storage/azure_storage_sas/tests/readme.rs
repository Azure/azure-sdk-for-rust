// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code, unused_variables)]

use azure_storage_common::models::UserDelegationKey;
use include_file::include_markdown;

#[ignore = "only compile doc examples"]
#[test]
fn readme() -> azure_core::Result<()> {
    let udk = UserDelegationKey::default();

    // Each macro invocation is in its own block to prevent errors with duplicate imports.
    include_markdown!("README.md", "read_blob_sas", scope);
    include_markdown!("README.md", "container_ip_range_sas", scope);

    Ok(())
}
