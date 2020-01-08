use crate::Collection;
use crate::Document;
use crate::Resource;
use crate::User;

/// This trait must be implemented by
/// every resource that can have permissions
/// associated with it. For example, you can
/// grant read access to a collection but not to a
/// database. So database does not implement
/// PermissionResource but collection does.
pub trait PermissionResource: Resource {}

impl PermissionResource for String {}

impl PermissionResource for &str {}

impl<'a> PermissionResource for std::borrow::Cow<'a, str> {}

impl<T> PermissionResource for Document<T> {}

impl PermissionResource for Collection {}

impl PermissionResource for User {}
