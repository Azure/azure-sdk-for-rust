use azure_core::Response;

use crate::prelude::*;

operation! {
    Connect,
    client: UsageClient,
    input: String,
}

impl ConnectBuilder {
    pub fn into_future(mut self) -> Connect {
        Box::pin(async move {
            let mut request = self.client.request("dbs", azure_core::Method::Post);

            #[derive(Serialize)]
            struct ConnectBody<'a> {
                pub id: &'a str,
            }
            let body = ConnectBody { id: "abcd" };

            request.set_json(&body);

            let response = self
                .client
                .send(&mut request, &mut self.context.clone())
                .await?;
            ConnectResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct ConnectResponse {}

impl ConnectResponse {
    pub async fn try_from(response: Response) -> azure_core::Result<Self> {
        Ok(Self {})
    }
}
