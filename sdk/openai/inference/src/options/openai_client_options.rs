use azure_core::ClientOptions;

#[derive(Clone, Debug, Default)]
pub struct OpenAIClientOptions {
    pub(crate) client_options: ClientOptions,
}

impl OpenAIClientOptions {
    pub fn builder() -> builders::OpenAIClientOptionsBuilder {
        builders::OpenAIClientOptionsBuilder::new()
    }
}

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

        pub fn build(&self) -> OpenAIClientOptions {
            self.options.clone()
        }
    }
}
