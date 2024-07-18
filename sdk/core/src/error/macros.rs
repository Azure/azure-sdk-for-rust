/// A convenient way to create a new error using the normal formatting infrastructure
#[macro_export]
macro_rules! format_err {
    ($kind:expr, $msg:literal $(,)?) => {{
        // Handle $:literal as a special case to make cargo-expanded code more
        // concise in the common case.
        $crate::error::Error::message($kind, $msg)
    }};
    ($kind:expr, $msg:expr $(,)?) => {{
        $crate::error::Error::message($kind, $msg)
    }};
    ($kind:expr, $msg:expr, $($arg:tt)*) => {{
        $crate::error::Error::with_message($kind, || { format!($msg, $($arg)*) })
    }};
}

/// Return early with an error if a condition is not satisfied.
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $kind:expr, $msg:literal $(,)?) => {
        if !$cond {
            return ::std::result::Result::Err($crate::format_err!($kind, $msg));
        }
    };
    ($cond:expr, $kind:expr, dicate $msg:expr $(,)?) => {
        if !$cond {
            return ::std::result::Result::Err($crate::format_err!($kind, $msg));
        }
    };
    ($cond:expr, $kind:expr, dicate $msg:expr, $($arg:tt)*) => {
        if !$cond {
            return ::std::result::Result::Err($crate::format_err!($kind, $msg, $($arg)*));
        }
    };
}

/// Return early with an error if two expressions are not equal to each other.
#[macro_export]
macro_rules! ensure_eq {
    ($left:expr, $right:expr, $kind:expr, $msg:literal $(,)?) => {
        $crate::ensure!($left == $right, $kind, $msg);
    };
    ($left:expr, $right:expr, $kind:expr, dicate $msg:expr $(,)?) => {
        $crate::ensure!($left == $right, $kind, $msg);
    };
    ($left:expr, $right:expr, $kind:expr, dicate $msg:expr, $($arg:tt)*) => {
        $crate::ensure!($left == $right, $kind, $msg, $($arg)*);
    };
}

/// Return early with an error if two expressions are equal to each other.
#[macro_export]
macro_rules! ensure_ne {
    ($left:expr, $right:expr, $kind:expr, $msg:literal $(,)?) => {
        $crate::ensure!($left != $right, $kind, $msg);
    };
    ($left:expr, $right:expr, $kind:expr, dicate $msg:expr $(,)?) => {
        $crate::ensure!($left != $right, $kind, $msg);
    };
    ($left:expr, $right:expr, $kind:expr, dicate $msg:expr, $($arg:tt)*) => {
        $crate::ensure!($left != $right, $kind, $msg, $($arg)*);
    };
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[allow(dead_code)]
    #[derive(Debug, PartialEq, Copy, Clone)]
    struct OperationError;

    impl Display for OperationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "OperationError")
        }
    }

    #[test]
    fn ensure_works() {
        fn test_ensure(predicate: bool) -> crate::Result<()> {
            ensure!(predicate, ErrorKind::Other, "predicate failed");
            Ok(())
        }

        fn test_ensure_eq(item1: &str, item2: &str) -> crate::Result<()> {
            ensure_eq!(item1, item2, ErrorKind::Other, "predicate failed");
            Ok(())
        }

        fn test_ensure_ne(item1: &str, item2: &str) -> crate::Result<()> {
            ensure_ne!(item1, item2, ErrorKind::Other, "predicate failed");
            Ok(())
        }

        let err = test_ensure(false).unwrap_err();
        assert_eq!(format!("{err}"), "predicate failed");
        assert_eq!(err.kind(), &ErrorKind::Other);

        assert!(test_ensure(true).is_ok());

        let err = test_ensure_eq("foo", "bar").unwrap_err();
        assert_eq!(format!("{err}"), "predicate failed");
        assert_eq!(err.kind(), &ErrorKind::Other);

        assert!(test_ensure_eq("foo", "foo").is_ok());

        let err = test_ensure_ne("foo", "foo").unwrap_err();
        assert_eq!(format!("{err}"), "predicate failed");
        assert_eq!(err.kind(), &ErrorKind::Other);

        assert!(test_ensure_ne("foo", "bar").is_ok());
    }
}
