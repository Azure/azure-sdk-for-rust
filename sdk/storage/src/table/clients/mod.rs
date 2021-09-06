mod entity_client;
mod partition_key_client;
mod table_client;
mod table_service_client;
//pub use entity_client::{AsEntityClient, EntityClient};
//pub use partition_key_client::{AsPartitionKeyClient, PartitionKeyClient};
pub use entity_client::EntityClient;
pub use table_client::{TableClient, TableOptions};
//pub use table_service_client::{AsTableServiceClient, TableServiceClient};
