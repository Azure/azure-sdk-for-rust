use http::Response;
use serde::{Deserialize, Serialize};

use crate::service::resources::Configuration;

/// The configuration response
pub type ConfigurationResponse = Configuration;

/// Representation of a multiple configurations response
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MultipleConfigurationResponse(Vec<ConfigurationResponse>);

impl std::convert::TryFrom<Response<bytes::Bytes>> for ConfigurationResponse {
    type Error = crate::Error;

    fn try_from(response: Response<bytes::Bytes>) -> Result<Self, Self::Error> {
        let body = response.body();

        let configuration_response: ConfigurationResponse = serde_json::from_slice(body)?;

        Ok(configuration_response)
    }
}

impl std::convert::TryFrom<Response<bytes::Bytes>> for MultipleConfigurationResponse {
    type Error = crate::Error;

    fn try_from(response: Response<bytes::Bytes>) -> Result<Self, Self::Error> {
        let body = response.body();

        let configuration_response: MultipleConfigurationResponse = serde_json::from_slice(body)?;

        Ok(configuration_response)
    }
}
