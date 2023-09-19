use crate::{container::public_access_from_header, prelude::*};
use azure_core::{headers::*, prelude::*, Body, Method};
use azure_storage::StoredAccessPolicyList;

operation! {
    SetACL,
    client: ContainerClient,
    public_access: PublicAccess,
    ?if_modified_since: IfModifiedSinceCondition,
    ?stored_access_policy_list: StoredAccessPolicyList,
    ?lease_id: LeaseId
}

impl SetACLBuilder {
    pub fn into_future(mut self) -> SetACL {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("restype", "container");
            url.query_pairs_mut().append_pair("comp", "acl");

            let xml = self.stored_access_policy_list.map(|xml| xml.to_xml());

            let mut headers = Headers::new();
            for (name, value) in self.public_access.as_headers() {
                headers.insert(name, value);
            }
            headers.add(self.lease_id);
            headers.add(self.if_modified_since);

            let mut request =
                ContainerClient::finalize_request(url, Method::Put, headers, xml.map(Body::from))?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            public_access_from_header(response.headers())
        })
    }
}

type SetACLResponse = PublicAccess;
