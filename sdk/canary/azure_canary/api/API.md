# azure_canary

- **Description**: Canary crate for Azure SDK pipeline testing
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`

```rust
pub use azure_canary::constant_example::MAX_CONNECTIONS;
pub use azure_canary::enum_example::Message;
pub use azure_canary::static_example::PROGRAM_NAME;
pub use azure_canary::struct_example::Person;
pub use azure_canary::struct_example::Point2D;
pub use azure_canary::trait_example::Shape;
pub use azure_canary::enum_example::Status;
pub fn add(left: u64, right: u64) -> u64;
pub trait NumericCore {
    fn is_valid(&self) -> bool;
    fn to_string(&self) -> String;
}
pub mod constant_example {
    pub struct Temperature {
        pub value: f64,
        pub unit: TemperatureUnit,
    }
    impl Temperature {
        const ABSOLUTE_ZERO_C: f64 = -273.15;
        fn is_below_freezing(&self) -> bool;
        fn new(value: f64, unit: TemperatureUnit) -> Self;
    }
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum TemperatureUnit {
        Celsius,
        Fahrenheit,
        Kelvin,
    }
    pub trait Limits {
        const MAX: Self;
        const MIN: Self;
    }
    pub const API_VERSION: &str = "v1.0.0";
    pub const DEFAULT_TIMEOUT: std::time::Duration = _;
    pub const MAX_CONNECTIONS: usize = 100;
    pub const MILLISECONDS_PER_DAY: u64 = _;
    pub mod config {
        pub const DB_TIMEOUT: u64 = 10;
    }
}
pub mod enum_example {
    #[derive(Debug)]
    pub enum ApiResult<T> {
        Success(T),
        NotFound { resource: String },
        Unauthorized { reason: String },
        ServerError { code: i32, message: String },
    }
    #[derive(Clone, Debug)]
    pub enum JsonValue {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<JsonValue>),
        Object(std::collections::HashMap<String, JsonValue>),
    }
    #[derive(Clone, Debug, PartialEq)]
    pub enum Message {
        Text(String),
        Number(i32),
        Empty,
        Complex { subject: String, content: String, urgent: bool },
    }
    impl Message {
        fn as_string(&self) -> String;
        fn text(content: &str) -> Self;
    }
    #[derive(Clone, Debug, PartialEq)]
    pub enum Status {
        Active,
        Inactive,
        Pending,
        Canceled,
    }
}
pub mod function_example {
    pub fn apply_twice<F, T>(f: F, value: T) -> T where F: Fn(T) -> T, T: Copy;
    pub fn factorial(n: u64) -> u64;
    pub fn first<T>(list: &[T]) -> Option<&T>;
    pub fn greater_than(n: i32) -> impl Fn(i32) -> bool;
    pub fn greet(name: &str, formal: bool) -> String;
    pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str;
    pub fn parse_number(s: &str) -> Result<i32, String>;
    pub fn print_sorted<T>(list: &mut [T]) where T: std::cmp::Ord + std::fmt::Debug;
    pub fn process_string(s: String) -> usize;
    pub struct Counter {
        pub count: u32,
    }
    impl Counter {
        fn increment(&mut self);
    }
}
pub mod modules_example {
    pub mod networking {
        pub struct Connection {
            pub address: String,
            pub port: u16,
            pub is_secure: bool,
        }
        pub mod protocols {
            pub mod http {
                pub fn create_request(method: Method, path: &str) -> String;
                #[derive(Clone, Copy, Debug)]
                pub enum Method {
                    GET,
                    POST,
                    PUT,
                    DELETE,
                }
            }
            pub mod tcp {
                pub fn connect(address: &str, port: u16) -> super::super::Connection;
            }
        }
    }
    pub mod user {
        #[derive(Clone, Debug)]
        pub struct User {
            pub username: String,
            pub email: String,
            pub is_active: bool,
        }
        impl User {
            fn new(username: String, email: String) -> Self;
        }
        pub mod auth {
            pub fn verify_access(role: &Role, resource: &str) -> bool;
            #[derive(Clone, Debug, PartialEq)]
            pub enum Role {
                Admin,
                User,
                Guest,
            }
        }
    }
}
pub mod static_example {
    pub use std::collections::hash::map::HashMap;
    pub fn count_operation() -> usize;
    #[derive(Clone, Copy, Debug)]
    pub struct AppVersion {
        pub major: u8,
        pub minor: u8,
        pub patch: u8,
    }
    impl Display for AppVersion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    pub static APP_VERSION: AppVersion = _;
    pub static mut ERROR_COUNT: u32 = 0;
    pub static OPERATION_COUNT: std::sync::atomic::AtomicUsize = _;
    pub static PROGRAM_NAME: &str = "Azure Template Example";
    pub mod logger {
        pub fn log(message: &str);
        pub static LOGGING_ENABLED: std::sync::atomic::AtomicBool = _;
    }
}
pub mod struct_example {
    #[derive(Debug)]
    pub struct Borrowed<'a> {
        pub text: &'a str,
        pub additional_text: Option<&'a str>,
    }
    impl<'a> Borrowed<'a> {
        fn new(text: &'a str) -> Self;
    }
    #[derive(Clone, Debug)]
    pub struct Container<T> {
        pub value: T,
        pub description: String,
    }
    #[derive(Debug)]
    pub struct Employee {
        pub personal_info: Person,
        pub employee_id: String,
        pub position: String,
    }
    impl Employee {
        fn full_description(&self) -> String;
    }
    #[derive(Clone, Copy, Debug)]
    pub struct Empty;
    #[derive(Clone, Debug)]
    pub struct Person {
        pub name: String,
        pub age: u32,
        pub email: Option<String>,
    }
    impl Person {
        fn new(name: String, age: u32) -> Self;
        fn with_email(self, email: String) -> Self;
    }
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Point2D(pub f32, pub f32);
    impl Point2D {
        fn distance_to(&self, other: &Self) -> f32;
        fn origin() -> Self;
    }
}
pub mod struct_fields_example {
    pub use std::collections::hash::map::HashMap;
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct ApiResponse {
        #[serde(rename = "success")]
        pub is_success: bool,
        #[serde(rename = "data")]
        pub response_data: String,
        #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
        pub error_message: Option<String>,
    }
    #[derive(Debug)]
    pub struct Configuration {
        pub host: String,
        pub port: u16,
        pub secure: bool,
    }
    #[derive(Debug)]
    pub struct Document<'a> {
        pub title: &'a str,
        pub content: &'a str,
        pub author: &'a str,
    }
    #[derive(Debug)]
    pub struct LargeData {
        pub data: Box<[u8]>,
        pub size: usize,
    }
    #[derive(Clone, Copy, Debug)]
    pub struct Point(pub f64, pub f64, pub f64);
    #[derive(Debug)]
    pub struct TraitBounded<T, U> where T: Debug + Clone, U: Debug + Default {
        pub first: T,
        pub second: U,
    }
    #[derive(Debug)]
    pub struct TypedId<T> {
        pub id: u64,
        pub _type: std::marker::PhantomData<T>,
    }
    #[derive(Debug)]
    pub struct User {
        pub id: u64,
        pub username: String,
        pub email: String,
        pub age: Option<u8>,
        pub is_active: bool,
    }
}
pub mod trait_example {
    pub struct NumberParser;
    impl<'a> Parser<'a, i32> for NumberParser {
        fn parse(&self, input: &'a str) -> Result<i32, &'static str>;
    }
    #[derive(Debug)]
    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }
    impl Shape for Rectangle {
        fn area(&self) -> f64;
        fn perimeter(&self) -> f64;
    }
    #[derive(Debug)]
    pub struct VecContainer<T> {
        pub items: Vec<T>,
    }
    impl<T> Container for VecContainer<T> {
        type Item = T;
        fn add(&mut self, item: <Self as >::Item);
        fn len(&self) -> usize;
    }
    pub trait Bounded {
        const MAX: Self;
        const MIN: Self;
    }
    pub trait Container {
        type Item;
        fn add(&mut self, item: <Self as >::Item);
        fn is_empty(&self) -> bool;
        fn len(&self) -> usize;
    }
    pub trait Parser<'a, T> {
        fn parse(&self, input: &'a str) -> Result<T, &'static str>;
    }
    pub trait Shape {
        fn area(&self) -> f64;
        fn describe(&self) -> String;
        fn perimeter(&self) -> f64;
    }
    pub trait SuperTrait: Debug + Display + Clone {
        fn super_method(&self) -> String;
    }
}
pub mod use_example {
    pub use core::fmt::Display;
    pub use core::time::Duration;
    pub use std::collections::hash::map::HashMap;
    pub use std::collections::hash::set::HashSet;
    pub use std::io::error::Result as IoResult;
    pub use azure_canary::use_example::nested_module::NestedItem;
    pub use core::fmt::Write;
    pub use std::env;
    pub use core::env;
    pub use alloc::fmt;
    pub use std::io::prelude::*;
    pub use std;
    pub struct ImportExample {
        pub data: HashMap<String, i32>,
        pub nested_item: nested_module::NestedItem,
    }
    pub struct ServiceClient {
        pub config: external_types::Config,
    }
    impl Configurable for ServiceClient {
        fn configure(&mut self, config: &Config);
    }
    pub mod external_types {
        pub struct Config {
            pub name: String,
            pub value: i32,
        }
        pub trait Configurable {
            fn configure(&mut self, config: &Config);
        }
    }
    pub mod nested_module {
        pub struct NestedItem {
            pub value: i32,
        }
        pub mod deeper {
            pub use azure_canary::use_example::nested_module::NestedItem;
            pub struct DeeperItem {
                pub name: String,
            }
        }
    }
}
```
