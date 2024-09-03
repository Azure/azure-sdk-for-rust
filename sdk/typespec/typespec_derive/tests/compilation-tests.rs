use std::{collections::HashMap, io::BufReader, process::Stdio};

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
        .fold(HashMap::new(), |mut map, mut msg| {
            let Some(primary_span) = msg.message.spans.iter().find(|s| s.is_primary) else {
                // No primary span, don't add this to the map.
                return map;
            };

            // Relativize the src path
            let relative_path = msg
                .target
                .src_path
                .strip_prefix(&test_root)
                .expect("src_path is not relative to test_root");
            msg.target.src_path = relative_path.into();

            // The package ID contains the absolute path. Just redact it.
            msg.package_id = PackageId {
                repr: "<redacted>".into(),
            };

            map.entry(primary_span.file_name.clone())
                .or_insert_with(|| FileResult {
                    messages: Vec::new(),
                })
                .messages
                .push(msg);
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
