use std::collections::HashMap;

/// A simple configration manager that stores key-value pairs.
pub struct ConfigManager {
    entries: HashMap<String, String>,
}

impl ConfigManager {
    /// Creates a new empty `ConfigManager`.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Inserts a value into the configuration.
    ///
    /// If the key already exists, the previous value is replaced and retunred.
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) -> Option<String> {
        self.entries.insert(key.into(), value.into())
    }

    /// Returns the value associated with the given key, if any.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut config = ConfigManager::new();
        config.insert("host", "localhost");
        assert_eq!(config.get("host"), Some(&"localhost".to_string()));
    }
}
