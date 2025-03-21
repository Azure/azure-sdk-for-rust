// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Endpoints for Azure Resource Manager in different Azure clouds
pub mod resource_manager_endpoint {
    static_url!(
        /// Azure Resource Manager China cloud endpoint
        AZURE_CHINA_CLOUD,
        "https://management.chinacloudapi.cn"
    );

    static_url!(
        /// Azure Resource Manager Germany cloud endpoint
        AZURE_GERMANY_CLOUD,
        "https://management.microsoftazure.de"
    );

    static_url!(
        /// Azure Resource Manager public cloud endpoint
        AZURE_PUBLIC_CLOUD,
        "https://management.azure.com"
    );

    static_url!(
        /// Azure Resource Manager US government cloud endpoint
        AZURE_US_GOVERNMENT_CLOUD,
        "https://management.usgovcloudapi.net"
    );
}

/// A list of known Azure authority hosts
pub mod authority_hosts {
    static_url!(
        /// China-based Azure Authority Host
        AZURE_CHINA_CLOUD,
        "https://login.chinacloudapi.cn"
    );

    static_url!(
        /// Germany-based Azure Authority Host
        AZURE_GERMANY_CLOUD,
        "https://login.microsoftonline.de"
    );

    static_url!(
        /// US Government Azure Authority Host
        AZURE_US_GOVERNMENT_CLOUD,
        "https://login.microsoftonline.us"
    );

    static_url!(
        /// Public Cloud Azure Authority Host
        AZURE_PUBLIC_CLOUD,
        "https://login.microsoftonline.com"
    );
}
