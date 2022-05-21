// cargo run --example gen_workspace

use autorust_codegen::{
    crates::{list_crate_names, list_dirs},
    jinja::{CargoToml, CheckAllServicesYml},
};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let dirs = list_dirs()?;
    let dirs: Vec<String> = dirs.iter().map(|dir| dir.as_str().replace("\\", "/").replace("../", "")).collect();

    let yml = CargoToml { dirs };
    yml.create("../Cargo.toml")?;

    let packages = list_crate_names()?;
    let packages = &packages.iter().map(String::as_str).collect();

    let yml = CheckAllServicesYml { packages };
    yml.create("../../.github/workflows/check-all-services.yml")?;

    // let yml = PublishServicesYml { packages };
    // yml.create("../../.github/workflows/publish-services.yml")?;
    Ok(())
}
