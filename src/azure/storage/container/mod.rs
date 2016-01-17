mod list_container_options;
pub use self::list_container_options::{ListContainerOptions, LIST_CONTAINER_OPTIONS_DEFAULT};

use azure::core;
use azure::core::errors;
use azure::core::enumerations;
use azure::core::parsing::{traverse, cast_must, cast_optional};
use azure::core::incompletevector::IncompleteVector;

use azure::storage::{LeaseStatus, LeaseState, LeaseDuration};
use azure::storage::client::Client;

use hyper::header::Headers;
use hyper::status::StatusCode;

use std::str::FromStr;
use chrono::datetime::DateTime;
use chrono::UTC;

use std::io::Read;

use std::fmt;

use xml::Element;

use azure::core::errors::TraversingError;
use azure::core::parsing::FromStringOptional;

header! { (XMSBlobPublicAccess, "x-ms-blob-public-access") => [PublicAccess] }

create_enum!(PublicAccess,
                            (None,          "none"),
                            (Container,     "container"),
                            (Blob,          "blob")
);


#[derive(Debug)]
pub struct Container {
    pub name: String,
    pub last_modified: DateTime<UTC>,
    pub e_tag: String,
    pub lease_status: LeaseStatus,
    pub lease_state: LeaseState,
    pub lease_duration: Option<LeaseDuration>,
}

impl Container {
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
        let last_modified = try!(cast_must::<DateTime<UTC>>(elem,
                                                            &["Properties", "Last-Modified"]));
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

    pub fn delete(&mut self, c: &Client) -> Result<(), core::errors::AzureError> {
        let uri = format!("{}://{}.blob.core.windows.net/{}?restype=container",
                          c.auth_scheme(),
                          c.account(),
                          self.name);

        let mut resp = try!(c.perform_request(&uri,
                                              core::HTTPMethod::Delete,
                                              &Headers::new(),
                                              None));

        try!(errors::check_status(&mut resp, StatusCode::Accepted));
        Ok(())
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

        let mut resp = try!(c.perform_request(&uri, core::HTTPMethod::Put, &headers, None));

        try!(errors::check_status(&mut resp, StatusCode::Created));

        Ok(())
    }

    // TODO
    // pub fn get_acl(c : &Client, gao : &GetAclOptions)

    pub fn list(c: &Client,
                lco: &ListContainerOptions)
                -> Result<IncompleteVector<Container>, core::errors::AzureError> {
        let mut uri = format!("{}://{}.blob.core.windows.net?comp=list&maxresults={}",
                              c.auth_scheme(),
                              c.account(),
                              lco.max_results);

        if !lco.include_metadata {
            uri = format!("{}&include=metadata", uri);
        }

        if let Some(ref prefix) = lco.prefix {
            uri = format!("{}&prefix={}", uri, prefix);
        }

        if let Some(ref nm) = lco.next_marker {
            uri = format!("{}&marker={}", uri, nm);
        }

        if let Some(ref timeout) = lco.timeout {
            uri = format!("{}&timeout={}", uri, timeout);
        }

        let mut resp = try!(c.perform_request(&uri, core::HTTPMethod::Get, &Headers::new(), None));

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
            v.push(try!(Container::parse(container)));
        }

        let next_marker = match try!(cast_optional::<String>(&elem, &["NextMarker"])) {
            Some(ref nm) if nm == "" => None,
            Some(nm) => Some(nm),
            None => None,
        };

        Ok(IncompleteVector::new(next_marker, v))
    }
}
