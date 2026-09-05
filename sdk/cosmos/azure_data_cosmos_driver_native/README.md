<!-- cspell:ignore azurecosmosdriver cdylib staticlib corrosion cbindgen ctest dotnet pinvoke pinvokeimpl pkgconfig cgo ctypes findlibrary ldflags downcallhandle nativelinker symbolloader symbollookup invokeexact byref byrefparam dllimport unmanagedfunctionpointer extern jna jansi callconv dlopen dlsym fixedstring nullable lifo lpdouble lpwchar lpwstr ptypes ofvoid invokestatic linkedhashmap nonblocking jvmti addr gchandle -->

# Azure Cosmos DB Driver — Native C Bindings (`azure_data_cosmos_driver_native`)

C ABI wrapper around [`azure_data_cosmos_driver`](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/cosmos/azure_data_cosmos_driver),
designed for cross-language SDK reuse (.NET, Java, Go, Python, native C/C++).
The full design is in
[NATIVE_WRAPPER_SPEC.md](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md);
the picture-first overview is in
[ASYNC_INVOCATION_ARCHITECTURE.md](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/ASYNC_INVOCATION_ARCHITECTURE.md);
this README is a short orientation and a quick-start for each supported
binding language.

## What this crate ships

- A `cdylib` + `staticlib` named `azurecosmosdriver`
  (`libazurecosmosdriver.{so,dylib,dll}`).
- A C header at [include/azurecosmosdriver.h](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver_native/include/azurecosmosdriver.h),
  regenerated on every build and **checked in** so language-binding consumers
  can vendor it without a Rust toolchain.
- A small C test harness under [c_tests/](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/cosmos/azure_data_cosmos_driver_native/c_tests) driven by CMake +
  [corrosion](https://github.com/corrosion-rs/corrosion).

## Status

The wrapper supports end-to-end CRUD against a real Cosmos account. The
remaining items below are surface-area additions on top of a functional core.
See [the spec](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md)
for the full design.

### Capability matrix (current)

| Capability                                                                      | Status                                                                                  |
| ------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| Master-key authentication                                                       | ✅                                                                                       |
| AAD token-credential authentication                                             | ✅ via host credential bridge                                                            |
| Resource-token authentication                                                   | ⏳ follow-up                                                                              |
| Sync driver creation (`_blocking`)                                              | ✅                                                                                       |
| Async driver creation (`_submit`)                                               | ✅                                                                                       |
| Cache-hit advisory (`5001 OPTIONS_IGNORED_ON_CACHE_HIT`)                        | ⏳ needs driver-side `was_cached` signal                                                 |
| Sync + async `resolve_container`                                                | ✅                                                                                       |
| Single + hierarchical partition keys                                            | ✅                                                                                       |
| Item-CRUD operations (read / create / upsert / replace / delete)                | ✅                                                                                       |
| Item PATCH                                                                      | ✅ (preview exposure is controlled by the consuming SDK)                                 |
| Container-CRUD operations (read / replace / delete)                             | ✅                                                                                       |
| Database + account-scope operations                                             | ✅                                                                                       |
| `cosmos_submit_singleton_operation` (point ops)                                 | ✅                                                                                       |
| `cosmos_submit_operation` (feeds + pagination)                                  | ✅                                                                                       |
| Response status / RU / body / activity-id / session-token / etag / continuation | ✅                                                                                       |
| Pagination (read-feeds + query result sets)                                     | ⏳ planned                                                                               |
| Multi-part response body iteration                                              | ⏳ planned                                                                               |
| Diagnostics accessors                                                           | ⏳ planned                                                                               |
| Patch instruction builder                                                       | ⏳ planned                                                                               |
| Transactional batch sub-operation builder                                       | ⏳ planned                                                                               |
| Custom per-operation request headers                                            | ✅ via `cosmos_CosmosOperationOptions.custom_headers` (array of `cosmos_CosmosHeaderKv`) |

## Building

```bash
# Rust side (produces the cdylib / staticlib and regenerates the header).
cargo build --release -p azure_data_cosmos_driver_native

# C test harness (requires CMake ≥ 3.20 and a C compiler).
cmake -B build sdk/cosmos/azure_data_cosmos_driver_native
cmake --build build
ctest --test-dir build --output-on-failure
```

The resulting shared library lands at:

- Linux:   `target/release/libazurecosmosdriver.so`
- macOS:   `target/release/libazurecosmosdriver.dylib`
- Windows: `target/release/azurecosmosdriver.dll`

Language bindings should either bundle the library next to their executable,
publish it to the system loader path, or use a per-language helper to point
at the build output (`LD_LIBRARY_PATH=…`, `[DllImport]` resolver, etc.).

---

## Error & status model

Every fallible C function returns a **packed 32-bit status**
(`cosmos_status_code_t`) instead of a bespoke error enum, so hosts learn one
taxonomy that is shared with wire responses:

```text
cosmos_status_code_t = (http_status << 16) | sub_status
```

- `COSMOS_STATUS_SUCCESS` (`0`) means success.
- Decode with `http = code >> 16` and `sub = code & 0xFFFF`; a `sub` of `0`
  means the operation had no sub-status.
- Pre-flight / plumbing failures that never hit the wire (a NULL argument,
  invalid UTF-8, a shut-down completion queue, …) still use a real HTTP status
  paired with a synthetic `CLIENT_FFI_*` / `CLIENT_*` sub-status, so they fit
  the same integer as service errors.

The synthetic sub-status codes the driver can produce are re-exported as
`cosmos_sub_status_t` / `COSMOS_SUB_STATUS_*` constants in the generated header.
These are a **named mirror of the driver's canonical
`azure_data_cosmos_driver::error::SubStatusCode` constants** — the
`CosmosSubStatus` enum in `src/error.rs` re-exports each value so C hosts get
stable, documented constant names. A host can therefore switch on
`sub == COSMOS_SUB_STATUS_CLIENT_FFI_NULL_ARGUMENT`, etc.

Synchronous entry points also hand back an owned, flat **rich error**
(`cosmos_error_t`) through their `out_error` slot — it carries the same packed
status plus the message and wire diagnostics inline, and is freed with
`cosmos_error_free`. Asynchronous failures surface the same information as
inline fields on `cosmos_completion_t` (`status`, `http_status_code`,
`sub_status`, `message`, `activity_id`, and the other diagnostic fields), so no
separate error object needs to be taken or freed.

---

## Usage examples — binding-language quick-starts

All four examples below run the same workflow against the local Cosmos DB
emulator (`https://localhost:8081/` with the well-known emulator master
key). The flow is:

```text
1. Build runtime + completion queue.
2. Build account ref → driver → resolved container.
3. Build a partition key.
4. CREATE → READ → DELETE one item, draining the completion queue between
   each step.
5. Free everything in reverse (LIFO).
```

The examples are minimal. The Go and Python samples show the production-shape
error handling (check every return code and outcome, read the rich error);
the C# and Java samples abbreviate it for space. All four skip the production
"receive-loop" thread pattern. See
[Notes that apply to all four bindings](#notes-that-apply-to-all-four-bindings)
below for the production-shape guidance.

> **API migration note.** The per-operation factory + mutator surface
> (`cosmos_operation_create_item`, `cosmos_operation_with_body`,
> `cosmos_operation_options_builder_*`, `cosmos_driver_submit`, …) has been
> **removed**. Operations are now described by a single flat,
> self-describing `cosmos_operation_request_t` struct (kind-tagged via
> `cosmos_CosmosOperationKind`, with per-call settings on the tri-state
> `cosmos_CosmosOperationOptions` seeded by `cosmos_operation_options_default`)
> and executed through two entry points:
>
> - `cosmos_submit_singleton_operation` — point operations
>   (create / read / replace / delete / patch item, database & container CRUD,
>   read/replace offer).
> - `cosmos_submit_operation` — feed/paginated operations
>   (queries, read-all, change feed); resumes from and surfaces a continuation
>   token.
>
> Item PATCH, `patch_max_attempts`, and bounded tracking are fields on the
> canonical `cosmos_operation_request_t`. Per-operation PATCH execution is
> selected through `cosmos_operation_options_t.patch_strategy`; leave it
> `COSMOS_PATCH_STRATEGY_UNSET` to inherit, or set `AUTO`, `CLIENT_SIDE`, or
> `SERVER_SIDE`. Consuming language SDKs decide whether and how to expose PATCH
> as preview. For unsafe instruction lists executed client-side, the driver
> stores `_azsdkPatchTracking` on the item. Passing NULL for
> `patch_tracking_id` generates an ID for the invocation. Retrieve the effective
> UUID from `cosmos_completion_patch_tracking_id`, then persist and reuse it for
> application retries. Cancelled completions also expose the resolved ID because
> the wrapper generates it before starting the driver operation. Entries use a
> 5-minute retention window by default;
> `patch_tracking_retention_seconds` configures a positive whole-second window.
> The default capacity is 1024; when full, the oldest entry is evicted. Duplicate
> suppression is bounded by the earlier of retention expiry or FIFO eviction.
> Every writer must preserve the reserved property and marker order.
>
> Query-plan selection is per operation through
> `cosmos_operation_options_t.query_plan_mode`. Leave it
> `COSMOS_QUERY_PLAN_MODE_UNSET` to inherit, or set
> `COSMOS_QUERY_PLAN_MODE_LOCAL_PREFERRED` or
> `COSMOS_QUERY_PLAN_MODE_GATEWAY_ONLY` for an individual query. The
> environment override remains authoritative over this field.
>
> The v1 functions take `(driver, const cosmos_operation_request_t *request, queue,
> user_data, out_pre_error)` and return a `cosmos_operation_handle_t *`.
> The checked-in [header](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver_native/include/azurecosmosdriver.h) is the authoritative
> source for the struct field layout and the 25 operation kinds. The C#
> example below is written against this new API; the Java, Go, and Python
> examples that follow are pending migration and currently show the **old**
> factory flow — translate them field-for-field from the C# example and the
> header until they are updated.
>
> **Minimizing FFI round-trips.** Two parts of the surface let a host avoid
> chatty per-field calls:
>
> - **Inline partition keys.** Instead of the
>   `cosmos_partition_key_builder_new` / `_add_*` / `_build` / `_free` dance,
>   fill an array of `cosmos_partition_key_component_t` (each entry is a
>   `kind` byte plus a nested `value` union whose active leg —
>   `value.string_value` / `value.number_value` / `value.bool_value` — is
>   selected by `kind`) and point
>   `cosmos_CosmosOperationRequest.partition_key_components` /
>   `partition_key_len` at it. When set, this takes precedence over the
>   `partition_key` handle and is assembled in one shot. The pre-built handle
>   path still works for reusable keys.
> - **Snapshot views.** `cosmos_response_view(resp, &view)` fills a flat
>   `cosmos_response_view_t` (status, RU, the four header strings, both
>   continuation tokens, and the body pointer/len) in one call, replacing up
>   to eight accessors. `cosmos_completion_view(c, &view)` does the same for a
>   completion's scalars (outcome, status, user-data, cancel flag). Every
>   borrowed pointer in a view stays valid until the owning handle is freed.
>   The ownership-transfer accessors (`cosmos_completion_take_response` /
>   `_take_error`) are intentionally not part of the views.

### .NET (C# 12 / .NET 8+)

Copy `azurecosmosdriver.{dll,so,dylib}` next to the executable, then
`dotnet run`.

```csharp
using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;

internal static class Cosmos
{
    const string Lib = "azurecosmosdriver";

    [DllImport(Lib)] public static extern int  cosmos_runtime_build(IntPtr options, out IntPtr runtime, out IntPtr err);
    [DllImport(Lib)] public static extern void cosmos_runtime_free(IntPtr runtime);

    [DllImport(Lib)] public static extern IntPtr  cosmos_completion_queue_create(IntPtr runtime, IntPtr options);
    [DllImport(Lib)] public static extern UIntPtr cosmos_completion_queue_wait(IntPtr q, out Completion outComp, UIntPtr max, uint timeoutMs);
    [DllImport(Lib)] public static extern void    cosmos_completion_queue_free(IntPtr q);
    [DllImport(Lib)] public static extern void    cosmos_completion_queue_free_completions(ref Completion completions, UIntPtr count);

    [DllImport(Lib)] public static extern int  cosmos_account_ref_with_master_key(byte[] endpoint, byte[] key, out IntPtr acct, out IntPtr err);
    [DllImport(Lib)] public static extern void cosmos_account_ref_free(IntPtr a);
    [DllImport(Lib)] public static extern int  cosmos_driver_get_or_create_blocking(IntPtr rt, IntPtr acct, IntPtr opts, out IntPtr drv, out IntPtr err);
    [DllImport(Lib)] public static extern void cosmos_driver_free(IntPtr d);
    [DllImport(Lib)] public static extern int  cosmos_driver_resolve_container_blocking(IntPtr rt, IntPtr drv, byte[] db, byte[] coll, out IntPtr c, out IntPtr err);
    [DllImport(Lib)] public static extern void cosmos_container_ref_free(IntPtr c);

    [DllImport(Lib)] public static extern int  cosmos_partition_key_create(ref PartitionKeyComponent components, UIntPtr len, out IntPtr pk);
    [DllImport(Lib)] public static extern void cosmos_partition_key_free(IntPtr pk);

    // Operation kinds (subset — see `cosmos_CosmosOperationKind` in the header).
    public const int KIND_CREATE_ITEM = 19;
    public const int KIND_READ_ITEM   = 20;
    public const int KIND_DELETE_ITEM = 23;
    public const int OUTCOME_OK       = 0;

    [StructLayout(LayoutKind.Sequential)]
    public struct PartitionKeyComponent
    {
        public int    kind;          // 0 = STRING
        public IntPtr string_value;  // char*
        public double number_value;
        public byte   bool_value;
    }

    // Flat, self-describing request (mirrors `cosmos_operation_request_t`).
    // Fill only the fields the `kind` needs; leave the rest NULL / sentinel.
    [StructLayout(LayoutKind.Sequential)]
    public struct OpRequest
    {
        public int       kind;
        public IntPtr    account;
        public IntPtr    database;
        public IntPtr    container;
        public IntPtr    item_id;                  // char*
        public IntPtr    resource_link;            // char*
        public IntPtr    partition_key;
        public IntPtr    partition_key_components; // borrowed component array
        public UIntPtr   partition_key_len;
        public IntPtr    feed_range;
        public IntPtr    body;                     // const uint8_t* — NULL iff body_len == 0
        public UIntPtr   body_len;                 // 0 = no body
        public IntPtr    session_token;            // char*
        public IntPtr    activity_id;              // char*
        public IntPtr    continuation_token;       // char*
        public int       max_item_count;           // < 0 = unset
        public uint      max_fan_out;               // 0 = unset
        public byte      patch_max_attempts;       // 0 = unset
        public sbyte     populate_index_metrics;   // tri-state bool (0/1/2)
        public sbyte     populate_query_metrics;   // tri-state bool (0/1/2)
        public int       precondition_kind;        // 0 = none
        public IntPtr    precondition_etag;        // char*
        public IntPtr    options;                  // cosmos_operation_options_t*
        public IntPtr    patch_tracking_id;                // UUID char*, NULL = generate
        public ushort    patch_tracking_capacity;          // 0 = driver default
        public uint      patch_tracking_retention_seconds; // 0 = driver default
    }

    // A drained completion. All pointers are borrowed until free_completions.
    [StructLayout(LayoutKind.Sequential)]
    public struct Completion
    {
        public int     outcome;
        public int     status;
        public IntPtr  user_data;
        public byte    was_cancel_requested;
        public ushort  http_status_code;
        public int     sub_status;
        public double  request_charge;
        public long    retry_after_ms;
        public byte    is_from_wire;
        public IntPtr  message;
        public IntPtr  activity_id;
        public IntPtr  session_token;
        public IntPtr  etag;
        public IntPtr  continuation;
        public IntPtr  next_continuation;
        public IntPtr  backtrace;
        public IntPtr  headers;
        public UIntPtr headers_len;
        public IntPtr  body;
        public UIntPtr body_len;
        public IntPtr  diagnostics;
        public IntPtr  driver;
        public IntPtr  container;
        public IntPtr  backing;
    }

    // The two — and only two — execution entry points.
    [DllImport(Lib)] public static extern IntPtr cosmos_submit_singleton_operation(IntPtr drv, ref OpRequest req, IntPtr q, IntPtr ud, out int preErr);
    [DllImport(Lib)] public static extern IntPtr cosmos_submit_operation(IntPtr drv, ref OpRequest req, IntPtr q, IntPtr ud, out int preErr);
    [DllImport(Lib)] public static extern void   cosmos_operation_handle_free(IntPtr h);
    [DllImport(Lib)] public static extern void   cosmos_error_free(IntPtr e);

    public static byte[] Cstr(string s) => Encoding.UTF8.GetBytes(s + "\0");
    public static int PackedHttp(int code) => (int)((uint)code >> 16);
    public static int PackedSub(int code) => (int)((uint)code & 0xffff);
    public static bool HasSub(int code) => PackedSub(code) != 0;
    public static string FormatStatus(int code) => code == 0 ? "success" : HasSub(code) ? $"http={PackedHttp(code)} sub={PackedSub(code)} raw={code}" : $"http={PackedHttp(code)} raw={code}";

    public static void CheckStatus(int status, IntPtr err, string what)
    {
        try
        {
            if (status != 0) throw new InvalidOperationException($"{what} failed: {FormatStatus(status)}");
        }
        finally
        {
            if (err != IntPtr.Zero) cosmos_error_free(err);
        }
    }
}

internal sealed record OperationResult(int Status, int HttpStatusCode, int SubStatus, double RequestCharge, byte[] Body, string? Message);

internal static class Program
{
    static OperationResult SubmitAndWait(IntPtr drv, ref Cosmos.OpRequest req, IntPtr q)
    {
        var h = Cosmos.cosmos_submit_singleton_operation(drv, ref req, q, IntPtr.Zero, out int pre);
        if (h == IntPtr.Zero) throw new InvalidOperationException($"submit pre-flight failed: {Cosmos.FormatStatus(pre)}");

        var comp = new Cosmos.Completion();
        UIntPtr n = UIntPtr.Zero;
        try
        {
            n = Cosmos.cosmos_completion_queue_wait(q, out comp, (UIntPtr)1, 30_000);
            if (n == UIntPtr.Zero) throw new InvalidOperationException("queue drained or shut down before a completion arrived");

            var message = comp.message == IntPtr.Zero ? null : Marshal.PtrToStringUTF8(comp.message);
            var body = Array.Empty<byte>();
            if (comp.body != IntPtr.Zero && comp.body_len != UIntPtr.Zero)
            {
                var len = checked((int)comp.body_len.ToUInt64());
                body = new byte[len];
                Marshal.Copy(comp.body, body, 0, len);
            }

            var result = new OperationResult(comp.status, comp.http_status_code, comp.sub_status, comp.request_charge, body, message);
            if (comp.outcome != Cosmos.OUTCOME_OK)
            {
                throw new InvalidOperationException($"operation failed ({Cosmos.FormatStatus(result.Status)}): {result.Message}");
            }
            return result;
        }
        finally
        {
            if (n != UIntPtr.Zero) Cosmos.cosmos_completion_queue_free_completions(ref comp, n);
            Cosmos.cosmos_operation_handle_free(h);
        }
    }

    static void Main()
    {
        // 1. Runtime + queue. Pass NULL for driver defaults. To add a user-agent
        //    suffix, call cosmos_runtime_options_default(), set .user_agent_suffix,
        //    and pass the options pointer to cosmos_runtime_build.
        Cosmos.CheckStatus(Cosmos.cosmos_runtime_build(IntPtr.Zero, out var rt, out var err), err, "runtime build");
        var q = Cosmos.cosmos_completion_queue_create(rt, IntPtr.Zero);

        // 2. Account → driver → container
        Cosmos.CheckStatus(Cosmos.cosmos_account_ref_with_master_key(
            Cosmos.Cstr("https://localhost:8081/"),
            Cosmos.Cstr("C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw=="),
            out var acct, out err), err, "account ref");
        Cosmos.CheckStatus(Cosmos.cosmos_driver_get_or_create_blocking(rt, acct, IntPtr.Zero, out var drv, out err), err, "driver create");
        Cosmos.CheckStatus(Cosmos.cosmos_driver_resolve_container_blocking(rt, drv, Cosmos.Cstr("sample-db"), Cosmos.Cstr("sample-coll"), out var coll, out err), err, "resolve container");

        // 3. Partition key
        var pkBytes = Cosmos.Cstr("tenant-42");
        var pkPin = GCHandle.Alloc(pkBytes, GCHandleType.Pinned);
        IntPtr pk;
        try
        {
            var component = new Cosmos.PartitionKeyComponent { kind = 0, string_value = pkPin.AddrOfPinnedObject() };
            Cosmos.CheckStatus(Cosmos.cosmos_partition_key_create(ref component, (UIntPtr)1, out pk), IntPtr.Zero, "partition key create");
        }
        finally { pkPin.Free(); }

        // 4. CREATE — fill a flat request and submit it through the singleton path.
        var body = JsonSerializer.SerializeToUtf8Bytes(new { id = "doc1", pk = "tenant-42", name = "hello" });
        var bodyPin = GCHandle.Alloc(body, GCHandleType.Pinned);
        try
        {
            var req = new Cosmos.OpRequest
            {
                kind           = Cosmos.KIND_CREATE_ITEM,
                container      = coll,
                partition_key  = pk,
                body           = bodyPin.AddrOfPinnedObject(),
                body_len       = (UIntPtr)body.Length,
                max_item_count = -1,
            };
            var create = SubmitAndWait(drv, ref req, q);
            Console.WriteLine($"CREATE status={create.HttpStatusCode} ru={create.RequestCharge:F2}");
        }
        finally { bodyPin.Free(); }

        // 5. READ — item-id addressed, no body.
        var idBytes = Cosmos.Cstr("doc1");
        var idPin = GCHandle.Alloc(idBytes, GCHandleType.Pinned);
        try
        {
            var req = new Cosmos.OpRequest
            {
                kind           = Cosmos.KIND_READ_ITEM,
                container      = coll,
                partition_key  = pk,
                item_id        = idPin.AddrOfPinnedObject(),
                max_item_count = -1,
            };
            var read = SubmitAndWait(drv, ref req, q);
            Console.WriteLine($"READ status={read.HttpStatusCode} body={Encoding.UTF8.GetString(read.Body)}");

            // 6. DELETE — same shape as READ, different kind.
            var del = new Cosmos.OpRequest
            {
                kind           = Cosmos.KIND_DELETE_ITEM,
                container      = coll,
                partition_key  = pk,
                item_id        = idPin.AddrOfPinnedObject(),
                max_item_count = -1,
            };
            var delete = SubmitAndWait(drv, ref del, q);
            Console.WriteLine($"DELETE status={delete.HttpStatusCode}");
        }
        finally { idPin.Free(); }

        // 7. Tear-down (LIFO)
        Cosmos.cosmos_partition_key_free(pk);
        Cosmos.cosmos_container_ref_free(coll);
        Cosmos.cosmos_driver_free(drv);
        Cosmos.cosmos_account_ref_free(acct);
        Cosmos.cosmos_completion_queue_free(q);
        Cosmos.cosmos_runtime_free(rt);
    }
}
```

### Java (Java 22+ with the FFM API — `java.lang.foreign`)

No JNI; no JNA. Compile with `--enable-native-access=ALL-UNNAMED`.

```java
import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
import static java.lang.foreign.ValueLayout.*;
import java.nio.charset.StandardCharsets;

public final class CosmosSample {
    static final Linker LINKER = Linker.nativeLinker();
    static final SymbolLookup LOOKUP = SymbolLookup.libraryLookup("azurecosmosdriver", Arena.global());

    static MethodHandle h(String name, FunctionDescriptor fd) {
        return LINKER.downcallHandle(LOOKUP.find(name).orElseThrow(), fd);
    }
    static MemorySegment cstr(Arena a, String s) { return a.allocateUtf8String(s); }

    static final MethodHandle RT_BUILD         = h("cosmos_runtime_build", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS));
    static final MethodHandle RT_FREE          = h("cosmos_runtime_free", FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle CQ_CREATE        = h("cosmos_completion_queue_create", FunctionDescriptor.of(ADDRESS, ADDRESS, ADDRESS));
    static final MethodHandle CQ_WAIT          = h("cosmos_completion_queue_wait", FunctionDescriptor.of(JAVA_LONG, ADDRESS, ADDRESS, JAVA_LONG, JAVA_INT));
    static final MethodHandle CQ_FREE          = h("cosmos_completion_queue_free", FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle CQ_FREE_COMPS    = h("cosmos_completion_queue_free_completions", FunctionDescriptor.ofVoid(ADDRESS, JAVA_LONG));
    static final MethodHandle ACCT_WITH_KEY    = h("cosmos_account_ref_with_master_key", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, ADDRESS));
    static final MethodHandle ACCT_FREE        = h("cosmos_account_ref_free", FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle DRV_GOC_BLK      = h("cosmos_driver_get_or_create_blocking", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS));
    static final MethodHandle DRV_FREE         = h("cosmos_driver_free", FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle RESOLVE_BLK      = h("cosmos_driver_resolve_container_blocking", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS));
    static final MethodHandle CONTAINER_FREE   = h("cosmos_container_ref_free", FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle PK_CREATE        = h("cosmos_partition_key_create", FunctionDescriptor.of(JAVA_INT, ADDRESS, JAVA_LONG, ADDRESS));
    static final MethodHandle PK_FREE          = h("cosmos_partition_key_free", FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle SUBMIT_SINGLETON = h("cosmos_submit_singleton_operation", FunctionDescriptor.of(ADDRESS, ADDRESS, ADDRESS, ADDRESS, JAVA_LONG, ADDRESS));
    static final MethodHandle OP_HND_FREE      = h("cosmos_operation_handle_free", FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle ERR_FREE         = h("cosmos_error_free", FunctionDescriptor.ofVoid(ADDRESS));

    // Operation kind values (cosmos_CosmosOperationKind) and completion outcome values.
    static final int KIND_CREATE_ITEM = 19;
    static final int KIND_READ_ITEM = 20;
    static final int KIND_DELETE_ITEM = 23;
    static final int OUTCOME_OK = 0;

    // Layout of cosmos_partition_key_component_t on LP64/LLP64.
    static final GroupLayout PK_COMPONENT = MemoryLayout.structLayout(
        JAVA_INT.withName("kind"),
        MemoryLayout.paddingLayout(4),
        ADDRESS.withName("string_value"),
        JAVA_DOUBLE.withName("number_value"),
        JAVA_BYTE.withName("bool_value"),
        MemoryLayout.paddingLayout(7));

    // Layout of the flat cosmos_operation_request_t. Field order MUST match the
    // header; cbindgen emits the C struct in declaration order.
    static final GroupLayout REQUEST = MemoryLayout.structLayout(
        JAVA_INT.withName("kind"),
        MemoryLayout.paddingLayout(4),
        ADDRESS.withName("account"),
        ADDRESS.withName("database"),
        ADDRESS.withName("container"),
        ADDRESS.withName("item_id"),
        ADDRESS.withName("resource_link"),
        ADDRESS.withName("partition_key"),
        ADDRESS.withName("partition_key_components"),
        JAVA_LONG.withName("partition_key_len"),
        ADDRESS.withName("feed_range"),
        ADDRESS.withName("body"),
        JAVA_LONG.withName("body_len"),
        ADDRESS.withName("session_token"),
        ADDRESS.withName("activity_id"),
        ADDRESS.withName("continuation_token"),
        JAVA_INT.withName("max_item_count"),
        JAVA_INT.withName("max_fan_out"),
        JAVA_BYTE.withName("patch_max_attempts"),
        JAVA_BYTE.withName("populate_index_metrics"),
        JAVA_BYTE.withName("populate_query_metrics"),
        MemoryLayout.paddingLayout(1),
        JAVA_INT.withName("precondition_kind"),
        ADDRESS.withName("precondition_etag"),
        ADDRESS.withName("options"),
        ADDRESS.withName("patch_tracking_id"),
        JAVA_SHORT.withName("patch_tracking_capacity"),
        MemoryLayout.paddingLayout(2),
        JAVA_INT.withName("patch_tracking_retention_seconds"));

    // Layout of cosmos_completion_t. Pointers and intptr_t/uintptr_t are 8 bytes.
    static final GroupLayout COMPLETION = MemoryLayout.structLayout(
        JAVA_INT.withName("outcome"),
        JAVA_INT.withName("status"),
        JAVA_LONG.withName("user_data"),
        JAVA_BYTE.withName("was_cancel_requested"),
        MemoryLayout.paddingLayout(1),
        JAVA_SHORT.withName("http_status_code"),
        JAVA_INT.withName("sub_status"),
        JAVA_DOUBLE.withName("request_charge"),
        JAVA_LONG.withName("retry_after_ms"),
        JAVA_BYTE.withName("is_from_wire"),
        MemoryLayout.paddingLayout(7),
        ADDRESS.withName("message"),
        ADDRESS.withName("activity_id"),
        ADDRESS.withName("session_token"),
        ADDRESS.withName("etag"),
        ADDRESS.withName("continuation"),
        ADDRESS.withName("next_continuation"),
        ADDRESS.withName("backtrace"),
        ADDRESS.withName("headers"),
        JAVA_LONG.withName("headers_len"),
        ADDRESS.withName("body"),
        JAVA_LONG.withName("body_len"),
        ADDRESS.withName("diagnostics"),
        ADDRESS.withName("driver"),
        ADDRESS.withName("container"),
        ADDRESS.withName("backing"));

    static final long PK_KIND = PK_COMPONENT.byteOffset(MemoryLayout.PathElement.groupElement("kind"));
    static final long PK_STRING = PK_COMPONENT.byteOffset(MemoryLayout.PathElement.groupElement("string_value"));
    static final long REQ_KIND = REQUEST.byteOffset(MemoryLayout.PathElement.groupElement("kind"));
    static final long REQ_CONTAINER = REQUEST.byteOffset(MemoryLayout.PathElement.groupElement("container"));
    static final long REQ_ITEM_ID = REQUEST.byteOffset(MemoryLayout.PathElement.groupElement("item_id"));
    static final long REQ_PARTITION_KEY = REQUEST.byteOffset(MemoryLayout.PathElement.groupElement("partition_key"));
    static final long REQ_BODY = REQUEST.byteOffset(MemoryLayout.PathElement.groupElement("body"));
    static final long REQ_BODY_LEN = REQUEST.byteOffset(MemoryLayout.PathElement.groupElement("body_len"));
    static final long REQ_MAX_ITEM_COUNT = REQUEST.byteOffset(MemoryLayout.PathElement.groupElement("max_item_count"));
    static final long C_OUTCOME = COMPLETION.byteOffset(MemoryLayout.PathElement.groupElement("outcome"));
    static final long C_STATUS = COMPLETION.byteOffset(MemoryLayout.PathElement.groupElement("status"));
    static final long C_HTTP_STATUS = COMPLETION.byteOffset(MemoryLayout.PathElement.groupElement("http_status_code"));
    static final long C_SUB_STATUS = COMPLETION.byteOffset(MemoryLayout.PathElement.groupElement("sub_status"));
    static final long C_RU = COMPLETION.byteOffset(MemoryLayout.PathElement.groupElement("request_charge"));
    static final long C_MESSAGE = COMPLETION.byteOffset(MemoryLayout.PathElement.groupElement("message"));
    static final long C_BODY = COMPLETION.byteOffset(MemoryLayout.PathElement.groupElement("body"));
    static final long C_BODY_LEN = COMPLETION.byteOffset(MemoryLayout.PathElement.groupElement("body_len"));

    record Result(int status, int httpStatus, int subStatus, double requestCharge, byte[] body) {}

    static int packedHttp(int code) { return code >>> 16; }
    static int packedSub(int code) { return code & 0xffff; }
    static boolean hasSub(int code) { return packedSub(code) != 0; }
    static String formatStatus(int code) {
        if (code == 0) return "success";
        return hasSub(code)
            ? "http=" + packedHttp(code) + " sub=" + packedSub(code) + " raw=" + code
            : "http=" + packedHttp(code) + " raw=" + code;
    }

    static MemorySegment outAddress(Arena arena) {
        MemorySegment out = arena.allocate(ADDRESS);
        out.set(ADDRESS, 0, MemorySegment.NULL);
        return out;
    }

    static void checkStatus(int status, MemorySegment outErr, String what) throws Throwable {
        MemorySegment err = outErr.equals(MemorySegment.NULL) ? MemorySegment.NULL : outErr.get(ADDRESS, 0);
        try {
            if (status != 0) {
                throw new RuntimeException(what + " failed: " + formatStatus(status));
            }
        } finally {
            if (!err.equals(MemorySegment.NULL)) {
                ERR_FREE.invokeExact(err);
            }
        }
    }

    static MemorySegment itemRequest(Arena arena, int kind, MemorySegment coll, MemorySegment pk, MemorySegment itemId, byte[] body) {
        MemorySegment req = arena.allocate(REQUEST);
        req.set(JAVA_INT, REQ_KIND, kind);
        req.set(ADDRESS, REQ_CONTAINER, coll);
        req.set(ADDRESS, REQ_PARTITION_KEY, pk);
        if (itemId != null && !itemId.equals(MemorySegment.NULL)) {
            req.set(ADDRESS, REQ_ITEM_ID, itemId);
        }
        if (body != null && body.length > 0) {
            MemorySegment bodySeg = arena.allocate(body.length);
            MemorySegment.copy(body, 0, bodySeg, JAVA_BYTE, 0, body.length);
            req.set(ADDRESS, REQ_BODY, bodySeg);
            req.set(JAVA_LONG, REQ_BODY_LEN, (long) body.length);
        }
        req.set(JAVA_INT, REQ_MAX_ITEM_COUNT, -1);
        return req;
    }

    static Result submit(Arena arena, MemorySegment drv, MemorySegment q, MemorySegment req, String name) throws Throwable {
        MemorySegment preErr = arena.allocate(JAVA_INT);
        MemorySegment hdl = (MemorySegment) SUBMIT_SINGLETON.invokeExact(drv, req, q, 0L, preErr);
        if (hdl.equals(MemorySegment.NULL)) {
            throw new RuntimeException("submit pre-flight failed: " + formatStatus(preErr.get(JAVA_INT, 0)));
        }

        MemorySegment comp = arena.allocate(COMPLETION);
        long n;
        try {
            n = (long) CQ_WAIT.invokeExact(q, comp, 1L, 30_000);
        } finally {
            OP_HND_FREE.invokeExact(hdl);
        }
        if (n == 0) {
            throw new RuntimeException("queue drained or shut down before a completion arrived");
        }

        try {
            int outcome = comp.get(JAVA_INT, C_OUTCOME);
            int status = comp.get(JAVA_INT, C_STATUS);
            int http = Short.toUnsignedInt(comp.get(JAVA_SHORT, C_HTTP_STATUS));
            int sub = comp.get(JAVA_INT, C_SUB_STATUS);
            double ru = comp.get(JAVA_DOUBLE, C_RU);
            MemorySegment msgPtr = comp.get(ADDRESS, C_MESSAGE);
            String message = msgPtr.equals(MemorySegment.NULL) ? "" : msgPtr.reinterpret(Long.MAX_VALUE).getUtf8String(0);
            MemorySegment bodyPtr = comp.get(ADDRESS, C_BODY);
            long bodyLen = comp.get(JAVA_LONG, C_BODY_LEN);
            byte[] body = bodyPtr.equals(MemorySegment.NULL) || bodyLen == 0
                ? new byte[0]
                : bodyPtr.reinterpret(bodyLen).toArray(JAVA_BYTE);

            if (outcome != OUTCOME_OK) {
                throw new RuntimeException(name + " failed (" + formatStatus(status) + "): " + message);
            }
            return new Result(status, http, sub, ru, body);
        } finally {
            CQ_FREE_COMPS.invokeExact(comp, n);
        }
    }

    public static void main(String[] args) throws Throwable {
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment rt = MemorySegment.NULL;
            MemorySegment q = MemorySegment.NULL;
            MemorySegment acct = MemorySegment.NULL;
            MemorySegment drv = MemorySegment.NULL;
            MemorySegment coll = MemorySegment.NULL;
            MemorySegment pk = MemorySegment.NULL;
            try {
                // 1. Runtime + queue. Pass NULL for driver defaults. To add a
                //    user-agent suffix, call cosmos_runtime_options_default(), set
                //    .user_agent_suffix, and pass the options pointer to build.
                MemorySegment outRt = outAddress(arena);
                MemorySegment outErr = outAddress(arena);
                checkStatus((int) RT_BUILD.invokeExact(MemorySegment.NULL, outRt, outErr), outErr, "runtime build");
                rt = outRt.get(ADDRESS, 0);
                q = (MemorySegment) CQ_CREATE.invokeExact(rt, MemorySegment.NULL);

                // 2. Account -> driver -> container
                MemorySegment outAcct = outAddress(arena);
                outErr = outAddress(arena);
                checkStatus((int) ACCT_WITH_KEY.invokeExact(
                    cstr(arena, "https://localhost:8081/"),
                    cstr(arena, "C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw=="),
                    outAcct, outErr), outErr, "account ref");
                acct = outAcct.get(ADDRESS, 0);

                MemorySegment outDrv = outAddress(arena);
                outErr = outAddress(arena);
                checkStatus((int) DRV_GOC_BLK.invokeExact(rt, acct, MemorySegment.NULL, outDrv, outErr), outErr, "driver create");
                drv = outDrv.get(ADDRESS, 0);

                MemorySegment outColl = outAddress(arena);
                outErr = outAddress(arena);
                checkStatus((int) RESOLVE_BLK.invokeExact(rt, drv, cstr(arena, "sample-db"), cstr(arena, "sample-coll"), outColl, outErr), outErr, "resolve container");
                coll = outColl.get(ADDRESS, 0);

                // 3. Partition key
                MemorySegment component = arena.allocate(PK_COMPONENT);
                component.set(JAVA_INT, PK_KIND, 0);
                component.set(ADDRESS, PK_STRING, cstr(arena, "tenant-42"));
                MemorySegment outPk = outAddress(arena);
                checkStatus((int) PK_CREATE.invokeExact(component, 1L, outPk), MemorySegment.NULL, "partition key create");
                pk = outPk.get(ADDRESS, 0);

                byte[] body = "{\"id\":\"doc1\",\"pk\":\"tenant-42\",\"name\":\"hello\"}".getBytes(StandardCharsets.UTF_8);

                // 4. CREATE — host SDK serializes its own JSON (Jackson, Gson, ...).
                Result create = submit(arena, drv, q, itemRequest(arena, KIND_CREATE_ITEM, coll, pk, MemorySegment.NULL, body), "CREATE");
                System.out.printf("CREATE status=%d ru=%.2f%n", create.httpStatus(), create.requestCharge());

                // 5. READ.
                Result read = submit(arena, drv, q, itemRequest(arena, KIND_READ_ITEM, coll, pk, cstr(arena, "doc1"), null), "READ");
                System.out.printf("READ status=%d body=%s%n", read.httpStatus(), new String(read.body(), StandardCharsets.UTF_8));

                // 6. DELETE.
                Result delete = submit(arena, drv, q, itemRequest(arena, KIND_DELETE_ITEM, coll, pk, cstr(arena, "doc1"), null), "DELETE");
                System.out.printf("DELETE status=%d%n", delete.httpStatus());
            } finally {
                // 7. Tear-down (LIFO)
                if (!pk.equals(MemorySegment.NULL)) PK_FREE.invokeExact(pk);
                if (!coll.equals(MemorySegment.NULL)) CONTAINER_FREE.invokeExact(coll);
                if (!drv.equals(MemorySegment.NULL)) DRV_FREE.invokeExact(drv);
                if (!acct.equals(MemorySegment.NULL)) ACCT_FREE.invokeExact(acct);
                if (!q.equals(MemorySegment.NULL)) CQ_FREE.invokeExact(q);
                if (!rt.equals(MemorySegment.NULL)) RT_FREE.invokeExact(rt);
            }
        }
    }
}
```

### Go

Pure `cgo`. `go run` after the linker can find `libazurecosmosdriver.{so,dylib,dll}`.

```go
package main

/*
#cgo LDFLAGS: -lazurecosmosdriver
#include <azurecosmosdriver.h>
#include <stdlib.h>
*/
import "C"

import (
    "encoding/json"
    "fmt"
    "log"
    "unsafe"
)

type Doc struct {
    ID   string `json:"id"`
    Pk   string `json:"pk"`
    Name string `json:"name"`
}

const (
    kindCreateItem = 19
    kindReadItem   = 20
    kindDeleteItem = 23
    outcomeOK      = 0
)

type result struct {
    status        C.cosmos_status_code_t
    httpStatus    uint16
    subStatus     int32
    requestCharge float64
    body          []byte
}

func packedHTTP(code C.cosmos_status_code_t) uint16 {
    return uint16(uint32(int32(code)) >> 16)
}

func packedSub(code C.cosmos_status_code_t) uint16 {
    return uint16(uint32(int32(code)) & 0xffff)
}

func hasSub(code C.cosmos_status_code_t) bool {
    return packedSub(code) != 0
}

func formatStatus(code C.cosmos_status_code_t) string {
    if code == 0 {
        return "success"
    }
    if hasSub(code) {
        return fmt.Sprintf("http=%d sub=%d raw=%d", packedHTTP(code), packedSub(code), int32(code))
    }
    return fmt.Sprintf("http=%d raw=%d", packedHTTP(code), int32(code))
}

func checkStatus(rc C.cosmos_status_code_t, err *C.cosmos_error_t, what string) {
    if rc != 0 {
        if err != nil {
            C.cosmos_error_free(err)
        }
        log.Fatalf("%s failed: %s", what, formatStatus(rc))
    }
    if err != nil {
        C.cosmos_error_free(err)
    }
}

// submit issues one request through the singleton entry point and blocks for
// its single completion. The drained completion owns borrowed allocations until
// C.cosmos_completion_queue_free_completions releases them.
//
// The request struct (and every pointer it carries) is only borrowed for the
// duration of the submit call, so all the C strings allocated by callers can be
// freed after submit returns — no ownership crosses the boundary.
func submit(drv *C.cosmos_driver_t, q *C.cosmos_completion_queue_t, req *C.cosmos_operation_request_t) (result, error) {
    var pre C.cosmos_status_code_t
    h := C.cosmos_submit_singleton_operation(drv, req, q, C.intptr_t(0), &pre)
    if h == nil {
        return result{}, fmt.Errorf("submit pre-flight failed: %s", formatStatus(pre))
    }

    var comp C.cosmos_completion_t
    n := C.cosmos_completion_queue_wait(q, &comp, C.uintptr_t(1), C.uint32_t(30000))
    C.cosmos_operation_handle_free(h)
    if n == 0 {
        return result{}, fmt.Errorf("queue drained or shut down before a completion arrived")
    }
    defer C.cosmos_completion_queue_free_completions(&comp, n)

    r := result{
        status:        C.cosmos_status_code_t(comp.status),
        httpStatus:    uint16(comp.http_status_code),
        subStatus:     int32(comp.sub_status),
        requestCharge: float64(comp.request_charge),
    }
    if comp.body != nil && comp.body_len > 0 {
        r.body = C.GoBytes(unsafe.Pointer(comp.body), C.int(comp.body_len))
    }
    if comp.outcome != outcomeOK {
        message := ""
        if comp.message != nil {
            message = C.GoString(comp.message)
        }
        return result{}, fmt.Errorf("operation failed (%s): %s", formatStatus(r.status), message)
    }
    return r, nil
}

// itemRequest builds a flat request for an item operation. partition_key,
// item_id, and body are all borrowed by the submit call, so the caller keeps
// ownership and frees them after submit returns.
func itemRequest(kind C.int32_t, container *C.cosmos_container_ref_t, pk *C.cosmos_partition_key_t, itemID *C.char, body []byte) C.cosmos_operation_request_t {
    req := C.cosmos_operation_request_t{
        kind:              kind,
        container:         container,
        partition_key:     pk,
        item_id:           itemID,
        max_item_count:    -1,
        precondition_kind: 0,
    }
    if len(body) > 0 {
        req.body = (*C.uint8_t)(unsafe.Pointer(&body[0]))
        req.body_len = C.uintptr_t(len(body))
    }
    return req
}

func main() {
    // 1. Runtime + queue. Pass nil for driver defaults. To add a user-agent
    //    suffix, call cosmos_runtime_options_default(), set .user_agent_suffix,
    //    and pass &options to cosmos_runtime_build.
    var rt *C.cosmos_runtime_t
    var err *C.cosmos_error_t
    checkStatus(C.cosmos_runtime_build(nil, &rt, &err), err, "runtime build")
    defer C.cosmos_runtime_free(rt)
    q := C.cosmos_completion_queue_create(rt, nil)
    defer C.cosmos_completion_queue_free(q)

    // 2. Account -> driver -> container.
    endp := C.CString("https://localhost:8081/")
    key := C.CString("C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==")
    var acct *C.cosmos_account_ref_t
    err = nil
    rc := C.cosmos_account_ref_with_master_key(endp, key, &acct, &err)
    C.free(unsafe.Pointer(endp))
    C.free(unsafe.Pointer(key))
    checkStatus(rc, err, "account ref")
    defer C.cosmos_account_ref_free(acct)

    var drv *C.cosmos_driver_t
    err = nil
    checkStatus(C.cosmos_driver_get_or_create_blocking(rt, acct, nil, &drv, &err), err, "driver create")
    defer C.cosmos_driver_free(drv)

    db := C.CString("sample-db")
    collName := C.CString("sample-coll")
    var container *C.cosmos_container_ref_t
    err = nil
    rc = C.cosmos_driver_resolve_container_blocking(rt, drv, db, collName, &container, &err)
    C.free(unsafe.Pointer(db))
    C.free(unsafe.Pointer(collName))
    checkStatus(rc, err, "resolve container")
    defer C.cosmos_container_ref_free(container)

    // 3. Partition key.
    pkVal := C.CString("tenant-42")
    component := C.cosmos_partition_key_component_t{kind: 0, string_value: pkVal}
    var pk *C.cosmos_partition_key_t
    rc = C.cosmos_partition_key_create(&component, C.uintptr_t(1), &pk)
    C.free(unsafe.Pointer(pkVal))
    if rc != 0 {
        log.Fatalf("partition key create failed: %s", formatStatus(rc))
    }
    defer C.cosmos_partition_key_free(pk)

    docID := C.CString("doc1")
    defer C.free(unsafe.Pointer(docID))

    // 4. CREATE.
    body, _ := json.Marshal(Doc{ID: "doc1", Pk: "tenant-42", Name: "hello"})
    createReq := itemRequest(C.int32_t(kindCreateItem), container, pk, nil, body)
    create, errGo := submit(drv, q, &createReq)
    if errGo != nil {
        log.Fatalf("CREATE: %v", errGo)
    }
    fmt.Printf("CREATE status=%d ru=%.2f\n", create.httpStatus, create.requestCharge)

    // 5. READ.
    readReq := itemRequest(C.int32_t(kindReadItem), container, pk, docID, nil)
    read, errGo := submit(drv, q, &readReq)
    if errGo != nil {
        log.Fatalf("READ: %v", errGo)
    }
    fmt.Printf("READ status=%d body=%s\n", read.httpStatus, read.body)

    // 6. DELETE.
    deleteReq := itemRequest(C.int32_t(kindDeleteItem), container, pk, docID, nil)
    deleteResult, errGo := submit(drv, q, &deleteReq)
    if errGo != nil {
        log.Fatalf("DELETE: %v", errGo)
    }
    fmt.Printf("DELETE status=%d\n", deleteResult.httpStatus)

    // All owned handles are released by the deferred frees above, in reverse
    // order of creation.
}
```

### Python (3.10+ with `ctypes`)

No build step. Stdlib `ctypes` only.

```python
import ctypes
import ctypes.util
import json
import sys

_path = ctypes.util.find_library("azurecosmosdriver") or "./libazurecosmosdriver.so"
lib = ctypes.CDLL(_path)

def _decl(name, argtypes, restype):
    fn = getattr(lib, name)
    fn.argtypes = argtypes
    fn.restype = restype
    return fn

void_p = ctypes.c_void_p
size_t = ctypes.c_size_t
intptr_t = ctypes.c_ssize_t
u8_p = ctypes.POINTER(ctypes.c_uint8)
c_char_p = ctypes.c_char_p

# Operation kind values (cosmos_CosmosOperationKind) and completion outcome values.
KIND_CREATE_ITEM = 19
KIND_READ_ITEM   = 20
KIND_DELETE_ITEM = 23
OUTCOME_OK = 0
ERROR_CODE_SUCCESS = 0


def packed_http(code: int) -> int:
    return ctypes.c_uint32(code).value >> 16


def packed_sub(code: int) -> int:
    return ctypes.c_uint32(code).value & 0xffff


def has_sub(code: int) -> bool:
    return packed_sub(code) != 0


def format_status(code: int) -> str:
    if code == 0:
        return "success"
    if has_sub(code):
        return f"http={packed_http(code)} sub={packed_sub(code)} raw={code}"
    return f"http={packed_http(code)} raw={code}"


class CosmosPartitionKeyComponent(ctypes.Structure):
    _fields_ = [
        ("kind", ctypes.c_int32),
        ("string_value", c_char_p),
        ("number_value", ctypes.c_double),
        ("bool_value", ctypes.c_uint8),
    ]


# Flat #[repr(C)] request struct. Only the fields used by item operations are
# populated; everything else stays NULL / sentinel.
class CosmosOperationRequest(ctypes.Structure):
    _fields_ = [
        ("kind", ctypes.c_int32),
        ("account", void_p),
        ("database", void_p),
        ("container", void_p),
        ("item_id", c_char_p),
        ("resource_link", c_char_p),
        ("partition_key", void_p),
        ("partition_key_components", ctypes.POINTER(CosmosPartitionKeyComponent)),
        ("partition_key_len", size_t),
        ("feed_range", void_p),
        ("body", u8_p),
        ("body_len", size_t),
        ("session_token", c_char_p),
        ("activity_id", c_char_p),
        ("continuation_token", c_char_p),
        ("max_item_count", ctypes.c_int32),
        ("max_fan_out", ctypes.c_uint32),
        ("patch_max_attempts", ctypes.c_uint8),
        ("populate_index_metrics", ctypes.c_int8),
        ("populate_query_metrics", ctypes.c_int8),
        ("precondition_kind", ctypes.c_int32),
        ("precondition_etag", c_char_p),
        ("options", void_p),
        ("patch_tracking_id", c_char_p),
        ("patch_tracking_capacity", ctypes.c_uint16),
        ("patch_tracking_retention_seconds", ctypes.c_uint32),
    ]


# A drained completion. All pointers are borrowed until free_completions.
class CosmosCompletion(ctypes.Structure):
    _fields_ = [
        ("outcome", ctypes.c_int32),
        ("status", ctypes.c_int32),
        ("user_data", intptr_t),
        ("was_cancel_requested", ctypes.c_uint8),
        ("http_status_code", ctypes.c_uint16),
        ("sub_status", ctypes.c_int32),
        ("request_charge", ctypes.c_double),
        ("retry_after_ms", ctypes.c_int64),
        ("is_from_wire", ctypes.c_uint8),
        ("message", c_char_p),
        ("activity_id", c_char_p),
        ("session_token", c_char_p),
        ("etag", c_char_p),
        ("continuation", c_char_p),
        ("next_continuation", c_char_p),
        ("backtrace", c_char_p),
        ("headers", void_p),
        ("headers_len", size_t),
        ("body", u8_p),
        ("body_len", size_t),
        ("diagnostics", void_p),
        ("driver", void_p),
        ("container", void_p),
        ("backing", void_p),
    ]


req_p = ctypes.POINTER(CosmosOperationRequest)
comp_p = ctypes.POINTER(CosmosCompletion)
component_p = ctypes.POINTER(CosmosPartitionKeyComponent)

_runtime_build         = _decl("cosmos_runtime_build", [void_p, ctypes.POINTER(void_p), ctypes.POINTER(void_p)], ctypes.c_int32)
_runtime_free          = _decl("cosmos_runtime_free", [void_p], None)
_cq_create             = _decl("cosmos_completion_queue_create", [void_p, void_p], void_p)
_cq_wait               = _decl("cosmos_completion_queue_wait", [void_p, comp_p, size_t, ctypes.c_uint32], size_t)
_cq_free               = _decl("cosmos_completion_queue_free", [void_p], None)
_cq_free_completions   = _decl("cosmos_completion_queue_free_completions", [comp_p, size_t], None)
_acct_with_key         = _decl("cosmos_account_ref_with_master_key", [c_char_p, c_char_p, ctypes.POINTER(void_p), ctypes.POINTER(void_p)], ctypes.c_int32)
_acct_free             = _decl("cosmos_account_ref_free", [void_p], None)
_driver_goc_blk        = _decl("cosmos_driver_get_or_create_blocking", [void_p, void_p, void_p, ctypes.POINTER(void_p), ctypes.POINTER(void_p)], ctypes.c_int32)
_driver_free           = _decl("cosmos_driver_free", [void_p], None)
_resolve_container_blk = _decl("cosmos_driver_resolve_container_blocking", [void_p, void_p, c_char_p, c_char_p, ctypes.POINTER(void_p), ctypes.POINTER(void_p)], ctypes.c_int32)
_container_free        = _decl("cosmos_container_ref_free", [void_p], None)
_pk_create             = _decl("cosmos_partition_key_create", [component_p, size_t, ctypes.POINTER(void_p)], ctypes.c_int32)
_pk_free               = _decl("cosmos_partition_key_free", [void_p], None)
_submit_singleton      = _decl("cosmos_submit_singleton_operation", [void_p, req_p, void_p, intptr_t, ctypes.POINTER(ctypes.c_int32)], void_p)
_op_hnd_free           = _decl("cosmos_operation_handle_free", [void_p], None)
_error_free            = _decl("cosmos_error_free", [void_p], None)


def check_status(code: int, err: void_p, what: str) -> None:
    try:
        if code != ERROR_CODE_SUCCESS:
            raise RuntimeError(f"{what} failed: {format_status(code)}")
    finally:
        if err:
            _error_free(err)


def submit(drv, q, req):
    """Issue one request and block for its single completion.

    Returns copied completion data on success. The request struct is only
    borrowed for the call, so the caller's buffers stay valid here and can be
    released afterward.
    """
    pre = ctypes.c_int32(0)
    h = _submit_singleton(drv, ctypes.byref(req), q, intptr_t(0), ctypes.byref(pre))
    if not h:
        raise RuntimeError(f"submit pre-flight failed: {format_status(pre.value)}")
    comp = CosmosCompletion()
    n = 0
    try:
        n = _cq_wait(q, ctypes.byref(comp), 1, 30_000)
    finally:
        _op_hnd_free(h)
    if n == 0:
        raise RuntimeError("queue drained or shut down before a completion arrived")

    try:
        message = comp.message.decode("utf-8") if comp.message else ""
        body = ctypes.string_at(comp.body, comp.body_len) if comp.body and comp.body_len else b""
        if comp.outcome != OUTCOME_OK:
            raise RuntimeError(f"operation failed ({format_status(comp.status)}): {message}")
        return {
            "status": comp.status,
            "http_status": comp.http_status_code,
            "sub_status": comp.sub_status,
            "request_charge": comp.request_charge,
            "body": body,
        }
    finally:
        _cq_free_completions(ctypes.byref(comp), n)


def item_request(kind, container, pk, item_id=None, body=b""):
    """Build a flat request for an item operation. `container`, `pk`, `item_id`,
    and `body` are borrowed by the submit call; the caller keeps ownership."""
    req = CosmosOperationRequest()
    req.kind = kind
    req.container = container
    req.partition_key = pk
    req.item_id = item_id
    req.max_item_count = -1
    if body:
        buf = (ctypes.c_uint8 * len(body)).from_buffer_copy(body)
        req.body = ctypes.cast(buf, u8_p)
        req.body_len = len(body)
        req._body_buf = buf  # keep the backing buffer alive for the call
    return req


def main() -> int:
    rt = void_p()
    q = None
    acct = void_p()
    drv = void_p()
    container = void_p()
    pk = void_p()
    try:
        # 1. Runtime + queue. Pass None for driver defaults. To add a user-agent
        #    suffix, call cosmos_runtime_options_default(), set .user_agent_suffix,
        #    and pass byref(options) to cosmos_runtime_build.
        err = void_p()
        check_status(_runtime_build(None, ctypes.byref(rt), ctypes.byref(err)), err, "runtime build")
        q = _cq_create(rt, None)

        # 2. Account -> driver -> container.
        err = void_p()
        check_status(_acct_with_key(
            b"https://localhost:8081/",
            b"C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==",
            ctypes.byref(acct), ctypes.byref(err),
        ), err, "account ref")
        err = void_p()
        check_status(_driver_goc_blk(rt, acct, None, ctypes.byref(drv), ctypes.byref(err)), err, "driver create")
        err = void_p()
        check_status(_resolve_container_blk(rt, drv, b"sample-db", b"sample-coll", ctypes.byref(container), ctypes.byref(err)), err, "resolve container")

        # 3. Partition key.
        component = CosmosPartitionKeyComponent(kind=0, string_value=b"tenant-42")
        check_status(_pk_create(ctypes.byref(component), 1, ctypes.byref(pk)), None, "partition key create")

        # 4. CREATE.
        body = json.dumps({"id": "doc1", "pk": "tenant-42", "name": "hello"}).encode("utf-8")
        create = submit(drv, q, item_request(KIND_CREATE_ITEM, container, pk, body=body))
        print(f"CREATE status={create['http_status']} ru={create['request_charge']:.2f}")

        # 5. READ.
        read = submit(drv, q, item_request(KIND_READ_ITEM, container, pk, b"doc1"))
        print(f"READ status={read['http_status']} body={read['body'].decode('utf-8')}")

        # 6. DELETE.
        delete = submit(drv, q, item_request(KIND_DELETE_ITEM, container, pk, b"doc1"))
        print(f"DELETE status={delete['http_status']}")
        return 0
    finally:
        # Tear down every owned handle that was successfully created, in
        # reverse order. The guards make this safe even if an early step
        # raised before its handle was populated.
        if pk:
            _pk_free(pk)
        if container:
            _container_free(container)
        if drv:
            _driver_free(drv)
        if acct:
            _acct_free(acct)
        if q:
            _cq_free(q)
        if rt:
            _runtime_free(rt)


if __name__ == "__main__":
    sys.exit(main())
```

---

## Notes that apply to all four bindings

1. **Handle every return code and outcome.** The Go and Python samples above
   show the pattern end-to-end: check the pre-flight `cosmos_status_code_t`,
   check `cosmos_completion_outcome` against `OK`, and on a non-OK outcome pull
   `cosmos_completion_take_error` to read the rich `cosmos_error_t` (then free
   it) before deciding whether to retry / surface / log. The C# and Java
   samples abbreviate this for space, but production bindings should follow the
   Go/Python shape.
2. **Single producer / single consumer per queue is the v1 contract.**
   Multiple producers (one per submit thread) are fine; multiple concurrent
   `cosmos_cq_wait` consumers on the same queue are undefined behavior. Most
   language bindings will dedicate one "receive-loop" thread that
   demultiplexes completions into per-call condition variables / channels via
   the `user_data` correlation pointer.
3. **`user_data`** is opaque — round-tripped verbatim onto the completion. The
   standard pattern is to allocate a small per-call struct on the heap, pass
   its pointer as `user_data`, and use it on the consumer side to correlate
   the completion with the calling thread.
4. **Lifetime ownership cheat-sheet:**
   - `_blocking` / `_create` / `_get_or_create_*` / `_build` produce handles
     the caller owns and must `_free`.
   - The submit entry points
     (`cosmos_submit_singleton_operation` /
     `cosmos_submit_operation`) only **borrow** the
     `cosmos_CosmosOperationRequest` and every pointer it carries for the
     duration of the call; the wrapper copies what it needs before returning,
     so the host may free its buffers immediately afterward.
   - `cosmos_completion_take_response` / `_take_error` transfer ownership out
     of the completion; the response/error is freed independently.
   - Completion handles must be freed via `cosmos_completion_free`.
   - Operation handles (`cosmos_operation_handle_t *`) returned by the submit
     entry points are freed via `cosmos_operation_handle_free`.
5. **Schema-agnostic data plane.** The wrapper never serializes user
   payloads — host SDKs build JSON (or any other body format the service
   accepts) themselves and hand the bytes to the request via
   `cosmos_operation_request_t.body` / `.body_len`.
   Bytes are **copied** before the submit call returns; callers may release
   their source buffer immediately.
6. **Diagnostics-on-error** is currently only available via the rich
   `cosmos_error_t` on `outcome == ERROR` completions. The success-path
   `cosmos_response_diagnostics` accessor is a planned follow-up.
7. **Single-runtime caching.** Drivers are cached by endpoint URL on the
   `cosmos_runtime_t` that created them. Multiple `cosmos_runtime_t`
   instances do **not** share their caches — see
   [section 4.4.1 in the spec](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md)
   for the full contract.

## Repository archaeology — files removed by PR #4103

The earlier `azure_data_cosmos_native` crate (removed in
[PR #4103](https://github.com/Azure/azure-sdk-for-rust/pull/4103),
commit `ccf43caae`) shipped a handful of files that have **not** been
reintroduced in this crate; their content now lives elsewhere:

| Old file                                                     | New location                                                                                                                                                           |
| ------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `azurecosmos.pc.in` (pkg-config template)                    | This crate ships a sibling `azurecosmosdriver.pc.in` with the same shape but a new package name.                                                                       |
| `docs/next_generation_sdks_design_principles.md`             | Folded into [NATIVE_WRAPPER_SPEC.md section 2](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md). |
| `c_tests/test_common.h` runtime / client / database fixtures | Re-added incrementally as the corresponding C entry points land.                                                                                                       |

If you are spelunking the git history of the old crate looking for a behavior
or test that "should be here", that table is the first place to check.
