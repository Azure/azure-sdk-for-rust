use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MockTransaction {
    pub(crate) name: String,
    pub(crate) number: Arc<AtomicUsize>,
}

impl MockTransaction {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            number: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn number(&self) -> usize {
        self.number.load(Ordering::SeqCst)
    }

    pub fn increment_number(&self) -> usize {
        self.number.fetch_add(1, Ordering::SeqCst)
    }

    pub(crate) fn file_path(&self) -> Result<PathBuf, crate::MockFrameworkError> {
        let path: PathBuf = PathBuf::from("SessionRecords").join(self.name());

        if !path.exists() {
            std::fs::create_dir(&path).map_err(|e| {
                crate::MockFrameworkError::IOError(
                    format!("cannot create transaction folder: {}", path.display()),
                    e,
                )
            })?;
        }

        Ok(path)
    }
}
