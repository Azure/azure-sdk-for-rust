// cargo run --example cancelled
// Analysis for https://github.com/Azure/azure-resource-manager-rpc/issues/144
// List of specification that have a provisioningState of Cancelled

use autorust_codegen::codegen_models::{all_schemas_resolved, SchemaGen};
use autorust_codegen::{get_mgmt_readmes, get_svc_readmes, io, Result, Spec, SpecReadme};
use std::collections::BTreeSet;

fn main() -> Result<()> {
    println!("CONTROL PLANE");
    check(&get_mgmt_readmes()?)?;
    println!();
    println!("DATA PLANE");
    check(&get_svc_readmes()?)?;
    Ok(())
}

fn has_provisioning_state_cancelled(schema: &SchemaGen) -> bool {
    for property in schema.properties() {
        for ev in property.schema().enum_values() {
            if "Cancelled".eq_ignore_ascii_case(ev.value()) {
                return true;
            }
        }
    }
    false
}

fn check(readmes: &[SpecReadme]) -> Result<()> {
    let mut services = BTreeSet::new();
    for readme in readmes {
        let readme_path = readme.readme();
        for tag in readme.config()?.tags() {
            let input_files = io::join_several(readme_path, &tag.input_files())?;
            match Spec::read_files(&input_files) {
                Ok(spec) => {
                    if let Ok(schemas) = all_schemas_resolved(&spec) {
                        for (_ref_key, schema) in &schemas {
                            if has_provisioning_state_cancelled(&schema) {
                                services.insert(readme.spec());
                            }
                        }
                    }
                }
                Err(_err) => {}
            }
        }
    }
    println!("{} services:", services.len());
    for service in services {
        println!("  {}", service);
    }
    Ok(())
}
