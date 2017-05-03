#![cfg(all(test,feature = "test_e2e"))]

extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate serde;

mod util;

use azure_sdk_for_rust::azure::storage::client::Client;
use azure_sdk_for_rust::azure::storage::container::{Container, PublicAccess,
                                                    LIST_CONTAINER_OPTIONS_DEFAULT};
use util::get_from_env;

use std::ops::Deref;

#[test]
fn create_and_delete_container() {
    let name : &'static str = "azuresdkrustetoets";
    
    let client = create_client();
    Container::create(&client, name, PublicAccess::Container).unwrap();

    let mut lco = LIST_CONTAINER_OPTIONS_DEFAULT.clone();
    lco.prefix = Some(name.to_owned());

    let list = Container::list(&client, &lco).unwrap();
    let cont_list :Vec<&Container> = list.deref().into_iter().filter(|e| e.name == name).collect();

    if cont_list.len() != 1 {
        panic!("More than 1 container returned with the same name!");
    }

    let mut cont = cont_list[0].clone();
    
    cont.delete(&client).unwrap();
}

#[test]
fn list_containers() {
    let client = create_client();

    trace!("running list_containers");
    let mut lco = LIST_CONTAINER_OPTIONS_DEFAULT.clone();
    lco.max_results = 2;

    loop {
        let ret = Container::list(&client, &lco).unwrap();

        trace!("ret {:?}\n\n", ret);
        if !ret.is_complete() {
            lco.next_marker = Some(ret.next_marker().unwrap().to_owned());
        } else {
            break;
        }
    }
}


fn create_client() -> Client {
    let azure_storage_account = get_from_env("AZURE_STORAGE_ACCOUNT");
    let azure_storage_key = get_from_env("AZURE_STORAGE_KEY");
    Client::new(&azure_storage_account, &azure_storage_key, false)
}
