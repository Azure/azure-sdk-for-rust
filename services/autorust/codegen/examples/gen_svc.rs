// cargo run --example gen_svc --release
// https://github.com/Azure/azure-rest-api-specs/blob/master/specification/batch/data-plane
use autorust_codegen::{self, gen::gen_crate, get_svc_readmes, Result, RunConfig};

const OUTPUT_FOLDER: &str = "../svc";

const ONLY_SERVICES: &[&str] = &["purview"];

fn main() -> Result<()> {
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
