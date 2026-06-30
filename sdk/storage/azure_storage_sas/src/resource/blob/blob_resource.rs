// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// A blob resource for user delegation SAS.
///
/// By default targets a base blob (`sr=b`). Use [`BlobResource::snapshot`] or
/// [`BlobResource::version`] to target a snapshot (`sr=bs`) or version (`sr=bv`).
pub struct BlobResource {
    container: String,
    blob: String,
    snapshot: Option<String>,
    version_id: Option<String>,
}

impl BlobResource {
    /// Creates a new blob resource targeting the base blob.
    pub fn new(container: impl Into<String>, blob: impl Into<String>) -> Self {
        Self {
            container: container.into(),
            blob: blob.into(),
            snapshot: None,
            version_id: None,
        }
    }

    /// Targets a specific snapshot of the blob (`sr=bs`).
    ///
    /// `snapshot` is the snapshot timestamp (e.g., `"2025-01-15T12:00:00.0000000Z"`).
    ///
    /// When using `BlobClient::user_delegation_sas`, you don't
    /// need to set this yourself; it is read from the endpoint URL's
    /// `snapshot=` query parameter automatically.
    pub fn snapshot(mut self, snapshot: impl Into<String>) -> Self {
        self.snapshot = Some(snapshot.into());
        self
    }

    /// Targets a specific version of the blob (`sr=bv`).
    ///
    /// When using `BlobClient::user_delegation_sas`, you don't
    /// need to set this yourself; it is read from the endpoint URL's
    /// `versionid=` query parameter automatically and preserved on the
    /// resulting URL.
    ///
    /// When using [`crate::SasBuilder`] directly, the version ID is not
    /// included in the SAS token; the caller is responsible for
    /// appending `&versionid=...` to the final request URL.
    pub fn version(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    pub(crate) fn signed_resource(&self) -> &'static str {
        if self.snapshot.is_some() {
            "bs"
        } else if self.version_id.is_some() {
            "bv"
        } else {
            "b"
        }
    }

    pub(crate) fn canonicalized_resource(&self, account: &str) -> String {
        format!("/blob/{}/{}/{}", account, self.container, self.blob)
    }

    pub(crate) fn snapshot_time(&self) -> Option<&str> {
        self.snapshot.as_deref()
    }

    /// Returns the value for the string-to-sign snapshot slot: the snapshot
    /// timestamp if set, otherwise the version ID. The service signs the
    /// version ID in this slot for a version SAS (`sr=bv`).
    pub(crate) fn snapshot_or_version_time(&self) -> Option<&str> {
        self.snapshot.as_deref().or(self.version_id.as_deref())
    }
}

/// Permissions for a blob SAS.
///
/// Serialization order: `racwdxytmeopi`.
#[derive(Clone, Copy, Default)]
pub struct BlobPermissions {
    read: bool,
    add: bool,
    create: bool,
    write: bool,
    delete: bool,
    delete_version: bool,
    permanent_delete: bool,
    tags: bool,
    move_blob: bool,
    execute: bool,
    ownership: bool,
    permissions: bool,
    set_immutability_policy: bool,
}

impl BlobPermissions {
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
        let mut s = String::with_capacity(13);
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
