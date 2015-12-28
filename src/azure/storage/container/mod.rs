use azure::core;
use azure::core::errors;
use azure::core::enumerations;
use azure::core::parsing::{traverse, cast_must, cast_optional, from_azure_time};

use azure::storage::{LeaseStatus, LeaseState, LeaseDuration};
use azure::storage::client::Client;

use azure::core::{XMSRange, XMSLeaseId, ContentMD5};

use mime::Mime;

use azure::storage::blob;
// use azure::core::parsing;
// use hyper::error;
use hyper::header::{Headers, ContentType, ContentLength, LastModified};
use hyper::status::StatusCode;
use hyper::client::Response;

use std::str::FromStr;
use chrono::datetime::DateTime;
use chrono::UTC;

use std::io::Read;

use azure::core::lease_id::LeaseId;
use azure::core::range::Range;

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
    pub last_modified: DateTime<UTC>,
    pub e_tag: String,
    pub lease_status: LeaseStatus,
    pub lease_state: LeaseState,
    pub lease_duration: Option<LeaseDuration>,
}

pub fn new(name: &str) -> Container {
    Container {
        name: name.to_owned(),
        last_modified: UTC::now(),
        e_tag: "".to_owned(),
        lease_status: LeaseStatus::Unlocked,
        lease_state: LeaseState::Available,
        lease_duration: None,
    }
}

pub fn parse(elem: &Element) -> Result<Container, core::errors::AzureError> {
    let name = try!(cast_must::<String>(elem, &["Name"]));
    let last_modified = try!(cast_must::<DateTime<UTC>>(elem, &["Properties", "Last-Modified"]));
    let e_tag = try!(cast_must::<String>(elem, &["Properties", "Etag"]));

    let lease_state = try!(cast_must::<LeaseState>(elem, &["Properties", "LeaseState"]));

    let lease_duration = try!(cast_optional::<LeaseDuration>(elem,
                                                             &["Properties", "LeaseDuration"]));

    let lease_status = try!(cast_must::<LeaseStatus>(elem, &["Properties", "LeaseStatus"]));

    Ok(Container {
        name: name,
        last_modified: last_modified,
        e_tag: e_tag,
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

        let mut resp = try!(c.perform_request(&uri, core::HTTPMethod::Delete, &Headers::new()));

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

        let mut resp = try!(c.perform_request(&uri, core::HTTPMethod::Get, &Headers::new()));

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

    pub fn get_blob_content(&self,
                            c: &Client,
                            blob_name: &str,
                            snapshot: Option<&DateTime<UTC>>,
                            range: Option<&Range>,
                            lease_id: Option<&LeaseId>,
                            get_md5: bool)
                            -> Result<(), core::errors::AzureError> {
        let uri = format!("{}://{}.blob.core.windows.net/{}/{}",
                          c.auth_scheme(),
                          c.account(),
                          self.name,
                          blob_name);


        println!("uri == {:?}", uri);

        let mut headers = Headers::new();

        if let Some(r) = range {
            headers.set(XMSRange(r.clone()));
        }

        if let Some(l) = lease_id {
            headers.set(XMSLeaseId(l.clone()));
        }

        let mut resp = try!(c.perform_request(&uri, core::HTTPMethod::Get, &headers));

        // if we have requested a range the response code should be 207 (partial content)
        // otherwise 200 (ok).
        if let Some(_) = range {
            try!(errors::check_status(&mut resp, StatusCode::PartialContent));
        }   else {
            try!(errors::check_status(&mut resp, StatusCode::Ok));
        }

        let content_type = match resp.headers.get::<ContentType>() {
            Some(ct) => (ct as &Mime).clone(),
            None => try!("application/octet-stream".parse::<Mime>()),
        };
        println!("content_type == {:?}", content_type);

        let content_length = match resp.headers.get::<ContentLength>() {
            Some(cl) => cl as &u64,
            None => return Err(errors::AzureError::HeaderNotFound("Content-Length".to_owned())),
        };
        println!("content_length == {:?}", content_length);

        let last_modified = match resp.headers.get::<LastModified>() {
            Some(lm) => try!(from_azure_time(&lm.to_string())),
            None => return Err(errors::AzureError::HeaderNotFound("Last-Modified".to_owned())),
        };
        println!("last_modified == {:?}", last_modified);

        let last_modified = match resp.headers.get::<LastModified>() {
            Some(lm) => try!(from_azure_time(&lm.to_string())),
            None => return Err(errors::AzureError::HeaderNotFound("Last-Modified".to_owned())),
        };
        println!("last_modified == {:?}", last_modified);

        let content_md5 = resp.headers.get::<ContentMD5>();

        // TODO: get the remaining headers (https://msdn.microsoft.com/en-us/library/azure/dd179440.aspx)

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

    let mut headers = Headers::new();

    if pa != PublicAccess::None {
        headers.set(XMSBlobPublicAccess(pa));
    }

    let mut resp = try!(c.perform_request(&uri, core::HTTPMethod::Put, &headers));

    try!(errors::check_status(&mut resp, StatusCode::Created));

    Ok(())
}

pub fn list(c: &Client) -> Result<Vec<Container>, core::errors::AzureError> {
    let uri = format!("{}://{}.blob.core.windows.net?comp=list",
                      c.auth_scheme(),
                      c.account());

    let mut resp = try!(c.perform_request(&uri, core::HTTPMethod::Get, &Headers::new()));

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
