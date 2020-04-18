#[derive(Debug)]
pub enum ParsingError {
    ElementNotFound(String),
}

/// use as
/// ```create_enum!(SecondCollection, (Pollo, "Pollo"), (Bianco, "Bianco"), (Giallo, "Giallo"));```
#[macro_export]
macro_rules! create_enum {
    ($en:ident, $(($na:ident, $x:expr)), *) => (
        #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
        pub enum $en {
            $(
                $na,
            )*
        }

        impl ::std::convert::Into<&'static str> for $en {
            fn into(self) -> &'static str {
                match self {
                    $(
                        $en::$na => $x,
                    )*
                }
            }
        }

        impl $crate::parsing::FromStringOptional<$en> for $en {
            fn from_str_optional(s : &str) -> Result<$en, $crate::errors::TraversingError> {
                match s.parse::<$en>() {
                    Err(e) => Err($crate::errors::TraversingError::ParsingError(e)),
                    Ok(v) => Ok(v)
                }
            }
        }

        impl ::std::str::FromStr for $en {
            type Err = $crate::enumerations::ParsingError;

            fn from_str(s: &str) -> Result<$en, $crate::enumerations::ParsingError> {
                match s {
                    $(
                        $x => Ok($en::$na),
                    )*
                    _ => Err($crate::enumerations::ParsingError::ElementNotFound(s.to_owned())),
                }
            }
        }

        impl ::std::convert::AsRef<str> for $en {
            fn as_ref(&self) -> &str {
                 match *self {
                    $(
                        $en::$na => $x,
                    )*
                }
            }
        }

        impl ::std::fmt::Display for $en {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match *self {
                    $(
                        $en::$na => write!(f, "{}", $x),
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
