// cargo run --example avs_mgmt
// https://github.com/Azure/azure-rest-api-specs/blob/master/specification/vmware/resource-manager

use autorust_codegen::*;

struct MainConfig {
    pub api_version: &'static str,
    pub input_file: &'static str,
    pub output_folder: &'static str,
}

fn main() -> Result<()> {
    let mut configs = Vec::new();

    configs.push(MainConfig {
        api_version: "2020-03-20",
        input_file: "../../../azure-rest-api-specs-pr/specification/vmware/resource-manager/Microsoft.AVS/stable/2020-03-20/vmware.json",
        output_folder: "../../../avs/src/fct/mock_api/src/v2020_03_20",
    });

    configs.push(MainConfig {
        api_version: "2021-06-01",
        input_file: "../../../azure-rest-api-specs-pr/specification/vmware/resource-manager/Microsoft.AVS/stable/2021-06-01/vmware.json",
        output_folder: "../../../avs/src/fct/mock_api/src/v2021_06_01",
    });

    configs.push(MainConfig {
        api_version: "2021-12-01",
        input_file: "../../../azure-rest-api-specs-pr/specification/vmware/resource-manager/Microsoft.AVS/stable/2021-12-01/vmware.json",
        output_folder: "../../../avs/src/fct/mock_api/src/v2021_12_01",
    });

    for config in configs {
        run(Config {
            runs: vec![Runs::Models, Runs::Routes],
            api_version: Some(config.api_version.to_owned()),
            output_folder: config.output_folder.into(),
            input_files: [config.input_file].iter().map(Into::into).collect(),
            ..Config::default()
        })?;
    }

    Ok(())
}
