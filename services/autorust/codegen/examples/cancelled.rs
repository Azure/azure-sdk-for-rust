// cargo run --example cancelled
// Analysis for https://github.com/Azure/azure-resource-manager-rpc/issues/144
// List of specification that have a provisioningState of Cancelled

use autorust_codegen::codegen_models::{all_schemas_resolved, SchemaGen};
use autorust_codegen::{get_mgmt_readmes, get_svc_readmes, io, Result, Spec, SpecReadme};
use std::collections::BTreeSet;

fn main() -> Result<()> {
    check("Control Plane", &get_mgmt_readmes()?)?;
    println!();
    check("Data Plane", &get_svc_readmes()?)?;
    Ok(())
}

fn has_cancelled_enum_value(schema: &SchemaGen) -> bool {
    for property in schema.properties() {
        if "provisioningState" == property.name() {
            for ev in property.schema().enum_values() {
                if "Cancelled".eq_ignore_ascii_case(ev.value()) {
                    return true;
                }
            }
        }
    }
    false
}

fn check(plane: &str, readmes: &[SpecReadme]) -> Result<()> {
    let mut services = BTreeSet::new();
    for readme in readmes {
        let readme_path = readme.readme();
        for tag in readme.config()?.tags() {
            let input_files = io::join_several(readme_path, &tag.input_files())?;
            match Spec::read_files(&input_files) {
                Ok(spec) => {
                    if let Ok(schemas) = all_schemas_resolved(&spec) {
                        for (_ref_key, schema) in &schemas {
                            if has_cancelled_enum_value(schema) {
                                services.insert(readme.spec());
                            }
                        }
                    }
                }
                Err(_err) => {}
            }
        }
    }
    println!("{} {} services:", plane, services.len());
    for service in services {
        println!("  {}", service);
    }
    Ok(())
}
