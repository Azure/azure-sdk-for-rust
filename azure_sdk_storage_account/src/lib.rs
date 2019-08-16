#![recursion_limit = "128"]

extern crate base64;
extern crate chrono;
extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_rustls;
extern crate md5;
extern crate ring;
extern crate time;
extern crate url;
extern crate uuid;
extern crate xml;
#[macro_use]
extern crate log;
extern crate azure_sdk_core;
extern crate azure_sdk_storage_core;
extern crate bytes;
extern crate quick_error;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate serde_xml_rs;
extern crate smallvec;
pub mod account;
pub mod prelude;

use azure_sdk_storage_core::client::Client;

pub trait Account {
    #[allow(clippy::needless_lifetimes)]
    fn get_account_information<'a>(&'a self) -> account::requests::GetAccountInformationBuilder<'a>;
}

impl Account for Client {
    #[allow(clippy::needless_lifetimes)]
    fn get_account_information<'a>(&'a self) -> account::requests::GetAccountInformationBuilder<'a> {
        account::requests::GetAccountInformationBuilder::new(self)
    }
}
