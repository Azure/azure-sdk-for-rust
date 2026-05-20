#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2021"

[dependencies]
serde = { version = "1.0.197", features = ["derive"] }
taplo = "0.14.0"
toml = "1.0.1"
---

use serde::Deserialize;
use std::path::{Path, PathBuf};

fn main() {
    let paths: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();
    if paths.is_empty() {
        eprintln!("error: no TOML files specified");
        std::process::exit(1);
    }

    let config = find_and_load_config(&paths[0]);
    let options = build_options(&config);
    let mut failed = false;

    for path in &paths {
        println!("Formatting {}", path.display());

        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("error: failed to read {}: {e}", path.display());
                failed = true;
                continue;
            }
        };

        let parse = taplo::parser::parse(&content);
        let errors: Vec<_> = parse.errors.iter().map(|e| e.range).collect();
        let dom = parse.into_dom();

        let formatted = match taplo::formatter::format_with_path_scopes(
            dom,
            options.clone(),
            &errors,
            build_scopes(&config, path),
        ) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error: failed to format {}: {e}", path.display());
                failed = true;
                continue;
            }
        };

        if let Err(e) = std::fs::write(path, formatted.as_bytes()) {
            eprintln!("error: failed to write {}: {e}", path.display());
            failed = true;
        }
    }

    if failed {
        std::process::exit(1);
    }
}

// Mirrors the structure of `taplo_common::config::Config` with only the fields we need.
// The `[formatting]` section deserializes to `OptionsIncomplete` (all fields optional), which is
// then applied as overrides on top of `Options::default()` via `Options::update`.
#[derive(Default, Deserialize)]
struct TaploConfig {
    #[serde(default)]
    formatting: Option<taplo::formatter::OptionsIncomplete>,
    #[serde(default)]
    rule: Vec<TaploRule>,
}

#[derive(Deserialize)]
struct TaploRule {
    // `None` means "match all files"; `Some([])` means "match no files".
    include: Option<Vec<String>>,
    // Rules with `keys` apply only as path scopes, not to global options.
    keys: Option<Vec<String>>,
    #[serde(default)]
    formatting: Option<taplo::formatter::OptionsIncomplete>,
}

fn find_and_load_config(first_file: &Path) -> TaploConfig {
    let abs = if first_file.is_absolute() {
        first_file.to_owned()
    } else {
        std::env::current_dir().unwrap_or_default().join(first_file)
    };

    let start_dir = abs.parent().unwrap_or(&abs);

    for dir in start_dir.ancestors() {
        let config_path = dir.join(".taplo.toml");
        if config_path.exists() {
            println!("Loading {}", config_path.display());
            match std::fs::read_to_string(&config_path)
                .map_err(|e| e.to_string())
                .and_then(|s| toml::from_str::<TaploConfig>(&s).map_err(|e| e.to_string()))
            {
                Ok(config) => return config,
                Err(e) => eprintln!("warning: failed to load {}: {e}", config_path.display()),
            }
            // Found the file but failed to parse; don't search further.
            break;
        }
    }

    println!("No .taplo.toml found, using default options");
    TaploConfig::default()
}

/// Builds the base formatting options from the global `[formatting]` section and any rules
/// that apply to all files (no `keys`, no `include` filter). Mirrors `update_format_options`.
fn build_options(config: &TaploConfig) -> taplo::formatter::Options {
    let mut options = taplo::formatter::Options::default();
    if let Some(fmt) = &config.formatting {
        options.update(fmt.clone());
    }
    for rule in config.rule.iter().filter(|r| r.keys.is_none()) {
        if let Some(fmt) = &rule.formatting {
            options.update(fmt.clone());
        }
    }
    options
}

/// Returns the key-path scopes for rules that match `path` and have `keys` set.
/// Mirrors `Config::format_scopes`.
fn build_scopes(
    config: &TaploConfig,
    path: &Path,
) -> Vec<(String, taplo::formatter::OptionsIncomplete)> {
    config
        .rule
        .iter()
        .filter(|r| rule_matches_file(r, path))
        .filter_map(|rule| match (&rule.keys, &rule.formatting) {
            (Some(keys), Some(fmt)) => Some(keys.iter().map(move |k| (k.clone(), fmt.clone()))),
            _ => None,
        })
        .flatten()
        .collect()
}

fn rule_matches_file(rule: &TaploRule, path: &Path) -> bool {
    match &rule.include {
        // `None` means no filter was specified — match all files.
        // `Some([])` means an explicit empty list — match no files.
        None => true,
        Some(patterns) => patterns.iter().any(|p| glob_matches(p, path)),
    }
}

fn glob_matches(pattern: &str, path: &Path) -> bool {
    let path_str = path
        .to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/");
    glob_str(pattern, &path_str)
}

/// Matches a glob pattern against a `/`-separated path string.
/// Supports `**/` (any number of path components), `*` (within a segment),
/// and `?` (single character within a segment).
fn glob_str(pattern: &str, path: &str) -> bool {
    if let Some(rest) = pattern.strip_prefix("**/") {
        // `**/rest` matches `rest` at any depth.
        if glob_str(rest, path) {
            return true;
        }
        let mut remaining = path;
        while let Some(i) = remaining.find('/') {
            remaining = &remaining[i + 1..];
            if glob_str(rest, remaining) {
                return true;
            }
        }
        false
    } else {
        glob_simple(pattern, path)
    }
}

/// Simple wildcard match within a single path string (no `**/` handling).
/// Uses the standard backtracking algorithm for `*` and `?`.
fn glob_simple(pattern: &str, s: &str) -> bool {
    let p = pattern.as_bytes();
    let s = s.as_bytes();
    let mut pi = 0usize;
    let mut si = 0usize;
    let mut star_pi = usize::MAX;
    let mut star_si = usize::MAX;

    while si < s.len() {
        if pi < p.len() && (p[pi] == b'?' || p[pi] == s[si]) {
            pi += 1;
            si += 1;
        } else if pi < p.len() && p[pi] == b'*' {
            star_pi = pi;
            star_si = si;
            pi += 1;
        } else if star_pi != usize::MAX {
            star_si += 1;
            si = star_si;
            pi = star_pi + 1;
        } else {
            return false;
        }
    }

    while pi < p.len() && p[pi] == b'*' {
        pi += 1;
    }

    pi == p.len()
}
