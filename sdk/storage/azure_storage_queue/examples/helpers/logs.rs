use azure_core::{error::Error, http::StatusCode};

/// Helper function to log operation results
pub fn log_operation_result<T>(result: &Result<T, Error>, operation: &str)
where
    T: std::fmt::Debug,
{
    match result {
        Ok(response) => println!("Successfully {}: {:?}", operation, response),
        Err(e) => match e.http_status() {
            Some(StatusCode::NotFound) => println!("Unable to {}, resource not found", operation),
            Some(StatusCode::Forbidden) => println!(
                "Unable to {}, access forbidden - check credentials",
                operation
            ),
            _ => {
                eprintln!("Error during {}: {}", operation, e);
                if let Some(status) = e.http_status() {
                    eprintln!("HTTP Status: {}", status);
                }
                eprintln!("Full Error: {:#?}", e);
            }
        },
    }
}
