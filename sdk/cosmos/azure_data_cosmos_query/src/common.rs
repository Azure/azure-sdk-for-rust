// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Common utilities shared across query modules.

use crate::ast::*;

/// Extract the root alias from a query's FROM clause.
///
/// For `FROM c` or `FROM root AS c`, this returns `Some("c")`.
/// For queries without a FROM clause, returns `None`.
pub(crate) fn get_root_alias(query: &SqlQuery) -> Option<String> {
    match &query.from {
        Some(from) => get_alias_from_collection(&from.collection),
        None => None,
    }
}

fn get_alias_from_collection(coll: &SqlCollectionExpression) -> Option<String> {
    match coll {
        SqlCollectionExpression::Aliased { collection, alias } => {
            alias.clone().or_else(|| match collection {
                SqlCollection::Path { root, .. } => Some(root.clone()),
                _ => None,
            })
        }
        SqlCollectionExpression::Join { left, .. } => get_alias_from_collection(left),
        SqlCollectionExpression::ArrayIterator { .. } => None,
    }
}
