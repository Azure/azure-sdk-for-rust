// #![feature(plugin)]
// #![plugin(clippy)]

#[macro_use]
extern crate hyper;
extern crate hyper_native_tls;
extern crate chrono;
#[macro_use]
extern crate url;
extern crate crypto;
extern crate base64;
extern crate xml;
extern crate uuid;
extern crate time;

#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


#[macro_use]
pub mod azure;
