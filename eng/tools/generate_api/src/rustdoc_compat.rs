// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use rustdoc_types::{Attribute, AttributeRepr, Item, ItemEnum, ReprKind};
use std::{fs, path::Path};

pub(crate) fn attribute_texts(item: &Item) -> Vec<String> {
    let mut attributes = source_attributes(item);

    for attribute in &item.attrs {
        let Some(text) = render_attribute(attribute) else {
            continue;
        };
        if !attributes
            .iter()
            .any(|candidate| same_attribute(candidate, &text))
        {
            attributes.push(text);
        }
    }

    // Format 45 placed non_exhaustive before repr regardless of source order.
    attributes.sort_by_key(|attribute| match attribute_name(attribute) {
        "cfg" | "must_use" | "non_exhaustive" => 0,
        "repr" => 2,
        _ => 1,
    });
    attributes
}

pub(crate) fn is_automatically_derived(item: &Item) -> bool {
    item.attrs
        .iter()
        .any(|attribute| matches!(attribute, Attribute::AutomaticallyDerived))
}

fn render_attribute(attribute: &Attribute) -> Option<String> {
    match attribute {
        Attribute::NonExhaustive => Some("#[non_exhaustive]".to_string()),
        Attribute::MustUse { reason: None } => Some("#[must_use]".to_string()),
        Attribute::MustUse {
            reason: Some(reason),
        } => Some(format!("#[must_use = {reason:?}]")),
        Attribute::MacroExport => Some("#[macro_export]".to_string()),
        Attribute::ExportName(name) => Some(format!("#[export_name = {name:?}]")),
        Attribute::LinkSection(name) => Some(format!("#[link_section = {name:?}]")),
        Attribute::AutomaticallyDerived => Some("#[automatically_derived]".to_string()),
        Attribute::Repr(repr) => Some(render_repr(repr)),
        Attribute::NoMangle => Some("#[no_mangle]".to_string()),
        Attribute::TargetFeature { enable } => Some(format!(
            "#[target_feature({})]",
            enable
                .iter()
                .map(|feature| format!("enable = {feature:?}"))
                .collect::<Vec<_>>()
                .join(", ")
        )),
        Attribute::Other(text) => render_other_attribute(text),
    }
}

fn render_repr(repr: &AttributeRepr) -> String {
    let mut parts = Vec::new();
    match repr.kind {
        ReprKind::Rust => {}
        ReprKind::C => parts.push("C".to_string()),
        ReprKind::Transparent => parts.push("transparent".to_string()),
        ReprKind::Simd => parts.push("simd".to_string()),
    }
    if let Some(align) = repr.align {
        parts.push(format!("align({align})"));
    }
    if let Some(packed) = repr.packed {
        parts.push(format!("packed({packed})"));
    }
    if let Some(int) = &repr.int {
        parts.push(int.clone());
    }

    format!("#[repr({})]", parts.join(", "))
}

fn render_other_attribute(text: &str) -> Option<String> {
    match text {
        "#[attr = Inline(Hint)]" => Some("#[inline]".to_string()),
        "#[attr = Inline(Always)]" => Some("#[inline(always)]".to_string()),
        "#[attr = Inline(Never)]" => Some("#[inline(never)]".to_string()),
        "#[attr = Optimize(Size)]" => Some("#[optimize(size)]".to_string()),
        "#[attr = Optimize(Speed)]" => Some("#[optimize(speed)]".to_string()),
        "#[attr = Cold]" => Some("#[cold]".to_string()),
        "#[attr = TrackCaller]" => Some("#[track_caller]".to_string()),
        "#[attr = CfgAttrTrace]" => None,
        _ if text.starts_with("#[attr = CfgTrace(") => render_cfg_trace(text),
        _ if text.starts_with("#[attr = ProcMacro") => None,
        _ if text.starts_with("#[attr = RustcDiagnosticItem(") => {
            let value = text
                .strip_prefix("#[attr = RustcDiagnosticItem(")?
                .strip_suffix(")]")?;
            Some(format!("#[rustc_diagnostic_item = {value}]"))
        }
        _ => Some(text.to_string()),
    }
}

fn source_attributes(item: &Item) -> Vec<String> {
    let Some(span) = &item.span else {
        return Vec::new();
    };
    let Ok(source) = fs::read_to_string(Path::new(&span.filename)) else {
        return Vec::new();
    };

    if matches!(&item.inner, ItemEnum::Module(module) if module.is_crate) {
        return source_crate_attributes(&source, span.begin.0.saturating_sub(1));
    }

    let mut attributes = source_attributes_before(&source, span.begin.0);
    if let Some((filename, line)) = cfg_trace_source(item) {
        if let Ok(source) = fs::read_to_string(filename) {
            let source_attributes = item
                .name
                .as_deref()
                .and_then(|name| source_attributes_near(&source, line, name))
                .unwrap_or_else(|| source_attributes_before(&source, line));
            for attribute in source_attributes {
                if !attributes.iter().any(|candidate| candidate == &attribute) {
                    attributes.push(attribute);
                }
            }
            if let Some(attribute) = source
                .lines()
                .skip(line)
                .take(10)
                .map(str::trim)
                .find(|line| line.starts_with("#[cfg_attr"))
            {
                let attribute = attribute.to_string();
                if !attributes.iter().any(|candidate| candidate == &attribute) {
                    attributes.push(attribute);
                }
            }
        }
    }
    attributes
}

fn source_attributes_near(source: &str, start_line: usize, item_name: &str) -> Option<Vec<String>> {
    source
        .lines()
        .enumerate()
        .skip(start_line)
        .take(20)
        .find(|(_, line)| {
            let trimmed = line.trim();
            !trimmed.starts_with('#') && trimmed.contains(item_name)
        })
        .map(|(index, _)| source_attributes_before(source, index + 1))
}

fn source_attributes_before(source: &str, line: usize) -> Vec<String> {
    let mut lines = source
        .lines()
        .take(line.saturating_sub(1))
        .collect::<Vec<_>>();
    let mut attributes = Vec::new();
    let mut pending = Vec::new();

    while let Some(line) = lines.pop() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("///") || trimmed.starts_with("//!") {
            continue;
        }

        if trimmed.starts_with("#[") || trimmed.starts_with("#![") {
            let mut attribute = trimmed.to_string();
            if !pending.is_empty() {
                attribute.push(' ');
                attribute.push_str(&pending.into_iter().rev().collect::<Vec<_>>().join(" "));
                pending = Vec::new();
            }
            if should_recover_source_attribute(&attribute) {
                attributes.push(attribute);
            }
            continue;
        }

        if !pending.is_empty() || trimmed.ends_with(']') {
            pending.push(trimmed);
            continue;
        }

        break;
    }

    attributes.reverse();
    attributes
}

fn cfg_trace_source(item: &Item) -> Option<(&str, usize)> {
    item.attrs.iter().find_map(|attribute| {
        let Attribute::Other(text) = attribute else {
            return None;
        };
        let text = text.strip_prefix("#[attr = CfgTrace(")?;
        let span = text.split("span: ").nth(1)?;
        let parts = span.rsplitn(5, ':').collect::<Vec<_>>();
        let filename = *parts.get(4)?;
        let line = parts.get(3)?.parse().ok()?;
        Some((filename, line))
    })
}

fn source_crate_attributes(source: &str, start_line: usize) -> Vec<String> {
    let mut attributes = Vec::new();
    let mut pending = String::new();

    for line in source.lines().skip(start_line) {
        let trimmed = line.trim();
        if pending.is_empty() && !(trimmed.starts_with("#![") || trimmed.starts_with("#[")) {
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }
            break;
        }

        if !pending.is_empty() {
            pending.push(' ');
        }
        pending.push_str(trimmed);
        if trimmed.ends_with(']') {
            if should_recover_crate_attribute(&pending) {
                attributes.push(std::mem::take(&mut pending));
            } else {
                pending.clear();
            }
        }
    }

    attributes
}

fn render_cfg_trace(text: &str) -> Option<String> {
    let mut predicates = Vec::new();
    let mut remaining = text;
    while let Some(index) = remaining.find("NameValue { name: \"") {
        remaining = &remaining[index + "NameValue { name: \"".len()..];
        let name_end = remaining.find('"')?;
        let name = &remaining[..name_end];
        remaining = &remaining[name_end + 1..];
        let value_start = remaining.find("value: Some(\"")? + "value: Some(\"".len();
        remaining = &remaining[value_start..];
        let value_end = remaining.find('"')?;
        let value = &remaining[..value_end];
        predicates.push(format!("{name} = {value:?}"));
        remaining = &remaining[value_end + 1..];
    }

    match predicates.len() {
        0 => None,
        1 => Some(format!("#[cfg({})]", predicates[0])),
        _ => {
            let operator = if text.contains("CfgTrace([Any(") {
                "any"
            } else {
                "all"
            };
            Some(format!("#[cfg({operator}({}))]", predicates.join(", ")))
        }
    }
}

fn should_recover_source_attribute(attribute: &str) -> bool {
    matches!(
        attribute_name(attribute),
        "cfg_attr"
            | "default"
            | "pin"
            | "pin_project"
            | "proc_macro_attribute"
            | "proc_macro_derive"
            | "serde"
    )
}

fn should_recover_crate_attribute(attribute: &str) -> bool {
    !matches!(
        attribute_name(attribute),
        "doc" | "derive" | "deprecated" | "macro_use"
    )
}

fn attribute_name(attribute: &str) -> &str {
    attribute
        .strip_prefix("#![")
        .or_else(|| attribute.strip_prefix("#["))
        .unwrap_or(attribute)
        .split(['(', '=', ']', ' ', ':'])
        .next()
        .unwrap_or_default()
}

fn same_attribute(left: &str, right: &str) -> bool {
    attribute_body(left) == attribute_body(right)
}

fn attribute_body(attribute: &str) -> &str {
    attribute
        .strip_prefix("#![")
        .or_else(|| attribute.strip_prefix("#["))
        .unwrap_or(attribute)
}

#[cfg(test)]
mod tests {
    use super::{
        render_cfg_trace, render_other_attribute, render_repr, source_attributes_before,
        source_crate_attributes,
    };
    use rustdoc_types::{AttributeRepr, ReprKind};

    #[test]
    fn renders_internal_attributes_like_format_45() {
        assert_eq!(
            render_other_attribute("#[attr = Inline(Hint)]").as_deref(),
            Some("#[inline]")
        );
        assert_eq!(
            render_other_attribute("#[attr = TrackCaller]").as_deref(),
            Some("#[track_caller]")
        );
        assert_eq!(render_other_attribute("#[attr = CfgAttrTrace]"), None);
    }

    #[test]
    fn renders_repr_like_format_45() {
        assert_eq!(
            render_repr(&AttributeRepr {
                kind: ReprKind::Rust,
                align: None,
                packed: None,
                int: Some("u16".to_string()),
            }),
            "#[repr(u16)]"
        );
    }

    #[test]
    fn renders_cfg_trace_like_format_45() {
        assert_eq!(
            render_cfg_trace(
                "#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"http\"), span: src/lib.rs:1:1: 1:1 (#0) }])]"
            )
            .as_deref(),
            Some("#[cfg(feature = \"http\")]")
        );
        assert_eq!(
            render_cfg_trace(
                "#[attr = CfgTrace([Any([NameValue { name: \"feature\", value: Some(\"json\"), span: src/lib.rs:1:1: 1:1 (#0) }, NameValue { name: \"feature\", value: Some(\"xml\"), span: src/lib.rs:1:1: 1:1 (#0) }], src/lib.rs:1:1: 1:1 (#0))])]"
            )
            .as_deref(),
            Some("#[cfg(any(feature = \"json\", feature = \"xml\"))]")
        );
    }

    #[test]
    fn recovers_attributes_before_declarations() {
        let source = r#"
#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
pub mod tokio;
"#;
        assert_eq!(
            source_attributes_before(source, 4),
            vec!["#[cfg_attr(docsrs, doc(cfg(feature = \"tokio\")))]"]
        );
    }

    #[test]
    fn recovers_crate_attributes_in_source_order() {
        let source = r#"
#![deny(unsafe_code)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

pub mod example;
"#;
        assert_eq!(
            source_crate_attributes(source, 1),
            vec![
                "#![deny(unsafe_code)]",
                "#![cfg_attr(docsrs, feature(doc_cfg))]",
                "#![warn(missing_docs)]",
            ]
        );
    }
}
