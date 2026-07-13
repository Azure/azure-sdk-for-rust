// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(
    author,
    version,
    about = "Generate public API artifacts for a Rust crate"
)]
struct Args {
    /// Path to the Cargo.toml for the target package.
    #[arg(long, value_name = "PATH")]
    manifest_path: PathBuf,

    /// Output format to generate. Defaults to review.
    #[arg(long, value_enum, default_value_t = OutputFormat::Review)]
    format: OutputFormat,

    /// Do not emit documentation comments in APIView output.
    #[arg(long)]
    no_docs: bool,

    /// Directory where generated files will be written.
    #[arg(long, value_name = "DIR")]
    output: PathBuf,
}

#[derive(Debug, Clone)]
pub(crate) struct Request {
    pub(crate) manifest_path: PathBuf,
    pub(crate) format: OutputFormat,
    pub(crate) no_docs: bool,
    pub(crate) output_dir: PathBuf,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
pub(crate) enum OutputFormat {
    Review,
    Apiview,
}

impl OutputFormat {
    pub(crate) fn default_file_name(self) -> &'static str {
        match self {
            Self::Review => "API.md",
            Self::Apiview => "apiview.json",
        }
    }
}

pub(crate) fn parse() -> Request {
    let args = Args::parse();
    Request {
        manifest_path: args.manifest_path,
        format: args.format,
        no_docs: args.no_docs,
        output_dir: args.output,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_review_format() {
        let args = Args::parse_from([
            "generate_api",
            "--manifest-path",
            "sdk/core/azure_core/Cargo.toml",
            "--output",
            "/tmp/generate_api",
        ]);

        assert_eq!(args.format, OutputFormat::Review);
        assert!(!args.no_docs);
    }

    #[test]
    fn accepts_explicit_apiview_format() {
        let args = Args::parse_from([
            "generate_api",
            "--manifest-path",
            "sdk/core/azure_core/Cargo.toml",
            "--format",
            "apiview",
            "--output",
            "/tmp/generate_api",
        ]);

        assert_eq!(args.format, OutputFormat::Apiview);
    }

    #[test]
    fn accepts_no_docs_switch() {
        let args = Args::parse_from([
            "generate_api",
            "--manifest-path",
            "sdk/core/azure_core/Cargo.toml",
            "--format",
            "apiview",
            "--no-docs",
            "--output",
            "/tmp/generate_api",
        ]);

        assert!(args.no_docs);
    }
}
