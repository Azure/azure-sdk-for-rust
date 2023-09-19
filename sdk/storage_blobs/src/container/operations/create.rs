use crate::{container::PublicAccess, prelude::*};
use azure_core::Method;
use azure_core::{headers::AsHeaders, headers::Headers, prelude::*};

operation! {
    Create,
    client: ContainerClient,
    ?public_access: PublicAccess,
    ?metadata: Metadata
}

impl CreateBuilder {
    pub fn into_future(mut self) -> Create {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("restype", "container");

            let mut headers = Headers::new();
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }

            for (name, value) in self
                .public_access
                .unwrap_or(PublicAccess::None)
                .as_headers()
            {
                headers.insert(name, value);
            }

            let mut request = ContainerClient::finalize_request(url, Method::Put, headers, None)?;

            let _response = self.client.send(&mut self.context, &mut request).await?;

            // TODO: Capture and return the response headers
            Ok(())
        })
    }
}

type CreateResponse = ();
