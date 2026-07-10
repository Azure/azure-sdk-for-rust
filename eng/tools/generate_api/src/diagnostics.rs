// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub(crate) fn info(message: impl AsRef<str>) {
    println!("{}", message.as_ref());
}

pub(crate) fn fatal(message: impl AsRef<str>) {
    eprintln!("{}", message.as_ref());
}
