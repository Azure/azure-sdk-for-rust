use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use chrono::{FixedOffset, Utc};
use std::error::Error;
use std::ops::Add;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let options = StorageAccountOptions::default();
    let storage_client =
        StorageAccountClient::new_access_key(account, master_key, options).as_storage_client();
    let blob_service_client = storage_client.as_blob_service_client();
    let container_client = storage_client.as_container_client(container_name);

    let res = blob_service_client
        .list_containers()
        .client_request_id("ciccio")
        .include_metadata(true)
        .execute()
        .await?;

    println!("{:?}", res);

    let mut metadata = Metadata::new();
    metadata.insert("prova".to_owned(), "pollo".to_owned());
    metadata.insert("canotto".to_owned(), "cigno".to_owned());

    // This is the builder pattern. Notice two things:
    // 1 - The various parameters are clearly defined.
    // 2 - If you forget a mandatory parameter the code won't compile. Type checking at compile
    //   time is waaay better than doing it at runtime!
    container_client
        .create()
        .public_access(PublicAccess::Container)
        .metadata(&metadata)
        .timeout(Duration::from_secs(100))
        .execute()
        .await?;

    // get acl without stored access policy list
    let result = container_client.get_acl().execute().await?;
    println!("\nget_acl() == {:?}", result);

    // set stored acess policy list
    let dt_start = Utc::now().with_timezone(&FixedOffset::east(0));
    let dt_end = dt_start.add(chrono::Duration::days(7));

    let mut sapl = StoredAccessPolicyList::default();
    sapl.stored_access
        .push(StoredAccessPolicy::new("pollo", dt_start, dt_end, "rwd"));

    let _result = container_client
        .set_acl(PublicAccess::Blob)
        .stored_access_policy_list(&sapl)
        .execute()
        .await?;

    // now we get back the acess policy list and compare to the one created
    let result = container_client.get_acl().execute().await?;

    println!("\nget_acl() == {:?}", result);

    println!("\n\nsapl() == {:?}", sapl);
    println!(
        "\nresult.stored_access_policy_list  == {:?}",
        result.stored_access_policy_list
    );

    assert!(result.public_access == PublicAccess::Blob);
    // we cannot compare the returned result because Azure will
    // trim the milliseconds
    // assert!(sapl == result.stored_access_policy_list);
    assert!(sapl.stored_access.len() == result.stored_access_policy_list.stored_access.len());
    for (i1, i2) in sapl
        .stored_access
        .iter()
        .zip(result.stored_access_policy_list.stored_access.iter())
    {
        assert!(i1.id == i2.id);
        assert!(i1.permission == i2.permission);
    }

    let res = container_client.get_properties().execute().await?;
    println!("\nget_properties() == {:?}", res);

    let res = container_client
        .acquire_lease(Duration::from_secs(15))
        .execute()
        .await?;
    println!("\nacquire_lease() == {:?}", res);

    container_client
        .delete()
        .lease_id(&res.lease_id) // we need to specify the lease or it won't work!
        .execute()
        .await?;

    Ok(())
}
