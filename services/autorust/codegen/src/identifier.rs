use crate::{ErrorKind, Result, ResultExt};
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::Ident;

pub trait ToIdent: ToOwned {
    fn to_ident(&self) -> Result<Ident>;
}

impl ToIdent for str {
    fn to_ident(&self) -> Result<Ident> {
        parse_ident(self)
    }
}

pub trait CamelCaseIdent: ToOwned {
    fn to_camel_case_id(&self) -> String;
    fn to_camel_case_ident(&self) -> Result<Ident>;
}

impl CamelCaseIdent for str {
    fn to_camel_case_id(&self) -> String {
        let is_number = starts_with_number(self);
        let mut txt = replace_first(self, true, true);
        txt = replace_first(&txt, true, false);
        txt = replace_special_chars(&txt);
        if !is_number {
            // will remove underscores
            txt = txt.to_pascal_case();
        }
        txt
    }

    fn to_camel_case_ident(&self) -> Result<Ident> {
        self.to_camel_case_id().to_ident()
    }
}

pub trait SnakeCaseIdent: ToOwned {
    fn to_snake_case_id(&self) -> String;
    fn to_snake_case_ident(&self) -> Result<Ident>;
}

impl SnakeCaseIdent for str {
    fn to_snake_case_id(&self) -> String {
        let mut txt = replace_first(self, false, true);
        txt = replace_special_chars(&txt);
        txt = txt.to_snake_case();
        suffix_keyword(&txt)
    }

    fn to_snake_case_ident(&self) -> Result<Ident> {
        self.to_snake_case_id().to_ident()
    }
}

pub fn id(text: &str) -> String {
    let mut txt = replace_first(text, false, false);
    txt = replace_special_chars(&txt);
    txt = remove_spaces(&txt);
    txt = suffix_keyword(&txt);
    txt
}

pub fn parse_ident(text: &str) -> Result<Ident> {
    syn::parse_str::<Ident>(&id(text)).with_context(ErrorKind::Parse, || format!("parse ident {text}"))
}

pub fn raw_str_to_ident(text: &str) -> Result<Ident> {
    syn::parse_str::<Ident>(text).with_context(ErrorKind::Parse, || format!("parse ident {text}"))
}

fn remove_spaces(text: &str) -> String {
    text.replace(' ', "")
}

/// replace special characters with underscores
fn replace_special_chars(text: &str) -> String {
    let mut txt = text.replace('.', "_");
    txt = txt.replace(',', "_");
    txt = txt.replace('-', "_");
    txt = txt.replace('/', "_");
    txt = txt.replace('*', "_");
    txt = txt.replace(':', "_");
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
        format!("{n}{text}")
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
        format!("{text}_")
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
            // names used by autorust that we shouldn't stomp on
            | "models"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use heck::ToSnakeCase;

    #[test]
    fn test_unicode() -> Result<()> {
        assert_eq!(unicode(',', false), "u2c");
        Ok(())
    }

    #[test]
    fn test_replace_first() -> Result<()> {
        assert_eq!(replace_first(".", false, false), "u2e");
        assert_eq!(replace_first("/", false, false), "u2f");
        assert_eq!(replace_first("", false, false), "u0");
        Ok(())
    }

    #[test]
    fn test_replace_special_chars() -> Result<()> {
        assert_eq!(replace_special_chars("."), "_");
        assert_eq!(replace_special_chars(","), "_");
        assert_eq!(replace_special_chars("-"), "_");
        assert_eq!(replace_special_chars("/"), "_");
        assert_eq!(replace_special_chars("*"), "_");
        Ok(())
    }

    #[test]
    fn test_odata_next_link() -> Result<()> {
        let idt = "odata.nextLink".to_snake_case();
        let idt = id(&idt);
        assert_eq!(idt, "odata_next_link");
        Ok(())
    }

    #[test]
    fn test_three_dot_two() -> Result<()> {
        let idt = id("3.2");
        assert_eq!(idt, "n3_2");
        Ok(())
    }

    #[test]
    fn test_system_assigned_user_assigned() -> Result<()> {
        assert_eq!("SystemAssigned, UserAssigned".to_camel_case_id(), "SystemAssignedUserAssigned");
        Ok(())
    }

    #[test]
    fn test_gcm_aes_128() -> Result<()> {
        assert_eq!("gcm-aes-128".to_camel_case_id(), "GcmAes128");
        Ok(())
    }

    #[test]
    fn test_5() -> Result<()> {
        assert_eq!("5".to_camel_case_id(), "N5");
        Ok(())
    }

    #[test]
    fn test_app_configuration() -> Result<()> {
        assert_eq!(
            "Microsoft.AppConfiguration/configurationStores".to_camel_case_id(),
            "MicrosoftAppConfigurationConfigurationStores"
        );
        Ok(())
    }

    #[test]
    fn test_microsoft_key_vault_vaults() -> Result<()> {
        assert_eq!("Microsoft.KeyVault/vaults".to_camel_case_id(), "MicrosoftKeyVaultVaults");
        Ok(())
    }

    #[test]
    fn test_azure_virtual_machine_best_practices() -> Result<()> {
        assert_eq!(
            "Azure virtual machine best practices - Dev/Test".to_camel_case_id(),
            "AzureVirtualMachineBestPracticesDevTest"
        );
        Ok(())
    }

    #[test]
    fn test_1_0() -> Result<()> {
        assert_eq!("1.0".to_camel_case_id(), "N1_0");
        Ok(())
    }

    #[test]
    fn test_async() -> Result<()> {
        assert_eq!("Async".to_snake_case_id(), "async_");
        Ok(())
    }

    #[test]
    fn test_attr_qualified_name() -> Result<()> {
        assert_eq!("attr:qualifiedName".to_snake_case_id(), "attr_qualified_name");
        Ok(())
    }

    #[test]
    fn test_filter() -> Result<()> {
        assert_eq!("$filter".to_snake_case_id(), "filter");
        Ok(())
    }

    #[test]
    fn test_odata_type() -> Result<()> {
        assert_eq!("@odata.type".to_camel_case_id(), "OdataType");
        Ok(())
    }

    #[test]
    fn test_10minutely() -> Result<()> {
        assert_eq!("_10minutely".to_camel_case_id(), "N10minutely");
        Ok(())
    }
}
