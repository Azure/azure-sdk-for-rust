// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Creates an enum with a fixed set of variants.
///
/// This macro creates an enum where each variant can be turned into and constructed from the corresponding string.
/// The [`std::str::FromStr`] implementation will return a [`typespec::error::Error`] if not supported (case-sensitive).
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate typespec_client_core;
/// create_enum!(
///     #[doc = "Example words"]
///     Words,
///     #[doc = "Poultry"]
///     (Chicken, "Chicken"),
///     (White, "White"),
///     (Yellow, "Yellow")
/// );
///
/// let word = Words::Chicken;
/// assert_eq!(word.to_string(), String::from("Chicken"));
/// ```
#[macro_export]
macro_rules! create_enum {
    ($(#[$type_meta:meta])* $name:ident, $($(#[$value_meta:meta])* ($variant:ident, $value:expr)),* $(,)?) => (
        $(#[$type_meta])*
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        #[non_exhaustive]
        pub enum $name {
            $(
                $(#[$value_meta])*
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

        impl ::std::convert::AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                 match self {
                    $(
                        $name::$variant => $value,
                    )*
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(
                        $name::$variant => write!(f, "{}", $value),
                    )*
                }
            }
        }

        create_enum!(@intern $name);
    );

    (@intern $name:ident) => (
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                s.parse().map_err(serde::de::Error::custom)
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, s: S) -> ::core::result::Result<S::Ok, S::Error>
            where S: serde::Serializer {
                return s.serialize_str(&self.to_string())
            }
        }

        impl $crate::parsing::FromStringOptional<$name> for $name {
            fn from_str_optional(s : &str) -> $crate::error::Result<$name> {
                s.parse::<$name>().map_err(::core::convert::Into::into)
            }
        }
    );
}

/// Creates an enum with a set of variants including `UnknownValue` which holds any unsupported string from which it was created.
///
/// This macro creates an enum where each variant can be turned into and constructed from the corresponding string.
/// The [`std::str::FromStr`] implementation will not return an error but instead store the string in `UnknownValue(String)`.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate typespec_client_core;
/// create_extensible_enum!(
///     #[doc = "Example words"]
///     Words,
///     #[doc = "Poultry"]
///     (Chicken, "Chicken"),
///     (White, "White"),
///     (Yellow, "Yellow")
/// );
///
/// let word: Words = "Turkey".parse().unwrap();
/// assert_eq!(word.to_string(), String::from("Turkey"));
/// ```
#[macro_export]
macro_rules! create_extensible_enum {
    ($(#[$type_meta:meta])* $name:ident, $($(#[$value_meta:meta])* ($variant:ident, $value:expr)),* $(,)?) => (
        $(#[$type_meta])*
        #[derive(Debug, PartialEq, Eq, Clone)]
        #[non_exhaustive]
        pub enum $name {
            $(
                $(#[$value_meta])*
                $variant,
            )*
            /// Any other value not defined in `$name`.
            UnknownValue(String),
        }

        impl<'a> ::std::convert::From<&'a $name> for &'a str {
            fn from(e: &'a $name) -> Self {
                match e {
                    $(
                        $name::$variant => $value,
                    )*
                    $name::UnknownValue(s) => s.as_ref(),
                }
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = ::std::convert::Infallible;

            fn from_str(s: &str) -> ::core::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
                Ok(match s {
                    $(
                        $value => $name::$variant,
                    )*
                    _ => $name::UnknownValue(s.to_string()),
                })
            }
        }

        impl ::std::convert::AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                 match self {
                    $(
                        $name::$variant => $value,
                    )*
                    $name::UnknownValue(s) => s.as_str(),
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(
                        $name::$variant => f.write_str($value),
                    )*
                    $name::UnknownValue(s) => f.write_str(s.as_str()),
                }
            }
        }

        create_enum!(@intern $name);
    );
}

/// Creates setter methods
///
/// The methods created are of the form `$name` that takes an argument of type `$typ`
/// and sets the field `$name` to result of calling `$transform` with the value of the argument.
///
/// In other words. The following macro call:
/// ```
/// # #[macro_use] extern crate typespec_client_core;
/// struct MyStruct<'a> { foo: Option<&'a str> };
/// impl <'a> MyStruct<'a> {
///     setters! { foo: &'a str => Some(foo), }
/// }
/// ```
/// Roughly expands to:
/// ```
/// struct MyStruct<'a> { foo: Option<&'a str> };
/// impl <'a> MyStruct<'a> {
///     fn foo(self, foo: &'a str) -> Self {
///         Self {
///             foo: Some(foo),
///             ..self
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! setters {
    (@single $(#[$meta:meta])* $name:ident : $typ:ty => $transform:expr) => {
        #[allow(clippy::redundant_field_names)]
        #[allow(clippy::needless_update)]
        #[allow(missing_docs)]
        #[must_use]
        $(#[$meta])*
        pub fn $name<P: ::std::convert::Into<$typ>>(self, $name: P) -> Self {
            let $name: $typ = $name.into();
            Self  {
                $name: $transform,
                ..self
            }
        }
    };
    // Terminal condition
    (@recurse) => {};
    // Recurse without transform
    (@recurse $(#[$meta:meta])* $name:ident : $typ:ty, $($tokens:tt)*) => {
        $crate::setters! { @recurse $(#[$meta])* $name: $typ => $name, $($tokens)* }
    };
    // Recurse with transform
    (@recurse $(#[$meta:meta])* $name:ident : $typ:ty => $transform:expr, $($tokens:tt)*) => {
        $crate::setters! { @single $(#[$meta])* $name : $typ => $transform }
        $crate::setters! { @recurse $($tokens)* }
    };
    ($($tokens:tt)*) => {
        $crate::setters! { @recurse $($tokens)* }
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    create_enum!(Colors, (Black, "Black"), (White, "White"), (Red, "Red"));
    create_enum!(ColorsMonochrome, (Black, "Black"), (White, "White"));

    // cspell:ignore metasyntactic
    create_extensible_enum!(
        Metasyntactic,
        (Foo, "foo"),
        (Bar, "bar"),
        (Baz, "baz"),
        (Qux, "qux"),
    );

    #[derive(Debug, Default, Deserialize, Serialize)]
    #[serde(default)]
    struct TestData {
        #[serde(skip_serializing_if = "Option::is_none")]
        color: Option<Colors>,
        #[serde(skip_serializing_if = "Option::is_none")]
        meta: Option<Metasyntactic>,
    }

    struct Options {
        a: Option<String>,
        b: u32,
    }

    #[allow(dead_code)]
    impl Options {
        setters! {
            a: String => Some(a),
            b: u32 => b,
        }
    }

    impl Default for Options {
        fn default() -> Self {
            Options { a: None, b: 1 }
        }
    }

    #[test]
    fn color_parse_1() {
        let color = "Black".parse::<Colors>().unwrap();
        assert_eq!(Colors::Black, color);
    }

    #[test]
    fn color_parse_2() {
        let color = "White".parse::<ColorsMonochrome>().unwrap();
        assert_eq!(ColorsMonochrome::White, color);
    }

    #[test]
    fn color_parse_err_1() {
        "Red".parse::<ColorsMonochrome>().unwrap_err();
    }

    #[test]
    fn setters() {
        let options = Options::default().a("test".to_owned());

        assert_eq!(Some("test".to_owned()), options.a);
        assert_eq!(1, options.b);
    }

    #[test]
    fn deserialize_enum() {
        let data: TestData = serde_json::from_str(r#"{"color": "Black"}"#).unwrap();
        assert_eq!(Some(Colors::Black), data.color);
    }

    #[test]
    fn deserialize_extensible_enum() {
        // Variant values are case-sensitive.
        let data: TestData = serde_json::from_str(r#"{"meta": "Foo"}"#).unwrap();
        assert_eq!(
            Some(Metasyntactic::UnknownValue(String::from("Foo"))),
            data.meta
        );
    }

    #[test]
    fn serialize_enum() {
        let data = TestData {
            color: Some(Colors::Red),
            ..Default::default()
        };
        let json = serde_json::to_string(&data).unwrap();
        assert_eq!(String::from(r#"{"color":"Red"}"#), json);
    }

    #[test]
    fn serialize_extensible_enum() {
        let data = TestData {
            meta: Some(Metasyntactic::Foo),
            ..Default::default()
        };
        let json = serde_json::to_string(&data).unwrap();
        assert_eq!(String::from(r#"{"meta":"foo"}"#), json);
    }
}
