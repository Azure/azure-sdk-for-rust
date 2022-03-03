// cargo run --example multiple_versions
// report tags that use multiple versions
// in general, we want to avoid this
// https://github.com/Azure/azure-sdk-for-rust/issues/563

use autorust_codegen::{get_mgmt_readmes, get_svc_readmes, path, Spec, SpecReadme};
use std::collections::BTreeSet;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    println!("CONTROL PLANE");
    check(&get_mgmt_readmes()?)?;
    println!();
    println!("DATA PLANE");
    check(&get_svc_readmes()?)?;
    Ok(())
}

fn check(readmes: &[SpecReadme]) -> Result<()> {
    let mut services = BTreeSet::new();
    let mut tags = 0;
    for readme in readmes {
        let readme_path = readme.readme();
        for tag in readme.config()?.tags() {
            let input_files = path::join_several(readme_path, &tag.input_files())?;
            match Spec::read_files(&input_files) {
                Ok(spec) => {
                    let versions = spec.api_versions();
                    if versions.len() > 1 {
                        println!("{} {}", readme.spec(), &tag.tag);
                        for version in versions {
                            println!("  {}", version);
                        }
                        tags += 1;
                        services.insert(readme.spec());
                    }
                }
                // Err(err) => println!("Error {}", err),
                Err(_err) => {}
            }
        }
    }
    println!();
    println!("{} tags", tags);
    println!("{} services:", services.len());
    for service in services {
        println!("  {}", service);
    }
    Ok(())
}
