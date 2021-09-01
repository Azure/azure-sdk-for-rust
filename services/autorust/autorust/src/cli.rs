use autorust_codegen::Config;
use clap::{App, Arg, ArgMatches};

pub type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("InputFileIsRequired")]
    InputFileIsRequired,
    #[error("OutputFolder")]
    OutputFolder,
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const INPUT_FILE: &str = "input-file";
const OUTPUT_FOLDER: &str = "output-folder";
const GENERATED: &str = "generated";
const API_VERSION: &str = "api-version";

pub fn config_try_new() -> Result<Config> {
    let arg_matches = new_app().get_matches();
    config_try_new_from_matches(&arg_matches)
}

fn config_try_new_from_matches(arg_matches: &ArgMatches) -> Result<Config> {
    let input_files = arg_matches
        .values_of(INPUT_FILE)
        .ok_or_else(|| Error::InputFileIsRequired)?
        .map(|s| s.into())
        .collect::<Vec<_>>();
    let output_folder = arg_matches
        .value_of(OUTPUT_FOLDER)
        .ok_or_else(|| Error::OutputFolder)?
        .to_owned()
        .into();
    let api_version = arg_matches.value_of(API_VERSION).map(String::from);
    Ok(Config {
        input_files,
        output_folder,
        api_version,
        ..Config::default()
    })
}

fn new_app() -> App<'static> {
    App::new(NAME)
        .version(VERSION)
        .arg(
            Arg::new(INPUT_FILE)
                .about("OpenAPI file to use as input (use this setting repeatedly to pass multiple files at once)")
                .long(INPUT_FILE)
                .required(true)
                .takes_value(true), // .multiple(true),
        )
        .arg(
            Arg::new(OUTPUT_FOLDER)
                .about("target folder for generated artifacts; default: \"<base folder>/generated\"")
                .long(OUTPUT_FOLDER)
                .takes_value(true)
                .default_value(GENERATED),
        )
        .arg(
            Arg::new(API_VERSION)
                .about("sets the api-version query parameter to use")
                .long(API_VERSION)
                .takes_value(true),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::ErrorKind;
    use std::path::PathBuf;

    pub type Error = Box<dyn std::error::Error + Send + Sync>;
    pub type Result<T> = std::result::Result<T, Error>;

    #[test]
    fn missing_required() {
        let m = new_app().try_get_matches_from(vec![""]);
        assert!(m.is_err(), "{:?}", m);
        assert_eq!(m.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn two_input_files() {
        let m = new_app().try_get_matches_from(vec![NAME, "--input-file", "abc.json", "--input-file", "def.json"]);
        assert!(m.is_ok(), "{:?}", m);
        let m = m.unwrap();
        assert_eq!(m.occurrences_of(INPUT_FILE), 2);
        assert_eq!(m.values_of(INPUT_FILE).unwrap().collect::<Vec<_>>(), ["abc.json", "def.json"]);
        assert_eq!(m.value_of(OUTPUT_FOLDER).unwrap(), GENERATED);
    }

    #[test]
    fn args_with_equals() {
        let m = new_app().try_get_matches_from(vec![NAME, "--input-file=abc.json", "--input-file=def.json", "--output-folder=src"]);
        assert!(m.is_ok(), "{:?}", m);
        let m = m.unwrap();
        assert_eq!(m.occurrences_of(INPUT_FILE), 2);
        assert_eq!(m.values_of(INPUT_FILE).unwrap().collect::<Vec<_>>(), ["abc.json", "def.json"]);
        assert_eq!(m.value_of(OUTPUT_FOLDER).unwrap(), "src");
    }

    #[test]
    fn test_new_config() -> Result<()> {
        let m = new_app().try_get_matches_from(vec![NAME, "--input-file=abc.json", "--input-file=def.json", "--output-folder=src"]);
        assert!(m.is_ok(), "{:?}", m);
        let m = m?;
        let c = config_try_new_from_matches(&m)?;
        let input_files: [PathBuf; 2] = ["abc.json".into(), "def.json".into()];
        assert_eq!(c.input_files, input_files);
        let output_folder: PathBuf = "src".into();
        assert_eq!(c.output_folder, output_folder);
        Ok(())
    }
}
