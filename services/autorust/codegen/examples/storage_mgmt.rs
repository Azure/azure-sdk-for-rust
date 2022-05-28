// cargo run --example storage_mgmt
// https://github.com/Azure/azure-rest-api-specs/tree/master/specification/storage/resource-manager

use autorust_codegen::{autorust_toml::PackageConfig, *};

fn main() -> Result<()> {
    let output_folder = "../azure-sdk-for-rust/services/storage/mgmt/src/v2020_08_01_preview";
    let input_files = [
        "../../../azure-rest-api-specs/specification/storage/resource-manager/Microsoft.Storage/preview/2020-08-01-preview/storage.json",
        "../../../azure-rest-api-specs/specification/storage/resource-manager/Microsoft.Storage/preview/2020-08-01-preview/blob.json",
        "../../../azure-rest-api-specs/specification/storage/resource-manager/Microsoft.Storage/preview/2020-08-01-preview/file.json",
        "../../../azure-rest-api-specs/specification/storage/resource-manager/Microsoft.Storage/preview/2020-08-01-preview/queue.json",
        "../../../azure-rest-api-specs/specification/storage/resource-manager/Microsoft.Storage/preview/2020-08-01-preview/table.json",
    ];
    run(
        &CrateConfig {
            run_config: &RunConfig::new("azure_mgmt_"),
            output_folder: output_folder.into(),
            input_files: input_files.iter().map(Into::into).collect(),
        },
        &PackageConfig::default(),
    )?;

    let output_folder = "../azure-sdk-for-rust/services/storage/mgmt/src/v2019_06_01";
    let input_files = [
        "../../../azure-rest-api-specs/specification/storage/resource-manager/Microsoft.Storage/stable/2019-06-01/storage.json",
        "../../../azure-rest-api-specs/specification/storage/resource-manager/Microsoft.Storage/stable/2019-06-01/blob.json",
        "../../../azure-rest-api-specs/specification/storage/resource-manager/Microsoft.Storage/stable/2019-06-01/file.json",
        "../../../azure-rest-api-specs/specification/storage/resource-manager/Microsoft.Storage/stable/2019-06-01/queue.json",
        "../../../azure-rest-api-specs/specification/storage/resource-manager/Microsoft.Storage/stable/2019-06-01/table.json",
    ];
    run(
        &CrateConfig {
            run_config: &RunConfig::new("azure_mgmt_"),
            output_folder: output_folder.into(),
            input_files: input_files.iter().map(Into::into).collect(),
        },
        &PackageConfig::default(),
    )?;

    Ok(())
}
