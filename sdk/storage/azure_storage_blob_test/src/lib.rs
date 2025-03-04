// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::Uuid;
use azure_core_test::Recording;
use azure_storage_blob::BlobClientOptions;

pub async fn recorded_test_setup(
    recording: &Recording,
    mut options: BlobClientOptions,
) -> (BlobClientOptions, String) {
    recording.instrument(&mut options.client_options);
    let endpoint = format!(
        "https://{}.blob.core.windows.net/",
        recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
    );

    (options, endpoint)
}
