mod cli;
use autorust_codegen::run;
use cli::config_try_new;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let config = config_try_new()?;
    run(config)?;
    Ok(())
}
