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

| Capability | Status |
|---|---|
| Master-key authentication | ✅ |
| Token-credential / resource-token authentication | ⏳ follow-up (needs `TokenCredential` FFI bridge) |
| Sync driver creation (`_blocking`) | ✅ |
| Async driver creation (`_submit`) | ✅ |
| Cache-hit advisory (`5001 OPTIONS_IGNORED_ON_CACHE_HIT`) | ⏳ needs driver-side `was_cached` signal |
| Sync + async `resolve_container` | ✅ |
| Single + hierarchical partition keys | ✅ |
| Item-CRUD operations (read / create / upsert / replace / delete / patch) | ✅ |
| Container-CRUD operations (read / replace / delete) | ✅ |
| Database + account-scope operations | ✅ |
| `cosmos_submit_singleton_operation` (point ops) | ✅ |
| `cosmos_submit_operation` (feeds + pagination) | ✅ |
| Response status / RU / body / activity-id / session-token / etag / continuation | ✅ |
| Pagination (read-feeds + query result sets) | ⏳ planned |
| Multi-part response body iteration | ⏳ planned |
| Diagnostics accessors | ⏳ planned |
| Patch instruction builder | ⏳ planned |
| Transactional batch sub-operation builder | ⏳ planned |
| Custom per-operation request headers | ✅ via `cosmos_CosmosOperationOptions.custom_headers` (array of `cosmos_CosmosHeaderKv`) |

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
> self-describing `cosmos_CosmosOperationRequest` struct (kind-tagged via
> `cosmos_CosmosOperationKind`, with per-call settings on the tri-state
> `cosmos_CosmosOperationOptions` seeded by `cosmos_operation_options_default`)
> and executed through exactly two entry points:
>
> - `cosmos_submit_singleton_operation` — point operations
>   (create / read / replace / delete / patch item, database & container CRUD,
>   read/replace offer).
> - `cosmos_submit_operation` — feed/paginated operations
>   (queries, read-all, change feed); resumes from and surfaces a continuation
>   token.
>
> Both take `(driver, const cosmos_CosmosOperationRequest *request, queue,
> user_data, out_pre_error)` and return a `cosmos_operation_handle_t *`.
> The checked-in [header](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver_native/include/azurecosmosdriver.h) is the authoritative
> source for the struct field layout and the 25 operation kinds. The C#
> example below is written against this new API; the Java, Go, and Python
> examples that follow are pending migration and currently show the **old**
> factory flow — translate them field-for-field from the C# example and the
> header until they are updated.

> **Minimizing FFI round-trips.** Two parts of the surface let a host avoid
> chatty per-field calls:
>
> - **Inline partition keys.** Instead of the
>   `cosmos_partition_key_builder_new` / `_add_*` / `_build` / `_free` dance,
>   fill an array of `cosmos_partition_key_component_t` (a tagged union: a
>   `kind` plus `string_value` / `number_value` / `bool_value`) and point
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

    [DllImport(Lib)] public static extern IntPtr cosmos_runtime_builder_new();
    [DllImport(Lib)] public static extern int    cosmos_runtime_builder_with_user_agent_suffix(IntPtr b, byte[] suffix);
    [DllImport(Lib)] public static extern int    cosmos_runtime_builder_build(IntPtr b, out IntPtr runtime, out IntPtr err);
    [DllImport(Lib)] public static extern void   cosmos_runtime_free(IntPtr runtime);

    [DllImport(Lib)] public static extern IntPtr cosmos_cq_create(IntPtr runtime, IntPtr options);
    [DllImport(Lib)] public static extern IntPtr cosmos_cq_wait(IntPtr q, uint timeoutMs);
    [DllImport(Lib)] public static extern void   cosmos_cq_free(IntPtr q);

    [DllImport(Lib)] public static extern int    cosmos_account_ref_with_master_key(byte[] endpoint, byte[] key, out IntPtr acct, out IntPtr err);
    [DllImport(Lib)] public static extern void   cosmos_account_ref_free(IntPtr a);
    [DllImport(Lib)] public static extern int    cosmos_driver_get_or_create_blocking(IntPtr rt, IntPtr acct, IntPtr opts, out IntPtr drv, out IntPtr err);
    [DllImport(Lib)] public static extern void   cosmos_driver_free(IntPtr d);
    [DllImport(Lib)] public static extern int    cosmos_driver_resolve_container_blocking(IntPtr rt, IntPtr drv, byte[] db, byte[] coll, out IntPtr c, out IntPtr err);
    [DllImport(Lib)] public static extern void   cosmos_container_ref_free(IntPtr c);

    [DllImport(Lib)] public static extern IntPtr cosmos_partition_key_builder_new();
    [DllImport(Lib)] public static extern int    cosmos_partition_key_builder_add_string(IntPtr b, byte[] v);
    [DllImport(Lib)] public static extern int    cosmos_partition_key_builder_build(IntPtr b, out IntPtr pk);
    [DllImport(Lib)] public static extern void   cosmos_partition_key_free(IntPtr pk);

    // Operation kinds (subset — see `cosmos_CosmosOperationKind` in the header).
    public const int KIND_CREATE_ITEM = 19;
    public const int KIND_READ_ITEM   = 20;
    public const int KIND_DELETE_ITEM = 23;

    // Flat, self-describing request (mirrors `cosmos_CosmosOperationRequest`).
    // Fill only the fields the `kind` needs; leave the rest NULL / sentinel.
    [StructLayout(LayoutKind.Sequential)]
    public struct OpRequest
    {
        public int       kind;
        public IntPtr    account;
        public IntPtr    database;
        public IntPtr    container;
        public IntPtr    item_id;             // char*
        public IntPtr    resource_link;       // char*
        public IntPtr    partition_key;
        public IntPtr    feed_range;
        public IntPtr    body;                // const uint8_t* — NULL iff body_len == 0
        public UIntPtr   body_len;            // 0 = no body
        public IntPtr    session_token;       // char*
        public IntPtr    activity_id;         // char*
        public IntPtr    continuation_token;  // char*
        public int       max_item_count;      // < 0 = unset
        public byte      patch_max_attempts;  // 0 = unset
        public sbyte     populate_index_metrics; // tri-state bool (0/1/2)
        public sbyte     populate_query_metrics; // tri-state bool (0/1/2)
        public int       precondition_kind;   // 0 = none
        public IntPtr    precondition_etag;   // char*
        public IntPtr    options;             // cosmos_CosmosOperationOptions*
    }

    // The two — and only two — execution entry points.
    [DllImport(Lib)] public static extern IntPtr cosmos_submit_singleton_operation(IntPtr drv, ref OpRequest req, IntPtr q, IntPtr ud, out int preErr);
    [DllImport(Lib)] public static extern IntPtr cosmos_submit_operation(IntPtr drv, ref OpRequest req, IntPtr q, IntPtr ud, out int preErr);
    [DllImport(Lib)] public static extern void   cosmos_operation_handle_free(IntPtr h);

    [DllImport(Lib)] public static extern int    cosmos_completion_outcome(IntPtr c);
    [DllImport(Lib)] public static extern IntPtr cosmos_completion_take_response(IntPtr c);
    [DllImport(Lib)] public static extern IntPtr cosmos_completion_take_error(IntPtr c);
    [DllImport(Lib)] public static extern void   cosmos_completion_free(IntPtr c);
    [DllImport(Lib)] public static extern ushort cosmos_response_status_code(IntPtr r);
    [DllImport(Lib)] public static extern double cosmos_response_request_charge(IntPtr r);
    [DllImport(Lib)] public static extern int    cosmos_response_body(IntPtr r, out IntPtr data, out UIntPtr len);
    [DllImport(Lib)] public static extern void   cosmos_response_free(IntPtr r);

    public static byte[] Cstr(string s) => Encoding.UTF8.GetBytes(s + "\0");
}

internal static class Program
{
    static IntPtr SubmitAndWait(IntPtr drv, ref Cosmos.OpRequest req, IntPtr q)
    {
        var h = Cosmos.cosmos_submit_singleton_operation(drv, ref req, q, IntPtr.Zero, out int pre);
        if (h == IntPtr.Zero) throw new InvalidOperationException($"submit pre-flight failed: {pre}");
        var c = Cosmos.cosmos_cq_wait(q, 30_000);
        Cosmos.cosmos_operation_handle_free(h);
        return c;
    }

    static void Main()
    {
        // 1. Runtime + queue
        var rb = Cosmos.cosmos_runtime_builder_new();
        Cosmos.cosmos_runtime_builder_with_user_agent_suffix(rb, Cosmos.Cstr("dotnet-sample"));
        Cosmos.cosmos_runtime_builder_build(rb, out var rt, out _);
        var q = Cosmos.cosmos_cq_create(rt, IntPtr.Zero);

        // 2. Account → driver → container
        Cosmos.cosmos_account_ref_with_master_key(
            Cosmos.Cstr("https://localhost:8081/"),
            Cosmos.Cstr("C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw=="),
            out var acct, out _);
        Cosmos.cosmos_driver_get_or_create_blocking(rt, acct, IntPtr.Zero, out var drv, out _);
        Cosmos.cosmos_driver_resolve_container_blocking(rt, drv, Cosmos.Cstr("sample-db"), Cosmos.Cstr("sample-coll"), out var coll, out _);

        // 3. Partition key
        var pkb = Cosmos.cosmos_partition_key_builder_new();
        Cosmos.cosmos_partition_key_builder_add_string(pkb, Cosmos.Cstr("tenant-42"));
        Cosmos.cosmos_partition_key_builder_build(pkb, out var pk);

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
            var comp = SubmitAndWait(drv, ref req, q);
            var resp = Cosmos.cosmos_completion_take_response(comp);
            Console.WriteLine($"CREATE status={Cosmos.cosmos_response_status_code(resp)} ru={Cosmos.cosmos_response_request_charge(resp):F2}");
            Cosmos.cosmos_response_free(resp); Cosmos.cosmos_completion_free(comp);
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
            var comp = SubmitAndWait(drv, ref req, q);
            var resp = Cosmos.cosmos_completion_take_response(comp);
            Cosmos.cosmos_response_body(resp, out var dataPtr, out var dataLen);
            Console.WriteLine($"READ status={Cosmos.cosmos_response_status_code(resp)} body={Marshal.PtrToStringUTF8(dataPtr, (int)dataLen)}");
            Cosmos.cosmos_response_free(resp); Cosmos.cosmos_completion_free(comp);

            // 6. DELETE — same shape as READ, different kind.
            var del = new Cosmos.OpRequest
            {
                kind           = Cosmos.KIND_DELETE_ITEM,
                container      = coll,
                partition_key  = pk,
                item_id        = idPin.AddrOfPinnedObject(),
                max_item_count = -1,
            };
            comp = SubmitAndWait(drv, ref del, q);
            resp = Cosmos.cosmos_completion_take_response(comp);
            Console.WriteLine($"DELETE status={Cosmos.cosmos_response_status_code(resp)}");
            Cosmos.cosmos_response_free(resp); Cosmos.cosmos_completion_free(comp);
        }
        finally { idPin.Free(); }

        // 7. Tear-down (LIFO)
        Cosmos.cosmos_partition_key_free(pk);
        Cosmos.cosmos_container_ref_free(coll);
        Cosmos.cosmos_driver_free(drv);
        Cosmos.cosmos_account_ref_free(acct);
        Cosmos.cosmos_cq_free(q);
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

    static final MethodHandle RT_BUILDER_NEW    = h("cosmos_runtime_builder_new",   FunctionDescriptor.of(ADDRESS));
    static final MethodHandle RT_BUILDER_UA     = h("cosmos_runtime_builder_with_user_agent_suffix", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS));
    static final MethodHandle RT_BUILDER_BUILD  = h("cosmos_runtime_builder_build", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS));
    static final MethodHandle RT_FREE           = h("cosmos_runtime_free",          FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle CQ_CREATE         = h("cosmos_cq_create",             FunctionDescriptor.of(ADDRESS, ADDRESS, ADDRESS));
    static final MethodHandle CQ_WAIT           = h("cosmos_cq_wait",               FunctionDescriptor.of(ADDRESS, ADDRESS, JAVA_INT));
    static final MethodHandle CQ_FREE           = h("cosmos_cq_free",               FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle ACCT_WITH_KEY     = h("cosmos_account_ref_with_master_key", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, ADDRESS));
    static final MethodHandle ACCT_FREE         = h("cosmos_account_ref_free",      FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle DRV_GOC_BLK       = h("cosmos_driver_get_or_create_blocking", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS));
    static final MethodHandle DRV_FREE          = h("cosmos_driver_free",           FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle RESOLVE_BLK       = h("cosmos_driver_resolve_container_blocking", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS));
    static final MethodHandle CONTAINER_FREE    = h("cosmos_container_ref_free",    FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle PKB_NEW           = h("cosmos_partition_key_builder_new",        FunctionDescriptor.of(ADDRESS));
    static final MethodHandle PKB_ADD_S         = h("cosmos_partition_key_builder_add_string", FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS));
    static final MethodHandle PKB_BUILD         = h("cosmos_partition_key_builder_build",      FunctionDescriptor.of(JAVA_INT, ADDRESS, ADDRESS));
    static final MethodHandle PK_FREE           = h("cosmos_partition_key_free",               FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle SUBMIT_SINGLETON  = h("cosmos_submit_singleton_operation", FunctionDescriptor.of(ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS));
    static final MethodHandle OP_HND_FREE       = h("cosmos_operation_handle_free", FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle COMP_OUTCOME      = h("cosmos_completion_outcome",    FunctionDescriptor.of(JAVA_INT, ADDRESS));
    static final MethodHandle COMP_STATUS       = h("cosmos_completion_status",     FunctionDescriptor.of(JAVA_INT, ADDRESS));
    static final MethodHandle COMP_TAKE_R       = h("cosmos_completion_take_response", FunctionDescriptor.of(ADDRESS, ADDRESS));
    static final MethodHandle COMP_TAKE_E       = h("cosmos_completion_take_error",  FunctionDescriptor.of(ADDRESS, ADDRESS));
    static final MethodHandle COMP_FREE         = h("cosmos_completion_free",       FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle ERR_MESSAGE       = h("cosmos_error_message",         FunctionDescriptor.of(ADDRESS, ADDRESS));
    static final MethodHandle ERR_FREE          = h("cosmos_error_free",            FunctionDescriptor.ofVoid(ADDRESS));
    static final MethodHandle RESP_STATUS       = h("cosmos_response_status_code",  FunctionDescriptor.of(JAVA_SHORT, ADDRESS));
    static final MethodHandle RESP_RU           = h("cosmos_response_request_charge", FunctionDescriptor.of(JAVA_DOUBLE, ADDRESS));
    static final MethodHandle RESP_FREE         = h("cosmos_response_free",         FunctionDescriptor.ofVoid(ADDRESS));

    // Operation kind (cosmos_operation_kind_t) and outcome (cosmos_completion_outcome_t).
    static final int KIND_CREATE_ITEM = 19;
    static final int OUTCOME_OK = 0;

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
        ADDRESS.withName("feed_range"),
        ADDRESS.withName("body"),
        JAVA_LONG.withName("body_len"),
        ADDRESS.withName("session_token"),
        ADDRESS.withName("activity_id"),
        ADDRESS.withName("continuation_token"),
        JAVA_INT.withName("max_item_count"),
        JAVA_BYTE.withName("patch_max_attempts"),
        JAVA_BYTE.withName("populate_index_metrics"),
        JAVA_BYTE.withName("populate_query_metrics"),
        MemoryLayout.paddingLayout(1),
        JAVA_INT.withName("precondition_kind"),
        MemoryLayout.paddingLayout(4),
        ADDRESS.withName("precondition_etag"),
        ADDRESS.withName("options"));

    public static void main(String[] args) throws Throwable {
        try (Arena arena = Arena.ofConfined()) {
            // 1. Runtime + queue
            MemorySegment rb = (MemorySegment) RT_BUILDER_NEW.invokeExact();
            RT_BUILDER_UA.invokeExact(rb, cstr(arena, "java-sample"));
            MemorySegment outRt = arena.allocate(ADDRESS);
            RT_BUILDER_BUILD.invokeExact(rb, outRt, MemorySegment.NULL);
            MemorySegment rt = outRt.get(ADDRESS, 0);
            MemorySegment q  = (MemorySegment) CQ_CREATE.invokeExact(rt, MemorySegment.NULL);

            // 2. Account -> driver -> container
            MemorySegment outAcct = arena.allocate(ADDRESS);
            ACCT_WITH_KEY.invokeExact(
                cstr(arena, "https://localhost:8081/"),
                cstr(arena, "C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw=="),
                outAcct, MemorySegment.NULL);
            MemorySegment acct = outAcct.get(ADDRESS, 0);
            MemorySegment outDrv = arena.allocate(ADDRESS);
            DRV_GOC_BLK.invokeExact(rt, acct, MemorySegment.NULL, outDrv, MemorySegment.NULL);
            MemorySegment drv = outDrv.get(ADDRESS, 0);
            MemorySegment outColl = arena.allocate(ADDRESS);
            RESOLVE_BLK.invokeExact(rt, drv, cstr(arena, "sample-db"), cstr(arena, "sample-coll"), outColl, MemorySegment.NULL);
            MemorySegment coll = outColl.get(ADDRESS, 0);

            // 3. Partition key
            MemorySegment pkb = (MemorySegment) PKB_NEW.invokeExact();
            PKB_ADD_S.invokeExact(pkb, cstr(arena, "tenant-42"));
            MemorySegment outPk = arena.allocate(ADDRESS);
            PKB_BUILD.invokeExact(pkb, outPk);
            MemorySegment pk = outPk.get(ADDRESS, 0);

            // 4. CREATE — host SDK serializes its own JSON (Jackson, Gson, ...).
            byte[] body = "{\"id\":\"doc1\",\"pk\":\"tenant-42\",\"name\":\"hello\"}".getBytes(StandardCharsets.UTF_8);
            MemorySegment bodySeg = arena.allocate(body.length);
            MemorySegment.copy(body, 0, bodySeg, JAVA_BYTE, 0, body.length);

            // Fill the flat request. Only the fields CREATE needs are set; the
            // rest stay zero/NULL (max_item_count = -1 means "unset"). The
            // request and every segment it points at are borrowed only for the
            // submit call — the confined arena keeps them alive across it.
            MemorySegment req = arena.allocate(REQUEST);
            req.set(JAVA_INT, REQUEST.byteOffset(MemoryLayout.PathElement.groupElement("kind")), KIND_CREATE_ITEM);
            req.set(ADDRESS, REQUEST.byteOffset(MemoryLayout.PathElement.groupElement("container")), coll);
            req.set(ADDRESS, REQUEST.byteOffset(MemoryLayout.PathElement.groupElement("partition_key")), pk);
            req.set(ADDRESS, REQUEST.byteOffset(MemoryLayout.PathElement.groupElement("item_id")), cstr(arena, "doc1"));
            req.set(ADDRESS, REQUEST.byteOffset(MemoryLayout.PathElement.groupElement("body")), bodySeg);
            req.set(JAVA_LONG, REQUEST.byteOffset(MemoryLayout.PathElement.groupElement("body_len")), (long) body.length);
            req.set(JAVA_INT, REQUEST.byteOffset(MemoryLayout.PathElement.groupElement("max_item_count")), -1);

            MemorySegment preErr = arena.allocate(JAVA_INT);
            MemorySegment hdl = (MemorySegment) SUBMIT_SINGLETON.invokeExact(drv, req, q, MemorySegment.NULL, preErr);
            if (hdl.equals(MemorySegment.NULL)) {
                throw new RuntimeException("submit pre-flight failed: " + preErr.get(JAVA_INT, 0));
            }
            MemorySegment comp = (MemorySegment) CQ_WAIT.invokeExact(q, 30_000);
            OP_HND_FREE.invokeExact(hdl);

            int outcome = (int) COMP_OUTCOME.invokeExact(comp);
            if (outcome != OUTCOME_OK) {
                MemorySegment err = (MemorySegment) COMP_TAKE_E.invokeExact(comp);
                String detail = "";
                if (!err.equals(MemorySegment.NULL)) {
                    MemorySegment msg = (MemorySegment) ERR_MESSAGE.invokeExact(err);
                    if (!msg.equals(MemorySegment.NULL)) {
                        detail = msg.reinterpret(Long.MAX_VALUE).getUtf8String(0);
                    }
                    ERR_FREE.invokeExact(err);
                }
                int status = (int) COMP_STATUS.invokeExact(comp);
                COMP_FREE.invokeExact(comp);
                throw new RuntimeException("CREATE failed (status=" + status + "): " + detail);
            }
            MemorySegment resp = (MemorySegment) COMP_TAKE_R.invokeExact(comp);
            System.out.printf("CREATE status=%d ru=%.2f%n",
                (short) RESP_STATUS.invokeExact(resp),
                (double) RESP_RU.invokeExact(resp));
            RESP_FREE.invokeExact(resp);
            COMP_FREE.invokeExact(comp);

            // 5/6. READ + DELETE — same shape (new request with kind = READ/DELETE
            // and no body); omitted for brevity.

            // 7. Tear-down (LIFO)
            PK_FREE.invokeExact(pk);
            CONTAINER_FREE.invokeExact(coll);
            DRV_FREE.invokeExact(drv);
            ACCT_FREE.invokeExact(acct);
            CQ_FREE.invokeExact(q);
            RT_FREE.invokeExact(rt);
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

// submit issues one request through the singleton entry point and blocks for
// its single completion. The caller owns the returned completion and must free
// it with C.cosmos_completion_free.
//
// The request struct (and every pointer it carries) is only borrowed for the
// duration of the submit call, so all the C strings allocated here are freed
// on return via defer — no ownership crosses the boundary.
func submit(drv *C.cosmos_driver_t, q *C.cosmos_cq_t, req *C.cosmos_operation_request_t) (*C.cosmos_completion_t, error) {
    var pre C.cosmos_error_code_t
    h := C.cosmos_submit_singleton_operation(drv, req, q, nil, &pre)
    if h == nil {
        return nil, fmt.Errorf("submit pre-flight failed: %d", int32(pre))
    }
    // The operation handle is the in-flight identity; we don't need it for a
    // blocking call, so release it as soon as the completion arrives.
    defer C.cosmos_operation_handle_free(h)

    comp := C.cosmos_cq_wait(q, 30000)
    if comp == nil {
        return nil, fmt.Errorf("queue drained or shut down before a completion arrived")
    }
    if outcome := C.cosmos_completion_outcome(comp); outcome != C.COSMOS_COMPLETION_OUTCOME_OK {
        // Pull the rich error off the completion, surface it, and free both.
        defer C.cosmos_completion_free(comp)
        if err := C.cosmos_completion_take_error(comp); err != nil {
            defer C.cosmos_error_free(err)
            msg := C.GoString(C.cosmos_error_message(err))
            return nil, fmt.Errorf("operation failed (status=%d): %s", int32(C.cosmos_completion_status(comp)), msg)
        }
        return nil, fmt.Errorf("operation failed (status=%d)", int32(C.cosmos_completion_status(comp)))
    }
    return comp, nil
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
    // 1. Runtime + queue. Each owned handle is freed in reverse order via defer
    //    immediately after it is created, so an early return never leaks.
    rb := C.cosmos_runtime_builder_new()
    ua := C.CString("go-sample")
    C.cosmos_runtime_builder_with_user_agent_suffix(rb, ua)
    C.free(unsafe.Pointer(ua))
    var rt *C.cosmos_runtime_t
    if rc := C.cosmos_runtime_builder_build(rb, &rt, nil); rc != C.COSMOS_ERROR_CODE_SUCCESS {
        log.Fatalf("runtime build failed: %d", int32(rc))
    }
    defer C.cosmos_runtime_free(rt)
    q := C.cosmos_cq_create(rt, nil)
    defer C.cosmos_cq_free(q)

    // 2. Account -> driver -> container.
    endp := C.CString("https://localhost:8081/")
    key := C.CString("C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==")
    var acct *C.cosmos_account_ref_t
    rc := C.cosmos_account_ref_with_master_key(endp, key, &acct, nil)
    C.free(unsafe.Pointer(endp))
    C.free(unsafe.Pointer(key))
    if rc != C.COSMOS_ERROR_CODE_SUCCESS {
        log.Fatalf("account ref failed: %d", int32(rc))
    }
    defer C.cosmos_account_ref_free(acct)

    var drv *C.cosmos_driver_t
    if rc := C.cosmos_driver_get_or_create_blocking(rt, acct, nil, &drv, nil); rc != C.COSMOS_ERROR_CODE_SUCCESS {
        log.Fatalf("driver create failed: %d", int32(rc))
    }
    defer C.cosmos_driver_free(drv)

    db := C.CString("sample-db")
    coll := C.CString("sample-coll")
    var container *C.cosmos_container_ref_t
    rc = C.cosmos_driver_resolve_container_blocking(rt, drv, db, coll, &container, nil)
    C.free(unsafe.Pointer(db))
    C.free(unsafe.Pointer(coll))
    if rc != C.COSMOS_ERROR_CODE_SUCCESS {
        log.Fatalf("resolve container failed: %d", int32(rc))
    }
    defer C.cosmos_container_ref_free(container)

    // 3. Partition key.
    pkb := C.cosmos_partition_key_builder_new()
    pkVal := C.CString("tenant-42")
    C.cosmos_partition_key_builder_add_string(pkb, pkVal)
    C.free(unsafe.Pointer(pkVal))
    var pk *C.cosmos_partition_key_t
    if rc := C.cosmos_partition_key_builder_build(pkb, &pk); rc != C.COSMOS_ERROR_CODE_SUCCESS {
        log.Fatalf("partition key build failed: %d", int32(rc))
    }
    defer C.cosmos_partition_key_free(pk)

    docID := C.CString("doc1")
    defer C.free(unsafe.Pointer(docID))

    // 4. CREATE.
    body, _ := json.Marshal(Doc{ID: "doc1", Pk: "tenant-42", Name: "hello"})
    createReq := itemRequest(C.COSMOS_OPERATION_KIND_CREATE_ITEM, container, pk, docID, body)
    comp, err := submit(drv, q, &createReq)
    if err != nil {
        log.Fatalf("CREATE: %v", err)
    }
    resp := C.cosmos_completion_take_response(comp)
    fmt.Printf("CREATE status=%d ru=%.2f\n", C.cosmos_response_status_code(resp), C.cosmos_response_request_charge(resp))
    C.cosmos_response_free(resp)
    C.cosmos_completion_free(comp)

    // 5. READ.
    readReq := itemRequest(C.COSMOS_OPERATION_KIND_READ_ITEM, container, pk, docID, nil)
    comp, err = submit(drv, q, &readReq)
    if err != nil {
        log.Fatalf("READ: %v", err)
    }
    resp = C.cosmos_completion_take_response(comp)
    var dataPtr *C.uint8_t
    var dataLen C.uintptr_t
    C.cosmos_response_body(resp, &dataPtr, &dataLen)
    read := C.GoBytes(unsafe.Pointer(dataPtr), C.int(dataLen))
    fmt.Printf("READ status=%d body=%s\n", C.cosmos_response_status_code(resp), read)
    C.cosmos_response_free(resp)
    C.cosmos_completion_free(comp)

    // 6. DELETE.
    deleteReq := itemRequest(C.COSMOS_OPERATION_KIND_DELETE_ITEM, container, pk, docID, nil)
    comp, err = submit(drv, q, &deleteReq)
    if err != nil {
        log.Fatalf("DELETE: %v", err)
    }
    resp = C.cosmos_completion_take_response(comp)
    fmt.Printf("DELETE status=%d\n", C.cosmos_response_status_code(resp))
    C.cosmos_response_free(resp)
    C.cosmos_completion_free(comp)

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
u8_p   = ctypes.POINTER(ctypes.c_uint8)
c_char_p = ctypes.c_char_p

# Operation kinds (cosmos_operation_kind_t) and outcome (cosmos_completion_outcome_t).
KIND_CREATE_ITEM = 19
KIND_READ_ITEM   = 20
KIND_DELETE_ITEM = 23
OUTCOME_OK = 0
ERROR_CODE_SUCCESS = 0

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
        ("feed_range", void_p),
        ("body", u8_p),
        ("body_len", size_t),
        ("session_token", c_char_p),
        ("activity_id", c_char_p),
        ("continuation_token", c_char_p),
        ("max_item_count", ctypes.c_int32),
        ("patch_max_attempts", ctypes.c_uint8),
        ("populate_index_metrics", ctypes.c_int8),
        ("populate_query_metrics", ctypes.c_int8),
        ("precondition_kind", ctypes.c_int32),
        ("precondition_etag", c_char_p),
        ("options", void_p),
    ]

req_p = ctypes.POINTER(CosmosOperationRequest)

_runtime_builder_new   = _decl("cosmos_runtime_builder_new", [], void_p)
_runtime_builder_ua    = _decl("cosmos_runtime_builder_with_user_agent_suffix", [void_p, c_char_p], ctypes.c_int32)
_runtime_builder_build = _decl("cosmos_runtime_builder_build", [void_p, ctypes.POINTER(void_p), ctypes.POINTER(void_p)], ctypes.c_int32)
_runtime_free          = _decl("cosmos_runtime_free", [void_p], None)
_cq_create             = _decl("cosmos_cq_create", [void_p, void_p], void_p)
_cq_wait               = _decl("cosmos_cq_wait", [void_p, ctypes.c_uint32], void_p)
_cq_free               = _decl("cosmos_cq_free", [void_p], None)
_acct_with_key         = _decl("cosmos_account_ref_with_master_key", [c_char_p, c_char_p, ctypes.POINTER(void_p), ctypes.POINTER(void_p)], ctypes.c_int32)
_acct_free             = _decl("cosmos_account_ref_free", [void_p], None)
_driver_goc_blk        = _decl("cosmos_driver_get_or_create_blocking", [void_p, void_p, void_p, ctypes.POINTER(void_p), ctypes.POINTER(void_p)], ctypes.c_int32)
_driver_free           = _decl("cosmos_driver_free", [void_p], None)
_resolve_container_blk = _decl("cosmos_driver_resolve_container_blocking", [void_p, void_p, c_char_p, c_char_p, ctypes.POINTER(void_p), ctypes.POINTER(void_p)], ctypes.c_int32)
_container_free        = _decl("cosmos_container_ref_free", [void_p], None)
_pkb_new               = _decl("cosmos_partition_key_builder_new", [], void_p)
_pkb_add_s             = _decl("cosmos_partition_key_builder_add_string", [void_p, c_char_p], ctypes.c_int32)
_pkb_build             = _decl("cosmos_partition_key_builder_build", [void_p, ctypes.POINTER(void_p)], ctypes.c_int32)
_pk_free               = _decl("cosmos_partition_key_free", [void_p], None)
_submit_singleton      = _decl("cosmos_submit_singleton_operation", [void_p, req_p, void_p, void_p, ctypes.POINTER(ctypes.c_int32)], void_p)
_op_hnd_free           = _decl("cosmos_operation_handle_free", [void_p], None)
_comp_outcome          = _decl("cosmos_completion_outcome", [void_p], ctypes.c_int32)
_comp_status           = _decl("cosmos_completion_status", [void_p], ctypes.c_int32)
_comp_take_resp        = _decl("cosmos_completion_take_response", [void_p], void_p)
_comp_take_error       = _decl("cosmos_completion_take_error", [void_p], void_p)
_comp_free             = _decl("cosmos_completion_free", [void_p], None)
_error_message         = _decl("cosmos_error_message", [void_p], c_char_p)
_error_free            = _decl("cosmos_error_free", [void_p], None)
_resp_status           = _decl("cosmos_response_status_code", [void_p], ctypes.c_uint16)
_resp_ru               = _decl("cosmos_response_request_charge", [void_p], ctypes.c_double)
_resp_body             = _decl("cosmos_response_body", [void_p, ctypes.POINTER(u8_p), ctypes.POINTER(size_t)], ctypes.c_int32)
_resp_free             = _decl("cosmos_response_free", [void_p], None)


def submit(drv, q, req):
    """Issue one request and block for its single completion.

    Returns the owned completion pointer on success; the caller must free it
    with _comp_free. Raises on pre-flight rejection or a non-OK outcome,
    freeing any completion / error it allocated so nothing leaks on the error
    path. The request struct is only borrowed for the call, so the caller's
    buffers stay valid here and can be released afterward.
    """
    pre = ctypes.c_int32(0)
    h = _submit_singleton(drv, ctypes.byref(req), q, None, ctypes.byref(pre))
    if not h:
        raise RuntimeError(f"submit pre-flight failed: {pre.value}")
    try:
        comp = _cq_wait(q, 30_000)
    finally:
        # The op handle is the in-flight identity; release it once the
        # completion has been delivered (or the wait gave up).
        _op_hnd_free(h)
    if not comp:
        raise RuntimeError("queue drained or shut down before a completion arrived")
    if _comp_outcome(comp) != OUTCOME_OK:
        try:
            err = _comp_take_error(comp)
            if err:
                try:
                    msg = _error_message(err)
                    detail = msg.decode("utf-8") if msg else ""
                finally:
                    _error_free(err)
                raise RuntimeError(f"operation failed (status={_comp_status(comp)}): {detail}")
            raise RuntimeError(f"operation failed (status={_comp_status(comp)})")
        finally:
            _comp_free(comp)
    return comp


def item_request(kind, container, pk, item_id, body=b""):
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
        # 1. Runtime + queue.
        rb = _runtime_builder_new()
        _runtime_builder_ua(rb, b"python-sample")
        if _runtime_builder_build(rb, ctypes.byref(rt), None) != ERROR_CODE_SUCCESS:
            raise RuntimeError("runtime build failed")
        q = _cq_create(rt, None)

        # 2. Account -> driver -> container.
        if _acct_with_key(
            b"https://localhost:8081/",
            b"C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==",
            ctypes.byref(acct), None,
        ) != ERROR_CODE_SUCCESS:
            raise RuntimeError("account ref failed")
        if _driver_goc_blk(rt, acct, None, ctypes.byref(drv), None) != ERROR_CODE_SUCCESS:
            raise RuntimeError("driver create failed")
        if _resolve_container_blk(rt, drv, b"sample-db", b"sample-coll", ctypes.byref(container), None) != ERROR_CODE_SUCCESS:
            raise RuntimeError("resolve container failed")

        # 3. Partition key.
        pkb = _pkb_new()
        _pkb_add_s(pkb, b"tenant-42")
        if _pkb_build(pkb, ctypes.byref(pk)) != ERROR_CODE_SUCCESS:
            raise RuntimeError("partition key build failed")

        # 4. CREATE.
        body = json.dumps({"id": "doc1", "pk": "tenant-42", "name": "hello"}).encode("utf-8")
        comp = submit(drv, q, item_request(KIND_CREATE_ITEM, container, pk, b"doc1", body))
        try:
            resp = _comp_take_resp(comp)
            print(f"CREATE status={_resp_status(resp)} ru={_resp_ru(resp):.2f}")
            _resp_free(resp)
        finally:
            _comp_free(comp)

        # 5. READ.
        comp = submit(drv, q, item_request(KIND_READ_ITEM, container, pk, b"doc1"))
        try:
            resp = _comp_take_resp(comp)
            data_ptr = u8_p()
            data_len = size_t()
            _resp_body(resp, ctypes.byref(data_ptr), ctypes.byref(data_len))
            body_bytes = ctypes.string_at(data_ptr, data_len.value) if data_ptr else b""
            print(f"READ status={_resp_status(resp)} body={body_bytes.decode('utf-8')}")
            _resp_free(resp)
        finally:
            _comp_free(comp)

        # 6. DELETE.
        comp = submit(drv, q, item_request(KIND_DELETE_ITEM, container, pk, b"doc1"))
        try:
            resp = _comp_take_resp(comp)
            print(f"DELETE status={_resp_status(resp)}")
            _resp_free(resp)
        finally:
            _comp_free(comp)
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
   show the pattern end-to-end: check the pre-flight `cosmos_error_code_t`,
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

| Old file | New location |
|---|---|
| `azurecosmos.pc.in` (pkg-config template) | This crate ships a sibling `azurecosmosdriver.pc.in` with the same shape but a new package name. |
| `docs/next_generation_sdks_design_principles.md` | Folded into [NATIVE_WRAPPER_SPEC.md section 2](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md). |
| `c_tests/test_common.h` runtime / client / database fixtures | Re-added incrementally as the corresponding C entry points land. |

If you are spelunking the git history of the old crate looking for a behavior
or test that "should be here", that table is the first place to check.
