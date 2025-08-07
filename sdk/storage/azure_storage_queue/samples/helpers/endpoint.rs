pub fn get_endpoint() -> String {
    // Retrieve the storage account endpoint from environment variable.
    let storage_account_name = std::env::var("AZURE_QUEUE_STORAGE_ACCOUNT_NAME");
    let storage_account_name = match storage_account_name {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Environment variable AZURE_QUEUE_STORAGE_ACCOUNT_NAME is not set");
            std::process::exit(1);
        }
    };

    format!("https://{}.queue.core.windows.net/", storage_account_name)
}

// This function is used only for the queue service client example, hence the `allow(dead_code)` attribute.
#[allow(dead_code)]
pub fn get_secondary_endpoint() -> String {
    // Retrieve the storage account endpoint from environment variable.
    let storage_account_name = std::env::var("AZURE_QUEUE_STORAGE_ACCOUNT_NAME");
    let storage_account_name = match storage_account_name {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Environment variable AZURE_QUEUE_STORAGE_ACCOUNT_NAME is not set");
            std::process::exit(1);
        }
    };

    format!(
        "https://{}-secondary.queue.core.windows.net/",
        storage_account_name
    )
}
