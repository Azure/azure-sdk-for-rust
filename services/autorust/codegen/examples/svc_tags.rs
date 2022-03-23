// cargo run --example svc_tags
// prints all the svc (data plane) tags

use autorust_codegen::Result;
use autorust_codegen::{self, get_svc_readmes};

fn main() -> Result<()> {
    let mut tag_count = 0;
    for (i, spec) in get_svc_readmes()?.iter().enumerate() {
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
