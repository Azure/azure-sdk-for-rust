mod list_container_options;
pub use self::list_container_options::{ListContainerOptions, LIST_CONTAINER_OPTIONS_DEFAULT};

use azure::core;
use azure::core::errors::AzureError;
use azure::core::errors::check_status_extract_body;
use azure::core::enumerations;
use azure::core::parsing::{traverse, cast_must, cast_optional};
use azure::core::incompletevector::IncompleteVector;

use azure::core::lease::{LeaseStatus, LeaseState, LeaseDuration};
use azure::storage::client::Client;

use futures::future::*;

use hyper::Method;
use hyper::StatusCode;

use std::str::FromStr;
use chrono::DateTime;
use chrono::Utc;

use std::fmt;

use xml::Element;

use azure::core::errors::TraversingError;
use azure::core::parsing::FromStringOptional;

header! { (XMSBlobPublicAccess, "x-ms-blob-public-access") => [PublicAccess] }

create_enum!(
    PublicAccess,
    (None, "none"),
    (Container, "container"),
    (Blob, "blob")
);


#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub last_modified: DateTime<Utc>,
    pub e_tag: String,
    pub lease_status: LeaseStatus,
    pub lease_state: LeaseState,
    pub lease_duration: Option<LeaseDuration>,
}

impl Container {
    pub fn new(name: &str) -> Container {
        Container {
            name: name.to_owned(),
            last_modified: Utc::now(),
            e_tag: "".to_owned(),
            lease_status: LeaseStatus::Unlocked,
            lease_state: LeaseState::Available,
            lease_duration: None,
        }
    }

    pub fn parse(elem: &Element) -> Result<Container, core::errors::AzureError> {
        let name = try!(cast_must::<String>(elem, &["Name"]));
        let last_modified = try!(cast_must::<DateTime<Utc>>(
            elem,
            &["Properties", "Last-Modified"]
        ));
        let e_tag = try!(cast_must::<String>(elem, &["Properties", "Etag"]));

        let lease_state = try!(cast_must::<LeaseState>(elem, &["Properties", "LeaseState"]));

        let lease_duration = try!(cast_optional::<LeaseDuration>(
            elem,
            &["Properties", "LeaseDuration"]
        ));

        let lease_status = try!(cast_must::<LeaseStatus>(
            elem,
            &["Properties", "LeaseStatus"]
        ));

        Ok(Container {
            name: name,
            last_modified: last_modified,
            e_tag: e_tag,
            lease_status: lease_status,
            lease_state: lease_state,
            lease_duration: lease_duration,
        })
    }

    pub fn delete(&mut self, c: &Client) -> impl Future<Item = (), Error = AzureError> {
        let uri = format!(
            "https://{}.blob.core.windows.net/{}?restype=container",
            c.account(),
            self.name
        );

        let req = c.perform_request(&uri, Method::Delete, |_| {}, None);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::Accepted).and_then(|_| ok(()))
        })
    }

    pub fn create(
        c: &Client,
        container_name: &str,
        pa: PublicAccess,
    ) -> impl Future<Item = (), Error = AzureError> {
        let uri = format!(
            "https://{}.blob.core.windows.net/{}?restype=container",
            c.account(),
            container_name
        );

        let req = c.perform_request(
            &uri,
            Method::Put,
            |ref mut headers| { headers.set(XMSBlobPublicAccess(pa)); },
            None,
        );

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::Created).and_then(|_| ok(()))
        })
    }

    // TODO
    // pub fn get_acl(c : &Client, gao : &GetAclOptions)

    pub fn list(
        c: &Client,
        lco: &ListContainerOptions,
    ) -> impl Future<Item = IncompleteVector<Container>, Error = AzureError> {
        let mut uri = format!(
            "https://{}.blob.core.windows.net?comp=list&maxresults={}",
            c.account(),
            lco.max_results
        );

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

        let req = c.perform_request(&uri, Method::Get, |_| {}, None);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::Ok).and_then(|body| {
                done(incomplete_vector_from_response(&body)).from_err()
            })
        })
    }
}

#[inline]
fn incomplete_vector_from_response(body: &str) -> Result<IncompleteVector<Container>, AzureError> {
    let elem: Element = body.parse()?;

    let mut v = Vec::new();

    // let containers = try!(traverse(&elem, &["Containers", "Container"]));
    // println!("containers == {:?}", containers);

    for container in traverse(&elem, &["Containers", "Container"], true)? {
        v.push(Container::parse(container)?);
    }

    let next_marker = match cast_optional::<String>(&elem, &["NextMarker"])? {
        Some(ref nm) if nm == "" => None,
        Some(nm) => Some(nm),
        None => None,
    };

    Ok(IncompleteVector::new(next_marker, v))
}
