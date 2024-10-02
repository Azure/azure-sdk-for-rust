// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
use azure_core::ClientOptions;

/// Options to be passed to [`OpenAIClient`](crate::clients::OpenAIClient).
///
/// Note: There are currently no options to be set.
/// This struct is a placeholder for future options.
// TODO: I was not  able to  find ClientOptions as a derive macros
#[derive(Clone, Debug, Default)]
pub struct OpenAIClientOptions {
    pub(crate) client_options: ClientOptions,
}

impl OpenAIClientOptions {
    /// Creates a new [`builders::OpenAIClientOptionsBuilder`].
    pub fn builder() -> builders::OpenAIClientOptionsBuilder {
        builders::OpenAIClientOptionsBuilder::new()
    }
}

/// Builder to construct a [`OpenAIClientOptions`].
pub mod builders {
    use super::*;

    #[derive(Clone, Debug, Default)]
    pub struct OpenAIClientOptionsBuilder {
        options: OpenAIClientOptions,
    }

    impl OpenAIClientOptionsBuilder {
        pub(super) fn new() -> Self {
            Self::default()
        }

        /// Builds the [`OpenAIClientOptions`].
        ///
        /// # Examples
        ///
        /// ```rust
        /// let options = azure_openai_inference::OpenAIClientOptions::builder().build();
        /// ```
        pub fn build(&self) -> OpenAIClientOptions {
            self.options.clone()
        }
    }
}
