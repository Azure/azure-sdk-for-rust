// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::{AppendBlobClientCreateOptions, PageBlobClientCreateOptions};

/// Provides usage helpers for setting the `PageBlobClientCreateOptions` optional configurations.
pub trait PageBlobClientCreateOptionsExt {
    /// Augments the current options bag to only create if the Page blob does not already exists.
    /// # Arguments
    ///
    /// * `self` - The options bag to be modified.
    fn with_if_not_exists(self) -> Self;
}

impl PageBlobClientCreateOptionsExt for PageBlobClientCreateOptions<'_> {
    fn with_if_not_exists(self) -> Self {
        Self {
            if_none_match: Some("*".into()),
            ..self
        }
    }
}

/// Provides usage helpers for setting the `AppendBlobClientCreateOptions` optional configurations.
pub trait AppendBlobClientCreateOptionsExt {
    /// Augments the current options bag to only create if the Append blob does not already exists.
    /// # Arguments
    ///
    /// * `self` - The options bag to be modified.
    fn with_if_not_exists(self) -> Self;
}

impl AppendBlobClientCreateOptionsExt for AppendBlobClientCreateOptions<'_> {
    fn with_if_not_exists(self) -> Self {
        Self {
            if_none_match: Some("*".into()),
            ..self
        }
    }
}
