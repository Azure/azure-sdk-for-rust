/// Creates setter methods
///
/// The methods created are of the form `with_$name` that takes an argument of type `$typ`
/// and sets the field $name to result of calling `$transform` with the value of the argument.
///
/// In other words. The following macro call:
/// ```
/// # #[macro_use] extern crate azure_core;
/// struct MyStruct<'a> { foo: Option<&'a str> };
/// impl <'a> MyStruct<'a> {
///     setters! { foo: &'a str => Some(foo), }
/// }
/// ```
/// Roughly expands to:
/// ```
/// struct MyStruct<'a> { foo: Option<&'a str> };
/// impl <'a> MyStruct<'a> {
///     fn with_foo(self, foo: &'a str) -> Self {
///         Self {
///             foo: Some(foo),
///             ..self
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! setters {
    (@single $name:ident : $typ:ty => $transform:expr) => {
        pub fn $name<T: ::std::convert::Into<$typ>>(self, $name: T) -> Self {
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
    (@recurse $name:ident : $typ:ty, $($tokens:tt)*) => {
        setters! { @recurse $name: $typ => $name, $($tokens)* }
    };
    // Recurse with transform
    (@recurse $name:ident : $typ:ty => $transform:expr, $($tokens:tt)*) => {
        setters! { @single $name : $typ => $transform }
        setters! { @recurse $($tokens)* }
    };
    ($($tokens:tt)*) => {
        setters! { @recurse $($tokens)* }
    }
}

/// The following macro invocation:
/// ```
/// # #[macro_use] extern crate azure_core;
/// create_enum!(Words, (Pollo, "Pollo"), (Bianco, "Bianco"), (Giallo, "Giallo"));
/// ```
/// Turns into a struct where each variant can be turned into and construct from the corresponding string.
#[macro_export]
macro_rules! create_enum {
    ($name:ident, $(($variant:ident, $value:expr)), *) => (
        #[derive(Debug, PartialEq, PartialOrd, Clone, Copy, ::serde::Serialize, ::serde::Deserialize)]
        pub enum $name {
            $(
                $variant,
            )*
        }

        impl ::std::convert::Into<&'static str> for $name {
            fn into(self) -> &'static str {
                match self {
                    $(
                        $name::$variant => $value,
                    )*
                }
            }
        }

        impl $crate::parsing::FromStringOptional<$name> for $name {
            fn from_str_optional(s : &str) -> ::std::result::Result<$name, $crate::errors::TraversingError> {
                match s.parse::<$name>() {
                    Ok(v) => Ok(v),
                    Err(e) => Err($crate::errors::TraversingError::ParsingError(e)),
                }
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::errors::ParsingError;

            fn from_str(s: &str) -> ::std::result::Result<$name, $crate::errors::ParsingError> {
                match s {
                    $(
                        $value => Ok($name::$variant),
                    )*
                    _ => Err($crate::errors::ParsingError::ElementNotFound(s.to_owned())),
                }
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
    #[should_panic(expected = "ElementNotFound(\"Red\")")]
    fn test_color_parse_err_1() {
        "Red".parse::<ColorsMonochrome>().unwrap();
    }
}
