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

        impl FromStringOptional<$en> for $en {
            fn from_str_optional(s : &str) -> Result<$en, TraversingError> {
                match s.parse::<$en>() {
                    Err(e) => Err(TraversingError::ParsingError(e)),
                    Ok(v) => Ok(v)
                }
            }
        }

        impl FromStr for $en {
            type Err = enumerations::ParsingError;

            fn from_str(s: &str) -> Result<$en, enumerations::ParsingError> {
                match s {
                    $(
                        $x => Ok($en::$na),
                    )*
                    _ => Err(enumerations::ParsingError::ElementNotFound(s.to_owned())),
                }
            }
        }

        impl AsRef<str> for $en {
            fn as_ref(&self) -> &str {
                 match *self {
                    $(
                        $en::$na => $x,
                    )*
                }
            }
        }

        impl fmt::Display for $en {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    use crate::enumerations;
    use crate::errors::TraversingError;
    use crate::parsing::FromStringOptional;
    use std::fmt;
    use std::str::FromStr;

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
