// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.

use cadl_fixed::models::DaysOfWeekEnum;
use cadl_fixed::FixedClient;

#[async_std::test]
async fn get_known_value() {
    let client = FixedClient::with_no_credential("http://localhost:3000", None).unwrap();
    let resp = client
        .get_fixed_string_client()
        .get_known_value(None)
        .await
        .unwrap();
    let value: DaysOfWeekEnum = resp.try_into().unwrap();
    assert_eq!(value, DaysOfWeekEnum::Monday);
}

#[async_std::test]
async fn put_known_value() {
    let client = FixedClient::with_no_credential("http://localhost:3000", None).unwrap();
    let req = DaysOfWeekEnum::Monday.try_into().unwrap();
    client
        .get_fixed_string_client()
        .put_known_value(req, None)
        .await
        .unwrap();
}

#[async_std::test]
#[ignore]
async fn put_unknown_value() {
    // can't send an arbitrary value for fixed enums in Rust
}
