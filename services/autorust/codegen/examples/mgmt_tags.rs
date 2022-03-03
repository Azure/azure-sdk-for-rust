// cargo run --example mgmt_tags
// prints all the mgmt (control plane, resource-manager) tags

use autorust_codegen::get_mgmt_readmes;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut tag_count = 0;
    for (i, spec) in get_mgmt_readmes()?.iter().enumerate() {
        println!("{} {}", i + 1, spec.spec());
        for tag in spec.config()?.tags() {
            println!("  {}", &tag.tag);
            for input_file in &tag.input_files {
                println!("    {}", input_file);
            }
            tag_count += 1;
        }
    }
    println!("{} tags", tag_count);
    Ok(())
}
