// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use futures::StreamExt;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;

#[cfg_attr(not(target_arch = "wasm32"), tokio::main)]
#[cfg_attr(target_arch = "wasm32", tokio::main(flavor = "current_thread"))]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Log traces to stdout.
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
        .with_max_level(LevelFilter::DEBUG)
        .init();

    // Get a response from a service client.
    let response = client::get_binary_data()?;

    // Normally you'd deserialize into a type or `collect()` the body,
    // but this better simulates fetching multiple chunks from a slow response.
    let mut body = response.into_stream();
    while let Some(data) = body.next().await {
        // Assume bytes are a string in this example.
        let page = String::from_utf8(data?.into())?;
        println!("{page}");
    }

    // You can also deserialize into a model from a slow response.
    let team = client::get_model().await?.into_body()?;
    println!("{team:#?}");

    Ok(())
}

#[allow(dead_code)]
mod client {
    use futures::Stream;
    use serde::Deserialize;
    use std::{cmp::min, task::Poll, time::Duration};
    use tracing::debug;
    use typespec_client_core::{
        http::{headers::Headers, response::AsyncResponse, BufResponse, Response, StatusCode},
        Bytes,
    };

    #[derive(Debug, Deserialize)]
    pub struct Team {
        pub name: Option<String>,
        #[serde(default)]
        pub members: Vec<Person>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Person {
        pub id: u32,
        pub name: Option<String>,
    }

    #[tracing::instrument]
    pub fn get_binary_data() -> typespec_client_core::Result<AsyncResponse<()>> {
        let bytes = Bytes::from_static(b"Hello, world!");
        let response = SlowResponse {
            bytes: bytes.repeat(5).into(),
            bytes_per_read: bytes.len(),
            bytes_read: 0,
        };

        Ok(BufResponse::new(StatusCode::Ok, Headers::new(), Box::pin(response)).into())
    }

    #[tracing::instrument]
    pub async fn get_model() -> typespec_client_core::Result<Response<Team>> {
        let bytes = br#"{
            "name": "Contoso Dev Team",
            "members": [
                {
                    "id": 1234,
                    "name": "Jan"
                },
                {
                    "id": 5678,
                    "name": "Bill"
                }
            ]
        }"#;
        let response = SlowResponse {
            bytes: Bytes::from_static(bytes),
            bytes_per_read: 64,
            bytes_read: 0,
        };
        let stream = BufResponse::new(StatusCode::Ok, Headers::new(), Box::pin(response));
        let buffered = stream.try_into_raw_response().await?;

        Ok(buffered.into())
    }

    struct SlowResponse {
        bytes: Bytes,
        bytes_per_read: usize,
        bytes_read: usize,
    }

    impl Stream for SlowResponse {
        type Item = typespec_client_core::Result<Bytes>;

        fn poll_next(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> Poll<Option<Self::Item>> {
            let self_mut = self.get_mut();
            if self_mut.bytes_read < self_mut.bytes.len() {
                debug!("writing partial response...");

                // Simulate a slow response.
                std::thread::sleep(Duration::from_millis(200));

                let end = self_mut.bytes_read
                    + min(
                        self_mut.bytes_per_read,
                        self_mut.bytes.len() - self_mut.bytes_read,
                    );
                let bytes = self_mut.bytes.slice(self_mut.bytes_read..end);
                self_mut.bytes_read += bytes.len();
                Poll::Ready(Some(Ok(bytes)))
            } else {
                debug!("done");
                Poll::Ready(None)
            }
        }
    }
}
