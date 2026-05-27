# query_plan_native

Native FFI query plan provider using the QueryPlanInterop C++ library.

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
  -> try_native_query_plan()
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
  -> if Err: fall through to trivial check, then Gateway
```

| Platform | Library name |
|----------|-------------|
| Windows  | `Cosmos.QueryPlanInterop.dll` |
| Linux    | `libQueryPlanInterop.so` |
| macOS    | `libQueryPlanInterop.dylib` |

## Running tests

```bash
# Unit tests (no native DLL needed)
cargo test -p azure_data_cosmos_driver --lib query_plan_native

# Integration tests (requires native DLL on PATH or QUERY_PLAN_INTEROP_LIB_DIR)
cargo test -p azure_data_cosmos_driver --lib query_plan_native \
    --features __query_plan_native_integration -- --test-threads=1
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
