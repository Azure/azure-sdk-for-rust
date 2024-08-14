// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Creates setter methods
///
/// The methods created are of the form `$name` that takes an argument of type `$typ`
/// and sets the field `$name` to result of calling `$transform` with the value of the argument.
///
/// In other words. The following macro call:
/// ```
/// # #[macro_use] extern crate typespec_client_core;
/// struct MyStruct<'a> { foo: Option<&'a str> };
/// impl <'a> MyStruct<'a> {
///     setters! { foo: &'a str => Some(foo), }
/// }
/// ```
/// Roughly expands to:
/// ```
/// struct MyStruct<'a> { foo: Option<&'a str> };
/// impl <'a> MyStruct<'a> {
///     fn foo(self, foo: &'a str) -> Self {
///         Self {
///             foo: Some(foo),
///             ..self
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! setters {
    (@single $(#[$meta:meta])* $name:ident : $typ:ty => $transform:expr) => {
        #[allow(clippy::redundant_field_names)]
        #[allow(clippy::needless_update)]
        #[allow(missing_docs)]
        #[must_use]
        $(#[$meta])*
        pub fn $name<P: ::std::convert::Into<$typ>>(self, $name: P) -> Self {
            let $name: $typ = $name.into();
            Self  {
                $name: $transform,
                ..self
            }
        }
    };
    // Terminal condition
    (@recurse) => {};
    // Recurse without transform
    (@recurse $(#[$meta:meta])* $name:ident : $typ:ty, $($tokens:tt)*) => {
        $crate::setters! { @recurse $(#[$meta])* $name: $typ => $name, $($tokens)* }
    };
    // Recurse with transform
    (@recurse $(#[$meta:meta])* $name:ident : $typ:ty => $transform:expr, $($tokens:tt)*) => {
        $crate::setters! { @single $(#[$meta])* $name : $typ => $transform }
        $crate::setters! { @recurse $($tokens)* }
    };
    ($($tokens:tt)*) => {
        $crate::setters! { @recurse $($tokens)* }
    }
}

#[cfg(test)]
mod test {
    struct Options {
        a: Option<String>,
        b: u32,
    }

    #[allow(dead_code)]
    impl Options {
        setters! {
            a: String => Some(a),
            b: u32 => b,
        }
    }

    impl Default for Options {
        fn default() -> Self {
            Options { a: None, b: 1 }
        }
    }

    #[test]
    fn test_setters() {
        let options = Options::default().a("test".to_owned());

        assert_eq!(Some("test".to_owned()), options.a);
        assert_eq!(1, options.b);
    }
}
