// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub fn url_encode(s: impl AsRef<[u8]>) -> String {
    url::form_urlencoded::byte_serialize(s.as_ref()).collect::<String>()
}

// TODO: Don't merge with this, it's just to prove the pack code fails on warnings
pub fn dummy_change() {}
