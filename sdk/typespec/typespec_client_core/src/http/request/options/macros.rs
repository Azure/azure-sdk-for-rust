// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Creates named value type that can be used to construct requests.
///
/// # Examples
///
/// The following macro invocation:
/// ```
/// # #[macro_use] extern crate typespec_client_core;
/// use typespec_client_core::http::headers::ACCEPT;
///
/// request_header!(
///     /// Builds an "accept" header.
///     Accept, ACCEPT,
/// );
/// ```
/// Turns into a Header value used to construct requests.
#[macro_export]
macro_rules! request_header {
    ($(#[$outer:meta])* $name:ident, $header:ident) => {
        $crate::request_header!($name, $header,);
    };
    ($(#[$outer:meta])* $name:ident, $header:ident, $(($(#[$inner:meta])*$variant:ident, $value:expr)), *) => {
        $crate::request_option!($(#[$outer])* $name);
        impl $name {
            $(
                $(#[$inner])*
                pub const $variant: $name = $name::from_static($value);
            )*
        }
        impl $crate::http::headers::Header for $name {
            fn name(&self) -> $crate::http::headers::HeaderName {
                $header
            }

            fn value(&self) -> $crate::http::headers::HeaderValue {
                $crate::http::headers::HeaderValue::from_cow(self.0.clone())
            }
        }
    };
}

/// Creates a named value type that can be used as a header or in a query string.
///
/// # Examples
///
/// The following macro invocation:
/// ```
/// # #[macro_use] extern crate typespec_client_core;
/// request_option!(Prefix);
/// ```
/// Turns into a request option useable either as a header or in a query string.
#[macro_export]
macro_rules! request_option {
    ($(#[$outer:meta])* $name:ident) => {
        #[derive(Debug, Clone)]
        $(#[$outer])*
        pub struct $name(std::borrow::Cow<'static, str>);

        impl $name {

            /// Creates a new instance of the request option type, which can be used as an HTTP header or query parameter.
            ///
            /// This function is typically used to construct a value representing a request option for use in HTTP requests.
            /// The specific usage (header or query parameter) depends on how the macro is invoked.
            pub fn new<S>(s: S) -> Self
            where
                S: Into<std::borrow::Cow<'static, str>>,
            {
                Self(s.into())
            }

            /// Creates a new instance of the request option from a static string slice.
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

/// Creates a named value that can be used in a query string.
///
/// # Examples
///
/// The following macro invocation:
/// ```
/// # #[macro_use] extern crate typespec_client_core;
/// request_query!(Prefix, "prefix");
/// ```
/// Turns into a request query option used to construct requests
#[macro_export]
macro_rules! request_query {
    ($(#[$outer:meta])* $name:ident, $option:expr) => {
        $crate::request_option!($(#[$outer])* $name);
        impl $crate::http::AppendToUrlQuery for $name {
            fn append_to_url_query(&self, url: &mut $crate::http::Url) {
                url.query_pairs_mut().append_pair($option, &self.0);
            }
        }
    };
}
