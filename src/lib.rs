// #![feature(plugin)]
// #![plugin(clippy)]

#[macro_use]
extern crate hyper;
extern crate chrono;
extern crate url;
extern crate crypto;
extern crate rustc_serialize;
extern crate xml;
extern crate uuid;
extern crate time;
extern crate mime;

#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;

#[macro_use]
pub mod azure;
