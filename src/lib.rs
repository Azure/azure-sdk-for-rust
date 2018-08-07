#![recursion_limit = "128"]

extern crate base64;
extern crate chrono;
extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate md5;
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
extern crate bytes;
extern crate serde_json;
extern crate serde_xml_rs;
extern crate smallvec;

#[macro_use]
mod azure;
pub use azure::*;
pub mod prelude;
