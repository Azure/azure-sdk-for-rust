// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Examples of 'use' statements in Rust

// Basic use statement importing a single item
pub use std::fmt::Display;

// Importing multiple items from the same module
pub use std::collections::{HashMap, HashSet};

// Importing all public items from a module with the glob operator
pub use std::io::prelude::*;

// Importing with a custom name using 'as'
pub use std::io::Result as IoResult;

// Re-exporting an item with 'pub use'
pub use std::time::Duration;

// Nested imports with self (importing both the module and items from it)
pub use std::{self, env};

// Combining nested imports
pub use std::fmt::{self, Write};

// Example module to demonstrate importing
pub mod nested_module {
    // An item to import later
    pub struct NestedItem {
        pub value: i32,
    }

    // A nested module within another module
    pub mod deeper {
        // An item in a deeper nested module
        pub struct DeeperItem {
            pub name: String,
        }

        // Using an item from a parent module
        pub use super::NestedItem;
    }
}

// Importing with relative paths (from the current module)
pub use self::nested_module::NestedItem;

// Creating a struct using imported types
pub struct ImportExample {
    // Use a type from std that we imported
    pub data: HashMap<String, i32>,
    // Use a type from our crate module
    pub nested_item: nested_module::NestedItem,
}

// External module to import from
pub mod external_types {
    pub struct Config {
        pub name: String,
        pub value: i32,
    }

    pub trait Configurable {
        fn configure(&mut self, config: &Config);
    }
}

// Importing and using items from the external module
use external_types::{Config, Configurable};

// A struct implementing the imported trait
pub struct ServiceClient {
    pub config: Config,
}

impl Configurable for ServiceClient {
    fn configure(&mut self, config: &Config) {
        self.config = Config {
            name: config.name.clone(),
            value: config.value,
        };
    }
}
