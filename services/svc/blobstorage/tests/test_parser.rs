use azure_core::{xml::read_xml, Result};
use azure_svc_blobstorage::models::ListContainersSegmentResponse;
use std::fs::read;

#[test]
fn parse_list_container_segment() -> Result<()> {
    let bytes = read("data/list_containers.txt")?;
    let body: ListContainersSegmentResponse = read_xml(&bytes)?;
    assert_eq!(body.next_marker, Some("/bmcrustsdktest/test1".to_owned()));
    let containers = body
        .containers
        .unwrap_or_default()
        .items
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<_>>();
    assert_eq!(containers, ["backup", "test1", "test2", "test3", "test4", "test5"]);
    Ok(())
}
