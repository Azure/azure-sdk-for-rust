// cargo run --example svc_tags
// prints all the svc (data plane) tags

use autorust_codegen::{self, get_svc_readmes};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    for (i, spec) in get_svc_readmes()?.iter().enumerate() {
        println!("{} {}", i + 1, spec.spec());
        for config in spec.configs() {
            println!("  {}", &config.tag);
        }
    }
    Ok(())
}
