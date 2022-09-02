#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not supported {}", .0)]
    NotSupported(String),

    #[error("Cannot convert value to requested type")]
    InvalidValueType,
}

#[inline]
pub(crate) fn not_supported_error(field_type: &str, method: &str, alternative: &str) -> Error {
    Error::NotSupported(
        format!("{field_type} cannot be retrived using {method} method. Use {alternative} to access the underlying Amqp Message")
    )
}
