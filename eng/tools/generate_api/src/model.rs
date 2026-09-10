// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::collections::BTreeMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct SourceLocation {
    pub(crate) path: String,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct ApiModel {
    pub(crate) package_name: String,
    pub(crate) package_version: String,
    pub(crate) parser_version: String,
    pub(crate) package_metadata: PackageMetadata,
    pub(crate) root_module: ApiModule,
}

impl ApiModel {
    pub(crate) fn new(
        package_name: String,
        package_version: String,
        package_metadata: PackageMetadata,
    ) -> Self {
        let root_module = ApiModule {
            path: package_name.clone(),
            declaration_location: None,
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: Vec::new(),
            modules: Vec::new(),
        };

        Self {
            package_name,
            package_version,
            parser_version: env!("CARGO_PKG_VERSION").to_string(),
            package_metadata,
            root_module,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PackageMetadata {
    pub(crate) description: Option<String>,
    pub(crate) edition: Option<String>,
    pub(crate) rust_version: Option<String>,
    pub(crate) features: BTreeMap<String, Vec<String>>,
}

impl Default for PackageMetadata {
    fn default() -> Self {
        Self {
            description: None,
            edition: None,
            rust_version: None,
            features: BTreeMap::from([("default".to_string(), Vec::new())]),
        }
    }
}

impl PackageMetadata {
    pub(crate) fn description_lines(&self) -> Option<Vec<&str>> {
        let description = self.description.as_deref()?;
        let description = description
            .strip_suffix("\r\n")
            .or_else(|| description.strip_suffix('\n'))
            .unwrap_or(description);

        Some(
            description
                .split('\n')
                .map(|line| line.strip_suffix('\r').unwrap_or(line))
                .collect(),
        )
    }

    pub(crate) fn feature_names(&self) -> impl Iterator<Item = &String> {
        self.features
            .get_key_value("default")
            .map(|(name, _)| name)
            .into_iter()
            .chain(
                self.features
                    .iter()
                    .filter(|(feature, _)| feature.as_str() != "default")
                    .map(|(name, _)| name),
            )
    }

    pub(crate) fn default_feature_children(&self) -> Vec<&String> {
        let mut children = self
            .features
            .get("default")
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        children.sort_unstable();
        children
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ApiModule {
    pub(crate) path: String,
    pub(crate) declaration_location: Option<SourceLocation>,
    pub(crate) doc_comments: Vec<String>,
    pub(crate) attributes: Vec<ApiAttribute>,
    pub(crate) items: Vec<ApiItem>,
    pub(crate) modules: Vec<ApiModule>,
}

impl ApiModule {
    pub(crate) fn local_name(&self) -> &str {
        self.path
            .rsplit_once("::")
            .map_or(self.path.as_str(), |(_, name)| name)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ApiItem {
    pub(crate) name: String,
    pub(crate) kind: ApiItemKind,
    pub(crate) declaration_location: Option<SourceLocation>,
    pub(crate) source_id: Option<String>,
    pub(crate) navigation_paths: Vec<ApiNavigationPath>,
    pub(crate) owner_name: Option<String>,
    pub(crate) owner_kind: Option<ApiItemKind>,
    pub(crate) owner_source_id: Option<String>,
    pub(crate) inherent_impl_sort_key: Option<InherentImplSortKey>,
    pub(crate) doc_comments: Vec<String>,
    pub(crate) attributes: Vec<ApiAttribute>,
    pub(crate) declaration: String,
    pub(crate) declaration_path_references: Vec<ApiPathReference>,
    pub(crate) members: Vec<ApiMember>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct InherentImplSortKey {
    pub(crate) type_arg_classes: Vec<u8>,
    pub(crate) rendered_self_type: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ApiAttribute {
    pub(crate) text: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ApiMember {
    pub(crate) name: String,
    pub(crate) kind: ApiMemberKind,
    pub(crate) declaration_location: Option<SourceLocation>,
    pub(crate) doc_comments: Vec<String>,
    pub(crate) attributes: Vec<ApiAttribute>,
    pub(crate) declaration: String,
    pub(crate) declaration_path_references: Vec<ApiPathReference>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct ApiNavigationPath {
    pub(crate) path: String,
    pub(crate) source_id: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct ApiPathReference {
    pub(crate) path: String,
    pub(crate) canonical_path: Option<String>,
    pub(crate) target_source_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ApiMemberKind {
    Associated,
    Field,
    Variant,
    MacroInput,
    Text,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ApiItemKind {
    Use,
    Macro,
    ProcMacro,
    Function,
    Struct,
    Enum,
    Trait,
    TraitAlias,
    InherentImpl,
    TraitImpl,
    Union,
    TypeAlias,
    Const,
    Static,
}

impl ApiItemKind {
    pub(crate) fn sort_rank(self, owner_kind: Option<Self>) -> usize {
        match self {
            Self::Use => 0,
            Self::Macro => 1,
            Self::ProcMacro => 2,
            Self::Function => 3,
            Self::Struct => 4,
            Self::Enum => 5,
            Self::Trait => 6,
            Self::TraitAlias => 7,
            Self::InherentImpl | Self::TraitImpl => {
                owner_kind.map_or(8, |kind| kind.sort_rank(None))
            }
            Self::Union => 9,
            Self::TypeAlias => 10,
            Self::Const => 11,
            Self::Static => 12,
        }
    }
}

impl ApiItem {
    fn sort_rank(&self) -> usize {
        self.kind.sort_rank(self.owner_kind)
    }

    fn sort_group_name(&self) -> &str {
        self.owner_name.as_deref().unwrap_or(&self.name)
    }

    fn kind_within_group(&self) -> usize {
        match self.kind {
            ApiItemKind::InherentImpl => 1,
            ApiItemKind::TraitImpl if self.owner_kind.is_some() => 2,
            _ => 0,
        }
    }

    fn inherent_impl_sort_key(&self) -> Option<&InherentImplSortKey> {
        self.inherent_impl_sort_key.as_ref()
    }
}

impl ApiModule {
    pub(crate) fn sorted_items(&self) -> Vec<&ApiItem> {
        let mut items: Vec<&ApiItem> = self.items.iter().collect();
        items.sort_by(|left, right| {
            left.sort_rank()
                .cmp(&right.sort_rank())
                .then_with(|| left.sort_group_name().cmp(right.sort_group_name()))
                .then_with(|| left.kind_within_group().cmp(&right.kind_within_group()))
                .then_with(|| match (left.kind, right.kind) {
                    (ApiItemKind::InherentImpl, ApiItemKind::InherentImpl) => left
                        .inherent_impl_sort_key()
                        .cmp(&right.inherent_impl_sort_key())
                        .then_with(|| left.declaration.cmp(&right.declaration)),
                    _ => left.name.cmp(&right.name),
                })
        });
        items
    }
}

#[cfg(test)]
mod tests;
