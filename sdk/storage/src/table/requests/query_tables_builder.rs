use crate::clients::TableServiceClient;
use http::method::Method;
use http::status::StatusCode;

#[derive(Debug, Clone)]
pub struct QueryTablesBuilder<'a> {
    table_service_client: &'a TableServiceClient,
}

impl<'a> QueryTablesBuilder<'a> {
    pub(crate) fn new(table_service_client: &'a TableServiceClient) -> Self {
        Self {
            table_service_client,
        }
    }

    pub async fn execute(&self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let url = self
            .table_service_client
            .storage_account_client()
            .table_storage_url()
            .join("Tables")?;

        // TODO: Add OData query parameters

        debug!("generated url = {}", url);

        let request = self.table_service_client.prepare_request(
            url.as_str(),
            &Method::GET,
            &|mut request| {
                request =
                    request.header(http::header::ACCEPT, "application/json;odata=fullmetadata");
                request
            },
            None,
        )?;

        debug!("request == {:?}", request);

        let response = self
            .table_service_client
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, StatusCode::OK)
            .await?;

        debug!("response == {:?}", response);

        let body = std::str::from_utf8(response.body())?;
        println!("response.headers() == {:?}", response.headers());
        println!("body == {}", body);

        Ok(())
    }
}
