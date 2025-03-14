// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Examples of functions in Rust

/// A function with multiple parameters and documentation
pub fn greet(name: &str, formal: bool) -> String {
    if formal {
        format!("Good day to you, {}.", name)
    } else {
        format!("Hello, {}!", name)
    }
}

/// A function that takes ownership of its parameters
pub fn process_string(s: String) -> usize {
    s.len() // s is dropped here because it's moved into this function
}

/// A generic function
pub fn first<T>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        None
    } else {
        Some(&list[0])
    }
}

/// A function with a where clause for more complex trait bounds
pub fn print_sorted<T>(list: &mut [T])
where
    T: std::cmp::Ord + std::fmt::Debug,
{
    list.sort();
    println!("{:?}", list);
}

/// A const function that can be evaluated at compile time
pub const fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// An unsafe function that requires the caller to ensure safety
/// # Safety
///
/// The caller must ensure that the pointer is valid and not null.
pub unsafe fn dereference_raw_pointer(ptr: *const i32) -> i32 {
    *ptr // Dereferencing a raw pointer is unsafe in Rust
}

/// An async function that can be awaited
pub async fn fetch_data(url: &str) -> Result<String, String> {
    // This is a simplified example - in a real case you'd use an HTTP client
    // like reqwest to actually fetch data
    Ok(format!("Data fetched from {}", url))
}

/// A function with a specific ABI (Application Binary Interface)
/// This example uses the "C" ABI for FFI (Foreign Function Interface)
pub extern "C" fn callable_from_c(value: i32) -> i32 {
    value * 2
}

/// A function that combines several attributes: unsafe, extern, and const
/// # Safety
pub const unsafe extern "C" fn complex_function(ptr: *const u8, len: usize) -> usize {
    // This is just an example - in real code, you'd have proper safety checks
    let slice = std::slice::from_raw_parts(ptr, len);
    slice.len()
}

/// A function returning a Result type for error handling
pub fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("Failed to parse '{}' as i32", s))
}

/// A higher-order function that takes a function as parameter
pub fn apply_twice<F, T>(f: F, value: T) -> T
where
    F: Fn(T) -> T,
    T: Copy,
{
    f(f(value))
}

/// A function returning a closure
pub fn greater_than(n: i32) -> impl Fn(i32) -> bool {
    move |x| x > n
}

/// A recursive function with tail recursion
pub fn factorial(n: u64) -> u64 {
    fn fact_tail(n: u64, acc: u64) -> u64 {
        if n == 0 {
            acc
        } else {
            fact_tail(n - 1, n * acc)
        }
    }
    fact_tail(n, 1)
}

/// A struct with methods
pub struct Counter {
    pub count: u32,
}

impl Counter {
    /// Method to increment the counter
    pub fn increment(&mut self) {
        self.count += 1;
    }
}

/// Function with a lifetime parameter
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
