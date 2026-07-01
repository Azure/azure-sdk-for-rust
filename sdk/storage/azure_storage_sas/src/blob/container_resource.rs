// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// A container resource for user delegation SAS.
#[derive(Debug)]
pub(crate) struct ContainerResource {
    container: String,
}

impl ContainerResource {
    /// Creates a new container resource.
    pub(crate) fn new(container: impl Into<String>) -> Self {
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
/// Serialization order: `racwdxyltmeopi`. Flags are set through the permission
/// setters on [`SasBuilder<ContainerState>`](crate::SasBuilder) and
/// [`SasBuilder<DirectoryState>`](crate::SasBuilder).
#[derive(Clone, Copy, Default)]
pub(crate) struct ContainerPermissions {
    pub(crate) read: bool,
    pub(crate) add: bool,
    pub(crate) create: bool,
    pub(crate) write: bool,
    pub(crate) delete: bool,
    pub(crate) delete_version: bool,
    pub(crate) permanent_delete: bool,
    pub(crate) list: bool,
    pub(crate) tags: bool,
    pub(crate) move_blob: bool,
    pub(crate) execute: bool,
    pub(crate) ownership: bool,
    pub(crate) permissions: bool,
    pub(crate) set_immutability_policy: bool,
}

impl ContainerPermissions {
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
