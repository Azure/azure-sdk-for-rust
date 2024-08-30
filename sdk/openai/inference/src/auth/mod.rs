use azure_core::{
    auth::Secret,
    headers::{HeaderName, HeaderValue, AUTHORIZATION},
    Header,
};

pub struct AzureKeyCredential(Secret);

pub struct OpenAIKeyCredential(Secret);

impl OpenAIKeyCredential {
    pub fn new(access_token: String) -> Self {
        Self(Secret::new(access_token))
    }
}

impl AzureKeyCredential {
    pub fn new(api_key: String) -> Self {
        Self(Secret::new(api_key))
    }
}

impl Header for AzureKeyCredential {
    fn name(&self) -> HeaderName {
        HeaderName::from_static("api-key")
    }

    fn value(&self) -> HeaderValue {
        HeaderValue::from_cow(format!("{}", self.0.secret()))
    }
}

impl Header for OpenAIKeyCredential {
    fn name(&self) -> HeaderName {
        AUTHORIZATION
    }

    fn value(&self) -> HeaderValue {
        HeaderValue::from_cow(format!("Bearer {}", &self.0.secret()))
    }
}
