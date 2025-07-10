// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub enum AttributeArray {
    Bool(Vec<bool>),
    I64(Vec<i64>),
    U64(Vec<u64>),
    String(Vec<String>),
}

pub enum AttributeValue {
    Bool(bool),
    I64(i64),
    U64(u64),
    String(String),
    Array(AttributeArray),
}
