use crate::{container::PublicAccess, prelude::*};
use azure_core::Method;
use azure_core::{headers::AsHeaders, headers::Headers, prelude::*};

#[derive(Debug, Clone)]
pub struct CreateBuilder {
    container_client: ContainerClient,
    public_access: PublicAccess,
    metadata: Option<Metadata>,
    timeout: Option<Timeout>,
    context: Context,
}

impl CreateBuilder {
    pub(crate) fn new(container_client: ContainerClient) -> Self {
        Self {
            container_client,
            public_access: PublicAccess::None,
            metadata: None,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        public_access: PublicAccess => public_access,
        metadata: Metadata => Some(metadata),
        timeout: Timeout => Some(timeout),
        context: Context => context,
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.container_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("restype", "container");

            self.timeout.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }

            for (name, value) in self.public_access.as_headers() {
                headers.insert(name, value);
            }

            let mut request =
                self.container_client
                    .prepare_request(url, Method::Put, headers, None)?;

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
