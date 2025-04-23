// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use safe_debug_tests::{Enum, Struct, Tuple};

#[test]
fn debug_tuple() {
    let x = Tuple(1, "foo");
    assert_eq!(format!("{x:?}"), r#"Tuple(1, "foo")"#);
}

#[test]
fn debug_struct() {
    let x = Struct { a: 1, b: "foo" };
    assert_eq!(format!("{x:?}"), r#"Struct { a: 1, b: "foo" }"#);
}

#[test]
fn debug_enum_unit() {
    let x = Enum::Unit;
    assert_eq!(format!("{x:?}"), r#"Unit"#);
}

#[test]
fn debug_enum_tuple() {
    let x = Enum::Tuple(1, "foo");
    assert_eq!(format!("{x:?}"), r#"Tuple(1, "foo")"#);
}

#[test]
fn debug_enum_struct() {
    let x = Enum::Struct { a: 1, b: "foo" };
    assert_eq!(format!("{x:?}"), r#"Struct { a: 1, b: "foo" }"#);
}
