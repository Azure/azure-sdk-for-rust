// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use typespec_client_core::fmt::SafeDebug;

#[derive(SafeDebug)]
#[safe(false)]
pub struct Tuple(#[safe(true)] pub i32, pub &'static str);

#[derive(SafeDebug)]
pub struct EmptyTuple();

#[derive(SafeDebug)]
pub struct Struct {
    #[safe(true)]
    pub a: i32,
    pub b: &'static str,
}

#[derive(SafeDebug)]
pub struct EmptyStruct {}

#[derive(SafeDebug)]
pub struct UnitStruct;

#[derive(SafeDebug)]
pub enum Enum {
    Unit,
    Tuple(i32, &'static str),
    EmptyTuple(),
    #[safe(true)]
    Struct {
        a: i32,
        #[safe(false)]
        b: &'static str,
    },
    EmptyStruct {},
}

#[derive(SafeDebug)]
#[safe(true)]
pub struct MostlySafeStruct {
    #[safe(false)]
    pub name: &'static str,
    pub title: &'static str,
}
