use azure_core::{date, prelude::*};
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use std::time::Duration;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let storage_client = StorageClient::new_access_key(&account, &access_key);
    let container_client = storage_client.container_client(container_name);

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
        .metadata(metadata)
        .into_future()
        .await?;

    // get acl without stored access policy list
    let result = container_client.get_acl().into_future().await?;
    println!("\nget_acl() == {:?}", result);

    // set stored access policy list
    let dt_start = OffsetDateTime::now_utc();
    let dt_end = dt_start + date::duration_from_days(7);

    let mut sapl = StoredAccessPolicyList::default();
    sapl.stored_access
        .push(StoredAccessPolicy::new("pollo", dt_start, dt_end, "rwd"));

    let _result = container_client
        .set_acl(PublicAccess::Blob)
        .stored_access_policy_list(sapl.clone())
        .into_future()
        .await?;

    // now we get back the acess policy list and compare to the one created
    let result = container_client.get_acl().into_future().await?;

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

    let res = container_client.get_properties().into_future().await?;
    println!("\nget_properties() == {:?}", res);

    let res = container_client
        .acquire_lease(Duration::from_secs(15))
        .into_future()
        .await?;
    println!("\nacquire_lease() == {:?}", res);

    container_client
        .delete()
        .lease_id(res.lease_id) // we need to specify the lease or it won't work!
        .into_future()
        .await?;

    Ok(())
}
