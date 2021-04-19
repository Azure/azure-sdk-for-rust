// cargo run --example mgmt_tags
// prints all the mgmt (control plane, resource-manager) tags

use autorust_codegen::*;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    for (i, spec) in get_mgmt_configs()?.iter().enumerate() {
        println!("{} {}", i + 1, spec.spec());
        for config in spec.configs() {
            println!("  {}", &config.tag);
        }
    }
    Ok(())
}
