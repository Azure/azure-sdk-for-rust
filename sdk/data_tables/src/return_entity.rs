use azure_core::Header;
use http::StatusCode;

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnEntity(bool);

impl ReturnEntity {
    pub fn new(s: impl Into<bool>) -> Self {
        Self(s.into())
    }

    pub(crate) fn expected_return_code(&self) -> StatusCode {
        match self.0 {
            true => StatusCode::CREATED,
            false => StatusCode::NO_CONTENT,
        }
    }
}

impl Header for ReturnEntity {
    fn name(&self) -> &'static str {
        "Prefer"
    }

    fn value(&self) -> String {
        match self.0 {
            true => "return-content",
            false => "return-no-content",
        }
        .to_owned()
    }
}

impl From<bool> for ReturnEntity {
    fn from(s: bool) -> Self {
        Self::new(s)
    }
}
