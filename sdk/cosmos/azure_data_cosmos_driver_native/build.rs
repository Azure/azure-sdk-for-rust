// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Skeleton build script for `azure_data_cosmos_driver_native`.
//
// In the full implementation this script will invoke `cbindgen` to emit
// `include/azurecosmosdriver.h`. For now it only exposes a build identifier
// string compiled into the binary so consumers can identify exact builds.

fn main() {
    let build_id = format!(
        "$Id: {}, Version: {}, Commit: {}, Branch: {}, Build ID: {}, Build Number: {}, Timestamp: {}$",
        "azurecosmosdriver",
        env!("CARGO_PKG_VERSION"),
        option_env!("BUILD_SOURCEVERSION").unwrap_or("unknown"),
        option_env!("BUILD_SOURCEBRANCH").unwrap_or("unknown"),
        option_env!("BUILD_BUILDID").unwrap_or("unknown"),
        option_env!("BUILD_BUILDNUMBER").unwrap_or("unknown"),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    );
    println!("cargo:rustc-env=BUILD_IDENTIFIER={build_id}");

    // TODO(phase-0-finalize): wire up cbindgen here to emit
    // `include/azurecosmosdriver.h`. See `docs/NATIVE_WRAPPER_SPEC.md` §5.
}
