// cargo run --example gen_mgmt --release
// https://github.com/Azure/azure-rest-api-specs/blob/master/specification/compute/resource-manager
use autorust_codegen::{self, gen::gen_crate, get_mgmt_readmes, Result, RunConfig};

const OUTPUT_FOLDER: &str = "../mgmt";

const ONLY_SERVICES: &[&str] = &[];

fn main() -> Result<()> {
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
