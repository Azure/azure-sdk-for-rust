// cargo run --release -p azure-autorust

use autorust_codegen::{
    crates::{list_crate_names, list_dirs},
    gen::gen_crate,
    get_mgmt_readmes, get_svc_readmes,
    jinja::{CargoToml, CheckAllServicesYml, PublishSdksYml, PublishServicesYml},
    Result, RunConfig,
};
use clap::Parser;

#[derive(Debug, clap::Parser)]
struct Args {
    /// Generate the publish GitHub workflows
    #[clap(long)]
    publish: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    gen_mgmt()?;
    gen_svc()?;
    gen_services_workspace()?;
    gen_workflow_check_all_services()?;
    if args.publish {
        gen_workflow_publish_sdks()?;
        gen_workflow_publish_services()?;
    }
    Ok(())
}

fn gen_mgmt() -> Result<()> {
    const OUTPUT_FOLDER: &str = "../mgmt";
    const ONLY_SERVICES: &[&str] = &[];
    let run_config = &mut RunConfig::new("azure_mgmt_");
    for (i, spec) in get_mgmt_readmes()?.iter().enumerate() {
        if !ONLY_SERVICES.is_empty() {
            if ONLY_SERVICES.contains(&spec.spec()) {
                println!("{} {}", i + 1, spec.spec());
                gen_crate(spec, run_config, OUTPUT_FOLDER)?;
            }
        } else {
            println!("{} {}", i + 1, spec.spec());
            gen_crate(spec, run_config, OUTPUT_FOLDER)?;
        }
    }
    Ok(())
}

fn gen_svc() -> Result<()> {
    const OUTPUT_FOLDER: &str = "../svc";
    const ONLY_SERVICES: &[&str] = &[];
    let run_config = &mut RunConfig::new("azure_svc_");
    for (i, spec) in get_svc_readmes()?.iter().enumerate() {
        if !ONLY_SERVICES.is_empty() {
            if ONLY_SERVICES.contains(&spec.spec()) {
                println!("{} {}", i + 1, spec.spec());
                gen_crate(spec, run_config, OUTPUT_FOLDER)?;
            }
        } else {
            println!("{} {}", i + 1, spec.spec());
            gen_crate(spec, run_config, OUTPUT_FOLDER)?;
        }
    }
    Ok(())
}

fn gen_services_workspace() -> Result<()> {
    let dirs = list_dirs()?;
    let dirs: Vec<String> = dirs.iter().map(|dir| dir.as_str().replace('\\', "/").replace("../", "")).collect();

    let yml = CargoToml { dirs };
    yml.create("../Cargo.toml")?;
    Ok(())
}

fn gen_workflow_check_all_services() -> Result<()> {
    let packages = list_crate_names()?;
    let packages = &packages.iter().map(String::as_str).collect();

    let yml = CheckAllServicesYml { packages };
    yml.create("../../.github/workflows/check-all-services.yml")?;
    Ok(())
}

fn gen_workflow_publish_sdks() -> Result<()> {
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

fn gen_workflow_publish_services() -> Result<()> {
    let packages = list_crate_names()?;
    let packages = &packages.iter().map(String::as_str).collect();
    let yml = PublishServicesYml { packages };
    yml.create("../../.github/workflows/publish-services.yml")?;
    Ok(())
}
