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

    /// Output format to generate.
    #[arg(long, value_enum)]
    format: OutputFormat,

    /// Directory where generated files will be written.
    #[arg(long, value_name = "DIR")]
    output: PathBuf,
}

#[derive(Debug, Clone)]
pub(crate) struct Request {
    pub(crate) manifest_path: PathBuf,
    pub(crate) format: OutputFormat,
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
        output_dir: args.output,
    }
}
