use crate::{Error, ErrorKind, Result, ResultExt};
use camino::{Utf8Path, Utf8PathBuf};
use path_abs::PathMut;
use std::path::PathBuf;

/// Joins two files paths together
///
/// If the first path ends with a file name (i.e., the last component has a file extension),
/// the file component is dropped from that path.
pub fn join<P1: AsRef<Utf8Path>, P2: AsRef<Utf8Path>>(a: P1, b: P2) -> Result<Utf8PathBuf> {
    let mut c = a.as_ref();
    if c.extension().is_some() {
        c = c
            .parent()
            .ok_or(Error::with_message(ErrorKind::Io, || "unable to get parent path of {c}"))?;
        // to directory
    }
    let mut c = PathBuf::from(c);
    let b = b.as_ref();
    c.append(b).with_context(ErrorKind::Io, || format!("append path {b} to {c:?}"))?;
    Utf8PathBuf::from_path_buf(c).map_err(|path| Error::with_message(ErrorKind::Io, || format!("converting path to UTF-8: {path:?}")))
}

pub fn join_several<P1: AsRef<Utf8Path>>(a: P1, b: &[Utf8PathBuf]) -> Result<Vec<Utf8PathBuf>> {
    b.iter().map(|b| join(&a, b)).collect()
}

pub fn read_file<P: AsRef<Utf8Path>>(path: P) -> Result<Vec<u8>> {
    let path = path.as_ref();
    std::fs::read(path).with_context(ErrorKind::Io, || format!("reading file {path}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_join() -> Result<()> {
        let a = "../../../azure-rest-api-specs/specification/vmware/resource-manager/Microsoft.AVS/stable/2020-03-20/vmware.json";
        let b = "../../../../../common-types/resource-management/v1/types.json";
        let c = join(a, b)?;
        assert_eq!(
            c,
            Utf8PathBuf::from("../../../azure-rest-api-specs/specification/common-types/resource-management/v1/types.json")
        );
        Ok(())
    }
}
