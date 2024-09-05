// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::Deserialize;
use typespec_client_core::http::Model;

#[derive(Model, Deserialize)]
#[typespec(foobar)]
pub struct NotAValidAttribute {}

#[derive(Model, Deserialize)]
#[typespec(crate = 42)]
pub struct NumericLiteralCrate {}

#[derive(Model, Deserialize)]
#[typespec(crate = "a" + "b")]
pub struct BinExprCrate {}

#[derive(Model, Deserialize)]
#[typespec(crate = @)]
pub struct UnexpectedTokenCrate {}

#[derive(Model, Deserialize)]
#[typespec(crate = "a b c")]
pub struct InvalidPathOnCrate {}

#[derive(Model, Deserialize)]
#[typespec(format = 42)]
pub struct NotAStringLiteralOnFormat {}

#[derive(Model, Deserialize)]
#[typespec = "whoop"]
pub struct NotAMetaListAttribute {}
