// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use azure_core::credentials::TokenCredential;
use azure_identity::{ManagedIdentityCredential, ManagedIdentityCredentialOptions, UserAssignedId};
use azure_storage_blob::BlobServiceClient;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct Params {
    #[serde(rename = "client-id")]
    client_id: Option<String>,
    #[serde(rename = "object-id")]
    object_id: Option<String>,
    #[serde(rename = "resource-id")]
    resource_id: Option<String>,
    #[serde(rename = "storage-name")]
    storage_name: String,
    #[serde()]
    test: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // cspell:ignore CUSTOMHANDLER
    let port = std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("Listening on http://{}", listener.local_addr()?);

    let router = Router::new().route("/api", get(handle_request));
    axum::serve(listener, router)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to handle ctrl-c");
        })
        .await?;

    Ok(())
}

async fn handle_request(Query(params): Query<Params>) -> Response {
    let credential = match params.test.as_str() {
        "managed-identity" => {
            let user_assigned_id = match (
                params.client_id.as_ref(),
                params.object_id.as_ref(),
                params.resource_id.as_ref(),
            ) {
                (Some(id), None, None) => Some(UserAssignedId::ClientId(id.clone())),
                (None, Some(id), None) => Some(UserAssignedId::ObjectId(id.clone())),
                (None, None, Some(id)) => Some(UserAssignedId::ResourceId(id.clone())),
                (None, None, None) => None,
                _ => {
                    return (
                        StatusCode::BAD_REQUEST,
                        "Multiple user-assigned identity parameters",
                    )
                        .into_response()
                }
            };
            let options = ManagedIdentityCredentialOptions {
                user_assigned_id,
                ..Default::default()
            };
            match ManagedIdentityCredential::new(Some(options)) {
                Ok(cred) => cred,
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("ManagedIdentityCredential::new returned '{e}'"),
                    )
                        .into_response()
                }
            }
        }
        test => return (StatusCode::BAD_REQUEST, format!("Unknown test '{test}'")).into_response(),
    };

    match try_storage(credential, &params.storage_name).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn try_storage(
    credential: Arc<dyn TokenCredential>,
    storage_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = format!("https://{}.blob.core.windows.net", storage_name);
    BlobServiceClient::new(endpoint.as_str(), credential, None)?
        .get_properties(None)
        .await
        .map_err(|e| format!("BlobServiceClient::get_properties failed: {:?}", e).into())
        .map(|_| ())
}
