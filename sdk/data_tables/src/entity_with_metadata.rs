use crate::EntityMetadata;
use azure_core::CollectedResponse;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct EntityWithMetadata<E: DeserializeOwned> {
    pub metadata: EntityMetadata,
    pub entity: E,
}

impl<S> TryFrom<CollectedResponse> for EntityWithMetadata<S>
where
    S: DeserializeOwned,
{
    type Error = azure_core::Error;

    fn try_from(response: CollectedResponse) -> Result<Self, Self::Error> {
        Ok(EntityWithMetadata {
            metadata: response.json()?,
            entity: response.json()?,
        })
    }
}
