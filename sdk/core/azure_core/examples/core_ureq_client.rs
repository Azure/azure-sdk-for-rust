// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{
    error::ErrorKind,
    http::{headers::Headers, BufResponse, ClientOptions, HttpClient, Request, Transport},
};
use azure_identity::DeveloperToolsCredential;
use azure_security_keyvault_secrets::{ResourceExt as _, SecretClient, SecretClientOptions};
use futures::TryStreamExt;
use std::{env, sync::Arc};
use typespec::error::ResultExt;
use ureq::tls::{TlsConfig, TlsProvider};

#[derive(Debug)]
struct Agent(ureq::Agent);

impl Default for Agent {
    fn default() -> Self {
        Self(
            ureq::Agent::config_builder()
                .https_only(true)
                .tls_config(
                    TlsConfig::builder()
                        .provider(TlsProvider::NativeTls)
                        .build(),
                )
                .build()
                .into(),
        )
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl HttpClient for Agent {
    async fn execute_request(&self, request: &Request) -> azure_core::Result<BufResponse> {
        let request = into_request(request)?;
        let response = self
            .0
            .run(request)
            .with_context(ErrorKind::Io, || "failed to send request")?;

        into_response(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vault_url = env::var("AZURE_KEYVAULT_URL")
        .map_err(|_| "Environment variable AZURE_KEYVAULT_URL is required")?;

    let credential = DeveloperToolsCredential::new(None)?;

    let agent = Arc::new(Agent::default());
    let options = SecretClientOptions {
        client_options: ClientOptions {
            transport: Some(Transport::new(agent)),
            ..Default::default()
        },
        ..Default::default()
    };

    let client = SecretClient::new(&vault_url, credential.clone(), Some(options))?;
    let mut pager = client.list_secret_properties(None)?;
    while let Some(secret) = pager.try_next().await? {
        let name = secret.resource_id()?.name;
        println!("Secret: {name}");
    }

    Ok(())
}

fn into_request(request: &Request) -> azure_core::Result<::http::Request<Vec<u8>>> {
    use ::http::{HeaderName, HeaderValue, Request};
    use azure_core::Bytes;

    let mut req: Request<Vec<u8>> = Default::default();
    *req.uri_mut() = request
        .url()
        .as_str()
        .parse()
        .with_context(ErrorKind::DataConversion, || "failed to parse url")?;
    *req.method_mut() = request
        .method()
        .as_str()
        .parse()
        .with_context(ErrorKind::DataConversion, || "failed to parse method")?;
    let headers = req.headers_mut();
    for (name, value) in request.headers().iter() {
        headers.insert(
            HeaderName::from_bytes(name.as_str().as_bytes())
                .with_context(ErrorKind::DataConversion, || "failed to parse header name")?,
            HeaderValue::from_bytes(value.as_str().as_bytes())
                .with_context(ErrorKind::DataConversion, || "failed to parse header value")?,
        );
    }
    let body: Bytes = request.body().into();
    *req.body_mut() = body.into();

    Ok(req)
}

fn into_response(response: ::http::Response<ureq::Body>) -> azure_core::Result<BufResponse> {
    use ::http::response::Parts;
    use azure_core::http::StatusCode;

    let (
        Parts {
            status, headers, ..
        },
        mut body,
    ) = response.into_parts();

    let status: StatusCode = status.as_u16().into();
    let mut response_headers = Headers::new();
    for (name, value) in headers.iter() {
        response_headers.insert(
            name.as_str().to_ascii_lowercase(),
            value
                .to_str()
                .with_context(ErrorKind::DataConversion, || "failed to parse header value")?
                .to_string(),
        );
    }
    let body: Vec<u8> = body
        .read_to_vec()
        .with_context(ErrorKind::Io, || "failed to read response body")?;

    Ok(BufResponse::from_bytes(status, response_headers, body))
}
