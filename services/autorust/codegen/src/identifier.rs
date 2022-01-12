use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ParseIdentError {} {}", text, source)]
    ParseIdentError { source: syn::Error, text: String },
}

pub trait CamelCaseIdent: ToOwned {
    fn to_camel_case_ident(&self) -> Result<TokenStream, Error>;
}

impl CamelCaseIdent for str {
    fn to_camel_case_ident(&self) -> Result<TokenStream, Error> {
        let is_number = starts_with_number(self);
        let mut txt = replace_first(self, true, true);
        txt = replace_first(&txt, true, false);
        txt = replace_special_chars(&txt);
        if !is_number {
            // will remove underscores
            txt = txt.to_pascal_case();
        }
        let idt = syn::parse_str::<syn::Ident>(&txt).map_err(|source| Error::ParseIdentError {
            source,
            text: self.to_owned(),
        })?;
        Ok(idt.into_token_stream())
    }
}

pub trait SnakeCaseIdent: ToOwned {
    fn to_snake_case_ident(&self) -> Result<TokenStream, Error>;
}

impl SnakeCaseIdent for str {
    fn to_snake_case_ident(&self) -> Result<TokenStream, Error> {
        let mut txt = replace_first(self, false, true);
        txt = replace_special_chars(&txt);
        txt = txt.to_snake_case();
        txt = suffix_keyword(&txt);
        let idt = syn::parse_str::<syn::Ident>(&txt).map_err(|source| Error::ParseIdentError {
            source,
            text: self.to_owned(),
        })?;
        Ok(idt.into_token_stream())
    }
}

pub fn ident(text: &str) -> Result<TokenStream, Error> {
    let mut txt = replace_first(text, false, false);
    txt = replace_special_chars(&txt);
    txt = remove_spaces(&txt);
    txt = suffix_keyword(&txt);
    let idt = syn::parse_str::<syn::Ident>(&txt).map_err(|source| Error::ParseIdentError {
        source,
        text: text.to_owned(),
    })?;
    Ok(idt.into_token_stream())
}

fn remove_spaces(text: &str) -> String {
    text.replace(" ", "")
}

/// replace special characters with underscores
fn replace_special_chars(text: &str) -> String {
    let mut txt = text.replace(".", "_");
    txt = txt.replace(",", "_");
    txt = txt.replace("-", "_");
    txt = txt.replace("/", "_");
    txt = txt.replace("*", "_");
    txt = txt.replace(":", "_");
    txt
}

fn starts_with_number(text: &str) -> bool {
    match text.chars().next() {
        Some(ch) => ch.is_numeric(),
        None => false,
    }
}

fn unicode(c: char, uppercase: bool) -> String {
    let s = c.escape_unicode().to_string();
    let u = if uppercase { 'U' } else { 'u' };
    format!("{}{}", u, &s[3..s.len() - 1])
}

fn replace_first(text: &str, uppercase: bool, remove: bool) -> String {
    let first = text.chars().next().unwrap_or_default();
    if first.is_numeric() {
        let n = if uppercase { 'N' } else { 'n' };
        format!("{}{}", n, text)
    } else if !first.is_ascii_alphanumeric() {
        if text.len() > 1 {
            if remove {
                text[1..].to_owned()
            } else {
                format!("{}{}", unicode(first, uppercase), &text[1..])
            }
        } else {
            unicode(first, uppercase)
        }
    } else {
        text.to_owned()
    }
}

/// add an underscore suffix it is a keyword
fn suffix_keyword(text: &str) -> String {
    if is_keyword(text) {
        format!("{}_", text)
    } else {
        text.to_owned()
    }
}

fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        // https://doc.rust-lang.org/grammar.html#keywords
        "abstract"
            | "alignof"
            | "as"
            | "async"
            | "become"
            | "box"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "do"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "final"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "macro"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "offsetof"
            | "override"
            | "priv"
            | "proc"
            | "pub"
            | "pure"
            | "ref"
            | "return"
            | "Self"
            | "self"
            | "sizeof"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "typeof"
            | "unsafe"
            | "unsized"
            | "use"
            | "virtual"
            | "where"
            | "while"
            | "yield"
        // required for Default::default support
            | "default"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use heck::ToSnakeCase;

    #[test]
    fn test_unicode() -> Result<(), Error> {
        assert_eq!(unicode(',', false), "u2c");
        Ok(())
    }

    #[test]
    fn test_replace_first() -> Result<(), Error> {
        assert_eq!(replace_first(".", false, false), "u2e");
        assert_eq!(replace_first("/", false, false), "u2f");
        assert_eq!(replace_first("", false, false), "u0");
        Ok(())
    }

    #[test]
    fn test_replace_special_chars() -> Result<(), Error> {
        assert_eq!(replace_special_chars("."), "_");
        assert_eq!(replace_special_chars(","), "_");
        assert_eq!(replace_special_chars("-"), "_");
        assert_eq!(replace_special_chars("/"), "_");
        assert_eq!(replace_special_chars("*"), "_");
        Ok(())
    }

    #[test]
    fn test_odata_next_link() -> Result<(), Error> {
        let idt = "odata.nextLink".to_snake_case();
        let idt = ident(&idt)?;
        assert_eq!(idt.to_string(), "odata_next_link");
        Ok(())
    }

    #[test]
    fn test_three_dot_two() -> Result<(), Error> {
        let idt = ident("3.2")?;
        assert_eq!(idt.to_string(), "n3_2");
        Ok(())
    }

    #[test]
    fn test_system_assigned_user_assigned() -> Result<(), Error> {
        assert_eq!(
            "SystemAssigned, UserAssigned".to_camel_case_ident()?.to_string(),
            "SystemAssignedUserAssigned"
        );
        Ok(())
    }

    #[test]
    fn test_gcm_aes_128() -> Result<(), Error> {
        assert_eq!("gcm-aes-128".to_camel_case_ident()?.to_string(), "GcmAes128");
        Ok(())
    }

    #[test]
    fn test_5() -> Result<(), Error> {
        assert_eq!("5".to_camel_case_ident()?.to_string(), "N5");
        Ok(())
    }

    #[test]
    fn test_app_configuration() -> Result<(), Error> {
        assert_eq!(
            "Microsoft.AppConfiguration/configurationStores".to_camel_case_ident()?.to_string(),
            "MicrosoftAppConfigurationConfigurationStores"
        );
        Ok(())
    }

    #[test]
    fn test_microsoft_key_vault_vaults() -> Result<(), Error> {
        assert_eq!(
            "Microsoft.KeyVault/vaults".to_camel_case_ident()?.to_string(),
            "MicrosoftKeyVaultVaults"
        );
        Ok(())
    }

    #[test]
    fn test_azure_virtual_machine_best_practices() -> Result<(), Error> {
        assert_eq!(
            "Azure virtual machine best practices - Dev/Test".to_camel_case_ident()?.to_string(),
            "AzureVirtualMachineBestPracticesDevTest"
        );
        Ok(())
    }

    #[test]
    fn test_1_0() -> Result<(), Error> {
        assert_eq!("1.0".to_camel_case_ident()?.to_string(), "N1_0");
        Ok(())
    }

    #[test]
    fn test_async() -> Result<(), Error> {
        assert_eq!("Async".to_snake_case_ident()?.to_string(), "async_");
        Ok(())
    }

    #[test]
    fn test_attr_qualified_name() -> Result<(), Error> {
        assert_eq!("attr:qualifiedName".to_snake_case_ident()?.to_string(), "attr_qualified_name");
        Ok(())
    }

    #[test]
    fn test_filter() -> Result<(), Error> {
        assert_eq!("$filter".to_snake_case_ident()?.to_string(), "filter");
        Ok(())
    }

    #[test]
    fn test_odata_type() -> Result<(), Error> {
        assert_eq!("@odata.type".to_camel_case_ident()?.to_string(), "OdataType");
        Ok(())
    }

    #[test]
    fn test_10minutely() -> Result<(), Error> {
        assert_eq!("_10minutely".to_camel_case_ident()?.to_string(), "N10minutely");
        Ok(())
    }
}
