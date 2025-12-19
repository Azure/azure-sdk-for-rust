use azure_core::http::{LoggingOptions, RequestContent};
use azure_storage_blob::{BlobContainerClient, BlobContainerClientOptions};
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Azurite configuration (local storage emulator)
    // Azurite Blob Service ports:
    // HTTP: 10000
    // HTTPS: 10001 is for Queue service, NOT Blob service
    // For blob service with HTTPS, you need to configure Azurite with custom certs
    let endpoint = env::var("AZURE_STORAGE_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:10000/devstoreaccount1".to_string());

    let container_name = "test-container";
    let blob_name = "hello_world.txt";
    let content = b"Hello, World!";

    // Create blob container client

    // Configure logging to show x-ms-version and x-ms-blob-type headers
    let mut client_options = BlobContainerClientOptions::default();
    client_options.client_options.logging = LoggingOptions {
        additional_allowed_header_names: vec!["x-ms-version".into(), "x-ms-blob-type".into()],
        additional_allowed_query_params: vec![],
    };
    client_options.version = "2024-05-04".to_string();

    let container_client =
        BlobContainerClient::new(&endpoint, container_name, None, Some(client_options))?;
    // Create blob client
    let blob_client = container_client.blob_client(blob_name);

    // Create container if it doesn't exist
    println!("Creating container '{}'...", container_name);
    match container_client.create_container(None).await {
        Ok(_) => println!("Container created successfully"),
        Err(e) => {
            let err_str = e.to_string();
            // 204 means container already exists in some implementations
            // 409 is the standard "ContainerAlreadyExists" error
            if err_str.contains("ContainerAlreadyExists")
                || err_str.contains("409")
                || err_str.contains("204")
            {
                println!("Container already exists, continuing...");
            } else {
                return Err(e.into());
            }
        }
    }

    // Upload the file
    println!("\nUploading blob '{}'...", blob_name);
    blob_client
        .upload(
            RequestContent::from(content.to_vec()),
            true, // overwrite if exists
            content.len() as u64,
            None,
        )
        .await?;
    println!("Blob uploaded successfully");

    // Download the file
    println!("\nDownloading blob '{}'...", blob_name);
    let response = blob_client.download(None).await?;
    let (_, _, body) = response.deconstruct();
    let downloaded_content = body.collect().await?;

    // Print the contents to stdout
    println!("\n=== File Contents ===");
    println!("{}", String::from_utf8_lossy(&downloaded_content));
    println!("=====================");

    Ok(())
}
