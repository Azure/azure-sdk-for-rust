use crate::authorization_policy::ResourceType;
use crate::models::DatabaseProperties;
use crate::{CosmosClient, ReadDatabaseOptions};
use azure_core::{Context, Request, Response};
use serde::Deserialize;
use time::OffsetDateTime;
use url::Url;

pub trait DatabaseClientMethods {
    // TODO:
    async fn read(
        &self,
        options: Option<ReadDatabaseOptions>,
    ) -> azure_core::Result<azure_core::Response<DatabaseProperties>>;
}

pub struct DatabaseClient {
    database_id: String,
    base_url: Url,
    root_client: CosmosClient,
}

impl DatabaseClient {
    pub(crate) fn new(root_client: CosmosClient, database_id: String) -> Self {
        let base_url = {
            let mut u = root_client.endpoint().clone();
            {
                let mut segments = u
                    .path_segments_mut()
                    .expect("The root client should have validated the format of the URL");
                segments.push("dbs");
                segments.push(&database_id);
            }
            u
        };

        Self {
            database_id,
            base_url,
            root_client,
        }
    }
}

impl DatabaseClientMethods for DatabaseClient {
    async fn read(
        &self,
        options: Option<ReadDatabaseOptions>,
    ) -> azure_core::Result<azure_core::Response<DatabaseProperties>> {
        let mut req = Request::new(self.base_url.clone(), azure_core::Method::Get);
        let ctx = Context::new().with_value(ResourceType::Databases);
        self.root_client.pipeline.send(&ctx, &mut req).await
    }
}
