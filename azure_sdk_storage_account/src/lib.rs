#![recursion_limit = "128"]

#[macro_use]
extern crate log;
pub mod account;
pub mod prelude;

use azure_sdk_storage_core::client::Client;

pub trait Account {
    #[allow(clippy::needless_lifetimes)]
    fn get_account_information<'a>(&'a self)
        -> account::requests::GetAccountInformationBuilder<'a>;
}

impl Account for Client {
    #[allow(clippy::needless_lifetimes)]
    fn get_account_information<'a>(
        &'a self,
    ) -> account::requests::GetAccountInformationBuilder<'a> {
        account::requests::GetAccountInformationBuilder::new(self)
    }
}
