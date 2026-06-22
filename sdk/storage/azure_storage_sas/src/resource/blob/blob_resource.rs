// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::fmt;

/// A blob resource for user delegation SAS.
///
/// By default targets a base blob (`sr=b`). Use [`Blob::snapshot`] or
/// [`Blob::version`] to target a snapshot (`sr=bs`) or version (`sr=bv`).
pub struct Blob {
    container: String,
    blob: String,
    snapshot: Option<String>,
    version_id: Option<String>,
}

impl Blob {
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
    /// When using `BlobClient::generate_user_delegation_sas_url`, you don't
    /// need to set this yourself; it is read from the endpoint URL's
    /// `snapshot=` query parameter automatically.
    pub fn snapshot(mut self, snapshot: impl Into<String>) -> Self {
        self.snapshot = Some(snapshot.into());
        self
    }

    /// Targets a specific version of the blob (`sr=bv`).
    ///
    /// When using `BlobClient::generate_user_delegation_sas_url`, you don't
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
}

impl fmt::Display for BlobPermissions {
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
