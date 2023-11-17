use azure_core::Result;
use azure_mgmt_recoveryservicesbackup::models::{WorkloadProtectableItemResourceList, WorkloadProtectableItemUnion};
use std::fs::read;

#[test]
/// This test is added to verify that the [WorkloadProtectableItemUnion] contains
/// discriminator values that indirectly are discriminator children of this discriminator
fn test_deserialization_of_nested_all_of_property() -> Result<()> {
    let bytes = read("tests/backup_protectable_item_list.json")?;

    let list: WorkloadProtectableItemResourceList = serde_json::from_slice(&bytes)?;

    let item = list.value.first().expect("There should be at least one item in the list");
    let properties = item.properties.as_ref().expect("The properties should be present");
    assert!(matches!(
        properties,
        &WorkloadProtectableItemUnion::MicrosoftClassicComputeVirtualMachines(_)
    ));

    Ok(())
}
