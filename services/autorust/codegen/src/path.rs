use std::path::{Path, PathBuf};

use path_abs::PathMut;

type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("PopUpPath")]
    PopUpPath { source: path_abs::Error },
    #[error("AppendPath")]
    AppendPath { source: path_abs::Error },
}

/// Joins two files paths together
///
/// If the first path ends with a file name (i.e., the last component has a file extension),
/// the file component is dropped from that path.
pub fn join<P1: AsRef<Path>, P2: AsRef<Path>>(a: P1, b: P2) -> Result<PathBuf> {
    let mut c = PathBuf::from(a.as_ref());
    if c.extension().is_some() {
        c.pop_up().map_err(|source| Error::PopUpPath { source })?; // to directory
    }
    c.append(&b).map_err(|source| Error::AppendPath { source })?;
    Ok(c)
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
            PathBuf::from("../../../azure-rest-api-specs/specification/common-types/resource-management/v1/types.json")
        );
        Ok(())
    }
}
