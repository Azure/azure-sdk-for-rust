use azure_core::{auth::TokenCredential, Result};
use azure_openai_inference::AzureOpenAIClient;

#[tokio::main]
pub async fn main() -> Result<()> {
    let endpoint =
        std::env::var("AZURE_OPENAI_ENDPOINT").expect("Set AZURE_OPENAI_ENDPOINT env variable");
    let secret = std::env::var("AZURE_OPENAI_KEY").expect("Set AZURE_OPENAI_KEY env variable");

    let azure_openai_client = AzureOpenAIClient::new(endpoint, secret)?;

    Ok(())
}
