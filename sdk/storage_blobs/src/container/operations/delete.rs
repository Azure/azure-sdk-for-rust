use crate::prelude::*;
use azure_core::{headers::Headers, prelude::*, Method};

#[derive(Debug, Clone)]
pub struct DeleteBuilder {
    container_client: ContainerClient,
    lease_id: Option<LeaseId>,
    #[allow(unused)]
    timeout: Option<Timeout>,
    context: Context,
}

impl DeleteBuilder {
    pub(crate) fn new(container_client: ContainerClient) -> Self {
        DeleteBuilder {
            container_client,
            lease_id: None,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        lease_id: LeaseId => Some(lease_id),
        timeout: Timeout => Some(timeout),
        context: Context => context,
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.container_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("restype", "container");

            let mut headers = Headers::new();
            headers.add(self.lease_id);

            let mut request =
                self.container_client
                    .finalize_request(url, Method::Delete, headers, None)?;

            let _response = self
                .container_client
                .send(&mut self.context, &mut request)
                .await?;

            // TODO: Capture and return the response headers
            Ok(())
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<()>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for DeleteBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
