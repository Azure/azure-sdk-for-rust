use azure_core::ClientOptions;

use crate::AzureServiceVersion;

/// Options to be passed to [`AzureOpenAIClient`](crate::clients::AzureOpenAIClient).
// TODO: I was not  able to  find ClientOptions as a derive macros
#[derive(Clone, Debug, Default)]
pub struct AzureOpenAIClientOptions {
    #[allow(dead_code)]
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

        /// Configures the [`AzureOpenAIClient`](crate::clients::AzureOpenAIClient) to use the specified API version.
        /// If no value is supplied, the latest version will be used as default. See [`AzureServiceVersion::get_latest()`](AzureServiceVersion::get_latest).
        pub fn with_api_version(mut self, api_service_version: AzureServiceVersion) -> Self {
            self.options.api_service_version = api_service_version;
            self
        }

        /// Builds the [`AzureOpenAIClientOptions`].
        ///
        /// # Examples
        ///
        /// ```rust
        /// let options = azure_openai_inference::OpenAIClientOptions::builder().build();
        /// ```
        pub fn build(&self) -> AzureOpenAIClientOptions {
            self.options.clone()
        }
    }
}
