use azure_identity::AzureCliCredential;
use azure_mgmt_keyvault::{
    models::{
        sku::{Family, Name},
        AccessPolicyEntry, Permissions, Sku, VaultCreateOrUpdateParameters, VaultProperties,
    },
    Client,
};
use std::{env::args, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let resource_group = args().nth(1).expect("please specify resource group name");
    let keyvault_name = args().nth(2).expect("please specify keyvault name");
    let location = args().nth(3).expect("please specify location");
    let object_id = args().nth(4).expect("please specify object id");

    let credential = Arc::new(AzureCliCredential::new());
    let subscription_id = AzureCliCredential::get_subscription()?;
    let tenant_id = AzureCliCredential::get_tenant()?;
    let client = Client::builder(credential).build();

    let sku = Sku::new(Family::A, Name::Standard);
    let mut properties = VaultProperties::new(tenant_id.clone(), sku);
    properties.access_policies = vec![AccessPolicyEntry::new(tenant_id, object_id, Permissions::new())];
    let parameters = VaultCreateOrUpdateParameters::new(location, properties);
    let vault = client
        .vaults_client()
        .create_or_update(resource_group, keyvault_name, parameters, subscription_id)
        .await?;

    println!("{vault:#?}");

    Ok(())
}
