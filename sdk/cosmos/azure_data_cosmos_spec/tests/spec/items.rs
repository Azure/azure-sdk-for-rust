// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos_spec::{
    assert_status, spec_test, testdata::Item, ExclusiveContainer, SharedContainer, SpecResult,
};

/// A point read returns the stored item unchanged.
#[spec_test]
pub async fn point_read_returns_stored_item(
    #[profile(basic_items)] container: SharedContainer,
) -> SpecResult {
    let expected = container.dataset().get("item-07");

    let response = container
        .client()
        .read_item(expected.partition_key(), &expected.id, None)
        .await?;

    assert_eq!(response.into_model::<Item>()?, expected);
    Ok(())
}

/// An item can be created, read, replaced, and deleted; reading it after the
/// delete returns 404 Not Found.
#[spec_test]
pub async fn crud_lifecycle(container: ExclusiveContainer) -> SpecResult {
    let client = container.client();
    let item = Item::new("lifecycle", "pk-0", 1);
    let pk = item.partition_key();

    client
        .create_item(pk.clone(), &item.id, &item, None)
        .await?;
    let read = client.read_item(pk.clone(), &item.id, None).await?;
    assert_eq!(read.into_model::<Item>()?, item);

    let replaced = Item {
        value: 2,
        ..item.clone()
    };
    client
        .replace_item(pk.clone(), &item.id, &replaced, None)
        .await?;
    let read = client.read_item(pk.clone(), &item.id, None).await?;
    assert_eq!(read.into_model::<Item>()?, replaced);

    client.delete_item(pk.clone(), &item.id, None).await?;
    assert_status!(client.read_item(pk, &item.id, None).await, 404);
    Ok(())
}
