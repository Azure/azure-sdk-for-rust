use crate::prelude::*;
use azure_core::{headers::Headers, CollectedResponse, Method};
use url::Url;

operation! {
    GetCertificateVersions,
    client: CertificateClient,
    name: String,
}

impl GetCertificateVersionsBuilder {
    pub fn into_future(mut self) -> GetCertificateVersions {
        Box::pin(async move {
            let mut versions = Vec::<CertificateProperties>::new();

            let mut uri = self.client.client.vault_url.clone();
            uri.set_path(&format!("certificates/{}/versions", self.name));

            loop {
                let headers = Headers::new();
                let mut request =
                    self.client
                        .client
                        .finalize_request(uri, Method::Get, headers, None)?;

                let response = self
                    .client
                    .client
                    .send(&mut self.context, &mut request)
                    .await?;

                let response = CollectedResponse::from_response(response).await?;
                let body = response.body();

                let response = serde_json::from_slice::<KeyVaultGetCertificatesResponse>(body)?;

                versions.extend(
                    response
                        .value
                        .into_iter()
                        .map(|s| CertificateProperties {
                            id: s.id.to_owned(),
                            name: self.name.to_string(),
                            version: s.id.split('/').collect::<Vec<_>>()[5].to_string(),
                            enabled: s.attributes.enabled,
                            created_on: s.attributes.created,
                            updated_on: s.attributes.updated,
                            not_before: s.attributes.nbf,
                            expires_on: s.attributes.exp,
                        })
                        .collect::<Vec<CertificateProperties>>(),
                );
                match response.next_link {
                    None => break,
                    Some(u) => uri = Url::parse(&u).unwrap(),
                }
            }

            // Return the certificate versions sorted by the time modified in descending order.
            versions.sort_by(|a, b| {
                if a.updated_on > b.updated_on {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            Ok(versions)
        })
    }
}

type GetCertificateVersionsResponse = Vec<CertificateProperties>;
