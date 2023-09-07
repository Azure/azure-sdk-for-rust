/// Example of how to use a custom policy to modify requests before they are sent.
/// This example overrides the version header in the request.
///
/// For more information see:
/// - `Pipeline`: https://docs.rs/azure_core/latest/azure_core/struct.Pipeline.html
/// - `Policy`: https://docs.rs/azure_core/latest/azure_core/trait.Policy.html

#[macro_use]
extern crate log;
use async_trait::async_trait;
use azure_core::error::{ErrorKind, ResultExt};
use azure_core::{headers::HeaderValue, ClientOptions, Context, Policy, PolicyResult, Request};
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use std::sync::Arc;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct VersionHeaderOverridePolicy {}

impl VersionHeaderOverridePolicy {
    fn new() -> Self {
        Default::default()
    }
}

const HEADER_VERSION: &str = "2023-01-03"; // Set this to desired version

// Define a `Policy` that overrides the version header in the request.
#[async_trait]
impl azure_core::Policy for VersionHeaderOverridePolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // Replace the version header with the one we want
        request.insert_header(
            azure_core::headers::VERSION,
            HeaderValue::from_static(HEADER_VERSION),
        );
        // Call the next policy in the chain, and return its response
        next[0].send(ctx, request, &next[1..]).await
    }
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let storage_credentials = StorageCredentials::Key(account.clone(), access_key);

    let version_header_override_policy =
        Arc::new(VersionHeaderOverridePolicy::new()) as Arc<dyn Policy>;

    let client_options: ClientOptions =
        ClientOptions::default().per_call_policies(vec![version_header_override_policy]);

    let service_client = ClientBuilder::new(account, storage_credentials)
        .client_options(client_options)
        .blob_service_client();

    // this is how you would use the SAS token:
    // let storage_client = StorageAccountClient::new_sas_token(http_client.clone(), &account,
    //      "sv=2018-11-09&ss=b&srt=o&se=2021-01-15T12%3A09%3A01Z&sp=r&st=2021-01-15T11%3A09%3A01Z&spr=http,https&sig=some_signature")?;

    let blob_client = service_client
        .container_client(&container)
        .blob_client(&blob);

    trace!("Requesting blob");

    // this is a single call that retrieves the first 1KB of the blob (or less if the blob is
    // smaller). The range(...) call is optional.
    let response = blob_client
        .get()
        .range(0u64..1024)
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;

    println!("{response:#?}");

    let mut complete_response = vec![];
    // this is how you stream a blob. You can specify the range(...) value as above if necessary.
    // In this case we are retrieving the whole blob in 8KB chunks.
    let mut stream = blob_client.get().chunk_size(0x2000u64).into_stream();
    while let Some(value) = stream.next().await {
        let data = value?.data.collect().await?;
        println!("received {:?} bytes", data.len());
        complete_response.extend(&data);
    }

    let s_content = String::from_utf8(complete_response).map_kind(ErrorKind::DataConversion)?;
    println!("s_content == {s_content}");

    Ok(())
}
