use crate::service::resources::Configuration;
use azure_core::{error::Error, CollectedResponse};
use serde::{Deserialize, Serialize};

/// The configuration response
pub type ConfigurationResponse = Configuration;

/// Representation of a multiple configurations response
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MultipleConfigurationResponse(Vec<ConfigurationResponse>);

impl ConfigurationResponse {
    pub(crate) async fn try_from(response: azure_core::Response) -> azure_core::Result<Self> {
        let collected = CollectedResponse::from_response(response).await?;
        let body = collected.body();
        Ok(serde_json::from_slice(body)?)
    }
}

impl std::convert::TryFrom<crate::service::CollectedResponse> for MultipleConfigurationResponse {
    type Error = Error;

    fn try_from(response: crate::service::CollectedResponse) -> azure_core::Result<Self> {
        let body = response.body();

        let configuration_response: MultipleConfigurationResponse = serde_json::from_slice(body)?;

        Ok(configuration_response)
    }
}
