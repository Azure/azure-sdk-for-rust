// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg_attr(feature = "debug", allow(dead_code))]
use rustc_version::Version;
use safe_debug_tests::*;
use std::sync::LazyLock;

const MSRV: Version = Version::new(1, 80, 0);
const MIN: Version = Version::new(1, 82, 0);

static RUSTC_VERSION: LazyLock<Version> =
    LazyLock::new(|| rustc_version::version().unwrap_or(MSRV));

#[cfg_attr(not(feature = "debug"), test)]
fn safe_debug_tuple() {
    let x = Tuple(1, "foo");
    if *RUSTC_VERSION < MIN {
        assert_eq!(format!("{x:?}"), r#"Tuple(1)"#);
    } else {
        assert_eq!(format!("{x:?}"), r#"Tuple(1, ..)"#);
    }
}

#[test]
fn safe_debug_empty_tuple() {
    let x = EmptyTuple();
    assert_eq!(format!("{x:?}"), r#"EmptyTuple"#);
}

#[cfg_attr(not(feature = "debug"), test)]
fn safe_debug_struct() {
    let x = Struct { a: 1, b: "foo" };
    assert_eq!(format!("{x:?}"), r#"Struct { a: 1, .. }"#);
}

#[test]
fn safe_debug_empty_struct() {
    let x = EmptyStruct {};
    assert_eq!(format!("{x:?}"), r#"EmptyStruct"#);
}

#[test]
fn safe_debug_unit_struct() {
    let x = UnitStruct;
    assert_eq!(format!("{x:?}"), r#"UnitStruct"#);
}

#[test]
fn safe_debug_enum_unit() {
    let x = Enum::Unit;
    assert_eq!(format!("{x:?}"), r#"Unit"#);
}

#[cfg_attr(not(feature = "debug"), test)]
fn safe_debug_enum_tuple() {
    let x = Enum::Tuple(1, "foo");
    if *RUSTC_VERSION < MIN {
        assert_eq!(format!("{x:?}"), r#"Tuple"#);
    } else {
        assert_eq!(format!("{x:?}"), r#"Tuple(..)"#);
    }
}

#[test]
fn safe_debug_enum_empty_tuple() {
    let x = Enum::EmptyTuple();
    assert_eq!(format!("{x:?}"), r#"EmptyTuple"#);
}

#[cfg_attr(not(feature = "debug"), test)]
fn safe_debug_enum_struct() {
    let x = Enum::Struct { a: 1, b: "foo" };
    assert_eq!(format!("{x:?}"), r#"Struct { a: 1, .. }"#);
}

#[test]
fn safe_debug_enum_empty_struct() {
    let x = Enum::EmptyStruct {};
    assert_eq!(format!("{x:?}"), r#"EmptyStruct"#);
}

#[test]
fn safe_debug_mostly_safe_struct() {
    let x = MostlySafeStruct {
        name: "Kelly Smith",
        title: "Staff Engineer",
    };
    assert_eq!(
        format!("{x:?}"),
        r#"MostlySafeStruct { title: "Staff Engineer", .. }"#
    );
}
