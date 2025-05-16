// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use safe_debug_tests::*;

#[test]
fn debug_tuple() {
    let x = Tuple(1, "foo");
    assert_eq!(format!("{x:?}"), r#"Tuple(1, "foo")"#);
}

#[test]
fn debug_empty_tuple() {
    let x = EmptyTuple();
    assert_eq!(format!("{x:?}"), r#"EmptyTuple"#);
}

#[test]
fn debug_struct() {
    let x = Struct { a: 1, b: "foo" };
    assert_eq!(format!("{x:?}"), r#"Struct { a: 1, b: "foo" }"#);
}

#[test]
fn debug_empty_struct() {
    let x = EmptyStruct {};
    assert_eq!(format!("{x:?}"), r#"EmptyStruct"#);
}

#[test]
fn debug_unit_struct() {
    let x = UnitStruct;
    assert_eq!(format!("{x:?}"), r#"UnitStruct"#);
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
fn debug_enum_empty_tuple() {
    let x = Enum::EmptyTuple();
    assert_eq!(format!("{x:?}"), r#"EmptyTuple"#);
}

#[test]
fn debug_enum_struct() {
    let x = Enum::Struct { a: 1, b: "foo" };
    assert_eq!(format!("{x:?}"), r#"Struct { a: 1, b: "foo" }"#);
}

#[test]
fn debug_enum_empty_struct() {
    let x = Enum::EmptyStruct {};
    assert_eq!(format!("{x:?}"), r#"EmptyStruct"#);
}

#[test]
fn debug_mostly_safe_struct() {
    let x = MostlySafeStruct {
        name: "Kelly Smith",
        title: "Staff Engineer",
    };
    assert_eq!(
        format!("{x:?}"),
        r#"MostlySafeStruct { name: "Kelly Smith", title: "Staff Engineer" }"#
    );
}
