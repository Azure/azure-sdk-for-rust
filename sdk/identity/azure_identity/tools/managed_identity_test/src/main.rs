// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use azure_identity::{ManagedIdentityCredential, ManagedIdentityCredentialOptions, UserAssignedId};
use azure_storage_blob::BlobServiceClient;
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    #[serde()]
    credential: String,
    #[serde(rename = "storage-name")]
    storage_name: String,
    #[serde(rename = "client-id")]
    client_id: Option<String>,
    #[serde(rename = "object-id")]
    object_id: Option<String>,
    #[serde(rename = "resource-id")]
    resource_id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("Server running on http://{}", listener.local_addr()?);

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
    match params.credential.as_str() {
        "mic" => match test_mic(params).await {
            Ok(response) => (StatusCode::OK, response).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        },
        credential => (
            StatusCode::BAD_REQUEST,
            format!("Unknown credential type '{credential}'"),
        )
            .into_response(),
    }
}

async fn test_mic(params: Params) -> Result<String, Box<dyn std::error::Error>> {
    let storage_name = &params.storage_name;
    let client_id = params.client_id.as_ref();
    let object_id = params.object_id.as_ref();
    let resource_id = params.resource_id.as_ref();

    if [client_id, object_id, resource_id]
        .iter()
        .filter(|param| param.is_some())
        .count()
        > 1
    {
        return Err(
            "Only one identity parameter (client-id, object-id, or resource-id) can be specified"
                .into(),
        );
    }

    let user_assigned_id = if let Some(client_id) = client_id {
        Some(UserAssignedId::ClientId(client_id.clone()))
    } else if let Some(object_id) = object_id {
        Some(UserAssignedId::ObjectId(object_id.clone()))
    } else if let Some(resource_id) = resource_id {
        Some(UserAssignedId::ResourceId(resource_id.clone()))
    } else {
        None
    };

    let options = ManagedIdentityCredentialOptions {
        user_assigned_id,
        ..Default::default()
    };

    let credential = ManagedIdentityCredential::new(Some(options))
        .map_err(|e| format!("Failed to create ManagedIdentityCredential: {}", e))?;

    let endpoint = format!("https://{}.blob.core.windows.net", storage_name);

    let client = BlobServiceClient::new(endpoint.as_str(), credential, None)?;

    match client.get_properties(None).await {
        Ok(_) => Ok("test passed".to_string()),
        Err(e) => Err(format!("BlobServiceClient::get_properties failed: {:?}", e).into()),
    }
}
