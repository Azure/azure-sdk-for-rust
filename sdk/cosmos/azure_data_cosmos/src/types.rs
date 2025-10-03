//! Internal module to define several newtypes used in the SDK.

macro_rules! string_newtype {
    ($(#[$attr:meta])* $name:ident) => {
        $(#[$attr])*
        #[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Creates a new `", stringify!($name), "` from a `String`.")]
            pub fn new(value: String) -> Self {
                Self(value)
            }

            #[doc = concat!("Returns a reference to the inner `str` of the `", stringify!($name), "`.")]
            pub fn value(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }
    };
}

string_newtype!(
    /// Represents a Resource ID, which is a unique identifier for a resource within a Cosmos DB account.
    ///
    /// In most cases, you don't need to use this type directly, as the SDK will handle resource IDs for you.
    ResourceId
);
