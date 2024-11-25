// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared utilities for testing client libraries built on `azure_core`.
//!
//! For a comprehensive suite of utilities for testing client libraries built on `azure_core`,
//! read documentation for the `azure_core_test` crate.

use crate::error::{Error, ErrorKind};
use std::{fmt, str::FromStr};

/// Whether to test client methods by playing back recordings, recording live sessions, or executing live sessions without recording.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestMode {
    /// Test client methods by playing back recordings.
    #[default]
    Playback,

    /// Record live sessions.
    Record,

    /// Execute live sessions without recording.
    Live,
}

impl TestMode {
    /// Gets the `TestMode` from the `AZURE_TEST_MODE` environment variable or returns the default if undefined.
    pub fn current() -> typespec::Result<Self> {
        std::env::var("AZURE_TEST_MODE").map_or_else(|_| Ok(TestMode::default()), |v| v.parse())
    }
}

impl fmt::Debug for TestMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl From<&TestMode> for &'static str {
    fn from(mode: &TestMode) -> Self {
        match mode {
            TestMode::Playback => "playback",
            TestMode::Record => "record",
            TestMode::Live => "live",
        }
    }
}

impl FromStr for TestMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "playback" => Ok(Self::Playback),
            "record" => Ok(Self::Record),
            "live" => Ok(Self::Live),
            _ => Err(Error::message(
                ErrorKind::DataConversion,
                "expected 'playback', 'record', or 'live'",
            )),
        }
    }
}
