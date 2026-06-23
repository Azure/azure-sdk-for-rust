// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::fmt;

/// A container resource for user delegation SAS.
pub struct ContainerResource {
    container: String,
}

impl ContainerResource {
    /// Creates a new container resource.
    pub fn new(container: impl Into<String>) -> Self {
        Self {
            container: container.into(),
        }
    }

    pub(crate) fn canonicalized_resource(&self, account: &str) -> String {
        format!("/blob/{}/{}", account, self.container)
    }
}

/// Permissions for a container or directory SAS.
///
/// Serialization order: `racwdxyltmeopi`.
#[derive(Clone, Copy, Default)]
pub struct ContainerPermissions {
    read: bool,
    add: bool,
    create: bool,
    write: bool,
    delete: bool,
    delete_version: bool,
    permanent_delete: bool,
    list: bool,
    tags: bool,
    move_blob: bool,
    execute: bool,
    ownership: bool,
    permissions: bool,
    set_immutability_policy: bool,
}

impl ContainerPermissions {
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

    /// Enables create permission.
    pub fn create(mut self) -> Self {
        self.create = true;
        self
    }

    /// Enables write permission.
    pub fn write(mut self) -> Self {
        self.write = true;
        self
    }

    /// Enables delete permission.
    pub fn delete(mut self) -> Self {
        self.delete = true;
        self
    }

    /// Enables delete version permission.
    pub fn delete_version(mut self) -> Self {
        self.delete_version = true;
        self
    }

    /// Enables permanent delete permission.
    pub fn permanent_delete(mut self) -> Self {
        self.permanent_delete = true;
        self
    }

    /// Enables list permission.
    pub fn list(mut self) -> Self {
        self.list = true;
        self
    }

    /// Enables tags permission.
    pub fn tags(mut self) -> Self {
        self.tags = true;
        self
    }

    /// Enables move blob permission.
    pub fn move_blob(mut self) -> Self {
        self.move_blob = true;
        self
    }

    /// Enables execute permission.
    pub fn execute(mut self) -> Self {
        self.execute = true;
        self
    }

    /// Enables ownership permission.
    pub fn ownership(mut self) -> Self {
        self.ownership = true;
        self
    }

    /// Enables permissions permission.
    pub fn permissions(mut self) -> Self {
        self.permissions = true;
        self
    }

    /// Enables set immutability policy permission.
    pub fn set_immutability_policy(mut self) -> Self {
        self.set_immutability_policy = true;
        self
    }
}

impl fmt::Display for ContainerPermissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.read {
            f.write_str("r")?;
        }
        if self.add {
            f.write_str("a")?;
        }
        if self.create {
            f.write_str("c")?;
        }
        if self.write {
            f.write_str("w")?;
        }
        if self.delete {
            f.write_str("d")?;
        }
        if self.delete_version {
            f.write_str("x")?;
        }
        if self.permanent_delete {
            f.write_str("y")?;
        }
        if self.list {
            f.write_str("l")?;
        }
        if self.tags {
            f.write_str("t")?;
        }
        if self.move_blob {
            f.write_str("m")?;
        }
        if self.execute {
            f.write_str("e")?;
        }
        if self.ownership {
            f.write_str("o")?;
        }
        if self.permissions {
            f.write_str("p")?;
        }
        if self.set_immutability_policy {
            f.write_str("i")?;
        }
        Ok(())
    }
}
