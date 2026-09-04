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
    /// The directory depth (`sdd`) is computed automatically after normalizing
    /// `\` to `/` and counting the resulting path segments.
    pub(crate) fn new(container: impl Into<String>, directory: impl Into<String>) -> Self {
        Self {
            container: container.into(),
            directory: directory.into(),
        }
    }

    pub(crate) fn depth(&self) -> u32 {
        let canonicalized_directory = self.canonicalized_directory();
        let trimmed = canonicalized_directory.trim_matches('/');
        if trimmed.is_empty() {
            0
        } else {
            trimmed.split('/').count() as u32
        }
    }

    pub(crate) fn canonicalized_resource(&self, account: &str) -> String {
        format!(
            "/blob/{}/{}/{}",
            account,
            self.container,
            self.canonicalized_directory()
        )
    }

    fn canonicalized_directory(&self) -> String {
        self.directory.replace('\\', "/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonicalized_resource_and_depth_normalize_backslashes() {
        let resource = DirectoryResource::new("container", r"dir1\dir2/dir3");

        assert_eq!(
            resource.canonicalized_resource("account"),
            "/blob/account/container/dir1/dir2/dir3"
        );
        assert_eq!(resource.depth(), 3);
    }

    #[test]
    fn backslash_root_has_zero_depth() {
        let resource = DirectoryResource::new("container", r"\");

        assert_eq!(resource.depth(), 0);
    }
}
