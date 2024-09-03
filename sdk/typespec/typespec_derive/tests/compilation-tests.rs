use std::{collections::HashMap, io::BufReader, path::PathBuf, process::Stdio};

use cargo_metadata::{CompilerMessage, Message, PackageId};
use serde::Serialize;

#[derive(Serialize)]
#[serde(transparent)]
struct FileResult {
    pub messages: Vec<CompilerMessage>,
}

#[test]
pub fn compilation_tests() {
    let test_root = {
        let mut cwd = std::env::current_dir().expect("failed to get current directory");
        cwd.push("compilation-tests");
        cwd
    };
    let repo_root = {
        let mut p = test_root.clone(); // [root]/sdk/typespec/typespec_derive/compilation-tests
        p.pop(); // [root]/sdk/typespec/typespec_derive
        p.pop(); // [root]/sdk/typespec
        p.pop(); // [root]/sdk
        p.pop(); // [root]
        p
    };

    // TODO: Create an initial map of all the files in the project.

    // Probably save to assume cargo is on the path, but if that's not the case, tests will start failing and we'll figure it out.
    let mut compilation = std::process::Command::new("cargo")
        .arg("build")
        .arg("--message-format")
        .arg("json")
        .current_dir(test_root.clone())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        // An error here does not mean a non-zero exit code, it means a failure to run the process.
        .expect("failed to execute compilation");
    let reader = BufReader::new(compilation.stdout.take().unwrap());
    let messages = Message::parse_stream(reader);
    let file_messages = messages
        .filter_map(|m| match m.expect("failed to parse message") {
            Message::CompilerMessage(m) => Some(m),
            _ => None,
        })
        // Group by primary_span's src path
        .fold(HashMap::new(), |mut map, msg| {
            // Clone a new message that we'll update, to satisfy the borrow checker.
            // If we don't do this, we have to do a lot of dancing around borrows, which is doable but this is easier.
            let mut new_msg = msg.clone();

            let Some(primary_span) = msg.message.spans.iter().find(|s| s.is_primary) else {
                // No primary span, don't add this to the map.
                return map;
            };

            // Relativize the src path
            let relative_path = msg
                .target
                .src_path
                .strip_prefix(&repo_root)
                .expect("src_path is not relative to test_root");
            new_msg.target.src_path = relative_path.into();

            // Relativize the file_name in each span
            new_msg.message.spans = msg
                .message
                .spans
                .iter()
                .map(|span| {
                    let mut span_file_name = PathBuf::from(&span.file_name);
                    if span_file_name.is_relative() {
                        span_file_name = test_root.join(span_file_name)
                    }
                    assert!(span_file_name.is_absolute());

                    let relative_span_path = span_file_name
                        .strip_prefix(&repo_root)
                        .expect(&format!(
                            "span path {} is not relative to test_root {:?}",
                            &span.file_name, &repo_root
                        ))
                        .to_path_buf();
                    let mut new_span = span.clone();
                    new_span.file_name = relative_span_path
                        .to_str()
                        .expect("failed to convert to string")
                        .into();

                    // Clear the expansion property, it just contains references to the macro (and absolute paths)
                    new_span.expansion = None;

                    new_span
                })
                .collect();

            // Clear the 'children' property, it has absolute paths and doesn't need to be validated (it contains 'help' and 'note' messages).
            new_msg.message.children.clear();

            // The package ID contains the absolute path. Just redact it.
            new_msg.package_id = PackageId {
                repr: "<redacted>".into(),
            };

            map.entry(primary_span.file_name.clone())
                .or_insert_with(|| FileResult {
                    messages: Vec::new(),
                })
                .messages
                .push(new_msg);
            map
        });

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

    if errors.len() > 0 {
        panic!("{}", errors);
    }
}
