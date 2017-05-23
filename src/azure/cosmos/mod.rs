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

    utf8_percent_encode(&str_unencoded, HTTP_VALUE).collect::<String>()
}

fn encode_str_to_sign(str_to_sign: &str, hmac_key: &str) -> String {
    let mut v_hmac_key: Vec<u8> = Vec::new();

    v_hmac_key.extend(base64::decode(hmac_key).unwrap());

    let mut hmac = Hmac::new(Sha256::new(), &v_hmac_key);
    hmac.input(str_to_sign.as_bytes());

    base64::encode(hmac.result().code())
}



fn string_to_sign(verb: &str,
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
