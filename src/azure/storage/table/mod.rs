use azure::core;
use azure::core::errors;
use azure::storage::client::Client;
use hyper::header::Headers;
use hyper::status::StatusCode;
use std::io::Read;

pub fn list_tables(client: &Client) -> Result<(), core::errors::AzureError> {
    let uri = format!("{}://{}.table.core.windows.net/Tables",
                            client.auth_scheme(),
                            client.account());
    let mut resp = try!(client.perform_request(&uri, core::HTTPMethod::Get, &Headers::new(), None));

    try!(errors::check_status(&mut resp, StatusCode::Ok));
    println!("{:?}", resp.status);

    let mut resp_s = String::new();
    try!(resp.read_to_string(&mut resp_s));

    println!("response == \n\n{:?}\n\n", resp_s);
    println!("he1");
    Ok(())
    // let sp = &resp_s;
    // let elem: Element = try!(sp.parse());

    // let mut v = Vec::new();

    // // let containers = try!(traverse(&elem, &["Containers", "Container"]));
    // // println!("containers == {:?}", containers);

    // for container in try!(traverse(&elem, &["Containers", "Container"], true)) {
    //     v.push(try!(Container::parse(container)));
    // }

    // let next_marker = match try!(cast_optional::<String>(&elem, &["NextMarker"])) {
    //     Some(ref nm) if nm == "" => None,
    //     Some(nm) => Some(nm),
    //     None => None,
    // };

    // let r = IncompleteVector::new(next_marker, v);
    // println("{:?}",r);
}