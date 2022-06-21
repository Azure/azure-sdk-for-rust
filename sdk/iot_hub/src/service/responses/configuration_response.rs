use crate::service::resources::Configuration;
use azure_core::error::Error;
use serde::{Deserialize, Serialize};

/// The configuration response
pub type ConfigurationResponse = Configuration;

/// Representation of a multiple configurations response
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MultipleConfigurationResponse(Vec<ConfigurationResponse>);

impl std::convert::TryFrom<crate::service::CollectedResponse> for ConfigurationResponse {
    type Error = Error;

    fn try_from(response: crate::service::CollectedResponse) -> azure_core::Result<Self> {
        let body = response.body();

        let configuration_response: ConfigurationResponse = serde_json::from_slice(body)?;

        Ok(configuration_response)
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
