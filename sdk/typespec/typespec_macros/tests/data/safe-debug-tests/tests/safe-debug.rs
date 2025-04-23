// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg_attr(feature = "debug", allow(dead_code))]
use safe_debug_tests::{Enum, Struct, Tuple};

#[cfg_attr(not(feature = "debug"), test)]
fn safe_debug_tuple() {
    let x = Tuple(1, "foo");
    assert_eq!(format!("{x:?}"), r#"Tuple(..)"#);
}

#[cfg_attr(not(feature = "debug"), test)]
fn safe_debug_struct() {
    let x = Struct { a: 1, b: "foo" };
    assert_eq!(format!("{x:?}"), r#"Struct { .. }"#);
}

#[test]
fn safe_debug_enum_unit() {
    let x = Enum::Unit;
    assert_eq!(format!("{x:?}"), r#"Unit"#);
}

#[cfg_attr(not(feature = "debug"), test)]
fn safe_debug_enum_tuple() {
    let x = Enum::Tuple(1, "foo");
    assert_eq!(format!("{x:?}"), r#"Tuple(..)"#);
}

#[cfg_attr(not(feature = "debug"), test)]
fn safe_debug_enum_struct() {
    let x = Enum::Struct { a: 1, b: "foo" };
    assert_eq!(format!("{x:?}"), r#"Struct { .. }"#);
}
