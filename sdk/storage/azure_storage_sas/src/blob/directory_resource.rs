// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// A directory resource (ADLS Gen2) for user delegation SAS.
#[derive(Debug)]
pub(crate) struct DirectoryResource {
    container: String,
    directory: String,
}

impl DirectoryResource {
    /// Creates a new directory resource.
    ///
    /// The directory depth (`sdd`) is computed automatically from the path
    /// by counting `/`-separated segments (e.g., `"dir1/dir2"` → depth 2).
    pub(crate) fn new(container: impl Into<String>, directory: impl Into<String>) -> Self {
        Self {
            container: container.into(),
            directory: directory.into(),
        }
    }

    pub(crate) fn depth(&self) -> u32 {
        let trimmed = self.directory.trim_matches('/');
        if trimmed.is_empty() {
            0
        } else {
            trimmed.split('/').count() as u32
        }
    }

    pub(crate) fn canonicalized_resource(&self, account: &str) -> String {
        format!("/blob/{}/{}/{}", account, self.container, self.directory)
    }
}
