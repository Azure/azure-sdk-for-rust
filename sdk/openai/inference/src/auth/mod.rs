mod azure_key_credential;
mod openai_key_credential;

pub(crate) use azure_key_credential::*;
pub(crate) use openai_key_credential::*;

pub(crate) const DEFAULT_SCOPE: [&'static str; 1] =
    ["https://cognitiveservices.azure.com/.default"];
