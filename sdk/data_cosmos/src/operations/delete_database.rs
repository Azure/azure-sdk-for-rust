use crate::headers::CommonHeaders;
use crate::prelude::*;
use azure_core::Response as HttpResponse;

operation! {
    DeleteDatabase,
    client: DatabaseClient,
    ?consistency_level: ConsistencyLevel
}

impl DeleteDatabaseBuilder {
    pub fn into_future(self) -> DeleteDatabase {
        Box::pin(async move {
            let mut request = self.client.database_request(azure_core::Method::Delete);
            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }

            let response = self
                .client
                .cosmos_client()
                .send(request, self.context.clone(), ResourceType::Databases)
                .await?;
            DeleteDatabaseResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeleteDatabaseResponse {
    pub common: CommonHeaders,
}

impl DeleteDatabaseResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let headers = response.headers();

        Ok(Self {
            common: CommonHeaders::try_from(headers)?,
        })
    }
}
