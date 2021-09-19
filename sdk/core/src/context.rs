use std::any::{Any, TypeId};
use std::collections::HashMap;
#[cfg(feature = "mock_transport_framework")]
use std::path::PathBuf;
use std::sync::Arc;

/// Pipeline execution context.
///
/// During a pipeline execution, context will be passed from the function starting the
/// pipeline down to each pipeline policy. Contrarily to the Request, the context can be mutated
/// by each pipeline policy and is not reset between retries. It can be used to pass the whole
/// pipeline execution history between policies.
/// For example, it could be used to signal that an execution failed because a CosmosDB endpoint is
/// down and the appropriate policy should try the next one).
///
/// The context can be passed between different calls to Azure as well: this allows to "give context" to them.
/// For example, the mock testing framework uses the context this way to correlate subsequent calls
/// of a single execution.
///
/// The `Context` is a strongly typed property bag. You can add and retrieve an instance of any
/// type (that implement `Any`, `Send` and `Sync`) from it, just call `set_property()` and `get_property()`. While
/// easy to use and type safe, this property bag supports only **one** instance for each type. If
/// you need more than one instance for a specific type, consider wrapping it in a custom type.
/// Also keep in mind that value mutation is not allowed: only swapping (by replacing) is supported.
#[derive(Clone, Debug, Default)]
pub struct Context {
    property_bag: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn property_bag(&self) -> &HashMap<TypeId, Arc<dyn Any + Send + Sync>> {
        &self.property_bag
    }

    pub fn get_property<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.property_bag.get(&TypeId::of::<T>()).map(|e| {
            e.downcast_ref::<T>().unwrap_or_else(|| {
                panic!(
                    "**BUG**: item in property bag not of expected type {:?}",
                    TypeId::of::<T>()
                )
            })
        })
    }

    pub fn set_property<T: Any + Send + Sync>(&mut self, t: T) {
        self.property_bag.insert(TypeId::of::<T>(), Arc::new(t));
    }
}

#[cfg(not(feature = "mock_transport_framework"))]
impl Context {
    pub fn start_mock_transaction(&mut self, _name: impl Into<String>) {}
}

#[cfg(feature = "mock_transport_framework")]
impl Context {
    pub fn start_mock_transaction(&mut self, name: impl Into<String>) {
        self.set_property(crate::MockTransaction::new(name));
    }

    pub(crate) fn get_mock_transaction(
        &self,
    ) -> Result<&crate::MockTransaction, crate::MockFrameworkError> {
        self.get_property::<crate::MockTransaction>()
            .ok_or(crate::MockFrameworkError::UninitializedTransaction)
    }

    pub(crate) fn increment_mock_transaction(&mut self) -> Result<(), crate::MockFrameworkError> {
        let current_transaction = self.get_mock_transaction()?;
        let new_transaction = crate::MockTransaction {
            name: current_transaction.name().to_owned(),
            number: current_transaction.number() + 1,
        };

        self.set_property(new_transaction);

        Ok(())
    }

    pub(crate) fn prepare_and_get_transaction_path(
        &self,
    ) -> Result<PathBuf, crate::MockFrameworkError> {
        let path: PathBuf =
            PathBuf::from("SessionRecords").join(self.get_mock_transaction()?.name());

        if !path.exists() {
            std::fs::create_dir(&path).map_err(|e| {
                crate::MockFrameworkError::IOError("cannot create transaction folder", e)
            })?;
        }

        Ok(path)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_types() {
        let mut c = Context::new();

        c.set_property(100u32);
        c.set_property(String::from("string"));
        c.set_property("static str");

        assert_eq!(Some(&100u32), c.get_property());
        assert_eq!(Some(&String::from("string")), c.get_property());
        assert_eq!(Some(&"static str"), c.get_property());
    }

    #[test]
    fn complex_type() {
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct Foo {
            foo: u32,
            bar: String,
        }

        let foo = Foo {
            foo: 100,
            bar: "bar".into(),
        };

        let mut c = Context::new();

        c.set_property(foo.clone());

        assert_eq!(Some(&foo), c.get_property());
    }
}
