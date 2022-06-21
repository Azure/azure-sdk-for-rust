use crate::EntityMetadata;
use azure_core::CollectedResponse;
use serde::de::DeserializeOwned;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct EntityWithMetadata<E: DeserializeOwned> {
    pub metadata: EntityMetadata,
    pub entity: E,
}

impl<S> TryFrom<CollectedResponse> for EntityWithMetadata<S>
where
    S: DeserializeOwned,
{
    type Error = serde_json::Error;

    fn try_from(response: CollectedResponse) -> Result<Self, Self::Error> {
        Ok(EntityWithMetadata {
            metadata: serde_json::from_slice(response.body())?,
            entity: serde_json::from_slice(response.body())?,
        })
    }
}
