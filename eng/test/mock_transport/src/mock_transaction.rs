use azure_core::error::{Error, ErrorKind, ResultExt};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub(crate) struct MockTransaction {
    pub(crate) name: String,
    pub(crate) number: Arc<AtomicUsize>,
    workspace_root: Arc<Mutex<Option<String>>>,
}

impl MockTransaction {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            number: Arc::new(AtomicUsize::new(0)),
            workspace_root: Arc::new(Mutex::new(None)),
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

    pub(crate) fn file_path(&self, create_when_not_exist: bool) -> azure_core::Result<PathBuf> {
        let root_path = {
            let mut cache = self.workspace_root.lock().unwrap();
            match &*cache {
                Some(root) => root.clone(),
                None => {
                    let root = workspace_root().context(
                        ErrorKind::MockFramework,
                        "could not read the workspace_root from the cargo metadata",
                    )?;
                    *cache = Some(root.clone());
                    root
                }
            }
        };
        let mut path = PathBuf::from(root_path);
        path.push("test");
        path.push("transactions");
        let name = self.name();
        if name.is_empty() {
            panic!(
                "The transaction name when running a request through the mock transport policy cannot be empty!"
            );
        }
        path.push(name);

        if !path.exists() {
            if create_when_not_exist {
                std::fs::create_dir_all(&path).with_context(ErrorKind::MockFramework, || {
                    format!("cannot create transaction folder: {}", path.display())
                })?;
            } else {
                return Err(Error::with_message(ErrorKind::MockFramework, || {
                    format!(
                        "the transaction location '{}' does not exist",
                        path.canonicalize().unwrap_or(path).display()
                    )
                }));
            }
        }

        Ok(path)
    }
}

/// Run cargo to get the root of the workspace
fn workspace_root() -> azure_core::Result<String> {
    let output = std::process::Command::new("cargo")
        .arg("metadata")
        .output()?;
    let output = String::from_utf8_lossy(&output.stdout);

    let key = "workspace_root\":\"";
    let index = output.find(key).ok_or_else(|| {
        Error::message(
            ErrorKind::MockFramework,
            "workspace_root key not found in metadata",
        )
    })?;
    let value = &output[index + key.len()..];
    let end = value.find('\"').ok_or_else(|| {
        Error::message(
            ErrorKind::MockFramework,
            "workspace_root value was malformed",
        )
    })?;
    Ok(value[..end].into())
}
