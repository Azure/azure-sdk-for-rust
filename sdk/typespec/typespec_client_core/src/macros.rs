// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// The following macro invocation:
/// ```
/// # #[macro_use] extern crate typespec_client_core;
/// create_enum!(Words, (Chicken, "Chicken"), (White, "White"), (Yellow, "Yellow"));
/// ```
/// Turns into a struct where each variant can be turned into and construct from the corresponding string.
#[macro_export]
macro_rules! create_enum {
    ($(#[$type_doc:meta])* $name:ident, $($(#[$val_doc:meta])* ($variant:ident, $value:expr)), *) => (
        $(#[$type_doc])*
        #[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
        pub enum $name {
            $(
                $(#[$val_doc])*
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
    create_enum!(Colors, (Black, "Black"), (White, "White"), (Red, "Red"));
    create_enum!(ColorsMonochrome, (Black, "Black"), (White, "White"));

    create_enum!(
        #[doc = "Defines operation states"]
        OperationState,

        #[doc = "The operation hasn't started"]
        (NotStarted, "notStarted"),

        #[doc = "The operation is in progress"]
        (InProgress, "inProgress"),

        #[doc = "The operation has completed"]
        (Completed, "completed")
    );

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

    #[test]
    fn test_setters() {
        let options = Options::default().a("test".to_owned());

        assert_eq!(Some("test".to_owned()), options.a);
        assert_eq!(1, options.b);
    }
}
