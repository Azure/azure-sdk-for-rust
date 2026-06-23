// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

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

    /// Serializes the enabled permissions to the SAS token format.
    pub(crate) fn to_sas_str(&self) -> String {
        let mut s = String::with_capacity(14);
        if self.read {
            s.push('r');
        }
        if self.add {
            s.push('a');
        }
        if self.create {
            s.push('c');
        }
        if self.write {
            s.push('w');
        }
        if self.delete {
            s.push('d');
        }
        if self.delete_version {
            s.push('x');
        }
        if self.permanent_delete {
            s.push('y');
        }
        if self.list {
            s.push('l');
        }
        if self.tags {
            s.push('t');
        }
        if self.move_blob {
            s.push('m');
        }
        if self.execute {
            s.push('e');
        }
        if self.ownership {
            s.push('o');
        }
        if self.permissions {
            s.push('p');
        }
        if self.set_immutability_policy {
            s.push('i');
        }
        s
    }
}
