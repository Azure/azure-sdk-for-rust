use crate::OverridableContext;
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
pub trait TypeMapContext {
    /// Creates a new, empty context that wraps the previous context.
    fn create_override(&self) -> OverridableContext<'_>;

    fn insert_or_replace<E>(&mut self, entity: E) -> Option<Arc<E>>
    where
        E: Send + Sync + 'static;

    fn insert<E>(self, entity: E) -> Self
    where
        E: Send + Sync + 'static;

    fn remove<E>(&mut self) -> Option<Arc<E>>
    where
        E: Send + Sync + 'static;

    fn get<E>(&self) -> Option<&E>
    where
        E: Send + Sync + 'static;
}
