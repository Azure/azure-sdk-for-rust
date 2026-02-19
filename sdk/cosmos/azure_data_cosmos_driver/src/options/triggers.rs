// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Trigger configuration options.

use crate::models::TriggerInvocation;

/// Collection of triggers to include in a request.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TriggerOptions {
    /// Triggers to execute before the operation.
    pub pre_triggers: Vec<TriggerInvocation>,
    /// Triggers to execute after the operation.
    pub post_triggers: Vec<TriggerInvocation>,
}

impl TriggerOptions {
    /// Creates a new empty trigger options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a pre-trigger to execute before the operation.
    pub fn with_pre_trigger(mut self, trigger: impl Into<TriggerInvocation>) -> Self {
        self.pre_triggers.push(trigger.into());
        self
    }

    /// Adds a post-trigger to execute after the operation.
    pub fn with_post_trigger(mut self, trigger: impl Into<TriggerInvocation>) -> Self {
        self.post_triggers.push(trigger.into());
        self
    }
}
