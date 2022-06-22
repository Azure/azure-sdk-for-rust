use crate::{container::PublicAccess, prelude::*};
use azure_core::{headers::AsHeaders, prelude::*};
use http::method::Method;

#[derive(Debug, Clone)]
pub struct CreateBuilder {
    container_client: ContainerClient,
    public_access: PublicAccess,
    metadata: Option<Metadata>,
    context: Context,
    timeout: Option<Timeout>,
}

impl CreateBuilder {
    pub(crate) fn new(container_client: ContainerClient) -> Self {
        Self {
            container_client,
            public_access: PublicAccess::None,
            metadata: None,
            context: Context::new(),
            timeout: None,
        }
    }

    setters! {
        public_access: PublicAccess => public_access,
        metadata: Metadata => Some(metadata),

        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.container_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("restype", "container");

            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.container_client
                    .prepare_request(url.as_str(), Method::PUT, None)?;
            for (name, value) in self.public_access.as_headers() {
                request.headers_mut().insert(name, value);
            }
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    request.add_mandatory_header(&m);
                }
            }

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
impl std::future::IntoFuture for CreateBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
