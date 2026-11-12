// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The spec harness's own command-line options, separated from libtest's.

use crate::{config::CONFIG_ARG, pool::NO_CLEANUP_ARG, results::RESULTS_DIR_ARG};
use std::{ffi::OsString, path::PathBuf};

/// Command-line options for the spec harness.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct HarnessArgs {
    /// `--results-dir <dir>`: where to write result files.
    pub results_dir: Option<PathBuf>,
    /// `--config <path>`: the TOML run configuration.
    pub config: Option<PathBuf>,
    /// `--no-cleanup`: keep every database and container the run creates.
    pub no_cleanup: bool,
    /// Everything else, for `libtest-mimic`.
    pub libtest: Vec<OsString>,
}

impl HarnessArgs {
    /// Removes the harness options (`--name <value>`, `--name=<value>`, or a
    /// bare flag) from `args`.
    pub fn parse(args: impl IntoIterator<Item = OsString>) -> Result<Self, String> {
        let mut parsed = Self::default();
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            let text = arg.to_string_lossy().into_owned();
            if text == NO_CLEANUP_ARG {
                parsed.no_cleanup = true;
                continue;
            }
            let mut matched = false;
            for (name, slot) in [
                (RESULTS_DIR_ARG, &mut parsed.results_dir),
                (CONFIG_ARG, &mut parsed.config),
            ] {
                if text == name {
                    let value = args.next().ok_or_else(|| format!("{name} needs a path"))?;
                    *slot = Some(PathBuf::from(value));
                } else if let Some(value) = text.strip_prefix(&format!("{name}=")) {
                    *slot = Some(PathBuf::from(value));
                } else {
                    continue;
                }
                matched = true;
                break;
            }
            if !matched {
                parsed.libtest.push(arg);
            }
        }
        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::{HarnessArgs, OsString, PathBuf};

    fn args(list: &[&str]) -> Vec<OsString> {
        list.iter().map(OsString::from).collect()
    }

    #[test]
    fn harness_options_are_removed_from_libtest_args() {
        let parsed = HarnessArgs::parse(args(&[
            "spec",
            "--results-dir",
            "out",
            "--exact",
            "--config",
            "run.toml",
            "--no-cleanup",
            "x",
        ]))
        .unwrap();
        assert!(parsed.no_cleanup);
        assert_eq!(parsed.results_dir, Some(PathBuf::from("out")));
        assert_eq!(parsed.config, Some(PathBuf::from("run.toml")));
        assert_eq!(parsed.libtest, args(&["spec", "--exact", "x"]));
    }

    #[test]
    fn options_accept_equals_form() {
        let parsed =
            HarnessArgs::parse(args(&["spec", "--results-dir=a/b", "--config=c.toml"])).unwrap();
        assert_eq!(parsed.results_dir, Some(PathBuf::from("a/b")));
        assert_eq!(parsed.config, Some(PathBuf::from("c.toml")));
        assert_eq!(parsed.libtest, args(&["spec"]));
    }

    #[test]
    fn options_need_a_value() {
        assert!(HarnessArgs::parse(args(&["spec", "--results-dir"])).is_err());
        assert!(HarnessArgs::parse(args(&["spec", "--config"])).is_err());
    }

    #[test]
    fn options_are_optional() {
        let parsed = HarnessArgs::parse(args(&["spec", "--list"])).unwrap();
        assert_eq!(parsed.results_dir, None);
        assert_eq!(parsed.config, None);
        assert!(!parsed.no_cleanup);
        assert_eq!(parsed.libtest, args(&["spec", "--list"]));
    }
}
