// cargo run --example resources_mgmt
// https://github.com/Azure/azure-rest-api-specs/tree/master/specification/resources/resource-manager

use autorust_codegen::*;

fn main() -> Result<()> {
    let output_folder = "../azure-sdk-for-rust/services/resources/mgmt/src/v2020_06_01";
    let input_files =
        ["../../../azure-rest-api-specs/specification/resources/resource-manager/Microsoft.Resources/stable/2020-06-01/resources.json"];
    run(&CrateConfig {
        run_config: &RunConfig::new("azure_mgmt_"),
        output_folder: output_folder.into(),
        input_files: input_files.iter().map(Into::into).collect(),
    })?;

    Ok(())
}
