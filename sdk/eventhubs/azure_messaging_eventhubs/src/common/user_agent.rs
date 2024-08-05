// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell: words AZSDK RUSTC consts

use std::env::consts::{ARCH, OS};

const UNKNOWN: &str = "unknown";

pub(crate) fn get_user_agent(application_id: &Option<String>) -> String {
    let rustc_version = option_env!("AZSDK_RUSTC_VERSION").unwrap_or(UNKNOWN);
    let mut crate_name = get_package_name();
    let crate_version = get_package_version();
    let platform_info = format!("({rustc_version}; {OS}; {ARCH})",);

    if let Some(name) = crate_name.strip_prefix("azure_") {
        crate_name = name.to_string();
    }

    match &application_id {
        Some(application_id) => {
            format!("{application_id} azsdk-rust-{crate_name}/{crate_version} {platform_info}")
        }
        None => format!("azsdk-rust-{crate_name}/{crate_version} {platform_info}"),
    }
}

pub(crate) fn get_package_version() -> String {
    option_env!("CARGO_PKG_VERSION")
        .unwrap_or(UNKNOWN)
        .to_string()
}

pub(crate) fn get_platform_info() -> String {
    OS.to_string()
}

pub(crate) fn get_package_name() -> String {
    option_env!("CARGO_PKG_NAME").unwrap_or(UNKNOWN).to_string()
}
