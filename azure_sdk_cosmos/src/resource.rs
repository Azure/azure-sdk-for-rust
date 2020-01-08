use crate::Collection;
use crate::Database;
use crate::Document;
use crate::User;

pub trait Resource {
    fn uri(&self) -> &str;
}

impl Resource for String {
    fn uri(&self) -> &str {
        &self
    }
}

impl Resource for &str {
    fn uri(&self) -> &str {
        self
    }
}

impl<'a> Resource for std::borrow::Cow<'a, str> {
    fn uri(&self) -> &str {
        &self
    }
}

impl<T> Resource for Document<T> {
    fn uri(&self) -> &str {
        self.document_attributes._self()
    }
}

impl Resource for Database {
    fn uri(&self) -> &str {
        &self._self
    }
}

impl Resource for Collection {
    fn uri(&self) -> &str {
        &self._self
    }
}

impl Resource for User {
    fn uri(&self) -> &str {
        &self._self
    }
}

// TODO: Missing StoredProcedure, Attachment, UDF, Trigger, Permission
