# Azure Cosmos DB Native C Bindings

This package provides C language bindings (FFI) for the Azure Cosmos DB Rust SDK (`azure_data_cosmos`). It enables C/C++ applications to interact with Azure Cosmos DB through a native C API backed by the Rust implementation.

## Overview

The `azure_data_cosmos_native` package consists of:

- **C Headers** (`include/azurecosmos.h`) - Public C API definitions
- **Rust FFI Layer** (`src/`) - Foreign Function Interface implementation that bridges C to Rust
- **Dynamic Library** (`libazurecosmos`) - Compiled native library for linking
- **Test Framework** (`c_tests/`) - Comprehensive C test suite with automatic test discovery

The library exposes core Cosmos DB operations through a C-compatible interface while leveraging the safety and performance of the Rust SDK implementation.

## Prerequisites

### All Platforms

- **Rust** (MSRV 1.85 or later)
  - Install from [https://rustup.rs/](https://rustup.rs/)
  - Verify: `rustc --version`

- **CMake** (3.10 or later)
  - Used for building and running C tests
  - Verify: `cmake --version`

- **C Compiler**
  - GCC, Clang, or MSVC depending on platform

### Linux

Use your distro package manager to install CMake, and a C compiler.
Use [https://rustup.rs/](https://rustup.rs/) to install Rust.

### macOS

Use [https://rustup.rs/](https://rustup.rs/) to install Rust.

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install CMake via Homebrew
brew install cmake
```

### Windows

Use [https://rustup.rs/](https://rustup.rs/) to install Rust.

- **Visual Studio 2022 or later** with C++ build tools
  - Download from [https://visualstudio.microsoft.com/](https://visualstudio.microsoft.com/)
  - Include "Desktop development with C++" workload

- **CMake**
  - Download from [https://cmake.org/download/](https://cmake.org/download/)
  - Or install via `winget install cmake`

## Building the Library

Build the library and C test suite:

```bash
# From the azure_data_cosmos_native directory, create and enter build directory
# CMake builds are generally done in a separate build directory.
mkdir build
cd build

# Configure
# On Windows, this generates a Visual Studio solution with multi-configuration support.
# On Linux/macOS, this generates Makefiles for single-configuration builds, depending on CMAKE_BUILD_TYPE (see below).
cmake ..

# Build (defaulting to Debug on multi-configuration generators, like MSBuild)
cmake --build .

# Test (after building)
ctest
```

### Platform-Specific Build Notes

#### Linux/macOS - Release Builds

By default, CMake generates Debug builds. For optimized Release builds:

```bash
mkdir build
cd build

# Configure for Release build
cmake -DCMAKE_BUILD_TYPE=Release ..

# Build
cmake --build .
```

**Note**: Changing `CMAKE_BUILD_TYPE` requires deleting and recreating the build directory.

#### Windows/MSVC - Debug and Release Builds

On Windows with Visual Studio, CMake generates a multi-configuration solution supporting both Debug and Release:

```bash
mkdir build
cd build

# Configure (generates solution with both Debug and Release)
cmake ..

# Build Debug configuration
cmake --build . --config Debug

# Build Release configuration
cmake --build . --config Release
```

No need to delete the build directory when switching between Debug and Release.

Build artifacts:
- `build/lib/[Debug|Release]/` - Compiled libraries (libazurecosmos.so/dylib/dll)
- `build/test/[Debug|Release]/` - Test executables
- `build/include/` - Header files
- `build/azurecosmos.pc` - pkg-config file (non-Windows only)

## Running Tests

### Prerequisites for Integration Tests

Integration tests require access to an Azure Cosmos DB instance or the [Cosmos DB Emulator](https://learn.microsoft.com/azure/cosmos-db/emulator).

Set the connection string via environment variable:

```bash
# Using Azure Cosmos DB Emulator
export AZURE_COSMOS_CONNECTION_STRING="emulator"

# Or with explicit connection string
export AZURE_COSMOS_CONNECTION_STRING="AccountEndpoint=https://...;AccountKey=...;"
```

**Note**: The special value `"emulator"` is recognized by the test framework and automatically expands to the well-known emulator connection string. It also ensures that TLS certificate validation is disabled for localhost.

### Running All Tests

```bash
cd build

# Run all tests
ctest

# Run with verbose output
ctest --verbose

# Run with output only on failure
ctest --output-on-failure
```

#### Windows

On Windows with multi-configuration generators (MSBuild), you MUST specify the configuration:

```bash
cd build

# Run tests using Debug configuration
ctest -C Debug

# Run tests using Release configuration
ctest -C Release

# Run with verbose output
ctest -C Debug --verbose
```

### Running Specific Tests

```bash
# Run all tests in a specific suite
ctest -R "item_crud::"

# Run a single test
ctest -R "version::version_match"

# Run multiple specific tests
ctest -R "error_handling::(null_pointer_handling|invalid_runtime_context)"

# List all available tests
ctest --show-only
```

## Development Workflow

All the SDK wrappers are built in Rust, but expose `extern "C"` functions so they are callable from C.
Building the `azure_data_cosmos_native` crate (with either `cargo build` or via CMake) will automatically build the Rust code and produce the native library AND the `azurecosmos.h` header file describing the C API (using [cbindgen](https://github.com/mozilla/cbindgen)). Tests are written in C and located in the `c_tests/` directory.

**Remember, when writing your tests, you are working in C! Memory management, error handling, and other aspects must be handled manually. The test framework provides some helpful macros to assist with this.**

### Adding Tests to an Existing Suite

To add a new test to an existing test file (e.g., `c_tests/item_crud.c`):

1. **Define the test function** as a function returning `int` and starting with `test_`.
Always include the `result` variable and the `cleanup` label, even if there is no cleanup needed.

```c
int test_my_new_test() {
    int result = TEST_PASS;
    test_context ctx;
    test_context_init(&ctx);

    // Setup - use REQUIRE macros for setup that must succeed or the test cannot continue
    REQUIRE(test_context_create_runtime(&ctx), "Failed to create runtime");
    REQUIRE(test_context_create_client(&ctx), "Failed to create client");

    // Your test logic here
    // Use ASSERT for non-critical checks. The test continues even if they fail, but will report failure.
    ASSERT(some_condition, "Condition should be true");

    // Use REQUIRE for critical checks (jump to cleanup on failure)
    REQUIRE_SUCCESS(cosmos_operation(&ctx.call_ctx, ...), "Operation failed");

cleanup:
    test_context_cleanup(&ctx);
    return result;
}
```

2. **Register the test** in the suite (at the bottom of the file):

```c
TEST_SUITE_BEGIN(item_crud)
    TEST_REGISTER(create_item)
    TEST_REGISTER(read_item)
    // ... existing tests ...
    TEST_REGISTER(my_new_test)  // Add your test here
TEST_SUITE_END
```

3. **Rebuild and test**:

```bash
cd build
make  # Auto-discovers new test and reconfigures
ctest -R "item_crud::my_new_test" -V
```

The test framework automatically discovers the new test during the build process.

### Available Assertion Macros

The test framework provides several assertion macros in [c_tests/test_common.h](c_tests/test_common.h):

- **`ASSERT(condition, message)`** - Check condition, continue on failure
- **`REQUIRE(condition, message)`** - Check condition, goto cleanup on failure
- **`ASSERT_ERROR_CODE(err, expected, message)`** - Check error code, continue on failure
- **`REQUIRE_ERROR_CODE(err, expected, message)`** - Check error code, goto cleanup on failure
- **`ASSERT_SUCCESS(err, message)`** - Check operation succeeded, continue on failure
- **`REQUIRE_SUCCESS(err, message)`** - Check operation succeeded, goto cleanup on failure
- **`ASSERT_NOT_NULL(ptr, message)`** - Check pointer is not null, continue on failure
- **`REQUIRE_NOT_NULL(ptr, message)`** - Check pointer is not null, goto cleanup on failure

Use `REQUIRE*` macros for setup and critical operations. Use `ASSERT*` macros for validation checks that should be reported but don't prevent further testing.

### Adding a New Test Suite

To create a completely new test suite:

1. **Create a new test file** in `c_tests/` (e.g., `c_tests/my_feature.c`):

```c
#include "test_common.h"
#include "../include/azurecosmos.h"

int test_first() {
    int result = TEST_PASS;
    test_context ctx;
    test_context_init(&ctx);

    REQUIRE(test_context_create_runtime(&ctx), "Failed to create runtime");

    // Your test logic

cleanup:
    test_context_cleanup(&ctx);
    return result;
}

int test_second() {
    int result = TEST_PASS;
    test_context ctx;
    test_context_init(&ctx);

    // Test implementation
    return result;
}

TEST_SUITE_BEGIN(my_feature)
    TEST_REGISTER(first)
    TEST_REGISTER(second)
TEST_SUITE_END
```

2. **Re-run CMake, and build**:

```bash
cd build

# CMake must regenerate build files to discover the new test suite
cmake ..

# The test suite should be built automatically and all it's tests discovered.
cmake --build .
```

The build system uses `file(GLOB)` to find all `.c` files in `c_tests/`, so your new suite is automatically:
- Compiled into a test executable
- Registered with CTest
- Available for `ctest` commands

3. **Verify your tests**:

```bash
ctest --show-only | grep my_feature  # List tests in your suite
ctest -R "my_feature::" -V           # Run all tests in your suite
```

### Verbose Logging

Set environment variables for detailed output:

```bash
# Rust logging
export COSMOS_LOG=debug

# Run test with verbose output
./build/test/item_crud create_item
```
