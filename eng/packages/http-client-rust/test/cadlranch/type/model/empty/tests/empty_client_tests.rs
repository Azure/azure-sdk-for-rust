// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.

use cadl_empty::models::{EmptyInput, EmptyInputOutput, EmptyOutput};
use cadl_empty::EmptyClient;

#[async_std::test]
async fn get_empty() {
    let client = EmptyClient::with_no_credential("http://localhost:3000", None).unwrap();
    let resp = client.get_empty(None).await.unwrap();
    let _value: EmptyOutput = resp.try_into().unwrap();
}

#[async_std::test]
async fn post_round_trip_empty() {
    let client = EmptyClient::with_no_credential("http://localhost:3000", None).unwrap();
    let req = EmptyInputOutput::try_into(EmptyInputOutput::default()).unwrap();
    let resp = client.post_round_trip_empty(req, None).await.unwrap();
    let _value: EmptyInputOutput = resp.try_into().unwrap();
}

#[async_std::test]
async fn put_empty() {
    let client = EmptyClient::with_no_credential("http://localhost:3000", None).unwrap();
    let req = EmptyInput::try_into(EmptyInput::default()).unwrap();
    let _resp = client.put_empty(req, None).await.unwrap();
}
