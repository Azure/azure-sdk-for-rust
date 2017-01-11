use azure::core::range;
use azure::core::HTTPMethod;
use azure::core::lease::{LeaseId, LeaseStatus, LeaseState, LeaseDuration, LeaseAction};
use chrono;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use hyper;
use hyper::Client;
use hyper::header::{Header, HeaderFormat, Headers,ContentEncoding, ContentLanguage, ContentLength, ContentType, Date,
                    IfModifiedSince, IfUnmodifiedSince, Accept, qitem};
use hyper::mime::{ Attr, Mime, SubLevel, TopLevel, Value };
use serialize::base64::{STANDARD, ToBase64, FromBase64};
use std::fmt::Display;
use std::io::Read;
use url;

pub enum ServiceType {
    Blob,
    // Queue, File,
    Table,
}

const AZURE_VERSION: &'static str = "2016-05-31";

header! { (XMSVersion, "x-ms-version") => [String] }
header! { (XMSDate, "x-ms-date") => [String] }
header! { (Authorization, "Authorization") => [String] }
header! { (ContentMD5, "Content-MD5") => [String] }
header! { (IfMatch, "If-Match") => [String] }
header! { (IfNoneMatch, "If-None-Match") => [String] }
header! { (Range, "Range") => [String] }
header! { (XMSRange, "x-ms-range") => [range::Range] }
header! { (XMSLeaseId, "x-ms-lease-id") => [LeaseId] }
header! { (XMSLeaseStatus, "x-ms-lease-status") => [LeaseStatus] }
header! { (XMSLeaseState, "x-ms-lease-state") => [LeaseState] }
header! { (XMSLeaseAction, "x-ms-lease-action") => [LeaseAction] }
header! { (XMSLeaseDuration, "x-ms-lease-duration") => [LeaseDuration] }
header! { (XMSLeaseDurationSeconds, "x-ms-lease-duration") => [u32] }
header! { (XMSLeaseBreakPeriod, "x-ms-lease-break-period") => [u32] }
header! { (XMSProposedLeaseId, "x-ms-proposed-lease-id") => [LeaseId] }
header! { (ETag, "ETag") => [String] }
header! { (XMSRangeGetContentMD5, "x-ms-range-get-content-md5") => [bool] }
header! { (XMSClientRequestId, "x-ms-client-request-id") => [String] }

fn generate_authorization(h: &Headers,
                              u: &url::Url,
                              method: HTTPMethod,
                              hmac_key: &str,
                              service_type: ServiceType)
                              -> String {
    let str_to_sign = string_to_sign(h, u, method, service_type);

    // println!("\nstr_to_sign == {:?}\n", str_to_sign);
    // println!("str_to_sign == {}", str_to_sign);

    let auth = encode_str_to_sign(&str_to_sign, hmac_key);
    // println!("auth == {:?}", auth);

    format!("SharedKey {}:{}", get_account(u), auth)
}

fn encode_str_to_sign(str_to_sign: &str, hmac_key: &str) -> String {
    let mut v_hmac_key: Vec<u8> = Vec::new();

    v_hmac_key.extend(hmac_key.from_base64().unwrap());

    let mut hmac = Hmac::new(Sha256::new(), &v_hmac_key);
    hmac.input(str_to_sign.as_bytes());

    // let res = hmac.result();
    // println!("{:?}", res.code());

    hmac.result().code().to_base64(STANDARD)
}

#[inline]
fn add_if_exists<H: Header + HeaderFormat + Display>(h: &Headers) -> String {
    let m = match h.get::<H>() {
        Some(ce) => ce.to_string(),
        None => String::default(),
    };

    m + "\n"
}

fn string_to_sign(h: &Headers, u: &url::Url, method: HTTPMethod, service_type: ServiceType) -> String {
    let mut str_to_sign = String::new();
    let verb = format!("{:?}", method);
    str_to_sign = str_to_sign + &verb.to_uppercase() + "\n";

    match service_type {
        ServiceType::Table => {},
        _ => {
            str_to_sign = str_to_sign + &add_if_exists::<ContentEncoding>(h);
            str_to_sign = str_to_sign + &add_if_exists::<ContentLanguage>(h);
            // content lenght must only be specified if != 0
            // this is valid from 2015-02-21
            let m = match h.get::<ContentLength>() {
                Some(ce) => {
                    if ce.to_be() != 0u64 {
                        ce.to_string()
                    } else {
                        String::default()
                    }
                }
                None => String::default(),
            };

            str_to_sign = str_to_sign + &m + "\n";
        }
    }

    str_to_sign = str_to_sign + &add_if_exists::<ContentMD5>(h);
    str_to_sign = str_to_sign + &add_if_exists::<ContentType>(h);

    match service_type {
        ServiceType::Table => {
            str_to_sign = str_to_sign + &add_if_exists::<XMSDate>(h);
        },
        _ => {
            str_to_sign = str_to_sign + &add_if_exists::<Date>(h);
            str_to_sign = str_to_sign + &add_if_exists::<IfModifiedSince>(h);
            str_to_sign = str_to_sign + &add_if_exists::<IfMatch>(h);
            str_to_sign = str_to_sign + &add_if_exists::<IfNoneMatch>(h);
            str_to_sign = str_to_sign + &add_if_exists::<IfUnmodifiedSince>(h);
            str_to_sign = str_to_sign + &add_if_exists::<Range>(h);
            str_to_sign = str_to_sign + &canonicalize_header(h);
        }
    }
    str_to_sign = str_to_sign + &canonicalized_resource(u);

    // expected
    // GET\n /*HTTP Verb*/
    // \n    /*Content-Encoding*/
    // \n    /*Content-Language*/
    // \n    /*Content-Length (include value when zero)*/
    // \n    /*Content-MD5*/
    // \n    /*Content-Type*/
    // \n    /*Date*/
    // \n    /*If-Modified-Since */
    // \n    /*If-Match*/
    // \n    /*If-None-Match*/
    // \n    /*If-Unmodified-Since*/
    // \n    /*Range*/
    // x-ms-date:Sun, 11 Oct 2009 21:49:13 GMT\nx-ms-version:2009-09-19\n
    //                                  /*CanonicalizedHeaders*/
    // /myaccount /mycontainer\ncomp:metadata\nrestype:container\ntimeout:20
    //                                  /*CanonicalizedResource*/
    //
    //

    str_to_sign
}

fn canonicalize_header(h: &Headers) -> String {
    // println!("{:?}", h);

    let mut v_headers = Vec::new();

    for header in h.iter().filter(|h| h.name().starts_with("x-ms")) {
        let s: String = header.name().to_owned().trim().to_lowercase();

        v_headers.push(s);
    }

    // println!("{:?}", v_headers);

    v_headers.sort();

    let mut can = String::new();

    for header_name in v_headers {
        let h = h.iter().find(|x| x.name() == header_name).unwrap();
        // println!("looking for {} => {:?}", header_name, h);

        let s = h.value_string();
        // println!("h.to_string() == {:?}", s);

        can = can + &header_name + ":" + &s + "\n";
    }

    // println!("{:?}", can);

    can
}

#[inline]
fn get_account(u: &url::Url) -> String {
    match u.host().unwrap().clone() {
        url::Host::Domain(dm) => {
            // println!("dom == {:?}", dm);

            let first_dot = dm.find(".").unwrap();
            String::from(&dm[0..first_dot])
        }
        _ => panic!("only Domains are supported in canonicalized_resource"),
    }
}

fn canonicalized_resource(u: &url::Url) -> String {
    let mut can_res: String = String::new();
    can_res = can_res + "/";

    let account = get_account(u);
    can_res = can_res + &account;

    let paths = u.path().unwrap();

    {
        let mut path = String::new();
        for p in paths.iter() {
            path.push_str("/");
            path.push_str(&*p);
        }

        can_res = can_res + &path;
    }
    can_res = can_res + "\n";

    // query parameters
    if let Some(query_pairs) = u.query_pairs() {
        let mut qps = Vec::new();
        {
            for qp in &query_pairs {
                // println!("adding to qps {:?}", qp);

                // add only once
                if !(qps.iter().any(|x: &String| x == &qp.0)) {
                    qps.push(qp.clone().0);
                }
            }
        }

        qps.sort();

        for qparam in qps {
            // find correct parameter
            let ret = lexy_sort(&query_pairs, &qparam);

            // println!("adding to can_res {:?}", ret);

            can_res = can_res + &qparam.to_lowercase() + ":";

            for (i, item) in ret.iter().enumerate() {
                if i > 0 {
                    can_res = can_res + ","
                }
                can_res = can_res + item;
            }

            can_res = can_res + "\n";
        }
    };

    can_res[0..can_res.len() - 1].to_owned()
}

fn lexy_sort(vec: &[(String, String)], query_param: &str) -> Vec<(String)> {
    let mut v_values = Vec::new();

    for item in vec.iter().filter(|x| x.0 == *query_param) {
        v_values.push(item.clone().1)
    }
    v_values.sort();

    v_values
}

pub fn perform_request(uri: &str,
                       method: HTTPMethod,
                       azure_key: &str,
                       headers: &Headers,
                       request_body: Option<(&mut Read, u64)>,
                       service_type: ServiceType)
                       -> Result<hyper::client::response::Response, hyper::error::Error> {
    let client = Client::new();

    let dt = chrono::UTC::now();
    let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));

    // let mut h = Headers::new();

    let u = url::Url::parse(uri).unwrap();

    // for header in additional_headers.iter() {
    //     println!("{:?}", header.value_string());
    //     h.set();
    // }

    let mut h = headers.clone();

    h.set(XMSDate(time));
    h.set(XMSVersion(AZURE_VERSION.to_owned()));
    if let ServiceType::Table = service_type {
        h.set(Accept(vec![
            qitem(Mime(TopLevel::Application, SubLevel::Json,
                    vec![(Attr::Charset, Value::Utf8)])),
        ]));
    }

    if let Some((_, size)) = request_body {
        h.set(ContentLength(size));
    }

    let auth = generate_authorization(&h, &u, method, azure_key, service_type);
    // println!("auth == {:?}", auth);

    h.set(Authorization(auth));

    // println!("{:?}", h);

    if let Some((mut rb, size)) = request_body {
        let b = hyper::client::Body::SizedBody(rb, size);

        match method {
            HTTPMethod::Get => client.get(&u.to_string()).headers(h).send(),
            HTTPMethod::Put => client.put(&u.to_string()).body(b).headers(h).send(),
            HTTPMethod::Post => client.post(&u.to_string()).body(b).headers(h).send(),
            HTTPMethod::Delete => client.delete(&u.to_string()).body(b).headers(h).send(),
        }
    } else {
        // no body
        match method {
            HTTPMethod::Get => client.get(&u.to_string()).headers(h).send(),
            HTTPMethod::Put => client.put(&u.to_string()).headers(h).send(),
            HTTPMethod::Post => client.post(&u.to_string()).headers(h).send(),
            HTTPMethod::Delete => client.delete(&u.to_string()).headers(h).send(),
        }
    }


}


mod test {
    extern crate hyper;
    extern crate chrono;
    extern crate url;

    #[test]
    fn test_canonicalize_header() {
        use super::*;

        let dt = chrono::DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900").unwrap();
        let time = format!("{}", dt.format("%a, %d %h %Y %T GMT%Z"));

        println!("time == {}", time);

        let mut h = hyper::header::Headers::new();

        h.set(XMSDate(time));
        h.set(XMSVersion("2015-04-05".to_owned()));

        assert_eq!(super::canonicalize_header(&h),
                   "x-ms-date:Fri, 28 Nov 2014 21:00:09 GMT+09:00\nx-ms-version:2015-04-05\n");
    }

    #[test]
    fn test_canonicalize_resource_1() {
        let url = url::Url::parse("http://myaccount.blob.core.windows.\
                                   net/mycontainer?restype=container&comp=metadata")
                      .unwrap();
        assert_eq!(super::canonicalized_resource(&url),
                   "/myaccount/mycontainer\ncomp:metadata\nrestype:container");
    }

    #[test]
    fn test_canonicalize_resource_2() {
        let url = url::Url::parse("http://myaccount.blob.core.windows.\
                                   net/mycontainer?restype=container&comp=list&include=snapshots&\
                                   include=metadata&include=uncommittedblobs")
                      .unwrap();
        assert_eq!(super::canonicalized_resource(&url),
                   "/myaccount/mycontainer\ncomp:list\ninclude:metadata,snapshots,\
                    uncommittedblobs\nrestype:container");
    }

    #[test]
    fn test_canonicalize_resource_3() {
        let url = url::Url::parse("https://myaccount-secondary.blob.core.windows.\
                                   net/mycontainer/myblob")
                      .unwrap();
        assert_eq!(super::canonicalized_resource(&url),
                   "/myaccount-secondary/mycontainer/myblob");
    }

    #[test]
    fn test_encode_str_to_sign_1() {
        let str_to_sign = "53d7e14aee681a00340300032015-01-01T10:00:00.0000000".to_owned();
        let hmac_key = "pXeTVaaaaU9XxH6fPcPlq8Y9D9G3Cdo5Eh2nMSgKj/DWqeSFFXDdmpz5Trv+L2hQNM+nGa704R\
                        f8Z22W9O1jdQ=="
                           .to_owned();

        assert_eq!(super::encode_str_to_sign(&str_to_sign, &hmac_key),
                   "gZzaRaIkvC9jYRY123tq3xXZdsMAcgAbjKQo8y0p0Fs=".to_owned());
    }

    #[test]
    fn test_encode_str_to_sign_2() {
        let str_to_sign = "This is the data to sign".to_owned();
        let hmac_key = "pXeTVaaaaU9XxH6fPcPlq8Y9D9G3Cdo5Eh2nMSgKj/DWqeSFFXDdmpz5Trv+L2hQNM+nGa704R\
                        f8Z22W9O1jdQ=="
                           .to_owned();

        assert_eq!(super::encode_str_to_sign(&str_to_sign, &hmac_key),
                   "YuKoXELO9M9HXeeGaSXBr4Nk+CgPAEQhcwJ6tVtBRCw=".to_owned());
    }

    #[test]
    fn test_encode_str_to_sign_3() {
        let str_to_sign = "This is the data to sign".to_owned();
        let hmac_key = "pXeTVaaaaU9XxH6fPcPlq8Y9D9G3Cdo5Eh2nMSgKj/DWqeSFFXDdmpz5Trv+L2hQNM+nGa704R\
                        f8Z22W9O1jdQ=="
                           .to_owned();

        assert_eq!(super::encode_str_to_sign(&str_to_sign, &hmac_key),
                   "YuKoXELO9M9HXeeGaSXBr4Nk+CgPAEQhcwJ6tVtBRCw=".to_owned());
    }
}