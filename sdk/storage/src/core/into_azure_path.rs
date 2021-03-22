use azure_core::errors::AzurePathParseError;
use std::borrow::Borrow;

pub trait IntoAzurePath {
    fn container_name(&self) -> Result<&str, AzurePathParseError>;
    fn blob_name(&self) -> Result<&str, AzurePathParseError>;

    fn components(&self) -> Result<(&str, &str), AzurePathParseError> {
        Ok((self.container_name()?, self.blob_name()?))
    }
}

impl<T> IntoAzurePath for (T, T)
where
    T: Borrow<str> + Clone,
{
    fn container_name(&self) -> Result<&str, AzurePathParseError> {
        Ok(self.0.borrow())
    }
    fn blob_name(&self) -> Result<&str, AzurePathParseError> {
        Ok(self.1.borrow())
    }
}

impl<'a> IntoAzurePath for &'a str {
    fn container_name(&self) -> Result<&str, AzurePathParseError> {
        split(self.borrow()).map(|(container_name, _blob_name)| container_name)
    }

    fn blob_name(&self) -> Result<&str, AzurePathParseError> {
        split(self.borrow()).map(|(_container_name, blob_name)| blob_name)
    }
}

fn split(b: &str) -> Result<(&str, &str), AzurePathParseError> {
    let slash_pos = b
        .find('/')
        .ok_or(AzurePathParseError::PathSeparatorNotFoundError)?;

    if slash_pos == 0 {
        return Err(AzurePathParseError::MissingContainerError);
    }

    if slash_pos + 1 == b.len() {
        return Err(AzurePathParseError::MissingBlobError);
    }

    if b[slash_pos + 1..].contains('/') {
        return Err(AzurePathParseError::MultiplePathSeparatorsFoundError);
    }

    Ok((&b[0..slash_pos], &b[slash_pos + 1..]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tuple() {
        let tuple = ("container", "blob");
        assert!(tuple.container_name().unwrap() == "container");
        assert!(tuple.blob_name().unwrap() == "blob");
    }

    #[test]
    fn single_slash() {
        let path = "container/blob";
        let p = &path;
        assert!(p.container_name().unwrap() == "container");
        assert!(path.blob_name().unwrap() == "blob");
    }

    #[test]
    #[should_panic(expected = "PathSeparatorNotFoundError")]
    fn no_slash() {
        let path = "containerblob";
        let p = &path;
        assert!(p.container_name().unwrap() == "container");
        assert!(path.blob_name().unwrap() == "blob");
    }

    #[test]
    #[should_panic(expected = "MultiplePathSeparatorsFoundError")]
    fn three_slashes() {
        let path = "container/blob/extra";
        let p = &path;
        assert!(p.container_name().unwrap() == "container");
        assert!(path.blob_name().unwrap() == "blob");
    }

    #[test]
    #[should_panic(expected = "MissingContainerError")]
    fn missing_container() {
        let path = "/blob";
        let p = &path;
        assert!(p.container_name().unwrap() == "container");
        assert!(path.blob_name().unwrap() == "blob");
    }

    #[test]
    #[should_panic(expected = "MissingBlobError")]
    fn missing_blob() {
        let path = "container/";
        let p = &path;
        assert!(p.container_name().unwrap() == "container");
        assert!(path.blob_name().unwrap() == "blob");
    }
}
