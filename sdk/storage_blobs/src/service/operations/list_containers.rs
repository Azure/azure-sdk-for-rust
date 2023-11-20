use crate::clients::BlobServiceClient;
use crate::container::Container;
use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    headers::Headers,
    prelude::*,
    Method, Pageable, Response,
};
use azure_storage::parsing_xml::{cast_optional, traverse};
use xml::Element;

#[derive(Debug, Clone)]
pub struct ListContainersBuilder {
    client: BlobServiceClient,
    prefix: Option<Prefix>,
    include_metadata: bool,
    include_deleted: bool,
    include_system: bool,
    max_results: Option<MaxResults>,
    context: Context,
}

impl ListContainersBuilder {
    pub(crate) fn new(client: BlobServiceClient) -> Self {
        Self {
            client,
            prefix: None,
            include_metadata: false,
            include_deleted: false,
            include_system: false,
            max_results: None,
            context: Context::new(),
        }
    }

    setters! {
        prefix: Prefix => Some(prefix),
        include_metadata: bool => include_metadata,
        include_deleted: bool => include_deleted,
        include_system: bool => include_system,
        max_results: MaxResults => Some(max_results),
        context: Context => context,
    }

    pub fn into_stream(self) -> Pageable<ListContainersResponse, Error> {
        let make_request = move |continuation: Option<NextMarker>| {
            let this = self.clone();
            let mut ctx = self.context.clone();
            async move {
                let mut url = this.client.url()?;

                url.query_pairs_mut().append_pair("comp", "list");

                this.prefix.append_to_url_query(&mut url);

                if let Some(next_marker) = continuation {
                    next_marker.append_to_url_query(&mut url);
                }

                let mut to_include = vec![];

                if this.include_metadata {
                    to_include.push("metadata");
                }
                if this.include_deleted {
                    to_include.push("deleted");
                }
                if this.include_system {
                    to_include.push("system");
                }
                if !to_include.is_empty() {
                    url.query_pairs_mut()
                        .append_pair("include", &to_include.join(","));
                }
                this.max_results.append_to_url_query(&mut url);

                let mut request =
                    BlobServiceClient::finalize_request(url, Method::Get, Headers::new(), None)?;

                let response = this.client.send(&mut ctx, &mut request).await?;

                ListContainersResponse::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

#[derive(Debug, Clone)]
pub struct ListContainersResponse {
    pub containers: Vec<Container>,
    pub next_marker: Option<String>,
}

impl ListContainersResponse {
    async fn try_from(response: Response) -> azure_core::Result<Self> {
        let body = response.into_body().collect_string().await?;
        let elem: Element = body.parse().map_kind(ErrorKind::Other)?;

        let mut containers = Vec::new();

        for container in traverse(&elem, &["Containers", "Container"], true)? {
            containers.push(Container::parse(container)?);
        }

        let next_marker = match cast_optional::<String>(&elem, &["NextMarker"])? {
            Some(nm) if nm.is_empty() => None,
            Some(nm) => Some(nm),
            None => None,
        };

        Ok(Self {
            containers,
            next_marker,
        })
    }
}

impl Continuable for ListContainersResponse {
    type Continuation = NextMarker;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_marker.clone().map(NextMarker::from)
    }
}
