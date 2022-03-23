use path_abs::PathMut;
use std::path::PathBuf;
use camino::{Utf8Path, Utf8PathBuf};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not create output directory {directory}")]
    CreateOutputDirectory { source: std::io::Error, directory: Utf8PathBuf },
    #[error("Could not create file {file}")]
    CreateFile { source: std::io::Error, file: Utf8PathBuf },
    #[error("Could not write file {file}")]
    WriteFile { source: std::io::Error, file: Utf8PathBuf },
    #[error("file name was not utf-8")]
    FileNameNotUtf8,
    #[error("Error popping path")]
    PopUpPath,
    #[error("Error appending path")]
    AppendPath(#[source] path_abs::Error),
    #[error("Error converting path to Utf8: {0}")]
    FromPathBuf(PathBuf),
    #[error(transparent)]
    Other(#[from] std::io::Error),
}

/// Joins two files paths together
///
/// If the first path ends with a file name (i.e., the last component has a file extension),
/// the file component is dropped from that path.
pub fn join<P1: AsRef<Utf8Path>, P2: AsRef<Utf8Path>>(a: P1, b: P2) -> Result<Utf8PathBuf> {
    let mut c = a.as_ref();
    if c.extension().is_some() {
        c = c.parent().ok_or(Error::PopUpPath)?; // to directory
    }
    let mut c = PathBuf::from(c);
    c.append(b.as_ref()).map_err(Error::AppendPath)?;
    Ok(Utf8PathBuf::from_path_buf(c).map_err(Error::FromPathBuf)?)
}

pub fn join_several<P1: AsRef<Utf8Path>>(a: P1, b: &[Utf8PathBuf]) -> Result<Vec<Utf8PathBuf>> {
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
            Utf8PathBuf::from("../../../azure-rest-api-specs/specification/common-types/resource-management/v1/types.json")
        );
        Ok(())
    }
}
