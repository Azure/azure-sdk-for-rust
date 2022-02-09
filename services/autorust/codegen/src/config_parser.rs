use serde::Deserialize;
use std::path::{Path, PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Received AutoRest configuration file did not contain an expected extension (e.g. '.md'): '{0}'")]
    ExpectedMd(PathBuf),
    #[error("Error reading the received CommonMark configuration file")]
    ReadingConfig,
    #[error("Configuration heading did not contain a tag.")]
    NoTagInHeading,
    #[error("Expected configuration tag to contain a YAML code block.")]
    ExpectedYamlCodeBlock,
    #[error("Error reading configuration block yaml")]
    ConfigurationBlockYaml,
    #[error("No `## Configuration` heading in the AutoRest literate configuration file")]
    NoConfigurationHeading,
    #[error("Received AutoRest configuration extension not supported: '{extension}' (in configuration file '{config_file}')")]
    NotSupportedExtension { extension: String, config_file: PathBuf },
    #[error("Markdown ended unexpectedly after configuration tag heading")]
    MarkdownEnded,
    #[error("Code block info did not contain UTF-8 characters.")]
    CodeBlockNotUtf8,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    #[serde(rename(deserialize = "input-file"))]
    pub input_files: Vec<String>,

    #[serde(skip_deserializing)]
    pub tag: String,
}

impl Configuration {
    pub fn input_files(&self) -> Vec<PathBuf> {
        self.input_files.iter().map(PathBuf::from).collect()
    }
}

/// Receives the AutoRest configuration file and parses it to its various configurations (by tags/API versions),
/// according to its extension.
/// e.g. for "path/to/config.md", it will get parsed as CommonMark [Literate Configuration](http://azure.github.io/autorest/user/literate-file-formats/configuration.html).
pub fn parse_configurations_from_autorest_config_file(config_file: &Path) -> Result<Vec<Configuration>> {
    let extension = config_file
        .extension()
        .ok_or_else(|| Error::ExpectedMd(config_file.to_path_buf()))?;
    let extension = extension.to_str().ok_or_else(|| Error::ExpectedMd(config_file.to_path_buf()))?;
    match extension.to_lowercase().as_str() {
        "md" => {
            use literate_config::*;
            let cmark_content = std::fs::read_to_string(config_file).map_err(|_| Error::ReadingConfig)?;
            Ok(parse_configurations_from_cmark_config(&cmark_content)?)
        }
        _ => Err(Error::NotSupportedExtension {
            extension: extension.to_owned(),
            config_file: config_file.to_path_buf(),
        }),
    }
}

mod literate_config {
    use super::*;
    use comrak::{
        nodes::{AstNode, NodeCode, NodeCodeBlock, NodeValue},
        parse_document, Arena, ComrakOptions,
    };

    // Per the [Literage Configuration format](https://azure.github.io/autorest/user/literate-file-formats/configuration.html),
    // The configurations should all be contained under a "## Configuration" heading.
    const LITERATE_CONFIGURATION_HEADING_TEXT: &str = "Configuration";

    // Prefix for level-3 headings (e.g. "### Tag: package-2018-01") which should contain YAML codeblocks containing the configurations.
    const LITERATE_CONFIGURATION_TAG_PREFIX: &str = "Tag: ";

    /// Receives the configurations for all tags/versions from the received
    /// [Literate Configuration](http://azure.github.io/autorest/user/literate-file-formats/configuration.html) [CommonMark](https://commonmark.org/) file.
    pub(crate) fn parse_configurations_from_cmark_config(cmark_content: &str) -> Result<Vec<Configuration>> {
        let arena = Arena::new();
        let root = parse_document(&arena, cmark_content, &ComrakOptions::default());

        // Get the AST node corresponding with "## Configuration".
        let configuration_heading_node = get_configuration_section_heading_node(root).ok_or(Error::NoConfigurationHeading)?;

        let mut configurations = Vec::new();

        // Traverse all next AST nodes until next level-2 heading node (e.g. "## Another Heading"),
        // in search of level-3 headings representing tags (e.g. "### Tag: package-2020-01") to parse the configuration from.
        let mut current_node = configuration_heading_node.next_sibling();
        while let Some(node) = current_node {
            if let Some(tag) = get_tag(node) {
                // Extract the configuration from the first node inside the tag heading ("Tag: ..."),
                // by looking at the first YAML code block.
                let code_block = extract_configuration_code_block_node(node)?.ok_or(Error::ExpectedYamlCodeBlock)?;
                let mut configuration: Configuration = serde_yaml::from_str(&code_block).map_err(|_| Error::ConfigurationBlockYaml)?;
                configuration.tag = tag;
                configurations.push(configuration);
            } else if is_header_at_level(node, 2) {
                break;
            }
            current_node = node.next_sibling();
        }

        Ok(configurations)
    }

    // based on https://github.com/kivikakk/comrak/blob/main/examples/headers.rs
    fn is_header_at_level<'a>(node: &'a AstNode<'a>, level: u32) -> bool {
        match node.data.clone().into_inner().value {
            NodeValue::Heading(heading) => heading.level == level,
            _ => false,
        }
    }

    // from https://github.com/kivikakk/comrak/blob/main/examples/headers.rs
    fn collect_text<'a>(node: &'a AstNode<'a>, output: &mut Vec<u8>) {
        match node.data.borrow().value {
            NodeValue::Text(ref literal) | NodeValue::Code(NodeCode { ref literal, .. }) => output.extend_from_slice(literal),
            NodeValue::LineBreak | NodeValue::SoftBreak => output.push(b' '),
            _ => {
                for n in node.children() {
                    collect_text(n, output);
                }
            }
        }
    }

    /// Returns the first "## Configuration" AST Node.
    /// There should only be one per Literate Configuration file.
    fn get_configuration_section_heading_node<'a>(root: &'a AstNode<'a>) -> Option<&'a AstNode<'a>> {
        root.children().find(|node| {
            if is_header_at_level(node, 2) {
                let mut text = Vec::new();
                collect_text(node, &mut text);
                if let Ok(text) = std::str::from_utf8(&text) {
                    text.trim() == LITERATE_CONFIGURATION_HEADING_TEXT
                } else {
                    false
                }
            } else {
                false
            }
        })
    }

    /// Returns the node tag if it is one.
    /// (e.g. "### Tag: package-2020-01")
    fn get_tag<'a>(node: &'a AstNode<'a>) -> Option<String> {
        if is_header_at_level(node, 3) {
            let mut text = Vec::new();
            collect_text(node, &mut text);
            if let Ok(text) = std::str::from_utf8(&text) {
                text.find(LITERATE_CONFIGURATION_TAG_PREFIX)
                    .map(|start| text[start + LITERATE_CONFIGURATION_TAG_PREFIX.len()..].to_owned())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Extracts the configuration code block for the received node.
    fn extract_configuration_code_block_node<'a>(configuration_tag_heading_node: &'a AstNode<'a>) -> Result<Option<String>> {
        let mut current_node = configuration_tag_heading_node.next_sibling().ok_or(Error::MarkdownEnded)?;
        loop {
            if let NodeValue::CodeBlock(NodeCodeBlock { info, literal, fenced, .. }) = &current_node.data.borrow().value {
                if !fenced {
                    continue;
                }
                let info = std::str::from_utf8(info).map_err(|_| Error::CodeBlockNotUtf8)?;
                if info.trim_start().to_lowercase().starts_with("yaml") {
                    let literal = std::str::from_utf8(literal).map_err(|_| Error::CodeBlockNotUtf8)?;
                    return Ok(Some(literal.to_owned()));
                }
            }
            current_node = current_node.next_sibling().ok_or(Error::MarkdownEnded)?;
        }
    }
}

fn starts_with_number(text: &str) -> bool {
    text.chars().next().unwrap_or_default().is_numeric()
}

pub fn get_input_file_api_version(input_file: &str) -> Option<String> {
    let parts: Vec<_> = input_file.split('/').collect();
    if parts.len() == 4 {
        Some(parts[2].to_owned())
    } else {
        None
    }
}

/// Create a Rust tag name, based on the feature naem.
pub fn to_tag_name(name: &str) -> String {
    name.chars()
        .map(|x| match x {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => x,
            _ => '_',
        })
        .collect()
}

/// Create a Rust module name, based on the feature naem.
pub fn to_mod_name(feature_name: &str) -> String {
    let mut name = feature_name.to_owned();
    if starts_with_number(&name) {
        name = format!("v{}", &name);
    }
    name.replace("-", "_").replace(".", "_").to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::{literate_config::*, *};

    #[test]
    fn test_get_input_file_api_version() {
        assert_eq!(
            Some("2019-05-05-preview".to_owned()),
            get_input_file_api_version("Microsoft.AlertsManagement/preview/2019-05-05-preview/ActionRules.json")
        );
    }

    #[test]
    fn test_tag_name() {
        assert_eq!("2019-06", to_tag_name("2019-06"));
        assert_eq!("2019_06", to_tag_name("2019.06"));
        assert_eq!("2019_06", to_tag_name("2019!06"));
    }

    #[test]
    fn test_mod_name() {
        assert_eq!("v2019_06", to_mod_name("2019-06"));
        assert_eq!("v2018_10_01_disks", to_mod_name("2018-10-01-Disks"));
    }

    #[test]
    fn literate_config_should_parse_one_configuration() -> Result<()> {
        let input = "
## Configuration

### Tag: package-2019-06

These settings apply only when `--tag=package-2019-06` is specified on the command line.

``` yaml $(tag) == 'package-2019-06'
input-file:
- Microsoft.Storage/stable/2019-06-01/storage.json
- Microsoft.Storage/stable/2019-06-01/blob.json
- Microsoft.Storage/stable/2019-06-01/file.json
- Microsoft.Storage/stable/2019-06-01/queue.json
- Microsoft.Storage/stable/2019-06-01/table.json
```
";
        let configurations = parse_configurations_from_cmark_config(input)?;
        assert_eq!(1, configurations.len());
        assert_eq!("package-2019-06", configurations[0].tag);
        assert_eq!(5, configurations[0].input_files.len());
        assert_eq!("Microsoft.Storage/stable/2019-06-01/storage.json", configurations[0].input_files[0]);
        assert_eq!("Microsoft.Storage/stable/2019-06-01/blob.json", configurations[0].input_files[1]);
        Ok(())
    }

    #[test]
    fn literate_config_should_parse_multiple_configuration() -> Result<()> {
        let input = "
## Configuration

### Tag: package-2019-06

These settings apply only when `--tag=package-2019-06` is specified on the command line.

``` yaml $(tag) == 'package-2019-06'
input-file:
- Microsoft.Storage/stable/2019-06-01/storage.json
- Microsoft.Storage/stable/2019-06-01/blob.json
- Microsoft.Storage/stable/2019-06-01/file.json
- Microsoft.Storage/stable/2019-06-01/queue.json
- Microsoft.Storage/stable/2019-06-01/table.json
```

### Tag: package-2015-05-preview

These settings apply only when `--tag=package-2015-05-preview` is specified on the command line.

``` yaml $(tag) == 'package-2015-05-preview'
input-file:
- Microsoft.Storage/preview/2015-05-01-preview/storage.json
```
";
        let configurations = parse_configurations_from_cmark_config(input)?;
        assert_eq!(2, configurations.len());
        assert_eq!("package-2019-06", configurations[0].tag);
        assert_eq!(5, configurations[0].input_files.len());
        assert_eq!("Microsoft.Storage/stable/2019-06-01/storage.json", configurations[0].input_files[0]);
        assert_eq!("Microsoft.Storage/stable/2019-06-01/blob.json", configurations[0].input_files[1]);

        assert_eq!("package-2015-05-preview", configurations[1].tag);
        assert_eq!(1, configurations[1].input_files.len());
        assert_eq!(
            "Microsoft.Storage/preview/2015-05-01-preview/storage.json",
            configurations[1].input_files[0]
        );
        Ok(())
    }

    #[test]
    fn literate_config_should_fail_for_invalid_heading() -> Result<()> {
        let invalid_input = "
## INVALID_HEADING

### Tag: package-2019-06

These settings apply only when `--tag=package-2019-06` is specified on the command line.

``` yaml $(tag) == 'package-2019-06'
input-file:
- Microsoft.Storage/stable/2019-06-01/storage.json
```
";
        assert!(matches!(
            parse_configurations_from_cmark_config(invalid_input),
            Err(Error::NoConfigurationHeading)
        ));
        Ok(())
    }

    #[test]
    fn should_ignore_invalid_tag_headings() -> Result<()> {
        let invalid_input = "
## Configuration

### INVALID_TAG: package-2019-06

These settings apply only when `--tag=package-2019-06` is specified on the command line.

``` yaml $(tag) == 'package-2019-06'
input-file:
- Microsoft.Storage/stable/2019-06-01/storage.json
```
";
        assert!(parse_configurations_from_cmark_config(invalid_input)?.is_empty());
        Ok(())
    }

    #[test]
    fn literate_config_should_ignore_code_blocks_after_configuration_heading() -> Result<()> {
        let input = "
## Configuration

### Tag: package-2019-06

These settings apply only when `--tag=package-2019-06` is specified on the command line.

``` yaml $(tag) == 'package-2019-06'
input-file:
- Microsoft.Storage/stable/2019-06-01/storage.json
```

## DIFFERENT HEADING

### Tag: package-2020-01-01

These settings apply only when `--tag=package-2020-01-01` is specified on the command line.

``` yaml $(tag) == 'package-2020-01-01'
input-file:
- Microsoft.Storage/stable/2020-01-01-01/storage.json
```
";

        let configurations = parse_configurations_from_cmark_config(input)?;
        assert_eq!(1, configurations.len());
        assert_eq!("package-2019-06", configurations[0].tag);
        Ok(())
    }
}
