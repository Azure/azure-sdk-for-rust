// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
mod azure_key_credential;
mod openai_key_credential;

pub(crate) use azure_key_credential::*;
pub(crate) use openai_key_credential::*;

pub(crate) const DEFAULT_SCOPE: [&str; 1] = ["https://cognitiveservices.azure.com/.default"];
