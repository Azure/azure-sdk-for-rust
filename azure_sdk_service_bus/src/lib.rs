#![recursion_limit = "128"]

extern crate base64;
extern crate chrono;
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
extern crate bytes;
extern crate quick_error;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate serde_xml_rs;
extern crate smallvec;

pub mod event_hub;
pub mod prelude;
