# query_plan_native

Native FFI query plan provider using the QueryPlanInterop C++ library.

Gated behind the `__internal_native_query_plan` feature flag. When disabled,
the driver uses the Gateway for all query plans. The module itself always
compiles so that tests can run independently of the feature flag.

## Library loading

The native library is loaded lazily on first query plan request.
If the DLL/.so is not found, the result is cached and the driver
falls back to the Gateway for all subsequent calls.

Search order:
1. `QUERY_PLAN_INTEROP_LIB_DIR` environment variable
2. OS default (`PATH` on Windows, `LD_LIBRARY_PATH` on Linux)

Call chain:
```
cosmos_driver::plan_operation()
  -> NativeQueryPlanProvider::get_query_plan()
     -> OnceLock: cached QueryPlanProvider (created once, reused)
        -> QueryPlanProvider::new(config)
           -> query_plan_native_lib()  [OnceLock: loaded once per process]
              Loads the DLL and resolves function pointers
              (CreateServiceProvider, GetPartitionKeyRangesFromQuery4).
              Cached in QUERY_PLAN_NATIVE_LIB static -- all calls share the same pointers.
              -> platform::load_library(LIB_NAME)
                 -> LoadLibraryA / dlopen (search order above)
              -> GetProcAddress / dlsym for each export
           -> calls CreateServiceProvider(config) via resolved pointer
        -> provider.get_partition_key_ranges(query, pk_paths, options)
           -> calls GetPartitionKeyRangesFromQuery4(...) via resolved pointer
           -> deserialize JSON -> QueryPlan
  -> if Err: fall through to gateway_query_plan()
```

| Platform | Library name |
|----------|-------------|
| Windows  | `Cosmos.QueryPlanInterop.dll` |
| Linux    | `libqueryplaninterop.so` |
| macOS    | `libqueryplaninterop.dylib` |

## Running tests

```bash
# Unit tests (no native DLL needed, always compiled)
cargo test -p azure_data_cosmos_driver --lib query_plan_native

# Integration tests (requires native DLL on PATH or QUERY_PLAN_INTEROP_LIB_DIR)
# Tests are ignored by default; opt in via test_category:
RUSTFLAGS='--cfg test_category="native_query_plan"' \
    cargo test -p azure_data_cosmos_driver --lib query_plan_native
```

On Windows (PowerShell):
```powershell
$env:QUERY_PLAN_INTEROP_LIB_DIR = "Q:\QueryPlanInterop"
$env:RUSTFLAGS = '--cfg test_category="native_query_plan"'
cargo test -p azure_data_cosmos_driver --lib query_plan_native
```

## Regenerating bindgen bindings

Requires `bindgen-cli` and `libclang`.

```bash
cargo install bindgen-cli

cd sdk/cosmos/azure_data_cosmos_driver/src/query_plan_native

LIBCLANG_PATH=/path/to/libclang bindgen bindgen_wrapper.h \
    --use-core --no-layout-tests \
    --allowlist-type "QueryPlanInterop.*" \
    --allowlist-function "CreateServiceProvider|UpdateServiceProvider|GetPartitionKeyRangesFromQuery4" \
    --output generated/native_bindings.rs \
    -- -x c++ -D_WIN32
```
