// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Stdio,
};

use cargo_metadata::{diagnostic::DiagnosticLevel, Message};
use serde::Serialize;

#[derive(Serialize)]
struct MessageExpectation {
    pub level: DiagnosticLevel,
    pub code: Option<String>,
    pub message: Option<String>,
    pub spans: Vec<MessageSpan>,
}

#[derive(Serialize)]
struct MessageSpan {
    pub file_name: String,
    pub line: usize, // We only check the line number of the span because other properties (like highlight span) can vary by compiler version.
}

#[derive(Serialize)]
#[serde(transparent)]
struct FileResult {
    pub messages: Vec<MessageExpectation>,
}

/// Finds relative paths to files within root, with 'path_prefix' tracking the relative directories descended
fn find_recursive(root: &Path, path_prefix: &Path, file_suffix: &str) -> Vec<String> {
    let mut vec = Vec::new();
    if !root.is_dir() {
        return vec;
    }

    for dirent in root.read_dir().expect("to be able to read the directory") {
        let dirent = dirent.expect("to have a valid directory entry (dirent)");
        let file_type = dirent.file_type().expect("to have a valid file type");
        let file_name = dirent
            .file_name()
            .into_string()
            .expect("to have a valid UTF-8 file name");
        let mut path = path_prefix.to_path_buf();
        path.push(&file_name);
        if file_type.is_file() && file_name.ends_with(file_suffix) {
            let file_name = format!("{}.rs", &file_name[..(file_name.len() - file_suffix.len())]);
            path.set_file_name(file_name);
            vec.push(path.to_str().expect("path is valid UTF-8").to_string());
        } else if file_type.is_dir() {
            vec.append(&mut find_recursive(&dirent.path(), &path, file_suffix));
        }
    }

    vec
}

#[test]
pub fn compilation_tests() {
    let test_root = {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests");
        p.push("data");
        p.push("compilation-tests");
        p
    };
    let repo_root = {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // [root]/sdk/typespec/typespec_macros
        p.pop(); // [root]/sdk/typespec
        p.pop(); // [root]/sdk
        p.pop(); // [root]
        p
    };

    let mut expected_files = find_recursive(&test_root, Path::new(""), ".expected.json");
    expected_files.sort();

    // Probably save to assume cargo is on the path, but if that's not the case, tests will start failing and we'll figure it out.
    let output = std::process::Command::new(env!("CARGO"))
        .arg("build")
        .arg("--message-format")
        .arg("json")
        .current_dir(test_root.clone())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("cargo to start running")
        .wait_with_output()
        .expect("the compilation to run to completion");
    assert!(!output.status.success(), "compilation should have failed");

    // Collect output and error and write them
    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");
    let stderr = String::from_utf8(output.stderr).expect("stderr should be valid UTF-8");
    println!(
        "==== Standard Output ====\n{}\n==== End Standard Output ====",
        &stdout
    );
    println!(
        "==== Standard Error ====\n{}\n==== End Standard Error ====",
        &stderr
    );
    let messages = Message::parse_stream(stdout.as_bytes());
    let file_messages = messages
        .filter_map(|m| match m.expect("failed to parse message") {
            Message::CompilerMessage(m) => Some(m),
            _ => None,
        })
        // Group by primary_span's src path
        .fold(HashMap::new(), |mut map, msg| {
            let Some(primary_span) = msg.message.spans.iter().find(|s| s.is_primary) else {
                // No primary span, don't add this to the map.
                return map;
            };

            // Convert the spans
            let spans: Vec<MessageSpan> = msg
                .message
                .spans
                .iter()
                .map(|span| {
                    let mut span_file_name = PathBuf::from(&span.file_name);
                    if span_file_name.is_relative() {
                        span_file_name = test_root.join(span_file_name)
                    }
                    assert!(span_file_name.is_absolute());

                    #[allow(clippy::expect_fun_call)]
                    let relative_span_path = span_file_name
                        .strip_prefix(&repo_root)
                        .expect(&format!(
                            "span path {} is not relative to test_root {:?}",
                            &span.file_name, &repo_root
                        ))
                        .to_path_buf();

                    MessageSpan {
                        file_name: relative_span_path
                            .to_str()
                            .expect("failed to convert span path to string")
                            .replace("\\", "/"),
                        line: span.line_start,
                    }
                })
                .collect();

            let expectation = MessageExpectation {
                code: msg.message.code.clone().map(|c| c.code),
                level: msg.message.level,
                message: match msg.message.code {
                    // If there's a 'code', the message comes from rustc (not our macro).
                    // In that case, clear the 'rendered' and 'message' properties, they can be volatile from compiler version to compiler version
                    Some(_) => None,
                    None => Some(msg.message.message),
                },
                spans,
            };

            map.entry(primary_span.file_name.clone())
                .or_insert_with(|| FileResult {
                    messages: Vec::new(),
                })
                .messages
                .push(expectation);
            map
        });
    let mut files_with_errors: Vec<String> = file_messages.keys().cloned().collect();
    files_with_errors.sort();
    println!("Found errors in files:\n{:?}", files_with_errors);
    println!("Expect errors in files:\n{:?}", expected_files);
    assert_eq!(files_with_errors, expected_files);

    // Now, for each group, generate/validate baselines depending on the env var AZSDK_GENERATE_BASELINES
    let generate_baselines = std::env::var("AZSDK_GENERATE_BASELINES")
        .map(|b| b.as_str() == "1")
        .unwrap_or(false);

    let mut errors = String::new();
    for (src_path, messages) in file_messages.iter() {
        let baseline_path = {
            let mut p = test_root.clone();
            p.push(src_path);
            p.set_extension("expected.json");
            p
        };

        let actual_path = {
            let mut p = test_root.clone();
            p.push(src_path);
            p.set_extension("actual.json");
            p
        };

        let serialized = serde_json::to_string_pretty(&messages)
            .expect("failed to serialize message")
            .trim()
            .to_string();

        if generate_baselines {
            std::fs::write(&baseline_path, serialized).expect("failed to write baseline");
        } else {
            assert!(baseline_path.exists());

            // Write the actual file
            std::fs::write(&actual_path, serialized.clone()).expect("failed to write actual");

            // Read the baseline file
            let baseline =
                String::from_utf8(std::fs::read(&baseline_path).expect("failed to read baseline"))
                    .expect("invalid baseline file")
                    .trim()
                    .to_string();
            if baseline != serialized {
                let diff_command = format!("diff {:?} {:?}", baseline_path, actual_path);
                errors.push_str(&format!(
                    "=== {} does not match baseline ===\nRun `{}` to compare.\n\nActual Payload:\n{}\n===\n",
                    src_path, diff_command, serialized
                ));
            }
        }
    }

    if !errors.is_empty() {
        panic!("{}", errors);
    }
}
