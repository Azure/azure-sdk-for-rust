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
use hyper_native_tls::NativeTlsClient;

use url;

use url::percent_encoding::utf8_percent_encode;
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

header! { (XMSVersion, "x-ms-version") => [String] }
header! { (XMSDate, "x-ms-date") => [String] }
header! { (Authorization, "Authorization") => [String] }

define_encode_set! {
    pub COMPLETE_ENCODE_SET = [url::percent_encoding::USERINFO_ENCODE_SET] | {
        '+', '-', '&'
    }
}

pub fn list_databases() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = hyper::net::HttpsConnector::new(ssl);
    let client = hyper::Client::with_connector(connector);

    let dt = chrono::UTC::now();
    let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));

    let u = url::Url::parse("https://mindflavor.documents.azure.com/dbs").unwrap();

    let mut h = Headers::new();

    h.set(XMSDate(time));
    h.set(XMSVersion(AZURE_VERSION.to_owned()));

    let key = "toset";
    let verb = "GET";
    let resource_link = "dbs";

    let auth = generate_authorization(key,
                                      verb,
                                      TokenType::Master,
                                      ResourceType::Databases,
                                      resource_link,
                                      &dt);
    println!("auth == {}", auth);
    h.set(Authorization(auth));

    let mut builder = client.get(&u.to_string());

    let res = builder.headers(h).send().unwrap();

    println!("res == {:?}", res);
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

    utf8_percent_encode(&str_unencoded, COMPLETE_ENCODE_SET).collect::<String>()
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
                   "type%3Dmaster%26ver%3D1.0%26sig%3DQkz%2Fr%2B1N2%2BPEnNijxGbGB%2FADvLsLBQmZ7uBBMuIwf4I%3D");
    }

    //    #[test]
    //    fn generate_authorization_01() {
    //        let time = chrono::DateTime::parse_from_rfc3339("2017-04-27T00:51:12.000000000+00:00")
    //            .unwrap();
    //        let time = time.with_timezone(&chrono::UTC);
    //        let ret = generate_authorization("dsZQi3KtZmCv1ljt3VNWNm7sQUF1y5rJfC6kv5JiwvW0EndXdDku/dkKBp8/ufDToSxL",
    //                                         "GET",
    //                                         TokenType::Master,
    //                                         ResourceType::Databases,
    //                                         "dbs/ToDoList",
    //                                         &time);
    //        assert_eq!(ret,
    //                   "type%3dmaster%26ver%3d1.0%26sig%3dc09PEVJrgp2uQRkr934kFbTqhByc7TVr3O");
    //    }
}
