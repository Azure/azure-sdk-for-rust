// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod common;

use common::fs::FileStreamBuilder;
use tokio::fs;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use typespec_client_core::http::RequestContent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Log traces to stdout.
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
        .with_max_level(LevelFilter::DEBUG)
        .init();

    // Asynchronously read the whole file into memory.
    let body = RequestContent::from(fs::read(file!()).await?);
    client::put_binary_data(body).await?;

    // Asynchronously stream the file with the service client request.
    let file = fs::File::open(file!()).await?;
    let file = FileStreamBuilder::new(file)
        // Simulate a slow, chunky request.
        .buffer_size(512usize)
        .build()
        .await?;
    client::put_binary_data(file.into()).await?;

    Ok(())
}

mod client {
    use futures::StreamExt;
    use tracing::debug;
    use typespec_client_core::{
        http::{
            headers::Headers, response::AsyncResponse, AsyncRawResponse, Body, RequestContent,
            StatusCode,
        },
        stream::BytesStream,
        Bytes,
    };

    #[tracing::instrument(skip(body))]
    pub async fn put_binary_data(
        body: RequestContent<Bytes>,
    ) -> typespec_client_core::Result<AsyncResponse> {
        let body: Body = body.into();

        let content = match body {
            Body::Bytes(ref bytes) => {
                debug!("received {} bytes", bytes.len());
                bytes.to_owned()
            }
            Body::SeekableStream(mut stream) => {
                debug!("received stream");
                let stream = stream.as_mut();

                let mut bytes = Vec::new();
                while let Some(Ok(buf)) = stream.next().await {
                    debug!("read {} bytes from stream", buf.len());
                    bytes.extend(buf);
                }

                bytes.into()
            }
        };

        // Assume bytes are a string in this example.
        let content = String::from_utf8(content.into())?;
        println!("{content}");

        Ok(AsyncRawResponse::new(
            StatusCode::NoContent,
            Headers::new(),
            Box::pin(BytesStream::new_empty()),
        )
        .into())
    }
}
