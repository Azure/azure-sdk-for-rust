// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.

use cadl_usage::models::{InputOutputRecord, InputRecord, OutputRecord};
use cadl_usage::UsageClient;

#[async_std::test]
async fn input() {
    let client = UsageClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut input_record = InputRecord::default();
    input_record.required_prop = Some(String::from("example-value"));
    let req = input_record.try_into().unwrap();
    let _resp = client.input(req, None).await.unwrap();
}

#[async_std::test]
async fn input_and_output() {
    let client = UsageClient::with_no_credential("http://localhost:3000", None).unwrap();
    let mut io_record = InputOutputRecord::default();
    io_record.required_prop = Some(String::from("example-value"));
    let req = io_record.try_into().unwrap();
    let resp = client.input_and_output(req, None).await.unwrap();
    let value: InputOutputRecord = resp.try_into().unwrap();
    assert_eq!(value.required_prop, Some(String::from("example-value")));
}

#[async_std::test]
async fn output() {
    let client = UsageClient::with_no_credential("http://localhost:3000", None).unwrap();
    let resp = client.output(None).await.unwrap();
    let value: OutputRecord = resp.try_into().unwrap();
    assert_eq!(value.required_prop, Some(String::from("example-value")));
}
