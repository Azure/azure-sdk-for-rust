use crate::{OverridableContext, TypeMapContext};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

/// Execution context.
///
/// The `Context` is modeled as an overridable chain. In other words, the `Context` allows to
/// override any value without losing the original one. The resulting type map is the union of the
/// type map of every chained `Context`, giving precedence to the last inserted one. As a simple
/// example suppose to have a `Context` with the key value of `Struct1` -> `Hello` and `Struct2` ->
/// `World`. Let's call it `C1`.
/// If we chain (using the `from_previous_context` function) this `Context` we get another
/// `Context`, called `C2`.
/// Calling `get` on C2 is now the same of calling `get` on `C1`: `Struct2` gives `World`. But now
/// we can replace `Struct2` with `Solar system` without losing the original `C1`'s `World`. If we
/// do that, `C2.get::<Struct2>()` will give you `Solar system`, while `C1.get::<Struct2>()` will
/// give you `World`.
#[derive(Clone, Debug)]
pub struct Context {
    type_map: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    /// Creates a new, empty Context.
    pub fn new() -> Self {
        Self {
            type_map: HashMap::new(),
        }
    }
}

impl TypeMapContext for Context {
    /// Creates a new, empty context that wraps the previous context.
    fn create_override(&self) -> OverridableContext<'_> {
        self.into()
    }

    /// Inserts or replaces an entity in the type map. If an entity with the same type was displaced
    /// by the insert, it will be returned to the caller.
    fn insert_or_replace<E>(&mut self, entity: E) -> Option<Arc<E>>
    where
        E: Send + Sync + 'static,
    {
        // we make sure that for every TypeId of E as key we ALWAYS retrieve an Option<Arc<E>>. That's why
        // the `unwrap` below is safe.
        self.type_map
            .insert(TypeId::of::<E>(), Arc::new(entity))
            .map(|displaced| displaced.downcast().unwrap())
    }

    /// Inserts an entity in the type map. If the an entity with the same type signature is
    /// already present it will be silently dropped. This function returns a mutable reference to
    /// the same Context so it can be chained to itself.
    fn insert<E>(mut self, entity: E) -> Self
    where
        E: Send + Sync + 'static,
    {
        self.type_map.insert(TypeId::of::<E>(), Arc::new(entity));

        self
    }

    /// Removes an entity from the type map. If present, the entity will be returned.
    fn remove<E>(&mut self) -> Option<Arc<E>>
    where
        E: Send + Sync + 'static,
    {
        self.type_map
            .remove(&TypeId::of::<E>())
            .map(|removed| removed.downcast().unwrap())
    }

    /// Returns a reference of the entity of the specified type signature, if it exists. This
    /// function traverses the `Context` chain in reverse order. Basically it returns the last
    /// value found in the chain. In other words, any subsequent `Context` can *override* the
    /// stored value without removing it from the actual `Context`. The result of this call is then
    /// the union of all the entities stored in every `Context` in the chain, giving precedence to
    /// the latest entity inserted (where latest means further down the chain).
    ///
    /// If there is no entity with the specific type signature, `None` is returned instead.
    fn get<E>(&self) -> Option<&E>
    where
        E: Send + Sync + 'static,
    {
        self.type_map
            .get(&TypeId::of::<E>())
            .map(|item| item.downcast_ref())
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn insert_get_string() {
        let mut context = Context::new();
        context.insert_or_replace("pollo".to_string());
        assert_eq!(Some(&"pollo".to_string()), context.get());
    }

    #[test]
    fn insert_get_custom_structs() {
        #[derive(Debug, PartialEq, Eq)]
        struct S1 {}
        #[derive(Debug, PartialEq, Eq)]
        struct S2 {}

        let mut context = Context::new();
        context.insert_or_replace(S1 {});
        context.insert_or_replace(S2 {});

        assert_eq!(Some(Arc::new(S1 {})), context.insert_or_replace(S1 {}));
        assert_eq!(Some(Arc::new(S2 {})), context.insert_or_replace(S2 {}));

        assert_eq!(Some(&S1 {}), context.get());
        assert_eq!(Some(&S2 {}), context.get());
    }

    #[test]
    fn insert_fluent_syntax() {
        #[derive(Debug, PartialEq, Eq, Default)]
        struct S1 {}
        #[derive(Debug, PartialEq, Eq, Default)]
        struct S2 {}

        let context = Context::new()
            .insert("static str")
            .insert("a String".to_string())
            .insert(S1::default())
            .insert(S1::default()) // notice we are REPLACING S1. This call will *not* increment the counter
            .insert(S2::default());

        assert_eq!(Some(&"static str"), context.get());
    }

    fn require_send_sync<T: Send + Sync>(_: &T) {}

    #[test]
    fn test_require_send_sync() {
        // this won't compile if Context as a whole is not Send + Sync
        require_send_sync(&Context::new())
    }

    #[test]
    fn mutability() {
        #[derive(Debug, PartialEq, Eq, Default)]
        struct S1 {
            num: u8,
        }
        let mut context = Context::new();
        context.insert_or_replace(Mutex::new(S1::default()));

        // the stored value is 0.
        assert_eq!(0, context.get::<Mutex<S1>>().unwrap().lock().unwrap().num);

        // we change the number to 42 in a thread safe manner.
        context.get::<Mutex<S1>>().unwrap().lock().unwrap().num = 42;

        // now the number is 42.
        assert_eq!(42, context.get::<Mutex<S1>>().unwrap().lock().unwrap().num);

        // we replace the struct with a new one.
        let displaced = context
            .insert_or_replace(Mutex::new(S1::default()))
            .unwrap();

        // the displaced struct still holds 42 as number
        assert_eq!(42, displaced.lock().unwrap().num);

        // the new struct has 0 has number.
        assert_eq!(0, context.get::<Mutex<S1>>().unwrap().lock().unwrap().num);

        context.insert_or_replace(Mutex::new(33u32));
        *context.get::<Mutex<u32>>().unwrap().lock().unwrap() = 42;
        assert_eq!(42, *context.get::<Mutex<u32>>().unwrap().lock().unwrap());
    }
}
