use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::create_generated_by_header,
    identifier::ident,
    write_file,
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("creating module name for feature {feature}: {source}")]
    ModName { source: crate::identifier::Error, feature: String },
    #[error("WriteFileError")]
    WriteFile(#[source] crate::Error),
}

pub fn create(feature_mod_names: &Vec<(String, String)>, path: &Path, print_writing_file: bool) -> Result<()> {
    write_file(path, &create_body(feature_mod_names)?, print_writing_file).map_err(Error::WriteFile)?;
    Ok(())
}

fn create_body(feature_mod_names: &Vec<(String, String)>) -> Result<TokenStream> {
    let mut cfgs = TokenStream::new();
    for (feature_name, mod_name) in feature_mod_names {
        let mod_name = ident(mod_name).map_err(|source| Error::ModName {
            source,
            feature: feature_name.to_owned(),
        })?;
        cfgs.extend(quote! {
            #[cfg(feature = #feature_name)]
            mod #mod_name;
            #[cfg(feature = #feature_name)]
            pub use #mod_name::{models, operations, API_VERSION};
        });
    }
    let generated_by = create_generated_by_header();
    Ok(quote! {
        #generated_by
        #cfgs
        use azure_core::setters;

        pub fn config(
            http_client: std::sync::Arc<dyn azure_core::HttpClient>,
            token_credential: Box<dyn azure_core::TokenCredential>,
        ) -> OperationConfigBuilder {
            OperationConfigBuilder {
                api_version: None,
                http_client,
                base_path: None,
                token_credential,
                token_credential_resource: None,
            }
        }

        pub struct OperationConfigBuilder {
            api_version: Option<String>,
            http_client: std::sync::Arc<dyn azure_core::HttpClient>,
            base_path: Option<String>,
            token_credential: Box<dyn azure_core::TokenCredential>,
            token_credential_resource: Option<String>,
        }

        impl OperationConfigBuilder {
            setters! {
                api_version: String => Some(api_version),
                base_path: String => Some(base_path),
                token_credential_resource: String => Some(token_credential_resource),
            }

            pub fn build(self) -> OperationConfig {
                OperationConfig {
                    api_version: self.api_version.unwrap_or(API_VERSION.to_owned()),
                    http_client: self.http_client,
                    base_path: self.base_path.unwrap_or("https://management.azure.com".to_owned()),
                    token_credential: Some(self.token_credential),
                    token_credential_resource: self.token_credential_resource.unwrap_or("https://management.azure.com/".to_owned()),
                }
            }
        }

        pub struct OperationConfig {
            api_version: String,
            http_client: std::sync::Arc<dyn azure_core::HttpClient>,
            base_path: String,
            token_credential: Option<Box<dyn azure_core::TokenCredential>>,
            token_credential_resource: String,
        }

        impl OperationConfig {
            pub fn api_version(&self) -> &str {
                self.api_version.as_str()
            }
            pub fn http_client(&self) -> &dyn azure_core::HttpClient {
                self.http_client.as_ref()
            }
            pub fn base_path(&self) -> &str {
                self.base_path.as_str()
            }
            pub fn token_credential(&self) -> Option<&dyn azure_core::TokenCredential> {
                self.token_credential.as_deref()
            }
            pub fn token_credential_resource(&self) -> &str {
                self.token_credential_resource.as_str()
            }
        }
    })
}
