use azure_core::ClientOptions;

#[derive(Clone, Debug, Default)]
pub struct OpenAIClientOptions {
    pub(crate) client_options: ClientOptions,
}
