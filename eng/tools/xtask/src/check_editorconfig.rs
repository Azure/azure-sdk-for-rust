// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::find_file;
use serde::Deserialize;
use std::{env, fs, ops::Not, process::exit};

pub fn run(args: impl Iterator<Item = String>) {
    // TODO: Find more than just the first .editorconfig in the ancestor directory tree.
    let config_path = find_file(
        env::current_dir().expect("current directory"),
        ".editorconfig",
    )
    .expect("expected .editorconfig");
    let config_content = fs::read_to_string(config_path).expect("read .editorconfig");

    // BUGBUG: A section named "*" and non-quoted strings are not supported.
    let config: EditorConfig = toml::from_str(&config_content).expect("deserialize .editorconfig");

    let mut missing_header = false;
    for path in args.filter(filter) {
        let content = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", &path));
        if !content
            .replace("\r\n", "\n")
            .starts_with(&config.all.file_header_template)
        {
            println!("Missing copyright header from {}", &path);
            missing_header = true;
        }
    }

    if missing_header {
        exit(1);
    }
}

#[allow(clippy::ptr_arg)]
fn filter(value: &String) -> bool {
    value.ends_with(".rs") && value.replace('\\', "/").contains("/generated/").not()
}

#[test]
fn test_filter() {
    assert!(filter(&"dir/file.rs".to_string()));
    assert!(filter(&"dir\\file.rs".to_string()));
    assert!(!filter(&"dir/file.txt".to_string()));
    assert!(filter(&"dir/generated_file.rs".to_string()));
    assert!(filter(&"dir\\generated_file.rs".to_string()));
    assert!(!filter(&"dir/generated/file.rs".to_string()));
    assert!(!filter(&"dir\\generated\\file.rs".to_string()));
}

#[derive(Deserialize)]
struct EditorConfig {
    // #[serde(rename = "*")]
    pub all: Section,
}

#[derive(Debug, Deserialize)]
struct Section {
    file_header_template: String,
}
