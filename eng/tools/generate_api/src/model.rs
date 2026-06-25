// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[derive(Debug, Clone)]
pub(crate) struct ApiModel {
    pub(crate) package_name: String,
    pub(crate) package_version: String,
    pub(crate) parser_version: String,
    pub(crate) root_module: ApiModule,
}

impl ApiModel {
    pub(crate) fn new(package_name: String, package_version: String) -> Self {
        let root_module = ApiModule {
            path: package_name.clone(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: Vec::new(),
            modules: Vec::new(),
        };

        Self {
            package_name,
            package_version,
            parser_version: env!("CARGO_PKG_VERSION").to_string(),
            root_module,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ApiModule {
    pub(crate) path: String,
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
    pub(crate) doc_comments: Vec<String>,
    pub(crate) attributes: Vec<ApiAttribute>,
    pub(crate) declaration: String,
    pub(crate) members: Vec<ApiMember>,
}

#[derive(Debug, Clone)]
pub(crate) struct ApiAttribute {
    pub(crate) text: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ApiMember {
    pub(crate) name: String,
    pub(crate) doc_comments: Vec<String>,
    pub(crate) attributes: Vec<ApiAttribute>,
    pub(crate) declaration: String,
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
    Union,
    TypeAlias,
    Const,
    Static,
}

impl ApiItemKind {
    pub(crate) const ORDER: [Self; 12] = [
        Self::Use,
        Self::Macro,
        Self::ProcMacro,
        Self::Function,
        Self::Struct,
        Self::Enum,
        Self::Trait,
        Self::TraitAlias,
        Self::Union,
        Self::TypeAlias,
        Self::Const,
        Self::Static,
    ];

    pub(crate) fn sort_rank(self) -> usize {
        Self::ORDER
            .iter()
            .position(|candidate| *candidate == self)
            .expect("all item kinds are present in the stable ordering")
    }
}
