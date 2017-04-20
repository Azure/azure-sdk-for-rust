// #![cfg(all(test,feature = "test_e2e"))]

extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rustc_serialize;

mod util;

use azure_sdk_for_rust::azure::storage::client::Client;
use azure_sdk_for_rust::azure::storage::container::{Container, PublicAccess,
                                                    LIST_CONTAINER_OPTIONS_DEFAULT};
use util::get_from_env;

// TODO, add validation logic

#[test]
fn list_containers() {
    let ref client = create_client();
    let _ = Container::create(client, "con1", PublicAccess::Container);

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
