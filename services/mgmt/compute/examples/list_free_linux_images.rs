/*
Lists the VM images in specified location that do not have a publisher plan that must be
agreed to before use.

Note, for this example, if any image has a publisher plan, the all of the rest
of the images in the offer are skipped.

cargo run --package azure_mgmt_compute --example list_free_linux_images eastus
*/

use azure_identity::AzureCliCredential;
use azure_mgmt_compute::models::os_disk_image::OperatingSystem;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let location = std::env::args().nth(1).expect("please specify location");

    let credential = Arc::new(AzureCliCredential {});
    let subscription_id = AzureCliCredential::get_subscription()?;
    let client = azure_mgmt_compute::Client::builder(credential)
        .build()
        .virtual_machine_images_client();

    let mut publishers = client
        .list_publishers(&location, &subscription_id)
        .into_future()
        .await?
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<_>>();
    publishers.sort();

    for publisher in publishers {
        let mut offers = client
            .list_offers(&location, &publisher, &subscription_id)
            .into_future()
            .await?
            .into_iter()
            .map(|x| x.name)
            .collect::<Vec<_>>();
        offers.sort();

        'offer: for offer in offers {
            let mut skus = client
                .list_skus(&location, &publisher, &offer, &subscription_id)
                .into_future()
                .await?
                .into_iter()
                .map(|x| x.name)
                .collect::<Vec<_>>();
            skus.sort();

            for sku in skus {
                let mut versions = client
                    .list(&location, &publisher, &offer, &sku, &subscription_id)
                    .into_future()
                    .await?
                    .into_iter()
                    .map(|x| x.name)
                    .collect::<Vec<_>>();
                versions.sort();

                for version in versions {
                    let id = format!("{}:{}:{}:{}", publisher, offer, sku, version);
                    eprintln!("checking {}", id);

                    let vm = client
                        .get(&location, &publisher, &offer, &sku, &version, &subscription_id)
                        .into_future()
                        .await?;

                    if let Some(props) = vm.properties {
                        if props.plan.is_some() {
                            continue 'offer;
                        }

                        if let Some(os) = props.os_disk_image {
                            if os.operating_system == OperatingSystem::Linux {
                                println!("{}", id);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
