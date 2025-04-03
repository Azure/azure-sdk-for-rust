// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::ClientOptions;
use azure_core_test::Recording;

pub fn recorded_test_setup(recording: &Recording) -> (ClientOptions, String) {
    let mut client_options = ClientOptions::default();
    recording.instrument(&mut client_options);
    let endpoint = format!(
        "https://{}.blob.core.windows.net/",
        recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
    );

    (client_options, endpoint)
}
