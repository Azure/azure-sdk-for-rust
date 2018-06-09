#![feature(try_from)]
#![recursion_limit = "128"]

extern crate chrono;
#[macro_use]
extern crate hyper;
extern crate mime;

extern crate futures;
extern crate hyper_tls;
extern crate native_tls;
extern crate tokio_core;

extern crate base64;
extern crate ring;
extern crate time;
#[macro_use]
extern crate url;
extern crate uuid;
extern crate xml;

#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate smallvec;

extern crate crypto;

extern crate serde_xml_rs;

#[macro_use]
mod azure;
pub use azure::*;
