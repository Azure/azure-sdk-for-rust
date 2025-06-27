// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::{AppendBlobClientCreateOptions, PageBlobClientCreateOptions};

pub trait PageBlobClientCreateOptionsExt {
    /// Augments the current options bag to only create if the page blob does not already exists.
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

pub trait AppendBlobClientCreateOptionsExt {
    /// Augments the current options bag to only create if the append blob does not already exists.
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
