// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::fmt;

use syn::{Ident, Path};

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

pub const RENAME: Symbol = Symbol("rename");
pub const RENAME_ALL: Symbol = Symbol("rename_all");
pub const VARIANT: Symbol = Symbol("variant");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, other: &Symbol) -> bool {
        *self == other.0
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, other: &Symbol) -> bool {
        *self == other.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, other: &Symbol) -> bool {
        self.is_ident(other.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, other: &Symbol) -> bool {
        self.is_ident(other.0)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
