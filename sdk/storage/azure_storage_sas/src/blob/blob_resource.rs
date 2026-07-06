// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// A blob resource for user delegation SAS.
///
/// By default targets a base blob (`sr=b`). A snapshot timestamp or version ID
/// is set through the [`snapshot`](crate::SasBuilder::snapshot) or
/// [`version`](crate::SasBuilder::version) builder setters.
#[derive(Debug)]
pub(crate) struct BlobResource {
    container: String,
    blob: String,
    pub(crate) snapshot: Option<String>,
    pub(crate) version_id: Option<String>,
}

impl BlobResource {
    /// Creates a new blob resource targeting the base blob.
    pub(crate) fn new(container: impl Into<String>, blob: impl Into<String>) -> Self {
        Self {
            container: container.into(),
            blob: blob.into(),
            snapshot: None,
            version_id: None,
        }
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
/// Serialization order: `racwdxytmeopi`. Flags are set through the permission
/// setters on [`SasBuilder<BlobState>`](crate::SasBuilder).
#[derive(Clone, Copy, Default)]
pub(crate) struct BlobPermissions {
    pub(crate) read: bool,
    pub(crate) add: bool,
    pub(crate) create: bool,
    pub(crate) write: bool,
    pub(crate) delete: bool,
    pub(crate) delete_version: bool,
    pub(crate) permanent_delete: bool,
    pub(crate) tags: bool,
    pub(crate) move_blob: bool,
    pub(crate) execute: bool,
    pub(crate) ownership: bool,
    pub(crate) permissions: bool,
    pub(crate) set_immutability_policy: bool,
}

impl BlobPermissions {
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
