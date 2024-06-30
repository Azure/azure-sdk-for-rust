// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Case {
    None,
    PascalCase,
    CamelCase,
    SnakeCase,
    Lowercase,
    Uppercase,
}

static CASES: &[(&str, Case)] = &[
    ("PascalCase", Case::PascalCase),
    ("camelCase", Case::CamelCase),
    ("snake_case", Case::SnakeCase),
    ("lowercase", Case::Lowercase),
    ("UPPERCASE", Case::Uppercase),
];

impl Case {
    pub fn from_str<'a>(value: &'a str) -> Result<Self, ParseError<'a>> {
        for (name, case) in CASES {
            if value == *name {
                return Ok(*case);
            }
        }

        Err(ParseError { case: value })
    }

    pub fn rename(self, variant: &str) -> String {
        match self {
            // Assumes variants are already PascalCase.
            Case::None | Case::PascalCase => variant.to_owned(),
            Case::CamelCase => variant[..1].to_ascii_lowercase() + &variant[1..],
            Case::SnakeCase => {
                let mut name = String::new();
                for (i, ch) in variant.char_indices() {
                    if i > 0 && ch.is_ascii_uppercase() {
                        name.push('_');
                    }
                    name.push(ch.to_ascii_lowercase());
                }
                name
            }
            Case::Lowercase => variant.to_ascii_lowercase(),
            Case::Uppercase => variant.to_ascii_uppercase(),
        }
    }
}

impl Default for Case {
    fn default() -> Self {
        Self::None
    }
}

pub struct ParseError<'a> {
    case: &'a str,
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "unknown case `rename_all` = {}, expected one of ",
            self.case,
        ))?;
        for (i, (name, ..)) in CASES.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            f.write_str(name)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let err = Case::from_str("other").unwrap_err();
        assert!(err.to_string().starts_with(
            "unknown case `rename_all` = other, expected one of PascalCase, camelCase"
        ));
    }

    #[test]
    fn rename_all() {
        for &(original, pascal_case, camel_case, snake_case, lowercase, uppercase) in &[
            (
                "VarName", "VarName", "varName", "var_name", "varname", "VARNAME",
            ),
            ("Base64", "Base64", "base64", "base64", "base64", "BASE64"),
        ] {
            assert_eq!(Case::None.rename(original), original);
            assert_eq!(Case::PascalCase.rename(original), pascal_case);
            assert_eq!(Case::CamelCase.rename(original), camel_case);
            assert_eq!(Case::SnakeCase.rename(original), snake_case);
            assert_eq!(Case::Lowercase.rename(original), lowercase);
            assert_eq!(Case::Uppercase.rename(original), uppercase);
        }
    }
}
