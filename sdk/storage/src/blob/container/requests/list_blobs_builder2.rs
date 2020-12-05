use crate::blob::blob::responses::ListBlobsResponse;
use crate::clients::ContainerClient;
use futures::stream::{unfold, Stream};
use http::method::Method;
use http::status::StatusCode;

#[derive(Debug, Clone)]
pub struct ListBlobBuilder2<'a> {
    container_client: &'a ContainerClient,
    next_marker: Option<&'a str>,
}

impl<'a> ListBlobBuilder2<'a> {
    pub(crate) fn new(container_client: &'a ContainerClient) -> Self {
        ListBlobBuilder2 {
            container_client,
            next_marker: None,
        }
    }

    pub fn with_next_marker(self, next_marker: &'a str) -> Self {
        Self {
            next_marker: Some(next_marker),
            ..self
        }
    }

    pub async fn execute(
        &self,
    ) -> Result<ListBlobsResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut uri = format!(
            "{}/{}?restype=container&comp=list",
            self.container_client
                .storage_client()
                .storage_account_client()
                .blob_storage_uri(),
            self.container_client.container_name()
        );

        // TODO: this will be better once PR #110 is accepted
        if let Some(next_marker) = self.next_marker {
            uri = format!("{}&{}", uri, next_marker);
        }

        trace!("list blob uri = {}", uri);

        let request =
            self.container_client
                .prepare_request(&uri, &Method::GET, &|request| request, None)?;

        let response = self
            .container_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, StatusCode::OK)
            .await?;

        Ok(ListBlobsResponse::from_response(
            self.container_client.container_name(),
            response.headers(),
            &std::str::from_utf8(response.body())?,
        )?)
    }
}

impl<'a> ListBlobBuilder2<'a> {
    pub fn stream(
        self,
    ) -> impl Stream<Item = Result<ListBlobsResponse, Box<dyn std::error::Error + Sync + Send>>> + 'a
    {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            NextMarker(String),
        };

        unfold(Some(States::Init), move |next_marker: Option<States>| {
            let req = self.clone();
            async move {
                debug!("next_marker == {:?}", &next_marker);
                let response = match next_marker {
                    Some(States::Init) => req.execute().await,
                    Some(States::NextMarker(next_marker)) => {
                        req.with_next_marker(&next_marker).execute().await
                    }
                    None => return None,
                };

                // the ? operator does not work in async move (yet?)
                // so we have to resort to this boilerplate
                let response = match response {
                    Ok(response) => response,
                    Err(err) => return Some((Err(err), None)),
                };

                let next_marker = match response.incomplete_vector.token() {
                    Some(ct) => Some(States::NextMarker(ct.to_owned())),
                    None => None,
                };

                Some((Ok(response), next_marker))
            }
        })
    }
}
