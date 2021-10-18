use heck::CamelCase;
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
        let mut txt = replace_special_chars(self);
        txt = prefix_number(&txt);
        txt = txt.to_camel_case();
        let idt = syn::parse_str::<syn::Ident>(&txt).map_err(|source| Error::ParseIdentError {
            source,
            text: self.to_owned(),
        })?;
        Ok(idt.into_token_stream())
    }
}

pub fn ident(text: &str) -> Result<TokenStream, Error> {
    let mut txt = replace_special_chars(text);
    txt = remove_spaces(&txt);
    txt = prefix_number(&txt);
    txt = prefix_keyword(&txt);
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
    txt
}

/// identifiers can not start with a number of underscore
fn starts_with_number(text: &str) -> bool {
    match text.chars().next() {
        Some(ch) => match ch {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' | '_' => true,
            _ => false,
        },
        None => false,
    }
}

/// add a prefix of `n` if it starts with a number
fn prefix_number(text: &str) -> String {
    if starts_with_number(text) {
        format!("n{}", text)
    } else {
        text.to_owned()
    }
}

/// add an underscore suffix it is a keyword
fn prefix_keyword(text: &str) -> String {
    if is_keyword(&text) {
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
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use heck::SnakeCase;

    fn unicode(c: char) -> String {
        let s = c.escape_unicode().to_string();
        format!("u{}", &s[3..s.len() - 1])
    }

    #[test]
    fn test_unicode() -> Result<(), Error> {
        assert_eq!(unicode(','), "u2c");
        Ok(())
    }

    #[test]
    fn test_chars() -> Result<(), Error> {
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
        assert_eq!("1.0".to_camel_case_ident()?.to_string(), "N10");
        Ok(())
    }
}
