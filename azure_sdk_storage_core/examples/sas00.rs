use azure_sdk_storage_core::prelude::*;
use chrono::*;
use url::Url;

pub fn main() {
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let start = Utc::now() - Duration::days(1);
    let end = Utc::now() + Duration::days(1);

    let path =
        Url::parse("https://azureskdforrust.blob.core.windows.net/test/ERRORLOG.1.cut").unwrap();

    let ip_range = IPRange {
        start: std::net::IpAddr::V4(<std::net::Ipv4Addr>::new(0, 0, 0, 0)),
        end: std::net::IpAddr::V4(<std::net::Ipv4Addr>::new(255, 255, 255, 255)),
    };

    let sas = BlobSASBuilder::new(&path)
        .with_key(&master_key)
        .with_validity_start(&start)
        .with_validity_end(&end)
        .with_ip_range(&ip_range)
        .with_content_type("text/plain")
        .allow_read()
        .finalize();

    println!("SAS == {}", sas);

    let path = Url::parse(
        "https://azureskdforrust.blob.core.windows.net/test?restype=container&comp=list",
    )
    .unwrap();

    let sas = ContainerSASBuilder::new(&path)
        .with_key(&master_key)
        .with_validity_start(&start)
        .with_validity_end(&end)
        .with_ip_range(&ip_range)
        .allow_read()
        .allow_list()
        .allow_write()
        .allow_add()
        .allow_create()
        .allow_delete()
        .finalize();

    println!("SAS == {}", sas);
}
