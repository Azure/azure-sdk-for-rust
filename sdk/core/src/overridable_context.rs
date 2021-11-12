use crate::{Context, TypeMapContext};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
enum PreviousContext<'a> {
    Context(&'a Context),
    OverridableContext(&'a OverridableContext<'a>),
}

/// Execution context.
///
/// The `OverridableContext` is modeled as an overridable chain. In other words, the `OverridableContext` allows to
/// override any value without losing the original one. The resulting type map is the union of the
/// type map of every chained `OverridableContext`, giving precedence to the last inserted one. As a simple
/// example suppose to have a `OverridableContext` with the key value of `Struct1` -> `Hello` and `Struct2` ->
/// `World`. Let's call it `C1`.
/// If we chain (using the `from_previous_context` function) this `OverridableContext` we get another
/// `OverridableContext`, called `C2`.
/// Calling `get` on C2 is now the same of calling `get` on `C1`: `Struct2` gives `World`. But now
/// we can replace `Struct2` with `Solar system` without losing the original `C1`'s `World`. If we
/// do that, `C2.get::<Struct2>()` will give you `Solar system`, while `C1.get::<Struct2>()` will
/// give you `World`.
#[derive(Debug)]
pub struct OverridableContext<'a> {
    previous_context: PreviousContext<'a>,
    type_map: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl<'a> OverridableContext<'a> {
    /// Creates a new, empty context that wraps the previous context.
    pub(crate) fn new(context: &'a Context) -> Self {
        Self {
            type_map: HashMap::new(),
            previous_context: PreviousContext::Context(context),
        }
    }

    fn get_previous<E>(&self) -> Option<&E>
    where
        E: Send + Sync + 'static,
    {
        match &self.previous_context {
            PreviousContext::Context(context) => context.get(),
            PreviousContext::OverridableContext(overridable_context) => {
                if let Some(value) = overridable_context.get_local() {
                    *value
                } else {
                    overridable_context.get_previous()
                }
            }
        }
    }

    fn get_local<E>(&self) -> Option<&E>
    where
        E: Send + Sync + 'static,
    {
        self.type_map
            .get(&TypeId::of::<E>())
            .map(|item| item.downcast_ref())
            .flatten()
    }
}

impl<'a> TypeMapContext for OverridableContext<'a> {
    /// Creates a new, empty context that wraps the previous context.
    fn create_override(&self) -> OverridableContext<'_> {
        OverridableContext {
            type_map: HashMap::new(),
            previous_context: PreviousContext::OverridableContext(self),
        }
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
    /// the same OverridableContext so it can be chained to itself.
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
    /// function traverses the `OverridableContext` chain in reverse order. Basically it returns the last
    /// value found in the chain. In other words, any subsequent `OverridableContext` can *override* the
    /// stored value without removing it from the actual `OverridableContext`. The result of this call is then
    /// the union of all the entities stored in every `OverridableContext` in the chain, giving precedence to
    /// the latest entity inserted (where latest means further down the chain).
    ///
    /// If there is no entity with the specific type signature, `None` is returned instead.
    fn get<E>(&self) -> Option<&E>
    where
        E: Send + Sync + 'static,
    {
        self.get_local().or_else(|| self.get_previous())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overriding() {
        #[derive(Debug, PartialEq, Eq, Default)]
        struct S1 {
            num: u8,
        }

        // this is the SDK user given OverridableContext.
        let mut context = Context::new();
        context.insert_or_replace(S1 { num: 1 });

        {
            // now let's simulate one policy that wants to change a value. First it wraps the context
            // into a new one:
            let mut context = context.create_override();
            // then it inserts the new value
            context.insert_or_replace(S1 { num: 2 });

            // now the value is 2.
            assert_eq!(2, context.get::<S1>().unwrap().num);

            // here context gets dropped
        }

        // the original context still has value of 1.
        assert_eq!(1, context.get::<S1>().unwrap().num);

        {
            // we can rewrap the context into another one if necessary:
            let mut context = context.create_override();
            context.insert_or_replace(S1 { num: 42 });

            // now the value is 42.
            assert_eq!(42, context.get::<S1>().unwrap().num);
        }
    }

    #[test]
    fn overriding_two_levels() {
        let mut context = Context::new();
        context.insert_or_replace(0u8);
        assert_eq!(0, *context.get::<u8>().unwrap());

        {
            // level 1
            let mut context = context.create_override();
            context.insert_or_replace(1u8);
            assert_eq!(1, *context.get::<u8>().unwrap());

            {
                // level 2
                let mut context = context.create_override();
                context.insert_or_replace(2u8);
                assert_eq!(2, *context.get::<u8>().unwrap());
            }

            assert_eq!(1, *context.get::<u8>().unwrap());
        }

        assert_eq!(0, *context.get::<u8>().unwrap());
    }
}
