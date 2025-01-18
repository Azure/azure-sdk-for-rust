pub mod module_example;

/// This is a sample module
pub mod sample_module {
    /// This is a sample function
    pub fn sample_function() {
        // function implementation
    }

    /// This is a sample struct
    pub struct SampleStruct {
        /// This is a sample field
        pub field: i32,
    }

    /// This is a sample struct
    #[derive(Debug)]
    pub struct SampleStructWithDebug {
        /// This is a sample field
        pub field: i32,
    }
}

#[allow(dead_code)]
static GLOBAL_FIELD1: i32 = 0;

pub trait MyTrait {
    // Define some methods or associated functions here
    fn example_method(&self);
}

pub fn foo<T: MyTrait, V: MyTrait>(v: &T) {
    // function implementation
    todo!()
}

pub fn bar<T, V>(v: &T)
where
    T: MyTrait,
    V: MyTrait,
{
    // function implementation
    todo!()
}

pub static GLOBAL_FIELD2: &(dyn MyTrait + Sync) = &DummyTrait;
struct DummyTrait;

unsafe impl Sync for DummyTrait {}

impl MyTrait for DummyTrait {
    fn example_method(&self) {
        // method implementation
    }
}
