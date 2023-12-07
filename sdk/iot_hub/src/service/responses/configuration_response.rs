use crate::service::resources::Configuration;
use azure_core::error::Error;
use serde::{Deserialize, Serialize};

/// The configuration response
pub type ConfigurationResponse = Configuration;

/// Representation of a multiple configurations response
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MultipleConfigurationResponse(Vec<ConfigurationResponse>);

impl std::convert::TryFrom<crate::service::CollectedResponse> for MultipleConfigurationResponse {
    type Error = Error;
    fn try_from(response: crate::service::CollectedResponse) -> azure_core::Result<Self> {
        response.json()
    }
}
