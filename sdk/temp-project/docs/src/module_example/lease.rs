/// This is a sample module
pub mod sample_module2 {
    /// This is a sample function
    pub fn sample_function2() {
        // function implementation
    }

    /// This is a sample struct
    pub struct SampleStruct2 {
        // Define the fields of the struct
        pub field1: String,
        pub field2: i32,
    }
}

// Example function that constructs and uses SampleStruct2
pub fn use_sample_struct2() {
    let instance = sample_module2::SampleStruct2 {
        field1: String::from("example"),
        field2: 42,
    };

    // Use the instance in some way
    println!("SampleStruct2: field1 = {}, field2 = {}", instance.field1, instance.field2);
}
