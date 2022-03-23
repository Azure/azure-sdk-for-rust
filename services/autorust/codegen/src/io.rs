use path_abs::PathMut;
use std::path::{Path, PathBuf};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not create output directory {}: {}", directory.display(), source)]
    CreateOutputDirectory { directory: PathBuf, source: std::io::Error },
    #[error("Could not create file {}: {}", file.display(), source)]
    CreateFile { file: PathBuf, source: std::io::Error },
    #[error("Could not write file {}: {}", file.display(), source)]
    WriteFile { file: PathBuf, source: std::io::Error },
    #[error("file name was not utf-8")]
    FileNameNotUtf8,
    #[error("Error popping path")]
    PopUpPath(#[source] path_abs::Error),
    #[error("Error appending path")]
    AppendPath(#[source] path_abs::Error),
    #[error(transparent)]
    Other(#[from] std::io::Error),
}

/// Joins two files paths together
///
/// If the first path ends with a file name (i.e., the last component has a file extension),
/// the file component is dropped from that path.
pub fn join<P1: AsRef<Path>, P2: AsRef<Path>>(a: P1, b: P2) -> Result<PathBuf> {
    let mut c = PathBuf::from(a.as_ref());
    if c.extension().is_some() {
        c.pop_up().map_err(Error::PopUpPath)?; // to directory
    }
    c.append(&b).map_err(Error::AppendPath)?;
    Ok(c)
}

pub fn join_several<P1: AsRef<Path>>(a: P1, b: &[PathBuf]) -> Result<Vec<PathBuf>> {
    b.iter().map(|b| join(&a, b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_join() -> Result<(), Error> {
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
