// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::Deserialize;
use typespec_client_core::http::Model;

#[derive(Model, Deserialize)]
#[typespec(format = "blarg")]
pub struct MyModel {}
