use crate::{Error, ErrorKind, Result, ResultExt};
use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct Configuration {
    basic_info: BasicInformation,
    tags: Vec<Tag>,
}

impl Configuration {
    pub fn title(&self) -> Option<&str> {
        self.basic_info.title.as_deref()
    }
    pub fn description(&self) -> Option<&str> {
        self.basic_info.description.as_deref()
    }
    pub fn openapi_type(&self) -> Option<&str> {
        self.basic_info.openapi_type.as_deref()
    }
    /// An optional `Basic Information` `tag` specifies the default
    pub fn tag(&self) -> Option<&str> {
        self.basic_info.tag.as_deref()
    }
    /// All `Tag`s in `Configuration`
    pub fn tags(&self) -> Vec<&Tag> {
        self.tags.iter().collect()
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct BasicInformation {
    title: Option<String>,
    description: Option<String>,
    /// "arm" or "data-plane"
    #[serde(rename(deserialize = "openapi-type"))]
    pub openapi_type: Option<String>,
    /// The default tag name.
    pub tag: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    #[serde(rename(deserialize = "input-file"))]
    input_files: Vec<String>,

    #[serde(skip_deserializing)]
    tag: String,
}
impl Tag {
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            input_files: Vec::new(),
        }
    }
}

impl Tag {
    pub fn input_files(&self) -> Vec<Utf8PathBuf> {
        self.input_files.iter().map(Utf8PathBuf::from).collect()
    }

    pub fn name(&self) -> &str {
        &self.tag
    }

    pub fn rust_feature_name(&self) -> String {
        to_rust_feature_name(&self.tag)
    }

    pub fn rust_mod_name(&self) -> String {
        to_rust_mod_name(&self.rust_feature_name())
    }
}

/// Receives the AutoRest configuration file and parses it to its various configurations (by tags/API versions),
/// according to its extension.
/// e.g. for "path/to/config.md", it will get parsed as CommonMark [Literate Tag](http://azure.github.io/autorest/user/literate-file-formats/configuration.html).
pub fn parse_configurations_from_autorest_config_file(config_file: &Utf8Path) -> Result<Configuration> {
    let extension = config_file
        .extension()
        .ok_or_else(|| Error::with_message(ErrorKind::Parse, || format!("expected md extension {config_file}")))?;
    match extension.to_lowercase().as_str() {
        "md" => {
            use literate_config::*;
            let cmark_content =
                std::fs::read_to_string(config_file).with_context(ErrorKind::Io, || format!("reading the md file {config_file}"))?;
            Ok(parse_configuration(&cmark_content)?)
        }
        _ => Err(Error::with_message(ErrorKind::Io, || {
            format!("AutoRest configuration file did not contain an expected extension '.md': '{config_file}'")
        })),
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
    const LITERATE_CONFIGURATION_HEADING_BASIC_INFORMATION: &str = "Basic Information";

    // Prefix for level-3 headings (e.g. "### Tag: package-2018-01") which should contain YAML codeblocks containing the configurations.
    const LITERATE_CONFIGURATION_TAG_PREFIX: &str = "Tag: ";

    /// Receives the configurations for all tags/versions from the received
    /// [Literate Configuration](http://azure.github.io/autorest/user/literate-file-formats/configuration.html) [CommonMark](https://commonmark.org/) file.
    pub(crate) fn parse_configuration(cmark_content: &str) -> Result<Configuration> {
        let arena = Arena::new();
        let root = parse_document(&arena, cmark_content, &ComrakOptions::default());

        // Get the AST node corresponding with "## Configuration".
        let configuration_heading_node = get_configuration_section_heading_node(root).ok_or_else(|| {
            Error::message(
                ErrorKind::Parse,
                "no `## Configuration` heading in the AutoRest literate configuration file",
            )
        })?;

        let mut tags = Vec::new();
        let mut basic_info = BasicInformation::default();

        // Traverse all next AST nodes until next level-2 heading node (e.g. "## Another Heading"),
        // in search of level-3 headings representing tags (e.g. "### Tag: package-2020-01") to parse the configuration from.
        let mut current_node = configuration_heading_node.next_sibling();
        while let Some(node) = current_node {
            if is_basic_information(node) {
                let yaml = extract_yaml(node)?
                    .ok_or_else(|| Error::message(ErrorKind::Parse, "expected configuration tag to contain a YAML code block"))?;
                basic_info = serde_yaml::from_str(&yaml).context(ErrorKind::DataConversion, "reading basic information block yaml")?;
            } else if let Some(tag_name) = get_tag_name(node) {
                // Extract the configuration from the first node inside the tag heading ("Tag: ..."),
                // by looking at the first YAML code block.
                let yaml = extract_yaml(node)?
                    .ok_or_else(|| Error::message(ErrorKind::Parse, "Expected configuration tag to contain a YAML code block."))?;
                let mut tag: Tag = serde_yaml::from_str(&yaml).context(ErrorKind::Parse, "reading configuration block yaml")?;
                for input_file in tag.input_files.iter_mut() {
                    *input_file = input_file.replace('\\', "/");
                }
                tag.tag = tag_name;
                tags.push(tag);
            } else if is_header_at_level(node, 2) {
                break;
            }
            current_node = node.next_sibling();
        }

        Ok(Configuration { basic_info, tags })
    }

    // based on https://github.com/kivikakk/comrak/blob/main/examples/headers.rs
    fn is_header_at_level<'a>(node: &'a AstNode<'a>, level: u8) -> bool {
        match node.data.clone().into_inner().value {
            NodeValue::Heading(heading) => heading.level == level,
            _ => false,
        }
    }

    // from https://github.com/kivikakk/comrak/blob/main/examples/headers.rs
    fn collect_text<'a>(node: &'a AstNode<'a>, output: &mut String) {
        match node.data.borrow().value {
            NodeValue::Text(ref literal) | NodeValue::Code(NodeCode { ref literal, .. }) => output.push_str(literal),
            NodeValue::LineBreak | NodeValue::SoftBreak => output.push(' '),
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
                let mut text = String::new();
                collect_text(node, &mut text);
                text.trim() == LITERATE_CONFIGURATION_HEADING_TEXT
            } else {
                false
            }
        })
    }

    /// Returns the node tag if it is one.
    /// (e.g. "### Tag: package-2020-01")
    fn get_tag_name<'a>(node: &'a AstNode<'a>) -> Option<String> {
        if is_header_at_level(node, 3) {
            let mut text = String::new();
            collect_text(node, &mut text);
            text.find(LITERATE_CONFIGURATION_TAG_PREFIX)
                .map(|start| text[start + LITERATE_CONFIGURATION_TAG_PREFIX.len()..].to_owned())
        } else {
            None
        }
    }

    /// Returns the basic information node if it is one.
    /// (e.g. "### Basic Information")
    fn is_basic_information<'a>(node: &'a AstNode<'a>) -> bool {
        if is_header_at_level(node, 3) {
            let mut text = String::new();
            collect_text(node, &mut text);
            text.contains(LITERATE_CONFIGURATION_HEADING_BASIC_INFORMATION)
        } else {
            false
        }
    }

    /// Extracts the yaml from the received node.
    fn extract_yaml<'a>(configuration_tag_heading_node: &'a AstNode<'a>) -> Result<Option<String>> {
        let mut current_node = configuration_tag_heading_node
            .next_sibling()
            .ok_or_else(|| Error::message(ErrorKind::Parse, "markdown ended unexpectedly after configuration tag heading"))?;
        loop {
            if let NodeValue::CodeBlock(NodeCodeBlock { info, literal, fenced, .. }) = &current_node.data.borrow().value {
                if !fenced {
                    continue;
                }
                if info.trim_start().to_lowercase().starts_with("yaml") {
                    return Ok(Some(literal.to_owned()));
                }
            }
            current_node = current_node
                .next_sibling()
                .ok_or_else(|| Error::message(ErrorKind::Parse, "markdown ended unexpectedly after configuration tag heading"))?;
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

/// Create a Rust feature name, based on the feature naem.
fn to_rust_feature_name(name: &str) -> String {
    name.chars()
        .map(|x| match x {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => x,
            _ => '_',
        })
        .collect()
}

/// Create a Rust module name, based on the feature naem.
fn to_rust_mod_name(feature_name: &str) -> String {
    let mut name = feature_name.to_owned();
    if starts_with_number(&name) {
        name = format!("v{}", &name);
    }
    name.replace(['-', '.'], "_").to_lowercase()
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
    fn test_rust_feature_name() {
        assert_eq!("2019-06", to_rust_feature_name("2019-06"));
        assert_eq!("2019_06", to_rust_feature_name("2019.06"));
        assert_eq!("2019_06", to_rust_feature_name("2019!06"));
    }

    #[test]
    fn test_rust_mod_name() {
        assert_eq!("v2019_06", to_rust_mod_name("2019-06"));
        assert_eq!("v2018_10_01_disks", to_rust_mod_name("2018-10-01-Disks"));
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
        let config = parse_configuration(input)?;
        let tags = &config.tags;
        assert_eq!(1, tags.len());
        assert_eq!("package-2019-06", tags[0].tag);
        assert_eq!(5, tags[0].input_files.len());
        assert_eq!("Microsoft.Storage/stable/2019-06-01/storage.json", tags[0].input_files[0]);
        assert_eq!("Microsoft.Storage/stable/2019-06-01/blob.json", tags[0].input_files[1]);
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
        let config = parse_configuration(input)?;
        let tags = &config.tags;
        assert_eq!(2, tags.len());
        assert_eq!("package-2019-06", tags[0].tag);
        assert_eq!(5, tags[0].input_files.len());
        assert_eq!("Microsoft.Storage/stable/2019-06-01/storage.json", tags[0].input_files[0]);
        assert_eq!("Microsoft.Storage/stable/2019-06-01/blob.json", tags[0].input_files[1]);

        assert_eq!("package-2015-05-preview", tags[1].tag);
        assert_eq!(1, tags[1].input_files.len());
        assert_eq!("Microsoft.Storage/preview/2015-05-01-preview/storage.json", tags[1].input_files[0]);
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
        assert!(parse_configuration(invalid_input).is_err());
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
        assert!(parse_configuration(invalid_input)?.tags.is_empty());
        Ok(())
    }

    #[test]
    fn literate_config_should_ignore_code_blocks_after_configuration_heading() -> Result<()> {
        let input = "
## Configuration

### Basic Information
``` yaml
title: Recovery Services Backup Client
description: Open API 2.0 Specs for Azure RecoveryServices Backup service
openapi-type: arm
tag: package-2021-12
```

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
        let config = parse_configuration(input)?;
        assert_eq!(1, config.tags.len());
        assert_eq!("package-2019-06", config.tags[0].tag);
        Ok(())
    }

    #[test]
    fn test_basic_info() -> Result<()> {
        let input = "
## Configuration

### Basic Information
``` yaml
title: Recovery Services Backup Client
description: Open API 2.0 Specs for Azure RecoveryServices Backup service
openapi-type: arm
tag: package-2021-12
```
";
        let config = parse_configuration(input)?;
        assert_eq!(Some("Recovery Services Backup Client"), config.title());
        assert_eq!(
            Some("Open API 2.0 Specs for Azure RecoveryServices Backup service"),
            config.description()
        );
        assert_eq!(Some("arm"), config.openapi_type());
        assert_eq!(Some("package-2021-12"), config.tag());
        Ok(())
    }
}
