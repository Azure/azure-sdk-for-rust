use crate::{container::operations::AcquireLeaseResponse, prelude::*};
use azure_core::Method;
use azure_core::{headers::*, prelude::*};

pub type RenewLeaseResponse = AcquireLeaseResponse;

#[derive(Debug, Clone)]
pub struct RenewLeaseBuilder {
    container_lease_client: ContainerLeaseClient,
    timeout: Option<Timeout>,
    context: Context,
}

impl RenewLeaseBuilder {
    pub(crate) fn new(container_lease_client: ContainerLeaseClient) -> Self {
        Self {
            container_lease_client,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        context: Context => context,
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.container_lease_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("restype", "container");
            url.query_pairs_mut().append_pair("comp", "lease");

            self.timeout.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.insert(LEASE_ACTION, "renew");
            headers.add(self.container_lease_client.lease_id());

            let mut request =
                self.container_lease_client
                    .prepare_request(url, Method::Put, headers, None)?;

            let response = self
                .container_lease_client
                .send(&mut self.context, &mut request)
                .await?;

            RenewLeaseResponse::from_headers(response.headers())
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<RenewLeaseResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for RenewLeaseBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
