use crate::prelude::*;
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
            uri.set_query(Some(API_VERSION_PARAM));

            loop {
                let resp_body = self
                    .client
                    .client
                    .request(reqwest::Method::GET, uri.to_string(), None)
                    .await?;
                let response =
                    serde_json::from_str::<KeyVaultGetCertificatesResponse>(&resp_body).unwrap();

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
