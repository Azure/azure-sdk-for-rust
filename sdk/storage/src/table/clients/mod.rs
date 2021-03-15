mod entity_client;
mod table_client;
mod table_service_client;
pub use entity_client::{AsEntityClient, EntityClient};
pub use table_client::{AsTableClient, TableClient};
pub use table_service_client::{AsTableServiceClient, TableServiceClient};
