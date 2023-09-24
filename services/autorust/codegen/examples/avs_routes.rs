// cargo run --example avs_routes

use autorust_codegen::{autorust_toml::PackageConfig, *};

struct MainConfig {
    pub input_file: &'static str,
    pub output_folder: &'static str,
}

fn main() -> Result<()> {
    let mut configs = Vec::new();

    configs.push(MainConfig {
        input_file: "../../../azure-rest-api-specs-pr/specification/vmware/resource-manager/Microsoft.AVS/stable/2020-03-20/vmware.json",
        output_folder: "../../../avs/src/fct/mock_api/src/v2020_03_20",
    });

    configs.push(MainConfig {
        input_file: "../../../azure-rest-api-specs-pr/specification/vmware/resource-manager/Microsoft.AVS/stable/2021-06-01/vmware.json",
        output_folder: "../../../avs/src/fct/mock_api/src/v2021_06_01",
    });

    configs.push(MainConfig {
        input_file: "../../../azure-rest-api-specs-pr/specification/vmware/resource-manager/Microsoft.AVS/stable/2021-12-01/vmware.json",
        output_folder: "../../../avs/src/fct/mock_api/src/v2021_12_01",
    });

    configs.push(MainConfig {
        input_file: "../../../azure-rest-api-specs-pr/specification/vmware/resource-manager/Microsoft.AVS/stable/2022-05-01/vmware.json",
        output_folder: "../../../avs/src/fct/mock_api/src/v2022_05_01",
    });

    configs.push(MainConfig {
        input_file: "../../../azure-rest-api-specs-pr/specification/vmware/resource-manager/Microsoft.AVS/stable/2023-03-01/vmware.json",
        output_folder: "../../../avs/src/fct/mock_api/src/v2023_03_01",
    });

    for config in configs {
        run(
            &CrateConfig {
                run_config: &RunConfig {
                    crate_name_prefix: "avs_mgmt_",
                    runs: vec![Runs::Models, Runs::Routes],
                    print_writing_file: true,
                },
                output_folder: config.output_folder.into(),
                input_files: [config.input_file].iter().map(Into::into).collect(),
            },
            &PackageConfig::default(),
        )?;
    }

    Ok(())
}
