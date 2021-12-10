use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct MockTransaction {
    pub(crate) name: String,
    pub(crate) number: Arc<AtomicUsize>,
}

impl MockTransaction {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            number: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn number(&self) -> usize {
        self.number.load(Ordering::SeqCst)
    }

    pub(crate) fn increment_number(&self) -> usize {
        self.number.fetch_add(1, Ordering::SeqCst)
    }

    pub(crate) fn file_path(
        &self,
        create_when_not_exist: bool,
    ) -> Result<PathBuf, super::MockFrameworkError> {
        let mut path = PathBuf::from(workspace_root().map_err(|e| {
            super::MockFrameworkError::TransactionStorageError(format!(
                "could not read the workspace_root from the cargo metadata: {}",
                e,
            ))
        })?);
        path.push("test");
        path.push("transactions");
        let name = self.name();
        if name.is_empty() {
            panic!(
                "`ClientOptions` and `TransportOptions` must be created with a non-empty transaction \
            name when using the `mock_transport_framework` feature. You can do this by using \
            `ClientOptions::new_with_transaction_name`"
            );
        }
        path.push(name);

        if !path.exists() {
            if create_when_not_exist {
                std::fs::create_dir_all(&path).map_err(|e| {
                    super::MockFrameworkError::IOError(
                        format!("cannot create transaction folder: {}", path.display()),
                        e,
                    )
                })?;
            } else {
                return Err(super::MockFrameworkError::MissingTransaction(format!(
                    "the transaction location '{}' does not exist",
                    path.canonicalize().unwrap_or(path).display()
                )));
            }
        }

        Ok(path)
    }
}

/// Run cargo to get the root of the workspace
fn workspace_root() -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("cargo")
        .arg("metadata")
        .output()?;
    let output = String::from_utf8_lossy(&output.stdout);

    let key = "workspace_root\":\"";
    let index = output
        .find(key)
        .ok_or_else(|| format!("workspace_root key not found in metadata"))?;
    let value = &output[index + key.len()..];
    let end = value
        .find("\"")
        .ok_or_else(|| format!("workspace_root value was malformed"))?;
    Ok(value[..end].into())
}
