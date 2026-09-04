// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;
use std::io::Cursor;

#[test]
fn sha256_matches_sha256sum() {
    assert_eq!(
        sha256(Cursor::new(b"one\ntwo\n")).unwrap(),
        [
            0xc3, 0xf9, 0xc8, 0xc2, 0x83, 0xa2, 0xb1, 0xf2, 0xf1, 0x89, 0x6f, 0x27, 0xa0, 0x1c,
            0xbe, 0x3c, 0xdd, 0xc0, 0xc9, 0xd9, 0x3f, 0x75, 0x2e, 0x46, 0x39, 0x03, 0x5a, 0x0f,
            0x5b, 0x36, 0xf6, 0xe8,
        ]
    );
}

#[test]
fn sha256_ignores_windows_line_endings() {
    let lf = sha256(Cursor::new(b"one\ntwo\n")).unwrap();
    let crlf = sha256(Cursor::new(b"one\r\ntwo\r\n")).unwrap();

    assert_eq!(lf, crlf);
}

#[test]
fn sha256_ignores_legacy_mac_line_endings() {
    let lf = sha256(Cursor::new(b"one\ntwo\n")).unwrap();
    let cr = sha256(Cursor::new(b"one\rtwo\r")).unwrap();

    assert_eq!(lf, cr);
}

#[test]
fn missing_file_is_positive() {
    let path = test_path("missing", line!());
    let _ = fs::remove_file(&path);

    assert!(!check_file(&path, "generated").unwrap());
}

#[test]
fn matching_file_is_positive_with_different_line_endings() {
    let path = test_path("matching", line!());
    fs::write(&path, "one\r\ntwo\r\n").unwrap();

    assert!(check_file(&path, "one\ntwo\n").unwrap());

    fs::remove_file(path).unwrap();
}

#[test]
fn mismatched_file_is_an_error() {
    let path = test_path("mismatched", line!());
    fs::write(&path, "existing\n").unwrap();

    let error = check_file(&path, "generated\n").unwrap_err();

    fs::remove_file(&path).unwrap();
    assert!(error.contains(&path.display().to_string()));
}

fn test_path(name: &str, line: u32) -> PathBuf {
    std::env::temp_dir().join(format!(
        "generate_api_{name}_{}_{}",
        std::process::id(),
        line
    ))
}
