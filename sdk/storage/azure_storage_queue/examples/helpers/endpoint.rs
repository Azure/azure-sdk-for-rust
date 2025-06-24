pub fn get_endpoint() -> String {
    // Retrieve the storage account endpoint from environment variable.
    let endpoint = std::env::var("AZURE_QUEUE_STORAGE_ACCOUNT");
    let endpoint = match endpoint {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Environment variable AZURE_QUEUE_STORAGE_ACCOUNT is not set");
            std::process::exit(1);
        }
    };

    // Validate endpoint format
    if !endpoint.ends_with("/") || !endpoint.starts_with("https://") {
        eprintln!("Endpoint must start with 'https://' and end with '/'");
        std::process::exit(1);
    }
    endpoint
}
