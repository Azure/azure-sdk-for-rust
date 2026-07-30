// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP client policies for use during testing Key Vault clients.

use async_trait::async_trait;
use azure_core::http::{
    policies::{Policy, PolicyResult},
    Context, Method, Request,
};
use std::sync::Arc;

/// Get the latest version for a specific versioned resource.
#[derive(Debug)]
pub struct GetLatestResource {
    pub collection: &'static str,
    pub name: &'static str,
    pub version: &'static str,
}

#[async_trait]
impl Policy for GetLatestResource {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        rewrite_latest_resource_path(request, self.collection, self.name, self.version);
        next[0].send(ctx, request, &next[1..]).await
    }
}

fn rewrite_latest_resource_path(
    request: &mut Request,
    collection: &str,
    name: &str,
    version: &str,
) {
    if request.method() != Method::Get {
        return;
    }

    let expected_path = format!("/{collection}/{name}/{version}");
    if request.url().path() == expected_path {
        request
            .url_mut()
            .set_path(&format!("/{collection}/{name}/"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::Url;

    #[test]
    fn rewrites_matching_get_to_latest_version() {
        let mut request = Request::new(
            Url::parse("https://example.vault.azure.net/secrets/secret-name/secret-version")
                .expect("valid URL"),
            Method::Get,
        );

        rewrite_latest_resource_path(&mut request, "secrets", "secret-name", "secret-version");

        assert_eq!(request.url().path(), "/secrets/secret-name/");
    }

    #[test]
    fn leaves_non_matching_requests_unchanged() {
        let mut list_request = Request::new(
            Url::parse("https://example.vault.azure.net/secrets").expect("valid URL"),
            Method::Get,
        );
        rewrite_latest_resource_path(
            &mut list_request,
            "secrets",
            "secret-name",
            "secret-version",
        );
        assert_eq!(list_request.url().path(), "/secrets");

        let mut post_request = Request::new(
            Url::parse("https://example.vault.azure.net/secrets/secret-name/secret-version")
                .expect("valid URL"),
            Method::Post,
        );
        rewrite_latest_resource_path(
            &mut post_request,
            "secrets",
            "secret-name",
            "secret-version",
        );
        assert_eq!(
            post_request.url().path(),
            "/secrets/secret-name/secret-version"
        );
    }
}
