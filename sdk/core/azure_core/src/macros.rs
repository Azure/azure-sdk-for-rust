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

/// The following macro invocation:
/// ```
/// # #[macro_use] extern crate azure_core;
/// request_header!(
///     /// Builds a client request id header
///     ClientRequestId, CLIENT_REQUEST_ID,
/// );
/// ```
/// Turns into a Header value used to construct requests.
#[macro_export]
macro_rules! request_header {
    ($(#[$outer:meta])* $name:ident, $header:ident) => {
        $crate::request_header!($name, $header,);
    };
    ($(#[$outer:meta])* $name:ident, $header:ident, $(($variant:ident, $value:expr)), *) => {
        $crate::request_option!($(#[$outer])* $name);
        impl $name {
            $(
                pub const $variant: $name = $name::from_static($value);
            )*
        }
        impl $crate::headers::Header for $name {
            fn name(&self) -> $crate::headers::HeaderName {
                $crate::headers::$header
            }

            fn value(&self) -> $crate::headers::HeaderValue {
                $crate::headers::HeaderValue::from_cow(self.0.clone())
            }
        }
    };
}

/// The following macro invocation:
/// ```
/// # #[macro_use] extern crate azure_core;
/// request_query!(Prefix, "prefix");
/// ```
/// Turns into a request query option used to construct requests
#[macro_export]
macro_rules! request_query {
    ($(#[$outer:meta])* $name:ident, $option:expr) => {
        $crate::request_option!($(#[$outer])* $name);
        impl $crate::AppendToUrlQuery for $name {
            fn append_to_url_query(&self, url: &mut $crate::Url) {
                url.query_pairs_mut().append_pair($option, &self.0);
            }
        }
    };
}

/// The following macro invocation:
/// ```
/// # #[macro_use] extern crate azure_core;
/// request_option!(Prefix);
/// ```
/// Turns into a request option useable either as a header or as a query string.
#[macro_export]
macro_rules! request_option {
    ($(#[$outer:meta])* $name:ident) => {
        #[derive(Debug, Clone)]
        $(#[$outer])*
        pub struct $name(std::borrow::Cow<'static, str>);

        impl $name {
            pub fn new<S>(s: S) -> Self
            where
                S: Into<std::borrow::Cow<'static, str>>,
            {
                Self(s.into())
            }

            pub const fn from_static(s: &'static str) -> Self {
                Self(std::borrow::Cow::Borrowed(s))
            }
        }

        impl<S> From<S> for $name
        where
            S: Into<std::borrow::Cow<'static, str>>,
        {
            fn from(s: S) -> Self {
                Self::new(s)
            }
        }
    };
}

/// The following macro invocation:
/// ```
/// # #[macro_use] extern crate azure_core;
/// create_enum!(Words, (Chicken, "Chicken"), (White, "White"), (Yellow, "Yellow"));
/// ```
/// Turns into a struct where each variant can be turned into and construct from the corresponding string.
#[macro_export]
macro_rules! create_enum {
    ($name:ident, $(($variant:ident, $value:expr)), *) => (
        #[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
        pub enum $name {
            $(
                $variant,
            )*
        }

        impl ::std::convert::From<$name> for &'static str {
            fn from(e: $name) -> Self {
                match e {
                    $(
                        $name::$variant => $value,
                    )*
                }
            }
        }

        impl $crate::parsing::FromStringOptional<$name> for $name {
            fn from_str_optional(s : &str) -> $crate::error::Result<$name> {
                s.parse::<$name>()
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::error::Error;

            fn from_str(s: &str) -> $crate::error::Result<$name> {
                match s {
                    $(
                        $value => Ok($name::$variant),
                    )*
                    _ => Err($crate::error::Error::with_message($crate::error::ErrorKind::DataConversion, || format!("unknown variant of {} found: \"{}\"",
                        stringify!($name),
                         s
                    )))
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;

                match s.as_ref() {
                    $(
                        $value => Ok(Self::$variant),
                    )*
                    _ => Err(serde::de::Error::custom("unsupported value")),
                }
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, s: S) -> ::core::result::Result<S::Ok, S::Error>
            where S: serde::Serializer {
                return s.serialize_str(&self.to_string())
            }
        }

        impl ::std::convert::AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                 match *self {
                    $(
                        $name::$variant => $value,
                    )*
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match *self {
                    $(
                        $name::$variant => write!(f, "{}", $value),
                    )*
                }
            }
        }
    )
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

#[cfg(test)]
mod test {
    create_enum!(Colors, (Black, "Black"), (White, "White"), (Red, "Red"));
    create_enum!(ColorsMonochrome, (Black, "Black"), (White, "White"));

    #[test]
    fn test_color_parse_1() {
        let color = "Black".parse::<Colors>().unwrap();
        assert_eq!(Colors::Black, color);
    }

    #[test]
    fn test_color_parse_2() {
        let color = "White".parse::<ColorsMonochrome>().unwrap();
        assert_eq!(ColorsMonochrome::White, color);
    }

    #[test]
    fn test_color_parse_err_1() {
        "Red".parse::<ColorsMonochrome>().unwrap_err();
    }
}
