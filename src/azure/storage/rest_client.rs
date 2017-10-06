use azure::core::range;
use azure::core::errors::AzureError;
use hyper::Method;
use azure::core::lease::{LeaseAction, LeaseDuration, LeaseId, LeaseState, LeaseStatus};
use chrono;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use hyper;
use hyper_tls;
use hyper::header::{ContentEncoding, ContentLanguage, ContentLength, ContentType, Date, Header,
                    Headers, IfModifiedSince, IfUnmodifiedSince};
use base64;
use std::fmt::Display;
use url;

use std::str::FromStr;

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

fn generate_authorization(
    h: &Headers,
    u: &url::Url,
    method: Method,
    hmac_key: &str,
    service_type: ServiceType,
) -> String {
    let str_to_sign = string_to_sign(h, u, method, service_type);

    // println!("\nstr_to_sign == {:?}\n", str_to_sign);
    // println!("str_to_sign == {}", str_to_sign);

    let auth = encode_str_to_sign(&str_to_sign, hmac_key);
    // println!("auth == {:?}", auth);

    format!("SharedKey {}:{}", get_account(u), auth)
}

fn encode_str_to_sign(str_to_sign: &str, hmac_key: &str) -> String {
    let mut v_hmac_key: Vec<u8> = Vec::new();

    v_hmac_key.extend(base64::decode(hmac_key).unwrap());

    let mut hmac = Hmac::new(Sha256::new(), &v_hmac_key);
    hmac.input(str_to_sign.as_bytes());

    // let res = hmac.result();
    // println!("{:?}", res.code());

    base64::encode(hmac.result().code())
}

#[inline]
fn add_if_exists<H: Header + Display>(h: &Headers) -> String {
    let m = match h.get::<H>() {
        Some(ce) => ce.to_string(),
        None => String::default(),
    };

    m + "\n"
}

#[allow(unknown_lints)]
#[allow(needless_pass_by_value)]
fn string_to_sign(h: &Headers, u: &url::Url, method: Method, service_type: ServiceType) -> String {
    let mut str_to_sign = String::new();
    let verb = format!("{:?}", method);
    str_to_sign = str_to_sign + &verb.to_uppercase() + "\n";

    match service_type {
        ServiceType::Table => {}
        _ => {
            str_to_sign = str_to_sign + &add_if_exists::<ContentEncoding>(h);
            str_to_sign = str_to_sign + &add_if_exists::<ContentLanguage>(h);
            // content lenght must only be specified if != 0
            // this is valid from 2015-02-21
            let m = match h.get::<ContentLength>() {
                Some(ce) => if ce.to_be() != 0u64 {
                    ce.to_string()
                } else {
                    String::default()
                },
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
            str_to_sign = str_to_sign + &canonicalized_resource_table(u);
        }
        _ => {
            str_to_sign = str_to_sign + &add_if_exists::<Date>(h);
            str_to_sign = str_to_sign + &add_if_exists::<IfModifiedSince>(h);
            str_to_sign = str_to_sign + &add_if_exists::<IfMatch>(h);
            str_to_sign = str_to_sign + &add_if_exists::<IfNoneMatch>(h);
            str_to_sign = str_to_sign + &add_if_exists::<IfUnmodifiedSince>(h);
            str_to_sign = str_to_sign + &add_if_exists::<Range>(h);
            str_to_sign = str_to_sign + &canonicalize_header(h);
            str_to_sign = str_to_sign + &canonicalized_resource(u);
        }
    }



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

            let first_dot = dm.find('.').unwrap();
            String::from(&dm[0..first_dot])
        }
        _ => panic!("only Domains are supported in canonicalized_resource"),
    }
}

// For table
fn canonicalized_resource_table(u: &url::Url) -> String {
    format!("/{}{}", get_account(u), u.path())
}

fn canonicalized_resource(u: &url::Url) -> String {
    let mut can_res: String = String::new();
    can_res += "/";

    let account = get_account(u);
    can_res += &account;

    let paths = u.path_segments().unwrap();

    {
        let mut path = String::new();
        for p in paths {
            path.push_str("/");
            path.push_str(&*p);
        }

        can_res += &path;
    }
    can_res += "\n";

    // query parameters
    let query_pairs = u.query_pairs(); //.into_owned();
    {
        let mut qps = Vec::new();
        {
            for qp in query_pairs {
                trace!("adding to qps {:?}", qp);

                // add only once
                if !(qps.iter().any(|x: &String| x == &qp.0)) {
                    qps.push(qp.0.into_owned());
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
                    can_res += ","
                }
                can_res += item;
            }

            can_res += "\n";
        }
    };

    can_res[0..can_res.len() - 1].to_owned()
}

fn lexy_sort(vec: &url::form_urlencoded::Parse, query_param: &str) -> Vec<(String)> {
    let mut v_values: Vec<String> = Vec::new();

    for item in vec.filter(|x| x.0 == *query_param) {
        v_values.push(item.1.into_owned())
    }
    v_values.sort();

    v_values
}

#[allow(unknown_lints)]
#[allow(too_many_arguments)]
pub fn perform_request<F>(
    client: &hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
    uri: &str,
    http_method: Method,
    azure_key: &str,
    headers_func: F,
    request_body: Option<&[u8]>,
    service_type: ServiceType,
) -> Result<hyper::client::FutureResponse, AzureError>
where
    F: FnOnce(&mut Headers),
{
    let dt = chrono::Utc::now();
    let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));

    let url = url::Url::parse(uri)?;
    let uri = hyper::Uri::from_str(uri)?;

    // for header in additional_headers.iter() {
    //     println!("{:?}", header.value_string());
    //     h.set();
    // }
    let mut request = hyper::Request::new(http_method.clone(), uri);

    // This will give the caller the ability to add custom headers.
    // The closure is needed to because request.headers_mut().set_raw(...) requires
    // a Cow with 'static lifetime...
    headers_func(request.headers_mut());

    request.headers_mut().set(XMSDate(time));
    request
        .headers_mut()
        .set(XMSVersion(AZURE_VERSION.to_owned()));

    if let Some(body) = request_body {
        let b = Vec::from(body);
        request.headers_mut().set(ContentLength(body.len() as u64));
        request.set_body(b);
    }

    let auth = generate_authorization(
        request.headers(),
        &url,
        http_method,
        azure_key,
        service_type,
    );

    request.headers_mut().set(Authorization(auth));

    // println!("{:?}", request.headers());

    Ok(client.request(request))
}


mod test {
    extern crate chrono;
    extern crate hyper;
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

        assert_eq!(
            super::canonicalize_header(&h),
            "x-ms-date:Fri, 28 Nov 2014 21:00:09 GMT+09:00\nx-ms-version:2015-04-05\n"
        );
    }

    #[test]
    fn str_to_sign_test() {
        use super::*;
        use hyper::header::{qitem, Accept};
        use azure::storage::table::{get_default_json_mime, get_json_mime_nometadata};

        let mut headers: Headers = Headers::new();
        headers.set(Accept(vec![qitem(get_json_mime_nometadata())]));
        headers.set(ContentType(get_default_json_mime()));


        let u: url::Url =
            url::Url::parse("https://mindrust.table.core.windows.net/TABLES").unwrap();
        let method: Method = Method::Post;
        let service_type: ServiceType = ServiceType::Table;

        let dt = chrono::DateTime::parse_from_rfc2822("Wed,  3 May 2017 14:04:56 +0000").unwrap();
        let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));

        headers.set(XMSDate(time));
        headers.set(XMSVersion(AZURE_VERSION.to_owned()));

        let s = string_to_sign(&headers, &u, method, service_type);

        assert_eq!(
            s,
            "POST

application/json; charset=utf-8
Wed, 03 May 2017 14:04:56 GMT
/mindrust/TABLES"
        );
    }

    #[test]
    fn test_canonicalize_resource_10() {
        let url = url::Url::parse("https://mindrust.table.core.windows.net/TABLES").unwrap();
        assert_eq!(super::canonicalized_resource(&url), "/mindrust/TABLES");
    }

    #[test]
    fn test_canonicalize_resource_1() {
        let url = url::Url::parse(
            "http://myaccount.blob.core.windows.\
             net/mycontainer?restype=container&comp=metadata",
        ).unwrap();
        assert_eq!(
            super::canonicalized_resource(&url),
            "/myaccount/mycontainer\ncomp:metadata\nrestype:container"
        );
    }

    #[test]
    fn test_canonicalize_resource_2() {
        let url = url::Url::parse(
            "http://myaccount.blob.core.windows.\
             net/mycontainer?restype=container&comp=list&include=snapshots&\
             include=metadata&include=uncommittedblobs",
        ).unwrap();
        assert_eq!(
            super::canonicalized_resource(&url),
            "/myaccount/mycontainer\ncomp:list\ninclude:metadata,snapshots,\
             uncommittedblobs\nrestype:container"
        );
    }

    #[test]
    fn test_canonicalize_resource_3() {
        let url = url::Url::parse(
            "https://myaccount-secondary.blob.core.windows.\
             net/mycontainer/myblob",
        ).unwrap();
        assert_eq!(
            super::canonicalized_resource(&url),
            "/myaccount-secondary/mycontainer/myblob"
        );
    }

    #[test]
    fn test_encode_str_to_sign_1() {
        let str_to_sign = "53d7e14aee681a00340300032015-01-01T10:00:00.0000000".to_owned();
        let hmac_key = "pXeTVaaaaU9XxH6fPcPlq8Y9D9G3Cdo5Eh2nMSgKj/DWqeSFFXDdmpz5Trv+L2hQNM+nGa704R\
                        f8Z22W9O1jdQ=="
            .to_owned();

        assert_eq!(
            super::encode_str_to_sign(&str_to_sign, &hmac_key),
            "gZzaRaIkvC9jYRY123tq3xXZdsMAcgAbjKQo8y0p0Fs=".to_owned()
        );
    }

    #[test]
    fn test_encode_str_to_sign_2() {
        let str_to_sign = "This is the data to sign".to_owned();
        let hmac_key = "pXeTVaaaaU9XxH6fPcPlq8Y9D9G3Cdo5Eh2nMSgKj/DWqeSFFXDdmpz5Trv+L2hQNM+nGa704R\
                        f8Z22W9O1jdQ=="
            .to_owned();

        assert_eq!(
            super::encode_str_to_sign(&str_to_sign, &hmac_key),
            "YuKoXELO9M9HXeeGaSXBr4Nk+CgPAEQhcwJ6tVtBRCw=".to_owned()
        );
    }

    #[test]
    fn test_encode_str_to_sign_3() {
        let str_to_sign = "This is the data to sign".to_owned();
        let hmac_key = "pXeTVaaaaU9XxH6fPcPlq8Y9D9G3Cdo5Eh2nMSgKj/DWqeSFFXDdmpz5Trv+L2hQNM+nGa704R\
                        f8Z22W9O1jdQ=="
            .to_owned();

        assert_eq!(
            super::encode_str_to_sign(&str_to_sign, &hmac_key),
            "YuKoXELO9M9HXeeGaSXBr4Nk+CgPAEQhcwJ6tVtBRCw=".to_owned()
        );
    }
}
