use crate::collection::Collection;
use crate::{Database, Document, PermissionMode, User};

/// A Cosmos resource such as databases, documents, collections, users, etc.
pub trait Resource {
    fn uri(&self) -> &str;

    fn read_permission(&self) -> PermissionMode<'_> {
        PermissionMode::read(self)
    }

    fn all_permission(&self) -> PermissionMode<'_> {
        PermissionMode::all(self)
    }
}

impl<T> Resource for Document<T> {
    fn uri(&self) -> &str {
        self.document_attributes._self()
    }
}

impl<T> Resource for &Document<T> {
    fn uri(&self) -> &str {
        self.document_attributes._self()
    }
}

impl Resource for Database {
    fn uri(&self) -> &str {
        &self._self
    }
}

impl Resource for &Database {
    fn uri(&self) -> &str {
        &self._self
    }
}

impl Resource for Collection {
    fn uri(&self) -> &str {
        &self._self
    }
}

impl Resource for &Collection {
    fn uri(&self) -> &str {
        &self._self
    }
}

impl Resource for User {
    fn uri(&self) -> &str {
        &self._self
    }
}

impl Resource for &User {
    fn uri(&self) -> &str {
        &self._self
    }
}

// TODO: Missing StoredProcedure, Attachment, UDF, Trigger, Permission
