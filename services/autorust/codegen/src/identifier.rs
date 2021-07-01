use heck::CamelCase;
use proc_macro2::TokenStream;
use quote::ToTokens;

pub type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ParseIdentError {} {}", text, source)]
    ParseIdentError { source: syn::Error, text: String },
}

pub trait CamelCaseIdent: ToOwned {
    fn to_camel_case_ident(&self) -> Result<TokenStream>;
}

impl CamelCaseIdent for str {
    fn to_camel_case_ident(&self) -> Result<TokenStream> {
        let mut txt = replace_chars_with_unicode_names(self);
        txt = replace_chars_with_underscore(&txt);
        txt = if starts_with_number(&txt) {
            prefix_with_underscore_if_starts_with_number(&txt)
        } else {
            txt.to_camel_case()
        };
        let idt = syn::parse_str::<syn::Ident>(&txt).map_err(|source| Error::ParseIdentError {
            source,
            text: txt.to_owned(),
        })?;
        Ok(idt.into_token_stream())
    }
}

pub fn ident(text: &str) -> Result<TokenStream> {
    let mut txt = replace_chars_with_underscore(text);
    txt = remove_spaces(&txt);
    txt = prefix_with_underscore_if_starts_with_number(&txt);
    txt = prefix_with_underscore_keywords(&txt);
    let idt = syn::parse_str::<syn::Ident>(&txt).map_err(|source| Error::ParseIdentError {
        source,
        text: txt.to_owned(),
    })?;
    Ok(idt.into_token_stream())
}

fn remove_spaces(text: &str) -> String {
    text.replace(" ", "")
}

fn replace_chars_with_underscore(text: &str) -> String {
    let mut txt = text.replace(".", "_");
    txt = txt.replace(",", "_");
    txt = txt.replace("-", "_");
    txt = txt.replace("/", "_");
    txt
}

/// Replace some special charaters with their unicode names
fn replace_chars_with_unicode_names(text: &str) -> String {
    text.replace("*", "Asterisk")
}

fn starts_with_number(text: &str) -> bool {
    match text.chars().next() {
        Some(ch) => match ch {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => true,
            _ => false,
        },
        None => false,
    }
}

fn prefix_with_underscore_if_starts_with_number(text: &str) -> String {
    if starts_with_number(text) {
        format!("_{}", text)
    } else {
        text.to_owned()
    }
}

fn prefix_with_underscore_keywords(text: &str) -> String {
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
    use heck::SnakeCase;

    use super::*;

    #[test]
    fn test_odata_next_link() -> Result<()> {
        let idt = "odata.nextLink".to_snake_case();
        assert_eq!(idt, "odata.next_link");
        let idt = ident(&idt)?;
        assert_eq!(idt.to_string(), "odata_next_link");
        Ok(())
    }

    #[test]
    fn test_three_dot_two() -> Result<()> {
        let idt = ident("3.2")?;
        assert_eq!(idt.to_string(), "_3_2");
        Ok(())
    }

    #[test]
    fn test_asterisk() -> Result<()> {
        assert_eq!("*".to_camel_case(), "");
        assert_eq!("*".to_camel_case_ident()?.to_string(), "Asterisk");
        Ok(())
    }

    #[test]
    fn test_system_assigned_user_assigned() -> Result<()> {
        assert_eq!(
            "SystemAssigned, UserAssigned".to_camel_case_ident()?.to_string(),
            "SystemAssignedUserAssigned"
        );
        Ok(())
    }

    #[test]
    fn test_gcm_aes_128() -> Result<()> {
        assert_eq!("gcm-aes-128".to_camel_case_ident()?.to_string(), "GcmAes128");
        Ok(())
    }

    #[test]
    fn test_5() -> Result<()> {
        assert_eq!("5".to_camel_case_ident()?.to_string(), "_5");
        Ok(())
    }

    #[test]
    fn test_app_configuration() -> Result<()> {
        assert_eq!(
            "Microsoft.AppConfiguration/configurationStores".to_camel_case_ident()?.to_string(),
            "MicrosoftAppConfigurationConfigurationStores"
        );
        Ok(())
    }

    #[test]
    fn test_microsoft_key_vault_vaults() -> Result<()> {
        assert_eq!(
            "Microsoft.KeyVault/vaults".to_camel_case_ident()?.to_string(),
            "MicrosoftKeyVaultVaults"
        );
        Ok(())
    }

    #[test]
    fn test_azure_virtual_machine_best_practices() -> Result<()> {
        assert_eq!(
            "Azure virtual machine best practices – Dev/Test".to_camel_case_ident()?.to_string(),
            "AzureVirtualMachineBestPracticesDevTest"
        );
        Ok(())
    }

    #[test]
    fn test_1_0() -> Result<()> {
        assert_eq!("1.0".to_camel_case_ident()?.to_string(), "_1_0");
        Ok(())
    }
}
