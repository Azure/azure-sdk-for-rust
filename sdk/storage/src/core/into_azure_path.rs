use azure_core::error::{Error, ErrorKind, Result};
use std::borrow::Borrow;

pub trait IntoAzurePath {
    fn container_name(&self) -> Result<&str>;
    fn blob_name(&self) -> Result<&str>;

    fn components(&self) -> Result<(&str, &str)> {
        Ok((self.container_name()?, self.blob_name()?))
    }
}

impl<T> IntoAzurePath for (T, T)
where
    T: Borrow<str> + Clone,
{
    fn container_name(&self) -> Result<&str> {
        Ok(self.0.borrow())
    }
    fn blob_name(&self) -> Result<&str> {
        Ok(self.1.borrow())
    }
}

impl<'a> IntoAzurePath for &'a str {
    fn container_name(&self) -> Result<&str> {
        split(self.borrow()).map(|(container_name, _blob_name)| container_name)
    }

    fn blob_name(&self) -> Result<&str> {
        split(self.borrow()).map(|(_container_name, blob_name)| blob_name)
    }
}

fn split(b: &str) -> Result<(&str, &str)> {
    let slash_pos = b.find('/').ok_or_else(|| {
        Error::with_message(ErrorKind::Other, || {
            format!("failed to find path separator. path: {b}")
        })
    })?;

    if slash_pos == 0 {
        return Err(Error::with_message(ErrorKind::Other, || {
            format!("path missing container. path: {b}")
        }));
    }

    if slash_pos + 1 == b.len() {
        return Err(Error::with_message(ErrorKind::Other, || {
            format!("path missing blob. path: {b}")
        }));
    }

    if b[slash_pos + 1..].contains('/') {
        return Err(Error::with_message(ErrorKind::Other, || {
            format!("path has multiple separators. path: {b}")
        }));
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
    fn no_slash() {
        let path = "containerblob";
        let p = &path;
        assert!(p.container_name().is_err());
        assert!(path.blob_name().is_err());
    }

    #[test]
    fn three_slashes() {
        let path = "container/blob/extra";
        let p = &path;
        assert!(p.container_name().is_err());
        assert!(path.blob_name().is_err());
    }

    #[test]
    fn missing_container() {
        let path = "/blob";
        let p = &path;
        assert!(p.container_name().is_err());
        assert!(path.blob_name().is_err());
    }

    #[test]
    fn missing_blob() {
        let path = "container/";
        let p = &path;
        assert!(p.container_name().is_err());
        assert!(path.blob_name().is_err());
    }
}
