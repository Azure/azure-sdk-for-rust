// cargo test --test azure_rest_api_specs
// These tests require cloning azure-rest-api-specs.
// git clone git@github.com:Azure/azure-rest-api-specs.git ../../../../azure-rest-api-specs

mod common;
use common::*;

const PATHS: &[&str] = &[
    "../../../../azure-rest-api-specs/specification/vmware/resource-manager/Microsoft.AVS/stable/2020-03-20/vmware.json",
    "../../../../azure-rest-api-specs/specification/batch/data-plane/Microsoft.Batch/stable/2020-03-01.11.0/BatchService.json",
    "../../../../azure-rest-api-specs/specification/security/resource-manager/common/v1/types.json",
    "../../../../azure-rest-api-specs/specification/cosmos-db/resource-manager/Microsoft.DocumentDB/stable/2020-04-01/cosmos-db.json",
    "../../../../azure-rest-api-specs/specification/alertsmanagement/resource-manager/Microsoft.AlertsManagement/preview/2019-05-05-preview/ActionRules.json",
    // https://github.com/Azure/azure-sdk-for-rust/issues/330
    // "../../../../azure-rest-api-specs/specification/apimanagement/resource-manager/Microsoft.ApiManagement/stable/2019-12-01/apimapis.json",
    "../../../../azure-rest-api-specs/specification/communication/data-plane/Chat/stable/2021-03-07/communicationserviceschat.json",
    "../../../../azure-rest-api-specs/specification/eventgrid/data-plane/Microsoft.EventGrid/stable/2018-01-01/EventGrid.json",
    "../../../../azure-rest-api-specs/specification/storage/data-plane/Microsoft.BlobStorage/preview/2021-02-12/blob.json",
    "../../../../azure-rest-api-specs/specification/storage/data-plane/Microsoft.FileStorage/preview/2021-02-12/file.json",
    "../../../../azure-rest-api-specs/specification/storage/data-plane/Microsoft.QueueStorage/preview/2018-03-28/queue.json",
    "../../../../azure-rest-api-specs/specification/storage/data-plane/Microsoft.StorageDataLake/preview/2020-10-02/DataLakeStorage.json",
    "../../../../azure-rest-api-specs/specification/deviceupdate/data-plane/Microsoft.DeviceUpdate/preview/2020-09-01/deviceupdate.json",
    "../../../../azure-rest-api-specs/specification/digitaltwins/data-plane/Microsoft.DigitalTwins/stable/2020-10-31/digitaltwins.json",
];

#[test]
fn can_roundtrip_azure_rest_api_specs() -> Result<()> {
    assert_roundtrip_eq(PATHS)
}
