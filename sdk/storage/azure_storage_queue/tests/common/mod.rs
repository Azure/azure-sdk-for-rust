// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::ClientOptions;
use azure_core_test::Recording;

/// Returns a randomized queue name with prefix "q" of length 12.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
pub fn get_queue_name(recording: &Recording) -> String {
    recording
        .random_string::<12>(Some("q"))
        .to_ascii_lowercase()
}

/// Takes in a Recording instance and returns an instrumented options bag, primary endpoint, and secondary endpoint.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
pub fn recorded_test_setup(recording: &Recording) -> (ClientOptions, String, String) {
    let mut client_options = ClientOptions::default();
    recording.instrument(&mut client_options);
    let account_name = recording.var("AZURE_STORAGE_ACCOUNT_NAME", None);
    let endpoint = format!("https://{}.queue.core.windows.net/", account_name.as_str());
    let secondary_endpoint = format!(
        "https://{}-secondary.queue.core.windows.net/",
        account_name.as_str()
    );
    (client_options, endpoint, secondary_endpoint)
}
