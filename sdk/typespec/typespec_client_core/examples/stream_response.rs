// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use futures::{Stream, StreamExt};
use std::{task::Poll, time::Duration};
use typespec_client_core::{
    http::{headers::Headers, Response, StatusCode},
    Bytes,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get a response from a service client.
    let response = get_response()?;

    // Normally you'd deserialize into a type or `collect()` the body,
    // but this better simulates fetching multiple chunks from a slow response.
    let mut body = response.into_body();
    while let Some(data) = body.next().await {
        // Assume bytes are a string in this example.
        let page = String::from_utf8(data?.into())?;
        println!("{page}");
    }
    Ok(())
}

fn get_response() -> typespec_client_core::Result<Response<()>> {
    Ok(Response::new(
        StatusCode::Ok,
        Headers::new(),
        Box::pin(SlowResponse::default()),
    ))
}

#[derive(Debug, Default)]
struct SlowResponse {
    chunk: usize,
}

impl Stream for SlowResponse {
    type Item = typespec_client_core::Result<Bytes>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        const MAX_CHUNKS: usize = 10;
        static DATA: Bytes = Bytes::from_static(b"Hello, world!");

        let self_mut = self.get_mut();
        if self_mut.chunk < MAX_CHUNKS {
            eprintln!("getting partial response...");
            std::thread::sleep(Duration::from_millis(300));
            self_mut.chunk += 1;
            Poll::Ready(Some(Ok(DATA.clone())))
        } else {
            eprintln!("done");
            Poll::Ready(None)
        }
    }
}
