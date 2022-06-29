use crate::{container::public_access_from_header, prelude::*};
use azure_core::Method;
use azure_core::{headers::*, prelude::*};
use azure_storage::core::StoredAccessPolicyList;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct SetACLBuilder {
    container_client: ContainerClient,
    public_access: PublicAccess,
    stored_access_policy_list: Option<StoredAccessPolicyList>,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl SetACLBuilder {
    pub(crate) fn new(container_client: ContainerClient, public_access: PublicAccess) -> Self {
        Self {
            container_client,
            public_access,
            stored_access_policy_list: None,
            timeout: None,
            lease_id: None,
            context: Context::new(),
        }
    }

    setters! {
        lease_id: LeaseId => Some(lease_id),
        timeout: Timeout => Some(timeout),
        stored_access_policy_list: StoredAccessPolicyList => Some(stored_access_policy_list),
        context: Context => context,
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.container_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("restype", "container");
            url.query_pairs_mut().append_pair("comp", "acl");

            self.timeout.append_to_url_query(&mut url);

            let xml = self.stored_access_policy_list.map(|xml| xml.to_xml());

            let mut headers = Headers::new();
            for (name, value) in self.public_access.as_headers() {
                headers.insert(name, value);
            }
            headers.add(self.lease_id);

            let mut request = self.container_client.finalize_request(
                url,
                Method::Put,
                headers,
                xml.map(Bytes::from),
            )?;

            let response = self
                .container_client
                .send(&mut self.context, &mut request)
                .await?;

            public_access_from_header(response.headers())
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<PublicAccess>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for SetACLBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
