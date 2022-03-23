// cargo run --example mgmt_tags
// prints all the mgmt (control plane, resource-manager) tags

use autorust_codegen::get_mgmt_readmes;
use autorust_codegen::Result;

fn main() -> Result<()> {
    let mut tag_count = 0;
    for (i, spec) in get_mgmt_readmes()?.iter().enumerate() {
        println!("{} {}", i + 1, spec.spec());
        for tag in spec.config()?.tags() {
            println!("  {}", &tag.name());
            for input_file in &tag.input_files() {
                println!("    {}", input_file.display());
            }
            tag_count += 1;
        }
    }
    println!("{} tags", tag_count);
    Ok(())
}
