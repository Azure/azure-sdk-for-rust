#![allow(unused_imports)]

use azure::core::range;
use azure::core::HTTPMethod;
use azure::core::lease::{LeaseId, LeaseStatus, LeaseState, LeaseDuration, LeaseAction};
use chrono;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use hyper;
use hyper::Client;
use hyper::header::{Header, HeaderFormat, Headers, ContentEncoding, ContentLanguage,
                    ContentLength, ContentType, Date, IfModifiedSince, IfUnmodifiedSince};
use base64;
use std::fmt::Display;
use std::io::Read;
use url;

use url::percent_encoding::{utf8_percent_encode, USERINFO_ENCODE_SET};
use hyper::header::parsing::HTTP_VALUE;

const AZURE_VERSION: &'static str = "2017-02-22";
const VERSION: &'static str = "1.0";

pub enum TokenType {
    Master,
    Resource,
}

#[derive(Clone,Copy)]
pub enum ResourceType {
    Databases,
    Collections,
    Documents,
}

pub fn generate_authorization(hmac_key: &str,
                              verb: &str,
                              token_type: TokenType,
                              resource_type: ResourceType,
                              resource_link: &str,
                              dt: &chrono::DateTime<chrono::UTC>)
                              -> String {
    let str_unencoded =
        format!("type={}&ver={}&sig={}",
                match token_type {
                    TokenType::Master => "master",
                    TokenType::Resource => "resource",
                },
                VERSION,
                encode_str_to_sign(&string_to_sign(verb, resource_type, resource_link, dt),
                                   hmac_key));

    utf8_percent_encode(&str_unencoded, url::percent_encoding::DEFAULT_ENCODE_SET)
        .collect::<String>()
}

fn encode_str_to_sign(str_to_sign: &str, hmac_key: &str) -> String {
    let mut v_hmac_key: Vec<u8> = Vec::new();

    v_hmac_key.extend(base64::decode(hmac_key).unwrap());

    let mut hmac = Hmac::new(Sha256::new(), &v_hmac_key);
    hmac.input(str_to_sign.as_bytes());

    base64::encode(hmac.result().code())
}



pub fn string_to_sign(verb: &str,
                      rt: ResourceType,
                      resource_link: &str,
                      dt: &chrono::DateTime<chrono::UTC>)
                      -> String {
    let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));

    // From official docs:
    // StringToSign = Verb.toLowerCase() + "\n" + ResourceType.toLowerCase() + "\n" + ResourceLink + "\n" + Date.toLowerCase() + "\n" + "" + "\n";
    // Notice the empty string at the end so we need to add two carriage returns

    format!("{}\n{}\n{}\n{}\n\n",
            verb.to_lowercase(),
            match rt { 
                ResourceType::Databases => "dbs",
                ResourceType::Collections => "colls",
                ResourceType::Documents => "docs",
            },
            resource_link,
            time.to_lowercase())


}

#[cfg(test)]
mod tests {
    use azure::cosmos::*;

    #[test]
    fn string_to_sign_00() {
        let time = chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00")
            .unwrap();
        let time = time.with_timezone(&chrono::UTC);
        let ret = string_to_sign("GET",
                                 ResourceType::Databases,
                                 "dbs/MyDatabase/colls/MyCollection",
                                 &time);
        assert_eq!(ret,
                   "get
dbs
dbs/MyDatabase/colls/MyCollection
mon, 01 jan 1900 01:00:00 gmt

");
    }

    #[test]
    fn generate_authorization_00() {
        let time = chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00")
            .unwrap();
        let time = time.with_timezone(&chrono::UTC);
        let ret = generate_authorization("8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg==",
                                         "GET",
                                         TokenType::Master,
                                         ResourceType::Databases,
                                         "dbs/MyDatabase/colls/MyCollection",
                                         &time);
        assert_eq!(ret,
                   "type%3dmaster%26ver%3d1.0%26sig%3dQkz%2fr%2b1N2%2bPEnNijxGbGB%2fADvLsLBQmZ7uBBMuIwf4I%3d");
    }

    #[test]
    fn generate_authorization_01() {
        let time = chrono::DateTime::parse_from_rfc3339("2017-04-27T00:51:12.000000000+00:00")
            .unwrap();
        let time = time.with_timezone(&chrono::UTC);
        let ret = generate_authorization("dsZQi3KtZmCv1ljt3VNWNm7sQUF1y5rJfC6kv5JiwvW0EndXdDku/dkKBp8/ufDToSxL",
                                         "GET",
                                         TokenType::Master,
                                         ResourceType::Databases,
                                         "dbs/ToDoList",
                                         &time);
        assert_eq!(ret,
                   "type%3dmaster%26ver%3d1.0%26sig%3dc09PEVJrgp2uQRkr934kFbTqhByc7TVr3O");
    }
}
