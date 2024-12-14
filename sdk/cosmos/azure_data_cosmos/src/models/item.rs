// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::Model;
use serde::de::DeserializeOwned;

/// Returned by Cosmos DB APIs that return a single item.
///
/// In some circumstances, an API that _can_ return an item will **not** return an item.
/// For example, you can use [`ItemOptions`](crate::options::ItemOptions) to configure APIs
/// that write new or updated items to avoid returning the updated item.
// Marking this non-exhaustive would make things very difficult for users.
// We encourage them to pattern match against it, but it's not clear how to deal with the "other" case required by being non_exhaustive.
// This isn't an enum representing a value returned by the server, so we wouldn't add things to this enum later.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Item<T> {
    // Why do we have our own enum and not use Option<T>?
    // We need something we can return from SDK APIs, which means it has to implement azure_core::Model.
    // It's not reasonable to implement azure_core::Model on Option<T> for any T that's deserializable, so we have our own enum.
    // This also allows us to use names that are more relevant to the scenario
    /// Indicates that the server returned the item content.
    Present(T),

    /// Indicates that the server omitted the item content in the response,
    /// as directed by the [`ItemOptions`](crate::options::ItemOptions) used when making the request
    #[default]
    Omitted,
}

impl<T> Item<T> {
    /// Returns the contained [`Item::Present`] value, consuming the `self` value.
    ///
    /// This function will panic if the value is [`Item::Omitted`].
    /// If you want a non-panicking option, either use pattern matching, or call `Item::into()`,
    /// to convert this instance into an [`Option<T>`] and use the functions provided by [`Option<T>`] to unwrap without panicking.
    pub fn unwrap(self) -> T {
        match self {
            Item::Present(t) => t,
            Item::Omitted => panic!("called `Item::unwrap()` on an `Omitted` value"),
        }
    }
}

impl<T> From<Item<T>> for Option<T> {
    /// Creates an [`Option<T>`] from an [`Item<T>`], where [`Item::Present`] maps to [`Option::Some`] and [`Item::Omitted`] maps to [`Option::None`].
    fn from(value: Item<T>) -> Self {
        match value {
            Item::Present(t) => Some(t),
            Item::Omitted => None,
        }
    }
}

// We have a custom implementation of `azure_core::Model` because we interpret an empty body as a special case.
impl<T: DeserializeOwned> Model for Item<T> {
    async fn from_response_body(
        body: azure_core::ResponseBody,
    ) -> typespec_client_core::Result<Self> {
        let bytes = body.collect().await?;
        if bytes.is_empty() {
            Ok(Item::Omitted)
        } else {
            let item = azure_core::json::from_json(bytes)?;
            Ok(Item::Present(item))
        }
    }
}

#[cfg(test)]
mod tests {
    use azure_core::{headers::Headers, Model, Response, StatusCode};
    use serde::Deserialize;

    use crate::models::Item;

    #[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
    struct Product {
        pub id: String,
        pub category: String,
    }

    #[tokio::test]
    pub async fn deserialize_present_item() {
        let json = r#"{"id": "item1", "category": "category1"}"#;
        let response = Response::<()>::from_bytes(StatusCode::Ok, Headers::new(), json);
        let item = Item::<Product>::from_response_body(response.into_raw_body())
            .await
            .unwrap();
        let expected = Product {
            id: "item1".to_string(),
            category: "category1".to_string(),
        };
        assert_eq!(Item::Present(expected.clone()), item);
        assert_eq!(Some(expected), item.into());
    }

    #[tokio::test]
    pub async fn deserialize_omitted_item() {
        let json = "";
        let response = Response::<()>::from_bytes(StatusCode::Ok, Headers::new(), json);
        let item = Item::<Product>::from_response_body(response.into_raw_body())
            .await
            .unwrap();
        assert_eq!(Item::Omitted, item);
        assert_eq!(Option::<Product>::None, item.into());
    }
}
