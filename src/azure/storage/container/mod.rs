use azure::core;
use azure::core::errors;
use azure::core::enumerations;
use azure::core::parsing::{traverse, inner_text, traverse_single, traverse_single_optional,
                           traverse_single_must};
use azure::storage::client::Client;
// use azure::core::parsing;
// use hyper::error;
use hyper::status::StatusCode;
use std::str::FromStr;
use chrono;

use std::io::Read;
// use hyper::Client;
// use hyper::header::Headers;
// use chrono;
// use url;

use std::fmt;

use xml::Element;

create_enum!(LeaseStatus,
                            (Locked,        "locked"),
                            (Unlocked,      "unlocked")
);

create_enum!(LeaseState,
                            (Available,     "available"),
                            (Leased,        "leased"),
                            (Expired,       "expired"),
                            (Breaking,      "breaking"),
                            (Broken,        "broken")
);

create_enum!(LeaseDuration,
                            (Infinite,      "infinite"),
                            (Fixed,         "fixed")
);

create_enum!(PublicAccess,
                            (None,          "none"),
                            (Container,     "container"),
                            (Blob,          "blob")
);

header! { (XMSBlobPublicAccess, "x-ms-blob-public-access") => [PublicAccess] }

#[derive(Debug)]
pub struct Container {
    pub name: String,
    pub last_modified: chrono::datetime::DateTime<chrono::UTC>,
    pub e_tag: String,
    pub lease_status: LeaseStatus,
    pub lease_state: LeaseState,
    pub lease_duration: Option<LeaseDuration>,
}

pub fn new(name: &str) -> Container {
    Container {
        name: name.to_owned(),
        last_modified: chrono::UTC::now(),
        e_tag: "".to_owned(),
        lease_status: LeaseStatus::Unlocked,
        lease_state: LeaseState::Available,
        lease_duration: None,
    }
}

impl Container {
    pub fn delete(&mut self, c: &Client) -> Result<(), core::errors::AzureError> {
        let uri = format!("{}://{}.blob.core.windows.net/{}?restype=container",
                          c.auth_scheme(),
                          c.account(),
                          self.name);

        let mut resp = try!(core::perform_request(&uri,
                                                  core::HTTPMethod::Delete,
                                                  c.key(),
                                                  &core::NO_EXTRA_HEADERS));

        try!(errors::check_status(&mut resp, StatusCode::Accepted));
        Ok(())
    }
}

pub fn create(c: &Client,
              container_name: &str,
              pa: PublicAccess)
              -> Result<(), core::errors::AzureError> {
    let uri = format!("{}://{}.blob.core.windows.net/{}?restype=container",
                      c.auth_scheme(),
                      c.account(),
                      container_name);

    // println!("uri == {:?}", uri);
    let mut extra_headers = Vec::new();

    if pa != PublicAccess::None {
        extra_headers.push(XMSBlobPublicAccess(pa));
    }

    let mut resp = try!(core::perform_request(&uri,
                                              core::HTTPMethod::Put,
                                              c.key(),
                                              &extra_headers));

    try!(errors::check_status(&mut resp, StatusCode::Created));

    Ok(())
}

pub fn list(c: &Client) -> Result<Vec<Container>, core::errors::AzureError> {
    let uri = format!("{}://{}.blob.core.windows.net?comp=list",
                      c.auth_scheme(),
                      c.account());

    let mut resp = try!(core::perform_request(&uri,
                                              core::HTTPMethod::Get,
                                              c.key(),
                                              &core::NO_EXTRA_HEADERS));

    try!(errors::check_status(&mut resp, StatusCode::Ok));

    // println!("{:?}", resp.status);

    let mut resp_s = String::new();
    match resp.read_to_string(&mut resp_s) {
        Ok(_) => (),
        Err(err) => return Err(errors::new_from_ioerror_string(err.to_string())),
    };

    // println!("response == \n\n{:?}\n\n", resp_s);

    let sp = &resp_s;
    let elem: Element = match sp.parse() {
        Ok(res) => res,
        Err(err) => return Err(errors::new_from_xmlerror_string(err.to_string())),
    };

    let mut v = Vec::new();

    // let containers = try!(traverse(&elem, &["Containers", "Container"]));
    // println!("containers == {:?}", containers);

    for container in try!(traverse(&elem, &["Containers", "Container"], true)) {
        // println!("container == {:?}", container);

        let name = try!(traverse_single(container, &["Name"]));
        let last_modified = try!(traverse_single(container, &["Properties", "Last-Modified"]));
        let e_tag = try!(traverse_single(container, &["Properties", "Etag"]));

        let lease_state = try!(traverse_single_must::<LeaseState>(container,
                                                                  &["Properties", "LeaseState"]));

        let lease_duration = try!(traverse_single_optional::<LeaseDuration>(container,
                                                                            &["Properties",
                                                                              "LeaseDuration"]));

        let lease_status = try!(traverse_single_must::<LeaseStatus>(container,
                                                                    &["Properties",
                                                                      "LeaseStatus"]));

        let time_str = try!(inner_text(last_modified)).to_owned();

        let dt = try!(chrono::DateTime::parse_from_rfc2822(&time_str));
        let dt_utc: chrono::DateTime<chrono::UTC> = dt.with_timezone(&chrono::UTC);

        v.push(Container {
            name: try!(inner_text(name)).to_owned(),
            last_modified: dt_utc,
            e_tag: try!(inner_text(e_tag)).to_owned(),
            lease_status: lease_status,
            lease_state: lease_state,
            lease_duration: lease_duration,
        });
    }

    Ok(v)
}
