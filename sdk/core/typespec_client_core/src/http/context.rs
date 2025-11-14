// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::any::{Any, TypeId};
use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::sync::Arc;

/// Pipeline execution context.
///
/// Do not store Personally-Identifiable Information (PII) in a `Context`.
/// It could easily leak in logs or traces.
#[derive(Clone, Debug)]
pub struct Context<'a> {
    type_map: Cow<'a, HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl<'a> Context<'a> {
    /// Creates a new, empty `Context`.
    pub fn new() -> Self {
        Self {
            type_map: Cow::Owned(HashMap::new()),
        }
    }

    /// Returns a new `Context` that borrows the type map of the given `context`.
    ///
    /// Once you [`Context::insert`] entities the type map is copied.
    #[deprecated(since = "0.7.0", note = "use to_borrowed() instead")]
    #[must_use]
    pub fn with_context<'b>(context: &'a Context) -> Context<'b>
    where
        'a: 'b,
    {
        let type_map = context.type_map.borrow();
        Self {
            type_map: Cow::Borrowed(type_map),
        }
    }

    /// Inserts or replaces an entity in the type map and returns `Self` to allow chaining.
    ///
    /// ## Examples
    ///
    /// ```
    /// use typespec_client_core::http::Context;
    ///
    /// let context = Context::new()
    ///     .with_value(1)
    ///     .with_value("test");
    /// assert_eq!(context.value(), Some(&"test"));
    /// ```
    #[must_use]
    pub fn with_value<E>(mut self, entity: E) -> Self
    where
        E: Send + Sync + 'static,
    {
        let type_map = self.type_map.to_mut();
        type_map.insert(TypeId::of::<E>(), Arc::new(entity));

        self
    }

    /// Inserts or replaces an entity in the type map. If an entity with the same type was displaced
    /// by the insert, it will be returned to the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// use typespec_client_core::http::Context;
    /// use std::sync::Arc;
    ///
    /// let mut context = Context::new().with_value("a".to_string());
    /// assert_eq!(context.insert("b".to_string()), Some(Arc::new("a".to_string())));
    /// assert_eq!(context.value(), Some(&"b".to_string()));
    /// ```
    pub fn insert<E>(&mut self, entity: E) -> Option<Arc<E>>
    where
        E: Send + Sync + 'static,
    {
        // We make sure that for every TypeId of E as key we ALWAYS retrieve an Option<Arc<E>>.
        // That's why the `expect` below is safe.
        let type_map = self.type_map.to_mut();
        type_map
            .insert(TypeId::of::<E>(), Arc::new(entity))
            .map(|displaced| displaced.downcast().expect("failed to unwrap downcast"))
    }

    /// Returns a reference of the entity of the specified type signature, if it exists.
    ///
    /// If there is no entity with the specific type signature, `None` is returned instead.
    pub fn value<E>(&self) -> Option<&E>
    where
        E: Send + Sync + 'static,
    {
        self.type_map
            .get(&TypeId::of::<E>())
            .and_then(|item| item.downcast_ref())
    }

    /// Returns `true` if the type map is empty; otherwise, `false`.
    pub fn is_empty(&self) -> bool {
        self.type_map.is_empty()
    }

    /// Returns the number of entities in the type map.
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.type_map.len()
    }

    /// Transforms this [`Context`] into a new [`Context`] that owns the underlying data, cloning it if necessary.
    ///
    /// If this [`Context`] already owns the underlying data, that data is moved into the new [`Context`] as-is.
    /// If this [`Context`] is borrowing it's underlying data, that data is cloned and returned as a new [`Context`].
    pub fn into_owned(self) -> Context<'static> {
        let type_map = match self.type_map {
            Cow::Owned(o) => o,
            Cow::Borrowed(o) => o.clone(),
        };
        Context {
            type_map: Cow::Owned(type_map),
        }
    }

    /// Transforms this [`Context`] into a new [`Context`] that owns the underlying data, cloning it if necessary.
    ///
    /// Clone the underlying data in the [`Context`] and return it in a new owned [`Context`].
    #[must_use]
    pub fn to_owned(&self) -> Context<'static> {
        let type_map = match &self.type_map {
            Cow::Owned(o) => o.clone(),
            Cow::Borrowed(o) => (*o).clone(),
        };
        Context {
            type_map: Cow::Owned(type_map),
        }
    }

    /// Returns a new `Context` that borrows the type map of the given `context`.
    ///
    /// Once you [`Context::insert`] entities the type map is copied.
    #[must_use]
    pub fn to_borrowed<'b>(&'a self) -> Context<'b>
    where
        'a: 'b,
    {
        let type_map = self.type_map.as_ref();
        Context {
            type_map: Cow::Borrowed(type_map),
        }
    }
}

impl Default for Context<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn insert_get_string() {
        let mut context = Context::new();
        context.insert("test".to_string());
        assert_eq!(Some(&"test".to_string()), context.value());
    }

    #[test]
    fn insert_get_custom_structs() {
        #[derive(Debug, PartialEq, Eq)]
        struct S1 {}
        #[derive(Debug, PartialEq, Eq)]
        struct S2 {}

        let mut context = Context::new();
        context.insert(S1 {});
        context.insert(S2 {});

        assert_eq!(Some(Arc::new(S1 {})), context.insert(S1 {}));
        assert_eq!(Some(Arc::new(S2 {})), context.insert(S2 {}));

        assert_eq!(Some(&S1 {}), context.value());
        assert_eq!(Some(&S2 {}), context.value());
    }

    #[test]
    fn insert_fluent_syntax() {
        #[derive(Debug, PartialEq, Eq, Default)]
        struct S1 {}
        #[derive(Debug, PartialEq, Eq, Default)]
        struct S2 {}

        let context = Context::new()
            .with_value("static str")
            .with_value("a String".to_string())
            .with_value(S1::default())
            .with_value(S1::default()) // notice we are REPLACING S1. This call will *not* increment the counter
            .with_value(S2::default());

        assert_eq!(4, context.len());
        assert_eq!(Some(&"static str"), context.value());
    }

    fn require_send_sync<T: Send + Sync>(_: &T) {}

    #[test]
    fn test_require_send_sync() {
        // this won't compile if Context as a whole is not Send + Sync
        require_send_sync(&Context::new());
    }

    #[test]
    fn mutability() {
        #[derive(Debug, PartialEq, Eq, Default)]
        struct S1 {
            num: u8,
        }
        let mut context = Context::new();
        context.insert(Mutex::new(S1::default()));

        // the stored value is 0.
        assert_eq!(0, context.value::<Mutex<S1>>().unwrap().lock().unwrap().num);

        // we change the number to 42 in a thread safe manner.
        context.value::<Mutex<S1>>().unwrap().lock().unwrap().num = 42;

        // now the number is 42.
        assert_eq!(
            42,
            context.value::<Mutex<S1>>().unwrap().lock().unwrap().num
        );

        // we replace the struct with a new one.
        let displaced = context.insert(Mutex::new(S1::default())).unwrap();

        // the displaced struct still holds 42 as number
        assert_eq!(42, displaced.lock().unwrap().num);

        // the new struct has 0 has number.
        assert_eq!(0, context.value::<Mutex<S1>>().unwrap().lock().unwrap().num);

        context.insert(Mutex::new(33u32));
        *context.value::<Mutex<u32>>().unwrap().lock().unwrap() = 42;
        assert_eq!(42, *context.value::<Mutex<u32>>().unwrap().lock().unwrap());
    }

    #[test]
    fn with_context_borrows() {
        let a = Context::new().with_value("a".to_string());
        let mut b = a.to_borrowed();

        // TODO: Use is_owned(), is_borrowed() once stabilized.
        let a_ptr = std::ptr::addr_of!(*a.type_map);
        let b_ptr = std::ptr::addr_of!(*b.type_map);
        assert_eq!(a_ptr, b_ptr);

        b.insert("b".to_string());
        let a_ptr = std::ptr::addr_of!(*a.type_map);
        let b_ptr = std::ptr::addr_of!(*b.type_map);
        assert_ne!(a_ptr, b_ptr);
    }
}
