use crate::blob_storage::prelude::*;
use crate::container::PublicAccess;
use azure_core::headers::{add_mandatory_header, add_optional_header};
use azure_core::prelude::*;
use http::method::Method;
use http::status::StatusCode;

#[derive(Debug, Clone)]
pub struct CreateBuilder<'a> {
    container_client: &'a ContainerClient,
    public_access: PublicAccess,
    metadata: Option<&'a Metadata>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> CreateBuilder<'a> {
    pub(crate) fn new(container_client: &'a ContainerClient) -> Self {
        Self {
            container_client,
            public_access: PublicAccess::None,
            metadata: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        public_access: PublicAccess => public_access,
        metadata: &'a Metadata => Some(metadata),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self
            .container_client
            .storage_client()
            .storage_account_client()
            .blob_storage_url()
            .join(self.container_client.container_name())?;

        url.query_pairs_mut().append_pair("restype", "container");

        self.timeout.append_to_url_query(&mut url);

        let request = self.container_client.prepare_request(
            url.as_str(),
            &Method::PUT,
            &|mut request| {
                request = add_mandatory_header(&self.public_access, request);
                request = add_optional_header(&self.metadata, request);
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let _response = self
            .container_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, StatusCode::CREATED)
            .await?;

        // TODO: Capture and return the response headers
        Ok(())
    }
}
