use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use chrono::{Duration, FixedOffset, Utc};
use std::collections::HashMap;
use std::error::Error;
use std::ops::Add;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let client = client::with_access_key(&account, &master_key);

    let res = client
        .list_containers()
        .with_client_request_id("ciccio")
        .with_include_metadata()
        .finalize()
        .await?;

    println!("{:?}", res);

    let mut metadata = HashMap::new();
    metadata.insert("prova", "pollo");
    metadata.insert("canotto", "cigno");

    // This is the builder pattern. Notice two things:
    // 1 - The various parameters are clearly defined.
    // 2 - If you forget a mandatory parameter the code won't compile. Type checking at compile
    //   time is waaay better than doing it at runtime!
    client
        .create_container()
        .with_container_name(&container_name)
        .with_public_access(PublicAccess::Container)
        .with_metadata(&metadata)
        .with_timeout(100)
        .finalize()
        .await?;

    // get acl without stored access policy list
    let result = client
        .get_container_acl()
        .with_container_name(&container_name)
        .finalize()
        .await?;
    println!("\nget_acl() == {:?}", result);

    // set stored acess policy list
    let dt_start = Utc::now().with_timezone(&FixedOffset::east(0));
    let dt_end = dt_start.add(Duration::days(7));

    let mut sapl = StoredAccessPolicyList::default();
    sapl.stored_access
        .push(StoredAccessPolicy::new("pollo", dt_start, dt_end, "rwd"));

    let _result = client
        .set_container_acl()
        .with_container_name(&container_name)
        .with_public_access(PublicAccess::Blob)
        .with_stored_access_policy_list(&sapl)
        .finalize()
        .await?;

    // now we get back the acess policy list and compare to the one created
    let result = client
        .get_container_acl()
        .with_container_name(&container_name)
        .finalize()
        .await?;

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

    let res = client
        .get_container_properties()
        .with_container_name(&container_name)
        .finalize()
        .await?;
    println!("\nget_properties() == {:?}", res);

    let res = client
        .acquire_container_lease()
        .with_container_name(&container_name)
        .with_lease_duration(15)
        .finalize()
        .await?;
    println!("\nacquire_lease() == {:?}", res);

    client
        .delete_container()
        .with_container_name(&container_name)
        .with_lease_id(&res.lease_id) // we need to specify the lease or it won't work!
        .finalize()
        .await?;

    Ok(())
}
