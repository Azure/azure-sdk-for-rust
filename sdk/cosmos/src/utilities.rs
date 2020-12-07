/// Creates setter methods
///
/// The methods created are of the form `with_$name` that takes an argument of type `$typ`
/// and sets the field $name to result of calling `$transform` with the value of the argument.
///
/// In other words. The following macro call:
/// ```
/// setters! { foo: &str => Some, }
/// ```
/// Roughly expands to:
/// ```
/// fn with_foo(&self, foo: &str) -> Self {
///     Self {
///         foo: Some(foo),
///         ...self
///     }
/// }
/// ```
macro_rules! setters {
    (@single $name:ident : $typ:ty => $transform:expr) => {
        paste::paste! {
            pub fn [<with_ $name>](self, $name: $typ) -> Self {
                Self  {
                    $name: $transform,
                    ..self
                }
            }
        }
    };
    // Terminal condition
    (@recurse) => {};
    // Recurse without transform
    (@recurse $name:ident : $typ:ty, $($tokens:tt)*) => {
        setters! { @recurse $name: $typ => $name, $($tokens)* }
    };
    // Recurse with transform
    (@recurse $name:ident : $typ:ty => $transform:expr, $($tokens:tt)*) => {
        setters! { @single $name : $typ => $transform }
        setters! { @recurse $($tokens)* }
    };
    ($($tokens:tt)*) => {
        setters! { @recurse $($tokens)* }
    }
}
