use azure::cosmos::authorization_token::{TokenType, AuthorizationToken};
use azure::core::HTTPMethod;

use url;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;

use base64;
use hyper;
use hyper::header::Headers;
use hyper_native_tls;

use chrono;

use url::percent_encoding::utf8_percent_encode;

use std::io::Read;

const AZURE_VERSION: &'static str = "2017-02-22";
const VERSION: &'static str = "1.0";
const TIME_FORMAT: &'static str = "%a, %d %h %Y %T GMT";

header! { (XMSVersion, "x-ms-version") => [String] }
header! { (XMSDate, "x-ms-date") => [String] }
header! { (Authorization, "Authorization") => [String] }

define_encode_set! {
    pub COMPLETE_ENCODE_SET = [url::percent_encoding::USERINFO_ENCODE_SET] | {
        '+', '-', '&'
    }
}


#[derive(Clone,Copy)]
pub enum ResourceType {
    Databases,
    Collections,
    Documents,
}

pub struct Client {
    hyper_client: hyper::client::Client,
}


impl Client {
    pub fn new() -> Result<Client, hyper_native_tls::native_tls::Error> {
        let ssl = hyper_native_tls::NativeTlsClient::new()?;
        let connector = hyper::net::HttpsConnector::new(ssl);
        let client = hyper::Client::with_connector(connector);

        Ok(Client { hyper_client: client })
    }

    fn perform_request(&self,
                       url: &url::Url,
                       http_method: HTTPMethod,
                       resource_type: ResourceType,
                       authorization_token: &AuthorizationToken,
                       mut headers: Headers) {
        let dt = chrono::UTC::now();
        let time = format!("{}", dt.format(TIME_FORMAT));


        // to do: calculate resource link
        let resource_link = "";

        let auth = generate_authorization(&authorization_token,
                                          http_method,
                                          resource_type,
                                          resource_link,
                                          &time);
        println!("perform_request::auth == {:?}", auth);

        headers.set(XMSDate(time));
        headers.set(XMSVersion(AZURE_VERSION.to_owned()));
        headers.set(Authorization(auth));

        println!("perform_request::headers == {:?}", headers);

        let builder = match http_method {
            HTTPMethod::Get => self.hyper_client.get(&url.to_string()),
            HTTPMethod::Put => self.hyper_client.put(&url.to_string()),
            HTTPMethod::Post => self.hyper_client.post(&url.to_string()),
            HTTPMethod::Delete => self.hyper_client.delete(&url.to_string()),
        };


        let mut res = builder.headers(headers).send().unwrap();

        println!("perform_request::res == {:?}", res);

        let mut res_body = String::new();

        res.read_to_string(&mut res_body).unwrap();

        println!("perform_request::res_body == {}", res_body);
    }


    pub fn list_databases(&self, authorization_token: &AuthorizationToken, account: &str) {
        let url = url::Url::parse(&format!("https://{}.documents.azure.com/dbs", account)).unwrap();
        let h = Headers::new();

        // nothing to add here, list databases only needs standard headers
        // which will be provied by perform_request

        self.perform_request(&url,
                             HTTPMethod::Get,
                             ResourceType::Databases,
                             authorization_token,
                             h);
    }
}

pub fn generate_authorization(authorization_token: &AuthorizationToken,
                              http_method: HTTPMethod,
                              resource_type: ResourceType,
                              resource_link: &str,
                              time: &str)
                              -> String {
    let string_to_sign = string_to_sign(http_method, resource_type, resource_link, time);
    println!("generate_authorization::string_to_sign == {:?}",
             string_to_sign);

    let str_unencoded = format!("type={}&ver={}&sig={}",
                                match authorization_token.token_type() {
                                    TokenType::Master => "master",
                                    TokenType::Resource => "resource",
                                },
                                VERSION,
                                encode_str_to_sign(&string_to_sign, authorization_token));

    println!("generate_authorization::str_unencoded == {:?}",
             str_unencoded);

    utf8_percent_encode(&str_unencoded, COMPLETE_ENCODE_SET).collect::<String>()
}

fn encode_str_to_sign(str_to_sign: &str, authorization_token: &AuthorizationToken) -> String {
    let mut hmac = Hmac::new(Sha256::new(), authorization_token.binary_form());
    hmac.input(str_to_sign.as_bytes());

    base64::encode(hmac.result().code())
}



pub fn string_to_sign(http_method: HTTPMethod,
                      rt: ResourceType,
                      resource_link: &str,
                      time: &str)
                      -> String {
    // From official docs:
    // StringToSign = Verb.toLowerCase() + "\n" + ResourceType.toLowerCase() + "\n" + ResourceLink + "\n" + Date.toLowerCase() + "\n" + "" + "\n";
    // Notice the empty string at the end so we need to add two carriage returns

    format!("{}\n{}\n{}\n{}\n\n",
            match http_method {
                HTTPMethod::Get => "get",
                HTTPMethod::Put => "put",
                HTTPMethod::Post => "post",
                HTTPMethod::Delete => "delete",
            },
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
    use azure::cosmos::client::*;
    use azure::cosmos::authorization_token;

    #[test]
    fn string_to_sign_00() {
        let time = chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00")
            .unwrap();
        let time = time.with_timezone(&chrono::UTC);
        let time = format!("{}", time.format(TIME_FORMAT));

        let ret = string_to_sign(HTTPMethod::Get,
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
        let time = format!("{}", time.format(TIME_FORMAT));

        let authorization_token =
            authorization_token::AuthorizationToken::new(authorization_token::TokenType::Master,
                                                         "8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg==".to_owned()).unwrap();



        let ret = generate_authorization(&authorization_token,
                                         HTTPMethod::Get,
                                         ResourceType::Databases,
                                         "dbs/MyDatabase/colls/MyCollection",
                                         &time);
        assert_eq!(ret,
                   "type%3Dmaster%26ver%3D1.0%26sig%3DQkz%2Fr%2B1N2%2BPEnNijxGbGB%2FADvLsLBQmZ7uBBMuIwf4I%3D");
    }

    #[test]
    fn generate_authorization_01() {
        let time = chrono::DateTime::parse_from_rfc3339("2017-04-27T00:51:12.000000000+00:00")
            .unwrap();
        let time = time.with_timezone(&chrono::UTC);
        let time = format!("{}", time.format(TIME_FORMAT));

        let authorization_token =
            authorization_token::AuthorizationToken::new(authorization_token::TokenType::Master,
                                                         "dsZQi3KtZmCv1ljt3VNWNm7sQUF1y5rJfC6kv5JiwvW0EndXdDku/dkKBp8/ufDToSxL".to_owned()).unwrap();

        let ret = generate_authorization(&authorization_token,
                                         HTTPMethod::Get,
                                         ResourceType::Databases,
                                         "dbs/ToDoList",
                                         &time);

        // This is the result shown in the MSDN page. Clearly is wrong :)
        // below is the right one.
        //assert_eq!(ret,
        //           "type%3dmaster%26ver%3d1.0%26sig%3dc09PEVJrgp2uQRkr934kFbTqhByc7TVr3O");

        assert_eq!(ret,
                   "type%3Dmaster%26ver%3D1.0%26sig%3DKvBM8vONofkv3yKm%2F8zD9MEGlbu6jjHDJBp4E9c2ZZI%3D");
    }
}
