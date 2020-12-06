use crate::clients::TableClient;
use http::method::Method;
use http::status::StatusCode;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct InsertEntityBuilder<'a, E>
where
    E: Serialize,
{
    table_client: &'a TableClient,
    entity: &'a E,
}

impl<'a, E> InsertEntityBuilder<'a, E>
where
    E: Serialize,
{
    pub(crate) fn new(table_client: &'a TableClient, entity: &'a E) -> Self {
        Self {
            table_client,
            entity,
        }
    }

    pub async fn execute(&self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let uri = format!(
            "{}/{}",
            self.table_client
                .table_service_client()
                .storage_account_client()
                .table_storage_uri(),
            self.table_client.table_name()
        );

        debug!("generated uri = {}", uri);

        let json = serde_json::to_string(self.entity)?;

        let request = self.table_client.prepare_request(
            &uri,
            &Method::POST,
            &|mut request| {
                request = request.header("Content-Type", "application/json");
                request = request.header("Accept", "application/json;odata=fullmetadata");
                request = request.header("Prefer", "return-no-content");
                request
            },
            Some(json.as_bytes()),
        )?;

        println!("request == {:?}", request);

        let response = self
            .table_client
            .table_service_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, StatusCode::NO_CONTENT)
            .await?;

        println!("response == {:?}", response);

        let body = std::str::from_utf8(response.body())?;
        println!("body == {}", body);

        Ok(())
    }
}
