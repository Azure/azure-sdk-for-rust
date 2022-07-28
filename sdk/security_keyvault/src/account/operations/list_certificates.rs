use crate::prelude::*;
use azure_core::{headers::Headers, CollectedResponse, Method};
use url::Url;

operation! {
    ListCertificates,
    client: CertificateClient,
}

impl ListCertificatesBuilder {
    pub fn into_future(mut self) -> ListCertificates {
        Box::pin(async move {
            let mut certificates = Vec::<CertificateProperties>::new();

            let mut uri = self.client.client.vault_url.clone();
            uri.set_path("certificates");

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

                let response =
                    serde_json::from_slice::<KeyVaultGetCertificatesResponse>(body).unwrap();

                certificates.extend(
                    response
                        .value
                        .into_iter()
                        .map(|s| CertificateProperties {
                            id: s.id.to_owned(),
                            name: s.id.split('/').collect::<Vec<_>>()[4].to_string(),
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

            Ok(certificates)
        })
    }
}

type ListCertificatesResponse = Vec<CertificateProperties>;
