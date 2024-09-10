// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Declare a `Future` with the given name
///
/// `Future::Output` will be set to `azure_core::Result<$NAMEResponse>`.
/// The `Future` will be `Send` for all targets but `wasm32`.
#[macro_export]
macro_rules! future {
    ($name:ident) => {
        $crate::future!($name<>);
    };
    ($name:ident<$($generic:ident)?>) => {
        azure_core::__private::paste! {
        #[cfg(target_arch = "wasm32")]
        pub type $name<$($generic)*> =
            std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = azure_core::Result<[<$name Response>]<$($generic)*>>> + 'static>>;
        #[cfg(not(target_arch = "wasm32"))]
        pub type $name<$($generic)*> =
            futures::future::BoxFuture<'static, azure_core::Result<[<$name Response>]<$($generic)*>>>;
        }
    };
}

// once Rust's `lazy_cell` feature lands, this should be replaced with that.
// ref: https://github.com/rust-lang/rust/issues/109736

#[macro_export]
macro_rules! static_url {
    ( $(#[$outer:meta])* $name:ident, $value:expr) => {
        $(#[$outer])*
        pub static $name: once_cell::sync::Lazy<$crate::Url> = once_cell::sync::Lazy::new(|| {
            $crate::Url::parse($value).expect("hardcoded URL must parse")
        });
    };
}
