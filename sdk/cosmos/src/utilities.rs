/// Creates a setter method
///
/// The method is of the form `with_$name` that takes an argument of type `$typ`
/// and sets self.$name to `Some($name)`.
/// In other words. The following macro call:
/// ```
/// setters! { foo => &str }
/// ```
/// Expands to:
/// ```
/// fn with_foo(&self, foo: &str) -> Self {
///     Self {
///         foo: Some(foo),
///         ...self
///     }
/// }
/// ```
macro_rules! setters {
    // The terminal condition
    (@single $name:ident : $typ:ty => $transform:path $(,)* ) => {
        paste::paste! {
            pub fn [<with_ $name>](self, $name: $typ) -> Self {
                Self  {
                    $name: Some($transform($name)),
                    ..self
                }
            }
        }
    };
    // Check for last setter in list (and add identity transform)
    (@single $name:ident : $typ:ty  $(,)* ) => {
        setters! { @single $name : $typ  => ::std::convert::identity }
    };
    // Final setter in list (without transform)
    (@recurse $name:ident : $typ:ty  $(,)* ) => {
        setters! { @single $name : $typ }
    };
    // Final setter in list (with transform)
    (@recurse $name:ident : $typ:ty => $transform:path $(,)* ) => {
        setters! { @single $name : $typ => $transform }
    };
    // Recurse without transform
    (@recurse $name:ident : $typ:ty, $($tokens:tt)*) => {
        setters! { @single $name : $typ => std::convert::identity}
        setters! { @recurse $($tokens)* }
    };
    // Recurse with transform
    (@recurse $name:ident : $typ:ty => $transform:path, $($tokens:tt)*) => {
        setters! { @single $name : $typ => $transform }
        setters! { @recurse $($tokens)* }
    };
    ($($tokens:tt)*) => {
        setters! { @recurse $($tokens)* }
    }
}
