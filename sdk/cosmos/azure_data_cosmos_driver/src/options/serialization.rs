// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::{Serialize, Serializer};
use std::time::Duration;

pub(super) fn duration_as_millis<S: Serializer>(
    duration: &Duration,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    duration.as_millis().serialize(serializer)
}

pub(super) fn optional_duration_as_millis<S: Serializer>(
    duration: &Option<Duration>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    duration
        .map(|duration| duration.as_millis())
        .serialize(serializer)
}
