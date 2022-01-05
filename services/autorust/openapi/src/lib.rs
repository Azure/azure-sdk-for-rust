mod autorest;
mod contact;
pub mod example;
mod external_documentation;
mod header;
mod info;
mod license;
mod openapi;
mod operation;
mod parameter;
mod paths;
mod reference;
mod schema;
mod security;
mod status_code;
mod tag;

pub use self::{
    autorest::*, contact::*, external_documentation::*, header::*, info::*, license::*, openapi::*, operation::*, parameter::*, paths::*,
    reference::*, schema::*, security::*, status_code::*, tag::*,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("api-version is missing")]
    MissingApiVersion,
}
