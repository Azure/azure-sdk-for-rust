// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Defines a static `Url`.
#[macro_export]
macro_rules! static_url {
    ( $(#[$outer:meta])* $name:ident, $value:expr) => {
        $(#[$outer])*
        pub static $name: ::std::sync::LazyLock<$crate::http::Url> = ::std::sync::LazyLock::new(|| {
            $crate::http::Url::parse($value).expect("hardcoded URL must parse")
        });
    };
}
