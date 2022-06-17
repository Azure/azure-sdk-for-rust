// cargo run --example gen_workspace

use autorust_codegen::{
    crates::{list_crate_names, list_dirs},
    jinja::{CargoToml, CheckAllServicesYml, PublishSdksYml, PublishServicesYml},
    Result,
};

fn main() -> Result<()> {
    let dirs = list_dirs()?;
    let dirs: Vec<String> = dirs.iter().map(|dir| dir.as_str().replace("\\", "/").replace("../", "")).collect();

    let yml = CargoToml { dirs };
    yml.create("../Cargo.toml")?;

    let packages = list_crate_names()?;
    let packages = &packages.iter().map(String::as_str).collect();

    let yml = CheckAllServicesYml { packages };
    yml.create("../../.github/workflows/check-all-services.yml")?;

    if std::env::args().any(|arg| arg == "publish") {
        publish_sdks()?;
        publish_services()?;
    }
    Ok(())
}

fn publish_sdks() -> Result<()> {
    let packages = &vec![
        "azure_core",
        "azure_data_cosmos",
        "azure_data_tables",
        "azure_identity",
        "azure_iot_hub",
        "azure_messaging_eventgrid",
        "azure_messaging_servicebus",
        "azure_security_keyvault",
        "azure_storage",
        "azure_storage_blobs",
        "azure_storage_datalake",
        "azure_storage_queues",
    ];
    let yml = PublishSdksYml { packages };
    yml.create("../../.github/workflows/publish-sdks.yml")?;
    Ok(())
}

fn publish_services() -> Result<()> {
    let packages = list_crate_names()?;
    let packages = &packages.iter().map(String::as_str).collect();
    let yml = PublishServicesYml { packages };
    yml.create("../../.github/workflows/publish-services.yml")?;
    Ok(())
}
