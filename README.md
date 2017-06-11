# Microsoft Azure SDK for Rust

[![Build Status](https://travis-ci.org/MindFlavor/AzureSDKForRust.svg?branch=master)](https://travis-ci.org/MindFlavor/AzureSDKForRust) [![Coverage Status](https://coveralls.io/repos/MindFlavor/AzureSDKForRust/badge.svg?branch=master&service=github)](https://coveralls.io/github/MindFlavor/AzureSDKForRust?branch=master) [![alpha](https://img.shields.io/badge/stability-alpha-yellow.svg)](https://img.shields.io/badge/stability-alpha-yellow.svg)

[![Crate](https://img.shields.io/crates/v/azure_sdk_for_rust.svg)](https://crates.io/crates/azure_sdk_for_rust) [![legal](https://img.shields.io/crates/l/azure_sdk_for_rust.svg)](LICENSE) [![cratedown](https://img.shields.io/crates/d/azure_sdk_for_rust.svg)](https://crates.io/crates/azure_sdk_for_rust) [![cratelastdown](https://img.shields.io/crates/dv/azure_sdk_for_rust.svg)](https://crates.io/crates/azure_sdk_for_rust)

[![tag](https://img.shields.io/github/tag/mindflavor/AzureSDKForRust.svg)](https://github.com/MindFlavor/AzureSDKForRust/tree/0.3.1)
[![release](https://img.shields.io/github/release/mindflavor/AzureSDKForRust.svg)](https://github.com/MindFlavor/AzureSDKForRust/tree/0.3.1)
[![commitssince](https://img.shields.io/github/commits-since/mindflavor/AzureSDKForRust/0.3.1.svg)](https://img.shields.io/github/commits-since/mindflavor/AzureSDKForRust/0.3.1.svg)

## Introduction
Microsoft Azure expose its technologies via REST API. These APIs are easily consumable from any language (good) but are weakly typed. With this library and its related [crate](https://crates.io/crates/azure_sdk_for_rust/) you can exploit the power of Microsoft Azure from Rust in a idiomatic way.

> **NOTE:** This repository is under heavy development and
is likely to break over time. The current releases will probabily contain bugs. As usual open issues if you find any.

## Disclaimer
Although I am a Microsoft employee, this is not a Microsoft endorsed project. It's simply a pet project of mine: I love Rust (who doesn't? :smirk:) and Microsoft Azure technologies so I thought to close the gap between them. It's also a good project for learning Rust. This library relies heavily on [Hyper](https://github.com/hyperium/hyper). As the time of writing master Hyper does not support Tokio yet: this SDK will than be _blocking_. I plan to switch to futures as soon as possible.

## Run E2E test
```
export AZURE_STORAGE_ACCOUNT=<account>
export AZURE_STORAGE_KEY=<key>

export AZURE_SERVICE_BUS_NAMESPACE=<azure_service_bus_namespace>
export AZURE_EVENT_HUB_NAME=<azure_event_hub_name>
export AZURE_POLICY_NAME=<azure_policy_name>
export AZURE_POLICY_KEY=<azure policy key>

cargo test --features=test_e2e
```

## Example
You can find examples in the [```tests```](https://github.com/MindFlavor/AzureSDKForRust/tree/master/tests) folder, in the [```examples```](https://github.com/MindFlavor/AzureSDKForRust/tree/master/examples) folder and in the [```src/main.rs```](https://github.com/MindFlavor/AzureSDKForRust/blob/master/src/main.rs) file (which I shall try to remove in the future). Here is a sample however:

### main.rs
```rust
extern crate azure_sdk_for_rust;
extern crate chrono;
#[macro_use]
extern crate mime;

use azure_sdk_for_rust::azure::core::lease::{LeaseState, LeaseStatus};
use azure_sdk_for_rust::azure::storage::client::Client;
use azure_sdk_for_rust::azure::storage::blob::{Blob, BlobType, PUT_OPTIONS_DEFAULT};
use azure_sdk_for_rust::azure::storage::container::{Container, PublicAccess, LIST_CONTAINER_OPTIONS_DEFAULT};

use chrono::UTC;

use mime::Mime;

fn main() {
  let azure_storage_account = &"azure_storage_account";
  let azure_storage_key= &"azure_storage_key";

  // create the client struct. The third argument, if false, forces to use
  // http instead of https. It's useful if you have trouble compiling
  // hyper with openSSL activated.
  let client = Client::new(azure_storage_account, azure_storage_key, false);


  // This call will list your containers.
  let containers = Container::list(&client, &LIST_CONTAINER_OPTIONS_DEFAULT).unwrap();
  println!("{:?}", containers);

  let container_name = "rust";
  // This call will create a new Azure Container called "wow"
  // with public blob access (see https://msdn.microsoft.com/en-us/library/azure/dd179468.aspx)
  // if it doesn't exist already.

  let cont = containers.iter().find(|x| x.name == container_name);
  if let None = cont {
  	Container::create(&client, container_name, PublicAccess::Blob).unwrap();
  }

  // this code will upload a file to the container just created.
  {
	use std::fs::metadata;
	use std::fs::File;

	let file_name: &'static str = "C:\\temp\\from_rust.txt";
	let container_name: &'static str = "wow";

	let metadata = metadata(file_name).unwrap();
	let mut file = File::open(file_name).unwrap();

	let new_blob = Blob {
		name: "from_rust.txt".to_owned(),
        container_name: container_name.to_owned(),
		snapshot_time: None,
		last_modified: UTC::now(),
		etag: "".to_owned(),
		content_length: metadata.len(),
		content_type: "application/octet-stream".parse::<Mime>().unwrap(),
		content_encoding: None,
		content_language: None,
		content_md5: None,
		cache_control: None,
		x_ms_blob_sequence_number: None,
		blob_type: BlobType::BlockBlob,
		lease_status: LeaseStatus::Unlocked,
		lease_state: LeaseState::Available,
		lease_duration: None,
		copy_id: None,
		copy_status: None,
		copy_source: None,
		copy_progress: None,
		copy_completion: None,
		copy_status_description: None,
	};

	new_blob.put(&client,
        &PUT_OPTIONS_DEFAULT,
		 Some((&mut file, metadata.len())))
		.unwrap();
  }


  // This code will look for the "todelete" container and
  // remove from Azure.
  let mut to_delete = containers.iter_mut().find(|x| x.name == "todelete").unwrap();
  to_delete.delete(&client).unwrap();
  println!("{:?} deleted!", to_delete);
}
```

## State of the art
Right now the key framework is in place (authentication, enumerations, parsing and so on). If you want to contribute please do!
Methods are added daily so please check the [CHANGELOG.md](CHANGELOG.md) for updates on the progress.
Also note that the project is in early stages so the APIs are bound to change at any moment. I will strive to keep things steady but since I'm new to Rust I'm sure I'll have to correct some serious mistake before too long :smile:.
I generally build for the latest nightly and leave to Travis to check the retrocompatibility.

## Contributing
If you want to contribute please do! No formality required! :wink:. Please note that asking for a pull request you accept to yield your code as per [Apache license, version 2.0](LICENSE).

### Implemented methods

#### Storage Container

| Method           | URL                                                                                                                          |
| ----             | ---                                                                                                                          |
| Create container | [https://msdn.microsoft.com/en-us/library/azure/dd179468.aspx](https://msdn.microsoft.com/en-us/library/azure/dd179468.aspx) |
| List containers  | [https://msdn.microsoft.com/en-us/library/azure/dd179352.aspx](https://msdn.microsoft.com/en-us/library/azure/dd179352.aspx) |
| Delete container | [https://msdn.microsoft.com/en-us/library/azure/dd179408.aspx](https://msdn.microsoft.com/en-us/library/azure/dd179408.aspx) |

#### Storage blobs

| Method          | URL                                                                                                                          |
| ----            | ---                                                                                                                          |
| List blobs      | [https://msdn.microsoft.com/en-us/library/azure/dd135734.aspx](https://msdn.microsoft.com/en-us/library/azure/dd135734.aspx) |
| Get blob        | [https://msdn.microsoft.com/en-us/library/azure/dd179440.aspx](https://msdn.microsoft.com/en-us/library/azure/dd179440.aspx) |
| Put blob        | [https://msdn.microsoft.com/en-us/library/azure/dd179451.aspx](https://msdn.microsoft.com/en-us/library/azure/dd179451.aspx) |
| Put blob page   | [https://msdn.microsoft.com/en-us/library/azure/dd179451.aspx](https://msdn.microsoft.com/en-us/library/azure/dd179451.aspx) |
| Clear blob page | [https://msdn.microsoft.com/en-us/library/azure/dd179451.aspx](https://msdn.microsoft.com/en-us/library/azure/dd179451.aspx) |
| Put block       | [https://msdn.microsoft.com/en-us/library/azure/dd135726.aspx](https://msdn.microsoft.com/en-us/library/azure/dd135726.aspx) |
| Lease blob      | [https://msdn.microsoft.com/library/azure/ee691972.aspx](https://msdn.microsoft.com/library/azure/ee691972.aspx)             |

#### Event Hubs

| Method     | URL                                                                                                                          |
| ----       | ---                                                                                                                          |
| Send Event | [https://msdn.microsoft.com/en-us/library/azure/dn790664.aspx](https://msdn.microsoft.com/en-us/library/azure/dn790664.aspx) |

#### Azure tables

| Method         | URL                                                                                                                                                  |
| ----           | ---                                                                                                                                                  |
| Create table   | [https://docs.microsoft.com/en-us/rest/api/storageservices/create-table](https://docs.microsoft.com/en-us/rest/api/storageservices/create-table)     |
| Query tables   | [https://docs.microsoft.com/en-us/rest/api/storageservices/query-tables](https://docs.microsoft.com/en-us/rest/api/storageservices/query-tables)     |
| Query entities | [https://docs.microsoft.com/en-us/rest/api/storageservices/query-entities](https://docs.microsoft.com/en-us/rest/api/storageservices/query-entities) |
| Insert entity  | [https://docs.microsoft.com/en-us/rest/api/storageservices/insert-entity](https://docs.microsoft.com/en-us/rest/api/storageservices/insert-entity)   |
| Update entity  | [https://docs.microsoft.com/en-us/rest/api/storageservices/update-entity2](https://docs.microsoft.com/en-us/rest/api/storageservices/update-entity2) |
| Delete entity  | [https://docs.microsoft.com/en-us/rest/api/storageservices/delete-entity1](https://docs.microsoft.com/en-us/rest/api/storageservices/delete-entity1) |

Azure tables entities can be manipulated in batches. The entities are serialized in ```JSON```.

#### Cosmos database

##### Database

| Method          | URL                                                                                                                                                |
| ----            | ---                                                                                                                                                |
| Create database | [https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-database](https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-database)   |
| List database   | [https://docs.microsoft.com/en-us/rest/api/documentdb/list-databases](https://docs.microsoft.com/en-us/rest/api/documentdb/list-databases)         |
| Get database    | [https://docs.microsoft.com/en-us/rest/api/documentdb/get-a-database](https://docs.microsoft.com/en-us/rest/api/documentdb/get-a-database)         |
| Delete database | [https://docs.microsoft.com/en-us/rest/api/documentdb/delete-a-database1](https://docs.microsoft.com/en-us/rest/api/documentdb/delete-a-database1) |

##### Collection

| Method          | URL                                                                                                                                                |
| ----            | ---                                                                                                                                                |
| Create collection  | [https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-collection](https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-collection)   |
| List collections   | [https://docs.microsoft.com/en-us/rest/api/documentdb/list-collections](https://docs.microsoft.com/en-us/rest/api/documentdb/list-collections)         |
| Get collection     | [https://docs.microsoft.com/en-us/rest/api/documentdb/get-a-collection](https://docs.microsoft.com/en-us/rest/api/documentdb/get-a-collection)         |
| Delete collection  | [https://docs.microsoft.com/en-us/rest/api/documentdb/delete-a-collection](https://docs.microsoft.com/en-us/rest/api/documentdb/delete-a-collection)   |
| Replace collection | [https://docs.microsoft.com/en-us/rest/api/documentdb/replace-a-collection](https://docs.microsoft.com/en-us/rest/api/documentdb/replace-a-collection) |

## License
This project is published under [Apache license, version 2.0](LICENSE).
