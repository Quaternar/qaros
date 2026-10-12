# QAR C# Bindings

## Goal

Map the QAR Streaming C API into a C# implementation with two layers:

1. **Pure DLL binding layer**: exact P/Invoke declarations, native structs, enums, delegates, constants, and helper marshalling code.
2. **Managed wrapper layer**: idiomatic C# classes that own or reference native handles, enforce lifetime/liveliness, expose C handle member accessors, convert callback APIs into tasks/events/streams, and return result objects instead of throwing by default.

Always target all three frameworks:

```xml
<TargetFrameworks>netstandard2.0;netstandard2.1;net8.0</TargetFrameworks>
<AllowUnsafeBlocks>true</AllowUnsafeBlocks>
```

Keep `netstandard2.0` as the compatibility baseline. Add efficient APIs for `netstandard2.1` and `net8.0` where they make sense, but do not break the shared result-based wrapper design.

## Namespace and naming rules

Follow the .NET naming guidelines: `<Company>.<Product>[.<Feature>]`, and the namespace, the
assembly (`Quaternar.Qaros.dll`, project `Quaternar.Qaros.csproj`) and the NuGet package
(`Quaternar.Qaros`, `Quaternar.Qaros.Demo`) share one name. The namespace carries the product,
so public types do not repeat a `Qar` prefix.

| Namespace | Visibility | Holds |
|---|---|---|
| `Quaternar.Qaros` | public | the managed API: `Runtime`, `Session`, `RenderSender`, `PeerId`, `Result<T>`, ... |
| `Quaternar.Qaros.Mathematics` | public | `Vector3`, `Vector3d`, `Quaternion`, `Pose`: kept apart so `using Quaternar.Qaros;` beside `UnityEngine` or `System.Numerics` does not make those names ambiguous |
| `Quaternar.Qaros.Interop` | internal | P/Invoke (`NativeMethods`), native structs, marshalling helpers |
| `Quaternar.Qaros.Tests[.Ext]` | tests | xUnit suites and the ext counterparts |

Naming rules:

- Public high-level managed types must not use a redundant `Qar` prefix. Use `Runtime`, `Session`, `RenderSender`, `RenderFrameInfo`, `PeerSpec`, `GuiPanel`, `AppVolume`, `Result<T>`, `ResultInfo`, `PeerId`, `Structure`, and `StructureType`.
- A new value type that shares its name with a common `UnityEngine` or `System.Numerics` type goes in `Quaternar.Qaros.Mathematics` (or another sub-namespace), never in `Quaternar.Qaros`.
- Internal native binding types may preserve the C concept name but should use a `Native` suffix when that avoids collisions, for example `RuntimeInitNative`, `SessionIdNative`, `StructureHeaderNative`, and `ResultNative`.
- The internal P/Invoke container is `NativeMethods` in `Quaternar.Qaros.Interop`.
- Preserve exact native symbol names in `[DllImport]` declarations, such as `qar_impl_runtime_create`.
- Strings go to the C API as UTF-8 through `NativeMemoryScope.AllocUtf8` / `NativeUtf8String.FromUtf8`, never the ANSI code page. Only a common name must be ASCII, and the C API checks it.
- In examples, write `using Quaternar.Qaros;` (plus `using Quaternar.Qaros.Mathematics;` when poses or vectors appear) and the unprefixed names.

## Required workflow

1. Read the C API header or diff and identify changed constants, enums, structs, callbacks, opaque handles, functions, default helpers, ownership/destruction functions, and async/subscription patterns.
2. Update the binding layer first, preserving C names where useful and avoiding business logic.
3. Update the wrapper layer second, using ownership rules derived from create/destroy/query/getter functions.
4. Classify callback APIs as one-shot async operations or continuous streams/subscriptions.
5. Add or update tests for struct layout, enum values, result handling, string buffers, callbacks, async completion, subscription disposal, and ownership.
6. Call out uncertain ABI details or missing destructor/accessor functions explicitly instead of guessing.

## Formatting

After every C# source change in this folder, run:

```powershell
.\scripts\format-csharp.ps1
```

Use the local `.editorconfig` for C# formatting. The formatter script also removes generated blank lines inside method bodies before running `dotnet format`. Keep braces tight to their declarations and blocks: do not leave blank lines between a declaration and `{`, directly after `{`, or directly before `}`.

## Target framework rules

Use one shared implementation wherever possible and conditional compilation only for APIs or performance paths that differ by target.

### netstandard2.0

Use only APIs available in `netstandard2.0`:

- `DllImport`, `IntPtr`, `UIntPtr`, `SafeHandle`, `Task`, `TaskCompletionSource<T>`, `IDisposable`, events, arrays, and classic delegates.
- No `LibraryImport`, `nint`, unmanaged function pointers, `IAsyncEnumerable<T>`, records, required members, or public `Span<T>` APIs.
- Use arrays or `IntPtr + length` for buffer access.

### netstandard2.1

Add efficient overloads where useful:

- `Span<T>` and `ReadOnlySpan<T>` for buffer, ID, and CPU frame access.
- Memory-based copy APIs for texture data.
- Keep array-based APIs available for callers that compile against `netstandard2.0`.

### net8.0

Add modern convenience/performance APIs where useful:

- Optional `IAsyncEnumerable<T>` adapters for continuous update streams.
- `TaskCreationOptions.RunContinuationsAsynchronously` for callback-backed tasks.
- Optional `NativeLibrary` resolver or platform-specific library loading helpers.
- Optional source-generated interop only if it is isolated and does not replace the required `DllImport` path.

Do not fork behavior between targets. Newer target APIs must be convenience or efficiency layers over the same ownership, callback, and result semantics.

## Cancellation rules

Use standard C# cancellation at the high-level wrapper boundary.

- Public high-level wrapper methods must expose `System.Threading.CancellationToken`, not the internal `CancelToken`.
- Keep `CancelToken` internal. It is native interop plumbing only, used to pass a `QarCancelToken*` to the C API.
- When the native C API accepts `QarCancelToken*` or an equivalent cancellation handle, map that parameter to a C# `CancellationToken` in the managed wrapper.
- Do not add a C# `CancellationToken` parameter when the native operation has no cancellation path. Call out the missing C API support instead of pretending the managed token cancels native work.
- For `CancellationToken.None` or any uncancelable token on one-shot/synchronous operations, pass `IntPtr.Zero` and do not allocate a native token.
- For cancelable tokens, allocate an owned internal `CancelToken`, register the C# token, and call native `Cancel()` from the registration callback.
- If the C# token is already canceled, signal the internal native token before invoking the native call. For one-shot async wrappers, complete the managed task as canceled when managed cancellation wins.
- Keep the internal native token and `CancellationTokenRegistration` alive for the full native operation lifetime. Dispose both exactly once when the native callback completes, startup fails, managed cancellation completes the task, the subscription is disposed, or the scope exits.
- Native QAR failures still return `Result<T>.Failure`; managed cancellation is the only path that should produce a canceled `Task`.
- For continuous subscriptions, always create an owned internal native token, even when the public C# token is uncancelable, because `IDisposable.Dispose()` must stop native delivery.

Recommended signatures:

```csharp
public Result<RenderSender> Create(RenderSenderOptions options, CancellationToken cancellationToken = default)
public Task<Result<RenderSender>> CreateAsync(RenderSenderOptions options, CancellationToken cancellationToken = default)
public Result<IDisposable> SubscribeUpdates(EventHandler<PeerSpecUpdatedEventArgs> handler, CancellationToken cancellationToken = default)
```

## Binding layer rules

Use this default native class shape:

```csharp
internal static class NativeMethods
{
    private const string LibraryName = "qar-streaming-c";

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern ResultNative qar_impl_library_init(ref LibraryInitNative init);
}
```

Prefer importing exported `qar_impl_*` functions, not `static inline qar_*` wrappers, because `static inline` functions are not DLL exports. If the DLL exports non-impl `qar_*` symbols, document that assumption and keep the binding layer consistent.

Always specify:

- `CallingConvention.Cdecl`
- `ExactSpelling = true`
- `[return: MarshalAs(UnmanagedType.I1)]` for C `bool` returns
- `[MarshalAs(UnmanagedType.I1)]` for C `bool` struct fields and callback parameters

Map C pointer handles as `IntPtr` in the binding layer. Use `UIntPtr` for C `size_t` in signatures and struct fields. Convert to checked `int` or `long` at the managed boundary.

## Structs, enums, IDs, and defaults

Map enums with explicit numeric values. Preserve QAR status values exactly.

Map blittable structs with sequential layout. Represent `void*`, `uint8_t*`, COM pointers, and extension-chain `next` pointers as `IntPtr`.

For `Qar*Id` structs, assume the fixed 16 bytes are ASCII characters. Implement public ID wrappers with:

- fixed 16-byte storage internally
- `ToString()` returning the ASCII text up to the first NUL byte
- `Parse`/`TryParse` from ASCII strings of at most 16 bytes
- equality and hash code by byte content
- `ToByteArray()` for exact interop/debugging

Do not decode QAR IDs as UTF-8 or culture-sensitive text.

## Init structures and `header.next` extension chains

Map every native init or extension struct header in the internal binding layer as an explicit ABI struct:

```csharp
[StructLayout(LayoutKind.Sequential)]
internal struct StructureHeaderNative
{
    public StructureType Type;
    public IntPtr Next;
}
```

Expose extension-chain inputs in the high-level API as managed descriptor objects that can use inheritance for shared header behavior:

```csharp
public abstract class Structure
{
    public StructureType Type { get; }
    public Structure Next { get; set; }

    protected Structure(StructureType type)
    {
        Type = type;
    }
}

public sealed class StreamParamsD3D11 : Structure
{
    public IntPtr D3D11Context { get; set; }
    public IntPtr D3D11Device { get; set; }
    public uint ColorBindFlags { get; set; }
    public uint DepthBindFlags { get; set; }
    public bool AcquireKeyedMutexSync { get; set; }

    public StreamParamsD3D11()
        : base(StructureType.StreamD3D11ParamsExt)
    {
        AcquireKeyedMutexSync = true;
    }
}
```

Use inheritance only for managed descriptors. Do not use C# class inheritance for internal P/Invoke structs. Internal native structs must remain `[StructLayout(LayoutKind.Sequential)]` value types matching the C ABI exactly.

Before a native call, copy the managed descriptor chain into unmanaged native structs. Native `header.next` must point to unmanaged memory containing the exact C struct layout, never to a managed object.

Recommended call pattern:

```csharp
using (var nativeChain = NativeStructureChain.From(options.Extension))
{
    var nativeInit = options.ToNative();
    nativeInit.Header.Next = nativeChain.Head;

    var nativeResult = NativeMethods.qar_impl_render_sender_create(
        sessionHandle,
        ref nativeInit,
        cancelHandle,
        out senderHandle);

    return Results.FromOutHandle(nativeResult, senderHandle, h => new RenderSender(h));
}
```

The chain builder must own all unmanaged allocations and wire `Header.Next` pointers in order:

```csharp
internal sealed class NativeStructureChain : IDisposable
{
    private readonly List<IntPtr> _allocations = new List<IntPtr>();

    public IntPtr Head { get; private set; }

    public static NativeStructureChain From(Structure head)
    {
        var chain = new NativeStructureChain();
        chain.Build(head);
        return chain;
    }

    private void Build(Structure head)
    {
        IntPtr previous = IntPtr.Zero;

        for (var current = head; current != null; current = current.Next)
        {
            IntPtr currentPtr = AllocateNativeCopy(current, IntPtr.Zero);
            if (previous == IntPtr.Zero)
                Head = currentPtr;
            else
                SetNextPointer(previous, currentPtr);

            previous = currentPtr;
        }
    }

    public void Dispose()
    {
        for (int i = _allocations.Count - 1; i >= 0; i--)
            Marshal.FreeHGlobal(_allocations[i]);

        _allocations.Clear();
        Head = IntPtr.Zero;
    }
}
```

`AllocateNativeCopy` must switch on `Structure.Type`, create the corresponding native struct, write the correct `StructureHeaderNative`, marshal fields, allocate unmanaged memory with `Marshal.AllocHGlobal`, and copy with `Marshal.StructureToPtr` or an equivalent target-specific efficient path.

For synchronous native calls, keep the unmanaged chain alive until the call returns.

For one-shot async native calls, keep the unmanaged chain alive until the completion callback fires, cancellation completes, or the operation state is disposed. Store the chain in the rooted callback state.

For continuous subscriptions, keep any native chain passed to the subscription alive until the returned subscription object is disposed.

Use `IntPtr.Zero` for `Header.Next` when no extensions are provided.

For inline default helpers that are not exported, mirror the C header in managed factory methods. Header `type` fields must match `StructureType` exactly.

## Result-first error handling

Use result-based errors by default, close to the C/C++ model. Do not make high-level APIs throw for normal native QAR failures.

Keep native `QarResult` internal:

```csharp
[StructLayout(LayoutKind.Sequential)]
internal struct QarResult
{
    public StatusCode Code;
    public uint ErrorHandle;
}
```

Expose managed result types:

```csharp
public readonly struct ResultInfo
{
    public StatusCode StatusCode { get; }
    public uint ErrorHandle { get; }
    public string Message { get; }
    public bool IsSuccess => StatusCode == StatusCode.Success;
}

public readonly struct Result<T>
{
    public ResultInfo Info { get; }
    public T Value { get; }
    public bool IsSuccess => Info.IsSuccess;
}

public readonly struct Unit
{
    public static readonly Unit Value = new Unit();
}
```

For C functions that return `QarResult` and write the actual value through an out pointer, return `Result<T>` where `T` is the managed value created from that out pointer. Examples:

- `qar_runtime_create(..., Runtime** runtime)` -> `Result<Runtime> CreateRuntime(...)`
- `qar_session_get_id(..., SessionId* session_id)` -> `Result<SessionId> GetId()`
- `qar_peer_spec_get_display_name(..., char* out_buffer, size_t buffer_size)` -> `Result<string> GetDisplayName()`

For C functions that return only status and mutate existing native state, return `Result<Unit>`, not `ResultInfo`. C# cannot use `Result<void>`, so `Unit` is the public void-like payload type. Keep `ResultInfo` as metadata stored in `Result<T>.Info`; do not expose public high-level wrapper methods returning bare `ResultInfo`.

Examples:

- `qar_impl_library_destroy()` -> `Result<Unit> Destroy()`
- `qar_impl_gui_panels_close_panel(...)` -> `Result<Unit> Close(GuiPanelId id)`
- `qar_impl_render_sender_show_frame(...)` -> `Result<Unit> ShowFrame(RenderFrameShow? frameShow = null)`

Use shared helpers such as `Results.FromNative(...)` or `Results.FromInfo(...)` to convert a native `QarResult`/`ResultInfo` into `Result<Unit>` consistently.

For async one-shot callbacks, return `Task<Result<T>>`. Native failures complete the task successfully with `IsSuccess == false`. Fault the task only for managed/runtime failures such as invalid wrapper state, callback marshalling failure, disposed objects, or unexpected exceptions.

Use `NativeCallException` only for optional convenience APIs such as `CreateRuntimeOrThrow`. Throwing APIs must not replace result-returning APIs.

## Ownership and lifetime model

Every native handle wrapper must make ownership explicit.

Use `SafeHandle` for C-owned opaque handles.

```csharp
internal sealed class RuntimeSafeHandle : SafeHandle
{
    private RuntimeSafeHandle() : base(IntPtr.Zero, true) { }
    public override bool IsInvalid => handle == IntPtr.Zero;
    protected override bool ReleaseHandle()
    {
        NativeMethods.qar_impl_runtime_destroy(handle);
        return true;
    }
}
```

Use this general pattern for every C-owned opaque handle:

```c
// C API
// object_t* object_create();
// void object_destroy(object_t*);
```

```csharp
// Internal binding
internal static class Native
{
    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern IntPtr object_create();

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern void object_destroy(IntPtr handle);
}

internal sealed class ObjectHandle : SafeHandle
{
    private ObjectHandle() : base(IntPtr.Zero, true) { }

    public override bool IsInvalid => handle == IntPtr.Zero;

    protected override bool ReleaseHandle()
    {
        Native.object_destroy(handle);
        return true;
    }
}
```

```csharp
// High-level wrapper
public sealed class Object : IDisposable
{
    private readonly ObjectHandle _handle;

    internal Object(ObjectHandle handle)
    {
        _handle = handle ?? throw new ArgumentNullException(nameof(handle));
    }

    public void Dispose()
    {
        _handle.Dispose();
    }
}
```

When the C create function returns `QarResult` and writes the handle through an out pointer, keep the same pattern but return `Result<Object>` from the high-level factory. The result value is the managed wrapper around the created `ObjectHandle`.

Use a non-owning wrapper or `SafeHandle` with `ownsHandle: false` for borrowed handles returned by update callbacks or query snapshots when the C API says the caller must not destroy them.

Ownership defaults:

- `*_create`, `*_join`, `*_begin_frame`, and query functions returning handles usually produce handles that must be destroyed by the matching `*_destroy` or `*_handle_destroy` function.
- `*_destroy` means semantic shutdown/destruction of the underlying object.
- `*_handle_destroy` means release a handle/reference without necessarily shutting down the underlying object.
- Callback handles are borrowed unless documentation explicitly transfers ownership. Snapshot callback data immediately if it must outlive the callback.
- `QarSessionInvite*` returned from `qar_runtime_create_session` must be freed with `qar_session_invite_destroy` or copied then destroyed.

High-level objects must throw `ObjectDisposedException` only for managed misuse after disposal. Native QAR operation failures still return `Result<T>`; use `Result<Unit>` for status-only operations.

## Callback classification

Classify every C callback API before designing the wrapper.

### One-shot async callbacks

Treat callback APIs as one-shot async operations when the callback reports final completion once, especially names containing `_async` where the callback includes `QarResult status` and an output handle/value.

Map to `Task<Result<T>>`:

```csharp
public Task<Result<RenderSender>> CreateRenderSenderAsync(RenderSenderOptions options, CancellationToken cancellationToken = default)
```

Use `TaskCompletionSource<T>` in `netstandard2.0`; use `RunContinuationsAsynchronously` for `net8.0` when available. Keep callback delegates and `GCHandle` user state alive until completion. Never let exceptions escape native callbacks.

When a native one-shot async API accepts `QarCancelToken*`, follow the repository-wide cancellation rules and expose only the standard C# cancellation style in the high-level managed API:

```csharp
public Task<Result<RenderSender>> CreateRenderSenderAsync(RenderSenderOptions options, CancellationToken cancellationToken = default)
```

The `System.Threading.CancellationToken` parameter must be a bridge over the native cancellation path:

- Do not expose `CancelToken` parameters from high-level public wrapper methods. Keep `CancelToken` as internal/native plumbing unless a low-level binding layer explicitly needs it.
- For `CancellationToken.None` or an uncancelable token, pass `IntPtr.Zero` and do not allocate a native token.
- For a cancelable C# token, create an owned `CancelToken`, register the C# token, and call native `Cancel()` when the C# token is canceled.
- Store the owned native token and `CancellationTokenRegistration` in the rooted async state so they remain alive until the native callback completes, managed cancellation completes the task, startup fails, or the state is otherwise cleaned up.
- Dispose the registration and owned native token exactly once during async-state cleanup.
- If the C# token cancels, signal the native token and complete the managed task as canceled (`TrySetCanceled`). Native QAR failures still complete successfully with `Result<T>.Failure`; only managed cancellation should produce a canceled task.
- Do not add a C# `CancellationToken` overload that pretends to cancel native work unless the underlying C API accepts a `QarCancelToken*` or equivalent. Call out the C API gap instead. For example, `session_invite_peer_async` currently has no native cancel-token parameter, so it cannot be mapped cleanly without extending the C API.

For callbacks with progress plus final completion, expose `IProgress<string>` or an event for updates and complete the task only from the final result callback.

### Continuous callbacks and observed objects

Treat callback APIs as continuous streams when names contain `subscribe`, `updates`, `watch`, or `listen`, or when callbacks may be invoked repeatedly until cancellation/disposal.

Map continuous callbacks to C# events and an `IDisposable` subscription object for all targets:

```csharp
public event EventHandler<PeerSpecUpdatedEventArgs> PeerUpdated;
public IDisposable SubscribePeerUpdates(CancellationToken cancellationToken = default);
```

The subscription owns delegate roots, user-state `GCHandle`s, a native cancellation token, any `CancellationTokenRegistration`, and a parent reference. On `Dispose`, cancel native delivery first, then release managed roots when safe. Guard late callbacks with an interlocked disposed flag. For subscriptions, create an owned native token even when the public C# token is uncancelable, because `IDisposable.Dispose()` needs a native token to stop delivery.

For borrowed observed handles such as `PeerSpec*`, `GuiPanel*`, and `AppVolume*`, snapshot the data inside the callback using native getters, then raise the event with a managed snapshot DTO. Do not store borrowed callback handles after the callback returns.

For `net8.0`, optionally add `IAsyncEnumerable<T>` adapters over the same subscription core. Do not require channels or async streams for `netstandard2.0`.

## Mapping examples by API category

### C-owned handle returning direct pointer

```c
// C API
object_t* object_create();
void object_destroy(object_t*);
```

```csharp
// Internal binding
internal static extern IntPtr object_create();
internal static extern void object_destroy(IntPtr handle);
```

```csharp
// Internal C# handle
internal sealed class ObjectHandle : SafeHandle
{
    private ObjectHandle() : base(IntPtr.Zero, true) { }
    public override bool IsInvalid => handle == IntPtr.Zero;
    protected override bool ReleaseHandle()
    {
        Native.object_destroy(handle);
        return true;
    }
}
```

```csharp
// High-level wrapper
public sealed class Object : IDisposable
{
    private readonly ObjectHandle _handle;
}
```

### C-owned handle returned through out pointer plus QarResult

```c
QarResult qar_runtime_create(const QarRuntimeInit* init, QarRuntime** runtime);
void qar_runtime_destroy(QarRuntime* runtime);
```

```csharp
internal static extern ResultNative qar_impl_runtime_create(ref RuntimeInitNative init, out IntPtr runtime);
internal static extern void qar_impl_runtime_destroy(IntPtr runtime);
```

```csharp
public Result<Runtime> CreateRuntime(RuntimeOptions options)
{
    var init = RuntimeInitNative.CreateDefault(options);
    IntPtr ptr;
    var native = NativeMethods.qar_impl_runtime_create(ref init, out ptr);
    var info = Results.ToInfo(native);
    return info.IsSuccess
         ? Result.Success(info, new Runtime(new RuntimeHandle(ptr)))
        : Result.Failure<Runtime>(info);
}
```

### Value getter through pointer

```c
QarResult qar_session_get_id(const QarSession* session, QarSessionId* session_id);
```

```csharp
public Result<SessionId> GetId()
{
    SessionIdNative nativeId;
    var result = NativeMethods.qar_impl_session_get_id(_handle.DangerousGetHandle(), out nativeId);
    var info = Results.ToInfo(result);
    return info.IsSuccess
         ? Result.Success(info, SessionId.FromNative(nativeId))
        : Result.Failure<SessionId>(info);
}
```

### String getter through buffer

```c
QarResult qar_peer_spec_get_display_name(QarPeerSpec* handle, char* out_buffer, size_t buffer_size);
```

```csharp
public Result<string> GetDisplayName()
{
    return NativeString.GetString(512, (buffer, size) =>
        NativeMethods.qar_impl_peer_spec_get_display_name(_handle.DangerousGetHandle(), buffer, size));
}
```

### Borrowed observed object from update callback

```c
void (*qar_peer_update_callback_t)(QarPeerSpec* handle, void* user_state);
```

```csharp
private static void OnPeerUpdated(IntPtr peerSpecHandle, IntPtr userState)
{
    var subscription = (PeerSubscription)GCHandle.FromIntPtr(userState).Target;
    if (subscription.IsDisposed) return;
    var snapshot = PeerSpecSnapshot.FromBorrowedHandle(peerSpecHandle);
    subscription.Raise(snapshot);
}
```

### Query count/fill API

```c
QarResult qar_query_peer_specs_count(QarSession* session, size_t* out_count);
QarResult qar_query_peer_specs(QarSession* session, QarPeerSpec** out_handles, size_t handles_buffer_size, size_t* out_handles_written);
```

```csharp
public Result<IReadOnlyList<PeerSpecSnapshot>> GetPeers()
{
    UIntPtr count;
    var countResult = NativeMethods.qar_impl_query_peer_specs_count(_handle.DangerousGetHandle(), out count);
    var countInfo = Results.ToInfo(countResult);
    if (!countInfo.IsSuccess) return Result.Failure<IReadOnlyList<PeerSpecSnapshot>>(countInfo);

    var handles = new IntPtr[checked((int)count)];
    UIntPtr written;
    var fillResult = NativeMethods.qar_impl_query_peer_specs(_handle.DangerousGetHandle(), handles, count, out written);
    var fillInfo = Results.ToInfo(fillResult);
    if (!fillInfo.IsSuccess) return Result.Failure<IReadOnlyList<PeerSpecSnapshot>>(fillInfo);

    var snapshots = handles.Take(checked((int)written)).Select(PeerSpecSnapshot.FromOwnedQueryHandle).ToArray();
    return Result.Success<IReadOnlyList<PeerSpecSnapshot>>(fillInfo, snapshots);
}
```

### One-shot async callback

```c
QarResult qar_render_sender_create_async(..., qar_render_sender_create_callback_t callback, void* user_state, CancelToken* token);
```

```csharp
public Task<Result<RenderSender>> CreateRenderSenderAsync(RenderSenderOptions options, CancellationToken cancellationToken = default)
{
#if NET8_0_OR_GREATER
    var tcs = new TaskCompletionSource<Result<RenderSender>>(TaskCreationOptions.RunContinuationsAsynchronously);
#else
    var tcs = new TaskCompletionSource<Result<RenderSender>>();
#endif
    // Root callback state with GCHandle until final callback completes.
    // Native QAR failure completes with Result<T>.Failure, not Task fault.
    return tcs.Task;
}
```

### Continuous subscription event plus net8 async stream

```c
QarResult qar_peer_subscribe_updates(Session* session, qar_peer_update_callback_t callback, void* user_state, CancelToken* token);
```

```csharp
public IDisposable SubscribePeerUpdates(EventHandler<PeerSpecUpdatedEventArgs> handler, CancellationToken cancellationToken = default)
{
    return new PeerSubscription(this, handler, cancellationToken);
}

#if NET8_0_OR_GREATER
public async IAsyncEnumerable<PeerSpecSnapshot> WatchPeerUpdatesAsync(
    [EnumeratorCancellation] CancellationToken cancellationToken = default)
{
    // Adapt the same subscription core to an async stream.
}
#endif
```

## Rendering-specific rules

Separate CPU and D3D11 paths.

For CPU frames, treat `texture_data` as borrowed frame memory unless the C API explicitly says the caller owns it. Use arrays or `IntPtr + length` for `netstandard2.0`; add `Span<byte>`/`ReadOnlySpan<byte>` overloads for `netstandard2.1` and `net8.0`.

For D3D11 frames, treat `ID3D11Texture2D*`, `ID3D11Device*`, `ID3D11DeviceContext*`, and keyed mutex pointers as borrowed COM interface pointers unless ownership is explicitly transferred. Do not call `Release` unless ownership transfer is proven.

`BeginFrame` returning `QarRenderFrameInfo*` must be paired with `qar_impl_render_frame_info_handle_destroy` through a safe handle. `ShowFrame` should validate disposed state only and rely on native validation for frame layout correctness.

## Change handling checklist

For every C API change:

- Add new constants and enum values with exact numeric values.
- Add or update native structs with `StructLayout` and fixed-array handling.
- Add or update QAR ASCII ID conversion helpers.
- Add new `DllImport` declarations for exported functions.
- Add delegate definitions for new callbacks.
- Classify callback APIs as one-shot task, progress plus task, or continuous event/subscription.
- Add safe handles for new owned opaque handles and use the correct destroy function.
- Add high-level wrapper methods/properties/events returning `Result<T>`; use `Result<Unit>` for status-only operations instead of bare `ResultInfo`.
- Add default factory methods for new init structs or inline default helpers.
- Add efficient `netstandard2.1`/`net8.0` overloads where useful without removing `netstandard2.0` APIs.
- Add tests for ABI layout, status conversion, ownership disposal, string marshalling, callback lifetime, task completion, and subscription disposal.
- Document unsupported platform-specific parts, especially D3D11-only APIs.

## Output expectations

When asked to implement or update bindings, produce code organized like this unless the repository already has a different convention:

```text
src/
  QarStreaming.Bindings/
    Native/NativeMethods.cs
    Native/QarTypes.cs
    Native/QarCallbacks.cs
    Native/QarDefaults.cs
    Interop/NativeString.cs
    Interop/ResultHelpers.cs
  QarStreaming/
    QarLibrary.cs
    Runtime.cs
    Session.cs
    CancelToken.cs
    RenderSender.cs
    QarRenderFrameInfo.cs
    PeerSpec.cs
    GuiPanel.cs
    AppVolume.cs
    ResultInfo.cs
    QarResult.cs
    Subscriptions/QarSubscription.cs
    Events/PeerSpecUpdatedEventArgs.cs
    Events/QarGuiPanelUpdatedEventArgs.cs
    Events/QarAppVolumeUpdatedEventArgs.cs
tests/
  QarStreaming.Tests/
    NativeLayoutTests.cs
    ResultTests.cs
    LifetimeTests.cs
    CallbackTests.cs
    AsyncTests.cs
    SubscriptionTests.cs
```
