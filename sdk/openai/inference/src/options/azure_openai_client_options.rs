use azure_core::ClientOptions;

use crate::AzureServiceVersion;

// TODO: I was not  able to  find ClientOptions as a derive macros
#[derive(Clone, Debug, Default)]
pub struct AzureOpenAIClientOptions {
    pub(crate) client_options: ClientOptions,
    pub(crate) api_service_version: AzureServiceVersion,
}

impl AzureOpenAIClientOptions {
    pub fn builder() -> builders::AzureOpenAIClientOptionsBuilder {
        builders::AzureOpenAIClientOptionsBuilder::new()
    }
}

pub mod builders {
    use super::*;

    #[derive(Clone, Debug, Default)]
    pub struct AzureOpenAIClientOptionsBuilder {
        options: AzureOpenAIClientOptions,
    }

    impl AzureOpenAIClientOptionsBuilder {
        pub(super) fn new() -> Self {
            Self::default()
        }
        pub fn with_api_version(mut self, api_service_version: AzureServiceVersion) -> Self {
            self.options.api_service_version = api_service_version;
            self
        }

        pub fn build(&self) -> AzureOpenAIClientOptions {
            self.options.clone()
        }
    }
}
