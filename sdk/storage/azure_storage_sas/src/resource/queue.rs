// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::fmt;

/// A queue resource for user delegation SAS.
pub struct Queue {
    queue: String,
}

impl Queue {
    /// Creates a new queue resource.
    pub fn new(queue: impl Into<String>) -> Self {
        Self {
            queue: queue.into(),
        }
    }

    pub(crate) fn canonicalized_resource(&self, account: &str) -> String {
        format!("/queue/{}/{}", account, self.queue)
    }
}

/// Permissions for a queue SAS.
///
/// Serialization order: `raup`.
#[derive(Clone, Copy, Default)]
pub struct QueuePermissions {
    pub read: bool,
    pub add: bool,
    pub update: bool,
    pub process: bool,
}

impl QueuePermissions {
    /// Creates a new permissions set with all permissions disabled.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables read permission.
    pub fn read(mut self) -> Self {
        self.read = true;
        self
    }

    /// Enables add permission.
    pub fn add(mut self) -> Self {
        self.add = true;
        self
    }

    /// Enables update permission.
    pub fn update(mut self) -> Self {
        self.update = true;
        self
    }

    /// Enables process permission.
    pub fn process(mut self) -> Self {
        self.process = true;
        self
    }
}

impl fmt::Display for QueuePermissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.read {
            f.write_str("r")?;
        }
        if self.add {
            f.write_str("a")?;
        }
        if self.update {
            f.write_str("u")?;
        }
        if self.process {
            f.write_str("p")?;
        }
        Ok(())
    }
}
