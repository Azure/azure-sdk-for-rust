use azure::core;
use azure::core::errors;
use azure::core::enumerations;
use azure::core::parsing::{traverse, inner_text, traverse_single_must,
                           from_azure_time,
                           traverse_inner_must, traverse_inner_optional};

use azure::storage::{LeaseStatus, LeaseState, LeaseDuration};
use azure::storage::client::Client;

use azure::storage::blob;
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

use azure::core::errors::TraversingError;
use azure::core::parsing::FromStringOptional;

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

pub fn parse(elem: &Element) -> Result<Container, core::errors::AzureError> {
    let name = try!(traverse_single_must(elem, &["Name"]));
    let last_modified = try!(traverse_single_must(elem, &["Properties", "Last-Modified"]));
    let e_tag = try!(traverse_single_must(elem, &["Properties", "Etag"]));

    let lease_state = try!(traverse_inner_must::<LeaseState>(elem,
                                                                   &["Properties", "LeaseState"]));

    let lease_duration = try!(traverse_inner_optional::<LeaseDuration>(elem,
                                                                             &["Properties",
                                                                               "LeaseDuration"]));

    let lease_status = try!(traverse_inner_must::<LeaseStatus>(elem,
                                                                     &["Properties",
                                                                       "LeaseStatus"]));

    let dt_utc = try!(from_azure_time(try!(inner_text(last_modified))));

    Ok(Container {
        name: try!(inner_text(name)).to_owned(),
        last_modified: dt_utc,
        e_tag: try!(inner_text(e_tag)).to_owned(),
        lease_status: lease_status,
        lease_state: lease_state,
        lease_duration: lease_duration,
    })
}

impl Container {
    pub fn delete(&mut self, c: &Client) -> Result<(), core::errors::AzureError> {
        let uri = format!("{}://{}.blob.core.windows.net/{}?restype=container",
                          c.auth_scheme(),
                          c.account(),
                          self.name);

        let mut resp = try!(c.perform_request(&uri,
                                              core::HTTPMethod::Delete,
                                              &core::NO_EXTRA_HEADERS));

        try!(errors::check_status(&mut resp, StatusCode::Accepted));
        Ok(())
    }

    pub fn list_blobs(&self,
                      c: &Client,
                      include_snapshots: bool,
                      include_metadata: bool,
                      include_uncommittedblobs: bool,
                      include_copy: bool)
                      -> Result<Vec<blob::Blob>, core::errors::AzureError> {

        let mut include = String::new();
        if include_snapshots {
            include = include + "snapshots";
        }
        if include_metadata {
            if include.len() > 0 {
                include = include + ",";
            }
            include = include + "metadata";
        }
        if include_uncommittedblobs {
            if include.len() > 0 {
                include = include + ",";
            }
            include = include + "uncommittedblobs";
        }
        if include_copy {
            if include.len() > 0 {
                include = include + ",";
            }
            include = include + "copy";
        }

        let mut uri = format!("{}://{}.blob.core.windows.net/{}?restype=container&comp=list",
                              c.auth_scheme(),
                              c.account(),
                              self.name);

        if include.len() > 0 {
            uri = format!("{}&include={}", uri, include);
        }

        let mut resp = try!(c.perform_request(&uri,
                                              core::HTTPMethod::Get,
                                              &core::NO_EXTRA_HEADERS));

        try!(errors::check_status(&mut resp, StatusCode::Ok));

        let mut resp_s = String::new();
        match resp.read_to_string(&mut resp_s) {
            Ok(_) => (),
            Err(err) => return Err(errors::new_from_ioerror_string(err.to_string())),
        };

        println!("resp_s == {:?}\n\n", resp_s);

        let sp = &resp_s;
        let elem: Element = match sp.parse() {
            Ok(res) => res,
            Err(err) => return Err(errors::new_from_xmlerror_string(err.to_string())),
        };

        let mut v = Vec::new();
        for node_blob in try!(traverse(&elem, &["Blobs", "Blob"], true)) {
            // println!("{:?}", blob);
            v.push(try!(blob::parse(node_blob)));
        }

        Ok(v)
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

    let mut resp = try!(c.perform_request(&uri, core::HTTPMethod::Put, &extra_headers));

    try!(errors::check_status(&mut resp, StatusCode::Created));

    Ok(())
}

pub fn list(c: &Client) -> Result<Vec<Container>, core::errors::AzureError> {
    let uri = format!("{}://{}.blob.core.windows.net?comp=list",
                      c.auth_scheme(),
                      c.account());

    let mut resp = try!(c.perform_request(&uri, core::HTTPMethod::Get, &core::NO_EXTRA_HEADERS));

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
        v.push(try!(parse(container)));
    }

    Ok(v)
}
