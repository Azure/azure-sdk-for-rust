use azure::core::errors::AzurePathParseError;
use std::borrow::Borrow;

pub trait IntoAzurePath {
    fn container_name(&self) -> Result<&str, AzurePathParseError>;
    fn blob_name(&self) -> Result<&str, AzurePathParseError>;
}

impl<T> IntoAzurePath for (T, T)
where
    T: Borrow<str>,
{
    fn container_name(&self) -> Result<&str, AzurePathParseError> {
        Ok(self.0.borrow())
    }
    fn blob_name(&self) -> Result<&str, AzurePathParseError> {
        Ok(self.1.borrow())
    }
}

impl IntoAzurePath for Borrow<str> {
    fn container_name(&self) -> Result<&str, AzurePathParseError> {
        match split(self.borrow()) {
            Ok((container_name, _blob_name)) => Ok(container_name),
            Err(error) => Err(error),
        }
    }

    fn blob_name(&self) -> Result<&str, AzurePathParseError> {
        match split(self.borrow()) {
            Ok((_container_name, blob_name)) => Ok(blob_name),
            Err(error) => Err(error),
        }
    }
}

fn split<'a>(b: &'a str) -> Result<(&'a str, &'a str), AzurePathParseError> {
    let slash_pos = match b.find('/') {
        Some(p) => p,
        None => return Err(AzurePathParseError::PathSeparatorNotFoundError),
    };

    if b[slash_pos + 1..].contains('/') {
        return Err(AzurePathParseError::MultiplePathSeparatorsFoundError);
    }

    Ok((&b[0..slash_pos], &b[slash_pos + 1..]))
}
