// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub trait BlobKind {}
impl BlobKind for Unset {}
impl BlobKind for Block {}
impl BlobKind for Page {}
impl BlobKind for Append {}
pub struct Unset;
pub struct Block;
pub struct Page;
pub struct Append;
