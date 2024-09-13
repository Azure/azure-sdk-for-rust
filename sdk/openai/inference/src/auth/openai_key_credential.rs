use azure_core::{
    auth::Secret,
    headers::{HeaderName, HeaderValue, AUTHORIZATION},
    Header,
};

pub struct OpenAIKeyCredential(Secret);

impl OpenAIKeyCredential {
    pub fn new(access_token: String) -> Self {
        Self(Secret::new(access_token))
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
