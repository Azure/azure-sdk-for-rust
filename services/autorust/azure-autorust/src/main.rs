// cargo run --release -p azure-autorust
// cargo run --release -p azure-autorust -- -p azure_svc_blobstorage
// cargo run --release -p azure-autorust -- -p azure_svc_queuestorage

use autorust_codegen::{
    crates::{list_crate_names, list_dirs},
    gen, get_mgmt_readmes, get_svc_readmes,
    jinja::{CheckAllServicesYml, PublishSdksYml, PublishServicesYml, WorkspaceCargoToml},
    Error, ErrorKind, Result, RunConfig,
};
use clap::Parser;

#[derive(Debug, clap::Parser)]
struct Args {
    /// Generate the publish GitHub workflows
    #[clap(long)]
    publish: bool,

    /// Specify specific package to generate. Multiple accepted.
    #[clap(long = "package", short = 'p')]
    package: Vec<String>,

    /// Run `cargo fmt` after generating the code
    #[clap(long, default_value = "true", action = clap::ArgAction::Set)]
    fmt: bool,
}

impl Args {
    pub fn packages(&self) -> Vec<&str> {
        self.package.iter().map(String::as_str).collect()
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let packages = &args.packages();
    gen_crates(packages)?;
    gen_services_workspace(packages)?;
    if packages.is_empty() {
        gen_workflow_check_all_services()?;
        if args.publish {
            gen_workflow_publish_sdks()?;
            gen_workflow_publish_services()?;
        }
    }
    if args.fmt {
        fmt(packages)?;
    }
    Ok(())
}

fn gen_crates(only_packages: &[&str]) -> Result<()> {
    let svc = get_svc_readmes()?.into_iter().map(|x| ("svc", x));
    let mgmt = get_mgmt_readmes()?.into_iter().map(|x| ("mgmt", x));

    for (i, (crate_type, spec)) in svc.chain(mgmt).enumerate() {
        let output_folder = format!("../{}", crate_type);
        let prefix = format!("azure_{}_", crate_type);

        let run_config = RunConfig::new(&prefix);
        let package_name = gen::package_name(&spec, &run_config);
        if !only_packages.is_empty() && !only_packages.contains(&package_name.as_str()) {
            continue;
        }

        println!("{} ({})", package_name, i);
        gen::gen_crate(&package_name, &spec, &run_config, &output_folder)?;
    }

    Ok(())
}

fn gen_services_workspace(only_packages: &[&str]) -> Result<()> {
    let dirs: Vec<String> = if only_packages.is_empty() {
        list_dirs()?
            .iter()
            .map(|dir| dir.as_str().replace('\\', "/").replace("../", ""))
            .collect()
    } else {
        only_packages
            .iter()
            .map(|p| p.replace("azure_mgmt_", "mgmt/").replace("azure_svc_", "svc/"))
            .collect()
    };

    let toml = WorkspaceCargoToml { dirs };
    toml.create("../Cargo.toml")?;
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

/// Run `cargo fmt` on the services workspace or a subset of packages.
fn fmt(only_packages: &[&str]) -> Result<()> {
    let services_dir = "../";
    let mut args = vec!["fmt"];
    if !only_packages.is_empty() {
        args.push("-p");
        for package in only_packages {
            args.push(package);
        }
    }
    let out = std::process::Command::new("cargo").current_dir(services_dir).args(args).output()?;
    if !out.status.success() {
        println!("cargo fmt failed");
        println!("{}", std::str::from_utf8(&out.stderr)?);
        return Err(Error::new(ErrorKind::Io, "cargo fmt failed"));
    }
    Ok(())
}
