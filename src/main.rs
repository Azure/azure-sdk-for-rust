#[macro_use]
extern crate hyper;
extern crate chrono;
extern crate url;
extern crate crypto;
extern crate rustc_serialize as serialize;
extern crate xml;


#[macro_use]
pub mod azure;

use azure::storage::client;
// use azure::storage::container::PublicAccess;

fn main() {
    let azure_storage_account = match std::env::var("AZURE_STORAGE_ACCOUNT") {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set AZURE_STORAGE_ACCOUNT env variable first!");
        }
    };

    let azure_storage_key = match std::env::var("AZURE_STORAGE_KEY") {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set AZURE_STORAGE_KEY env variable first!");
        }
    };

    let client = client::new(azure_storage_account, azure_storage_key, false);

    // client.create_container("balocco3", PublicAccess::Blob).unwrap();
    // // println!("{:?}", new);
    //
    let mut ret = client.list_containers().unwrap();
    println!("{:?}", ret);

    let vhds = ret.iter_mut().find(|x| x.name == "canotto").unwrap();

    let blobs = vhds.list_blobs(&client, true, true, true, true).unwrap();

    println!("len == {:?}", blobs.len());

    blobs.iter()
         .map(|x| {
             println!("{}, {} KB ({:?})",
                      x.name,
                      (x.content_length / 1024),
                      x.lease_state)
         })
         .collect::<Vec<()>>();

    // bal2.delete(&client).unwrap();
    // println!("{:?} deleted!", bal2);

    // let ret = client.delete_container("balocco2").unwrap();
    // println!("{:?}", ret);
    // inc_a!("main");
}
