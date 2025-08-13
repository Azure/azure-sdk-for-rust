# Cosmos Client C Tests

This directory contains tests written in C that utilize the Cosmos Client library. The tests are designed to validate the functionality of the Cosmos Client C API, ensuring that it correctly interacts with the Azure CosmosDB service.

Building this directory requires CMake, Rust, and a C compiler.

## Running the tests

To run the tests, follow these steps:

1. Create a `build` directory:

```bash
mkdir build
cd build
```

1. Configure the project with CMake:

```bash
cmake ..
```

1. Run the build AND tests:

```bash
make && make test
```

## Test Structure

Each test is a separate C program located in the `c_tests` directory.
The tests are compiled into executables that can be run independently.
Tests must be manually listed in `CMakeLists.txt` to be included in the build process:

```cmake
set(TEST_FILES
    ./c_tests/version.c
    ./c_tests/your_test_here.c)
```

Once a test is present in the `TEST_FILES` list, it will be automatically compiled and linked against the Cosmos Client library (either static, or dynamic, depending on the value of `BUILD_SHARED_LIBS` when `cmake` is run, which defaults to `ON`).

Helper code should be added to an include file in the `c_tests` directory, such as `c_tests/test_helpers.h`, and included in the test files as needed.
