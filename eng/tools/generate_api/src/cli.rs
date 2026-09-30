// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use clap::{Parser, ValueEnum};
use std::path::{Path, PathBuf};

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

    /// Path to the repository root. Defaults to the current directory.
    #[arg(long, value_name = "PATH")]
    root: Option<PathBuf>,

    /// Output format to generate. Defaults to markdown.
    #[arg(long, value_enum, default_value_t = OutputFormat::Markdown)]
    format: OutputFormat,

    /// Emit Markdown API review metadata, source map, and documentation patch.
    #[arg(long)]
    review: bool,

    /// Check generated content against existing files without writing them.
    #[arg(long)]
    check: bool,

    /// Directory where generated files will be written. Defaults to the crate directory.
    #[arg(long = "output-dir", alias = "output", value_name = "DIR")]
    output_dir: Option<PathBuf>,

    /// Directory where Markdown review state is written. Defaults to --output-dir.
    #[arg(long, value_name = "DIR")]
    working_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub(crate) struct Request {
    pub(crate) manifest_path: PathBuf,
    pub(crate) root: Option<PathBuf>,
    pub(crate) format: OutputFormat,
    pub(crate) review: bool,
    pub(crate) check: bool,
    pub(crate) output_dir: Option<PathBuf>,
    pub(crate) working_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
pub(crate) enum OutputFormat {
    Markdown,
    Apiview,
}

/// File name of the patch that adds documentation comments back to `api.md`.
pub(crate) const DOCUMENTATION_PATCH_FILE_NAME: &str = "api.documentation.patch";
pub(crate) const MARKDOWN_METADATA_FILE_NAME: &str = "api.metadata.yml";
pub(crate) const SOURCE_MAP_FILE_NAME: &str = "api.md.map";
pub(crate) const STATE_DIRECTORY_NAME: &str = "state";
pub(crate) const PACKAGE_RELATIVE_PATH_FILE_NAME: &str = "package-relative-path.txt";
pub(crate) const VERSION_FILE_NAME: &str = "version.txt";

impl OutputFormat {
    pub(crate) fn default_file_name(self) -> &'static str {
        match self {
            Self::Markdown => "api.md",
            Self::Apiview => "apiview.json",
        }
    }
}

impl Request {
    pub(crate) fn output_dir<'a>(&'a self, package_dir: &'a Path) -> &'a Path {
        self.output_dir.as_deref().unwrap_or(package_dir)
    }

    pub(crate) fn working_dir<'a>(&'a self, package_dir: &'a Path) -> &'a Path {
        self.working_dir
            .as_deref()
            .or(self.output_dir.as_deref())
            .unwrap_or(package_dir)
    }
}

pub(crate) fn parse() -> Result<Request, String> {
    let args = Args::parse();
    Request::try_from(args)
}

impl TryFrom<Args> for Request {
    type Error = String;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        if args.review && args.format != OutputFormat::Markdown {
            return Err("--review can only be used with --format markdown".to_string());
        }

        if args.working_dir.is_some() && (!args.review || args.format != OutputFormat::Markdown) {
            return Err(
                "--working-dir can only be used with --format markdown --review".to_string(),
            );
        }

        Ok(Self {
            manifest_path: args.manifest_path,
            root: args.root,
            format: args.format,
            review: args.review,
            check: args.check,
            output_dir: args.output_dir,
            working_dir: args.working_dir,
        })
    }
}

#[cfg(test)]
mod tests;
