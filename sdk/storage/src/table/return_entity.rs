use azure_core::AddAsHeader;
use http::request::Builder;
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

impl AddAsHeader for ReturnEntity {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(
            "Prefer",
            match self.0 {
                true => "return-content",
                false => "return-no-content",
            },
        )
    }
}

impl From<bool> for ReturnEntity {
    fn from(s: bool) -> Self {
        Self::new(s)
    }
}
