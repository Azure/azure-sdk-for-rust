// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.

use cadl_collectionfmt::CollectionFormatClient;

#[async_std::test]
async fn csv() {
    let client = CollectionFormatClient::with_no_credential("http://localhost:3000", None).unwrap();
    client
        .get_collection_format_header_client()
        .csv(
            vec!["blue".to_string(), "red".to_string(), "green".to_string()],
            None,
        )
        .await
        .unwrap();
}
