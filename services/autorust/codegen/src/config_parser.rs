#![allow(dead_code)]
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    #[serde(rename(deserialize = "input-file"))]
    pub input_files: Vec<String>,

    #[serde(skip_deserializing)]
    pub tag: String,
}

/// Receives the AutoRest configuration file and parses it to its various configurations (by tags/API versions),
/// according to its extension.
/// e.g. for "path/to/config.md", it will get parsed as CommonMark [Literate Configuration](http://azure.github.io/autorest/user/literate-file-formats/configuration.html).
pub fn parse_configurations_from_autorest_config_file(config_file: &PathBuf) -> Vec<Configuration> {
    let extension = config_file.extension().expect(&format!(
        "Received AutoRest configuration file did not contain an expected extension (e.g. '.md'): '{0}'",
        config_file.as_path().to_str().unwrap()
    ));
    let extension = extension.to_str().unwrap();
    match extension.to_lowercase().as_str() {
        "md" => {
            use literate_config::*;

            let cmark_content =
                std::fs::read_to_string(config_file).expect("Unexpected error when reading the received CommonMark configuration file");
            parse_configurations_from_cmark_config(&cmark_content)
        }
        _ => panic!(
            "Received AutoRest configuration extension not supported: '{0}' (in configuration file '{1}')",
            extension,
            config_file.as_path().to_str().unwrap()
        ),
    }
}

mod literate_config {
    use comrak::{
        nodes::{AstNode, NodeCodeBlock, NodeHeading, NodeValue},
        parse_document, Arena, ComrakOptions,
    };

    use super::*;

    // Per the [Literage Configuration format](https://azure.github.io/autorest/user/literate-file-formats/configuration.html),
    // The configurations should all be contained under a "## Configuration" heading.
    const LITERATE_CONFIGURATION_HEADING_TEXT: &str = "Configuration";

    // Prefix for level-3 headings (e.g. "### Tag: package-2018-01") which should contain YAML codeblocks containing the configurations.
    const LITERATE_CONFIGURATION_TAG_PREFIX: &str = "Tag: ";

    /// Receives the configurations for all tags/versions from the received
    /// [Literate Configuration](http://azure.github.io/autorest/user/literate-file-formats/configuration.html) [CommonMark](https://commonmark.org/) file.
    pub(crate) fn parse_configurations_from_cmark_config(cmark_content: &str) -> Vec<Configuration> {
        let arena = Arena::new();
        let root = parse_document(&arena, &cmark_content, &ComrakOptions::default());

        // Get the AST node corresponding with "## Configuration".
        let configuration_heading_node = get_configuration_section_heading_node(root)
            .expect("No `## Configuration` heading in the AutoRest literate configuration file");

        let mut configurations = Vec::new();

        // Traverse all next AST nodes until next level-2 heading node (e.g. "## Another Heading"),
        // in search of level-3 headings representing tags (e.g. "### Tag: package-2020-01") to parse the configuration from.
        let mut current_node = configuration_heading_node.next_sibling();
        while let Some(node) = current_node {
            if is_configuration_tag_heading_node(node) {
                // Extract the configuration from the first node inside the tag heading ("Tag: ..."),
                // by looking at the first YAML code block.
                let code_block = extract_configuration_code_block_node(node).expect(&format!(
                    "Expected configuration tag ('{0}'...) to contain a YAML code block.",
                    LITERATE_CONFIGURATION_TAG_PREFIX
                ));
                let mut configuration: Configuration = serde_yaml::from_str(&code_block).expect("TODO(PR)");
                configuration.tag = extract_configuration_tag_from_heading_node(node);
                configurations.push(configuration);
            } else {
                let node_data = node.data.borrow();
                if matches!(node_data.value, NodeValue::Heading(NodeHeading { level, .. }) if level == 2) {
                    // Encountered another heading of level 2 - stop traversal.
                    break;
                }
            }
            current_node = node.next_sibling();
        }

        configurations
    }

    /// Returns the first "## Configuration" AST Node.
    /// There should only be one per Literate Configuration file.
    fn get_configuration_section_heading_node<'a>(root: &'a AstNode<'a>) -> Option<&'a AstNode<'a>> {
        root.children().find(|node| {
            let node_data = node.data.borrow();
            matches!(node_data.value, NodeValue::Heading(NodeHeading { level, .. })
                if level == 2 && std::str::from_utf8(&node_data.content).unwrap_or("").trim() == LITERATE_CONFIGURATION_HEADING_TEXT)
        })
    }

    /// Returns whether the received AST node represents a heading of a configuration for a tag.
    /// (e.g. "### Tag: package-2020-01")
    fn is_configuration_tag_heading_node<'a>(node: &'a AstNode<'a>) -> bool {
        let node_data = node.data.borrow();
        matches!(node_data.value, NodeValue::Heading(NodeHeading { level, .. })
                if level == 3 && std::str::from_utf8(&node_data.content).unwrap_or("").starts_with(LITERATE_CONFIGURATION_TAG_PREFIX))
    }

    /// Returns the tag (API version) from the AST node which represents the heading of a configuration for a tag
    /// (e.g. ""### Tag: package-2020-01"")
    fn extract_configuration_tag_from_heading_node<'a>(node: &'a AstNode<'a>) -> String {
        let node_data = node.data.borrow();
        let heading = std::str::from_utf8(&node_data.content).unwrap_or("");
        let tag_start_index = heading
            .find(LITERATE_CONFIGURATION_TAG_PREFIX)
            .expect("Configuration heading did not contain a tag.");
        heading[tag_start_index + LITERATE_CONFIGURATION_TAG_PREFIX.len()..].to_owned()
    }

    /// Extracts the configuration code block for the received node.
    fn extract_configuration_code_block_node<'a>(configuration_tag_heading_node: &'a AstNode<'a>) -> Option<String> {
        let mut current_node = configuration_tag_heading_node
            .next_sibling()
            .expect("Markdown configuration ended unexpectedly after configuration tag heading");
        loop {
            if let NodeValue::CodeBlock(NodeCodeBlock { info, literal, fenced, .. }) = &current_node.data.borrow().value {
                if !fenced {
                    continue;
                }
                let info = std::str::from_utf8(&info).expect("Code block info did not contain UTF-8 characters.");
                if info.trim_start().to_lowercase().starts_with("yaml") {
                    let literal = std::str::from_utf8(&literal).expect("Code block content did not contain UTF-8 characters.");
                    return Some(literal.to_owned());
                }
            }
            current_node = current_node.next_sibling()?;
        }
    }
}

fn starts_with_number(text: &str) -> bool {
    match text.chars().next().unwrap_or_default() {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => true,
        _ => false,
    }
}

/// Get an API Version from tag.
/// It is a date in yyyy-mm-dd format followed by an optional "-preview".
pub fn to_api_version(package: &Configuration) -> Option<String> {
    let re = regex::Regex::new(r"\d{4}-\d{2}-\d{2}(:?-\w*)?").unwrap();
    let captures: Vec<String> = re.captures_iter(&package.tag).into_iter().map(|c| c[0].to_string()).collect();
    let api_version = if captures.len() == 1 {
        let parts: Vec<_> = captures[0].split("-").collect();
        match parts.len() {
            3 => Some(captures[0].clone()),
            4 => match parts[3] {
                "preview" => Some(captures[0].clone()),
                _ => None,
            },
            _ => None,
        }
    } else {
        None
    };
    match api_version {
        Some(_) => api_version,
        None => {
            if package.input_files.len() > 0 {
                get_input_file_api_version(&package.input_files[0])
            } else {
                None
            }
        }
    }
}

pub fn get_input_file_api_version(input_file: &str) -> Option<String> {
    let parts: Vec<_> = input_file.split("/").collect();
    if parts.len() == 4 {
        Some(parts[2].to_owned())
    } else {
        None
    }
}

/// Create a Rust module name, based on the feature naem.
pub fn to_mod_name(feature_name: &str) -> String {
    let mut name = feature_name.to_owned();
    if starts_with_number(&name) {
        name = format!("v{}", &name);
    }
    name.replace("-", "_").to_lowercase()
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

    fn new_package_from_tag(tag: &str) -> Configuration {
        Configuration {
            tag: tag.to_owned(),
            input_files: Vec::new(),
        }
    }

    #[test]
    fn test_api_version_name() {
        assert_eq!(
            Some("2019-06-01".to_owned()),
            to_api_version(&new_package_from_tag("package-2019-06-01"))
        );
        assert_eq!(
            Some("2019-06-01-preview".to_owned()),
            to_api_version(&new_package_from_tag("package-2019-06-01-preview"))
        );
        assert_eq!(None, to_api_version(&new_package_from_tag("package-2019-06-01-Disk")));
        assert_eq!(None, to_api_version(&new_package_from_tag("package-2019-06-01-only")));
    }

    #[test]
    fn test_mod_name() {
        assert_eq!("v2019_06", to_mod_name("2019-06"));
        assert_eq!("v2018_10_01_disks", to_mod_name("2018-10-01-Disks"));
    }

    #[test]
    fn literate_config_should_parse_one_configuration() {
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
        let configurations = parse_configurations_from_cmark_config(input);
        assert_eq!(1, configurations.len());
        assert_eq!("package-2019-06", configurations[0].tag);
        assert_eq!(5, configurations[0].input_files.len());
        assert_eq!("Microsoft.Storage/stable/2019-06-01/storage.json", configurations[0].input_files[0]);
        assert_eq!("Microsoft.Storage/stable/2019-06-01/blob.json", configurations[0].input_files[1]);
    }

    #[test]
    fn literate_config_should_parse_multiple_configuration() {
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
        let configurations = parse_configurations_from_cmark_config(input);
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
    }

    #[test]
    #[should_panic]
    fn literate_config_should_fail_for_invalid_heading() {
        let invalid_input = "
## INVALID_HEADING

### Tag: package-2019-06

These settings apply only when `--tag=package-2019-06` is specified on the command line.

``` yaml $(tag) == 'package-2019-06'
input-file:
- Microsoft.Storage/stable/2019-06-01/storage.json
```
";
        parse_configurations_from_cmark_config(invalid_input);
    }

    #[test]
    fn should_ignore_invalid_tag_headings() {
        let invalid_input = "
## Configuration

### INVALID_TAG: package-2019-06

These settings apply only when `--tag=package-2019-06` is specified on the command line.

``` yaml $(tag) == 'package-2019-06'
input-file:
- Microsoft.Storage/stable/2019-06-01/storage.json
```
";
        assert!(parse_configurations_from_cmark_config(invalid_input).is_empty());
    }

    #[test]
    fn literate_config_should_ignore_code_blocks_after_configuration_heading() {
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

        let configurations = parse_configurations_from_cmark_config(input);
        assert_eq!(1, configurations.len());
        assert_eq!("package-2019-06", configurations[0].tag);
    }
}
