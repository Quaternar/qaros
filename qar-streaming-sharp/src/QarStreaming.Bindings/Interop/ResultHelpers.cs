using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros;

namespace Quaternar.Qaros.Interop
{
    internal static class Results
    {
        internal static ResultInfo ToInfo(ResultNative result)
        {
            if (result.Code == StatusCode.Success)
                return new ResultInfo(result.Code, result.ErrorHandle, string.Empty);
            return new ResultInfo(result.Code, result.ErrorHandle, () => LoadMessage(result));
        }

        private static string LoadMessage(ResultNative result)
        {
            try
            {
                var buffer = new byte[NativeConstants.MaxStringLength];
                NativeMethods.qar_impl_result_message(result, buffer, (UIntPtr)buffer.Length);
                return NativeString.DecodeNullTerminated(buffer);
            }
            catch (DllNotFoundException)
            {
                throw;
            }
            catch (EntryPointNotFoundException)
            {
                throw;
            }
            catch
            {
                return string.Empty;
            }
        }

        internal static Result<Unit> FromInfo(ResultInfo info)
        {
            return info.IsSuccess ? Result<Unit>.Success(info, Unit.Value) : Result<Unit>.Failure(info);
        }

        internal static Result<Unit> FromNative(ResultNative result)
        {
            return FromInfo(ToInfo(result));
        }

        internal static Result<T> FromHandle<T>(ResultInfo info, IntPtr handle, Func<IntPtr, T> factory)
        {
            if (!info.IsSuccess)
                return Result<T>.Failure(info);
            if (handle == IntPtr.Zero)
                return Result<T>.Failure(new ResultInfo(StatusCode.LogicError, 0, "Native call succeeded but returned a null handle."));
            return Result<T>.Success(info, factory(handle));
        }

        internal static Result<IDisposable> SubscriptionResult(ResultInfo info, NativeSubscription subscription)
        {
            if (!info.IsSuccess)
            {
                subscription.DisposeAfterFailedStart();
                return Result<IDisposable>.Failure(info);
            }
            return Result<IDisposable>.Success(info, subscription);
        }
    }

    internal static class ResultExtensions
    {
        internal static string ValueOrDefault(this Result<string> result)
        {
            return result.IsSuccess ? result.Value : string.Empty;
        }
    }

    internal sealed class NativeMemoryScope : IDisposable
    {
        private readonly List<IntPtr> _allocations = new List<IntPtr>();

        private readonly List<SafeHandle> _borrowed = new List<SafeHandle>();

        /// <summary>
        /// A NUL-terminated UTF-8 copy of a UTF-16 string, freed with the scope: every <c>char*</c>
        /// of the C API is UTF-8, whatever the ANSI code page. Null for null.
        /// </summary>
        public IntPtr AllocUtf8(string? value)
        {
            if (value == null)
                return IntPtr.Zero;
            var bytes = Encoding.UTF8.GetBytes(value);
            var ptr = Marshal.AllocHGlobal(bytes.Length + 1);
            _allocations.Add(ptr);
            Marshal.Copy(bytes, 0, ptr, bytes.Length);
            Marshal.WriteByte(ptr, bytes.Length, 0);
            return ptr;
        }

        public IntPtr AllocBytes(byte[]? value)
        {
            if (value == null || value.Length == 0)
                return IntPtr.Zero;
            var ptr = Marshal.AllocHGlobal(value.Length);
            _allocations.Add(ptr);
            Marshal.Copy(value, 0, ptr, value.Length);
            return ptr;
        }

        public IntPtr AllocStruct<T>(T value)
        {
            var ptr = Marshal.AllocHGlobal(Marshal.SizeOf(typeof(T)));
            _allocations.Add(ptr);
            Marshal.StructureToPtr(value!, ptr, false);
            return ptr;
        }

        public IntPtr AllocArray<T>(T[] values)
        {
            if (values == null || values.Length == 0)
                return IntPtr.Zero;
            var size = Marshal.SizeOf(typeof(T));
            var ptr = Marshal.AllocHGlobal(size * values.Length);
            _allocations.Add(ptr);
            for (var i = 0; i < values.Length; i++)
                Marshal.StructureToPtr(values[i]!, IntPtr.Add(ptr, i * size), false);
            return ptr;
        }

        public IntPtr AllocPointerArray(PeerId[] ids)
        {
            if (ids == null || ids.Length == 0)
                return IntPtr.Zero;
            var pointers = new IntPtr[ids.Length];
            for (var i = 0; i < ids.Length; i++)
                pointers[i] = AllocStruct(ids[i].ToNative());
            var ptr = Marshal.AllocHGlobal(IntPtr.Size * pointers.Length);
            _allocations.Add(ptr);
            for (var i = 0; i < pointers.Length; i++)
                Marshal.WriteIntPtr(ptr, i * IntPtr.Size, pointers[i]);
            return ptr;
        }

        /// <summary>
        /// The raw value of <paramref name="handle"/>, kept alive until the scope is disposed: the
        /// reference taken here stops a finalizer or a concurrent <c>Dispose</c> from releasing the
        /// native object while a native call still borrows it.
        /// </summary>
        /// <exception cref="ObjectDisposedException">The handle is already closed.</exception>
        public IntPtr Borrow(SafeHandle handle)
        {
            var added = false;
            handle.DangerousAddRef(ref added);
            if (added)
                _borrowed.Add(handle);
            return handle.DangerousGetHandle();
        }

        public void Dispose()
        {
            for (var i = _borrowed.Count - 1; i >= 0; i--)
                _borrowed[i].DangerousRelease();
            _borrowed.Clear();
            for (var i = _allocations.Count - 1; i >= 0; i--)
                Marshal.FreeHGlobal(_allocations[i]);
            _allocations.Clear();
        }
    }

    /// <summary>
    /// The native handle of one wrapper object (session, sender, frame info, cancel token) and its
    /// dispose state. Every native call takes the handle through <see cref="Borrow"/>, which holds
    /// a reference on it until the borrow is released, so neither a concurrent
    /// <see cref="Dispose"/> nor the finalizer frees the native object while a call still uses it
    /// (hub G56). The owning wrapper keeps no other path to the raw handle.
    /// </summary>
    internal sealed class NativeHandleOwner
    {
        // Also the lock that orders Dispose against the release of a borrow (Release): a private
        // object no SafeHandle code locks, so no gate is allocated per wrapper.
        private readonly SafeHandle _handle;

        private readonly string _objectName;

        private volatile bool _disposed;

        // Cancelled by Dispose, created on first use: a waiting call linked to it ends instead of
        // keeping a disposed object alive (see Disposing).
        private CancellationTokenSource? _disposing;

        // The native form of Disposing, created on first use and shared by every waiting call made
        // without a cancelable token of the caller's (see DisposeToken).
        private CancelToken? _disposeToken;

        internal NativeHandleOwner(SafeHandle handle, string objectName)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
            _objectName = objectName;
        }

        /// <summary>
        /// Disposed by its owner, or its handle closed or invalid. A borrow taken before the
        /// <see cref="Dispose"/> keeps the native object alive, but the owner is unusable from it on.
        /// </summary>
        internal bool IsDisposed { get { return _disposed || _handle.IsClosed || _handle.IsInvalid; } }

        /// <exception cref="ObjectDisposedException">The owner is disposed.</exception>
        internal void ThrowIfDisposed()
        {
            if (IsDisposed)
                throw new ObjectDisposedException(_objectName);
        }

        /// <summary>
        /// Cancelled when the owner is disposed. A call that may wait (a begin-frame through a
        /// paused stream, a sender creation, a hub request) links its native cancel token to it, so
        /// a Dispose ends the call: without that, the call's borrow would keep the disposed object
        /// alive, and an async call whose native completion only the object's teardown brings would
        /// never complete.
        /// </summary>
        internal CancellationToken Disposing
        {
            get
            {
                var source = Volatile.Read(ref _disposing);
                if (source == null)
                {
                    var created = new CancellationTokenSource();
                    source = Interlocked.CompareExchange(ref _disposing, created, null) ?? created;
                    if (!ReferenceEquals(source, created))
                        created.Dispose();
                }
                // A Dispose that ran before the source was published did not see it.
                if (_disposed)
                    CancelQuietly(source);
                return source.Token;
            }
        }

        /// <summary>
        /// A native token cancelled when the owner is disposed, shared by the waiting calls that
        /// carry no cancelable token of the caller's, so the per-frame begin-frame creates no token of
        /// its own. A call borrows it for its duration (its async completion included); null when
        /// the token cannot be created, then the call makes one of its own.
        /// </summary>
        /// <exception cref="ObjectDisposedException">The owner is disposed.</exception>
        internal CancelToken? DisposeToken
        {
            get
            {
                ThrowIfDisposed();
                var token = Volatile.Read(ref _disposeToken);
                if (token == null)
                {
                    var created = CancelToken.Create();
                    if (!created.IsSuccess)
                        return null;
                    token = Interlocked.CompareExchange(ref _disposeToken, created.Value, null) ?? created.Value;
                    if (!ReferenceEquals(token, created.Value))
                        created.Value.Dispose();
                }
                // A Dispose that ran before the token was published did not see it.
                if (_disposed)
                    EndDisposeToken(token);
                return token;
            }
        }

        /// <summary>
        /// Borrows <see cref="DisposeToken"/> for one waiting call; false when no shared token can
        /// be created, then the call makes one of its own.
        /// </summary>
        /// <exception cref="ObjectDisposedException">The owner is disposed.</exception>
        internal bool TryBorrowDisposeToken(out CancelToken? token, out HandleBorrow borrow)
        {
            token = DisposeToken;
            borrow = default(HandleBorrow);
            if (token == null)
                return false;
            try
            {
                borrow = token.Borrow();
            }
            catch (ObjectDisposedException)
            {
                // A concurrent Dispose ended the shared token: report the owner, not the token.
                throw new ObjectDisposedException(_objectName);
            }
            return true;
        }

        // Cancels the waiting calls that share the token, then drops the owner's reference on it;
        // their borrows keep the native token until they are done. Safe to run twice.
        private static void EndDisposeToken(CancelToken token)
        {
            try
            {
                token.Cancel();
            }
            catch (ObjectDisposedException)
            {
            }
            token.Dispose();
        }

        private static void CancelQuietly(CancellationTokenSource source)
        {
            try
            {
                source.Cancel();
            }
            catch (AggregateException)
            {
            }
        }

        /// <summary>
        /// The raw handle with a reference held on it until the returned borrow is disposed. A
        /// disposed owner refuses before the call.
        /// </summary>
        /// <exception cref="ObjectDisposedException">The owner is disposed.</exception>
        internal HandleBorrow Borrow()
        {
            ThrowIfDisposed();
            var added = false;
            // Throws ObjectDisposedException when a Dispose released the handle since the check.
            _handle.DangerousAddRef(ref added);
            if (!added)
                throw new ObjectDisposedException(_objectName);
            return new HandleBorrow(this, _handle.DangerousGetHandle());
        }

        /// <summary>
        /// Drops one borrow. While the owner is not disposed this only drops a reference: the owner's
        /// <see cref="Dispose"/> releases the native object later, on its own thread. When it was
        /// disposed during the borrow, this release is the last reference and destroys the native
        /// object, which may join the very thread it runs on (a native completion or a subscription
        /// handler calling back into the library): it is handed to the thread pool instead. The gate
        /// keeps a concurrent Dispose from falling between the check and the release.
        /// </summary>
        internal void Release()
        {
            lock (_handle)
            {
                if (!_disposed)
                {
                    _handle.DangerousRelease();
                    return;
                }
            }
            ThreadPool.QueueUserWorkItem(ReleaseOnThreadPool, _handle);
        }

        private static void ReleaseOnThreadPool(object? handle)
        {
            ((SafeHandle)handle!).DangerousRelease();
        }

        /// <summary>
        /// Makes the owner unusable, cancels the waiting calls linked to <see cref="Disposing"/> and
        /// releases its own reference. A call still borrowing the handle defers the native release
        /// until it is done.
        /// </summary>
        internal void Dispose()
        {
            lock (_handle)
                _disposed = true;
            // Orders the flag before the reads below: a Disposing or DisposeToken getter that
            // publishes after this point sees the flag and cancels on its own.
            Thread.MemoryBarrier();
            var source = Volatile.Read(ref _disposing);
            if (source != null)
                CancelQuietly(source);
            var token = Volatile.Read(ref _disposeToken);
            if (token != null)
                EndDisposeToken(token);
            _handle.Dispose();
        }
    }

    /// <summary>
    /// One reference on a <see cref="NativeHandleOwner"/>'s handle, held for a native call:
    /// <c>using (var borrowed = owner.Borrow())</c> around the call, or kept by an async call's state
    /// until its native completion fires. A struct, so the per-frame calls allocate nothing.
    /// Dispose it exactly once; the default value holds nothing.
    /// </summary>
    internal readonly struct HandleBorrow : IDisposable
    {
        private readonly NativeHandleOwner? _owner;

        internal HandleBorrow(NativeHandleOwner owner, IntPtr handle)
        {
            _owner = owner;
            Handle = handle;
        }

        /// <summary>The raw handle, valid until this borrow is disposed.</summary>
        public IntPtr Handle { get; }

        public void Dispose()
        {
            if (_owner != null)
                _owner.Release();
        }
    }

    internal sealed class NativeCancellationScope : IDisposable
    {
        private readonly CancelToken? _token;

        // False for an owner's shared dispose token (NativeHandleOwner.DisposeToken).
        private readonly bool _ownsToken;

        // The token is borrowed for the scope's lifetime, which encloses the native call.
        private readonly HandleBorrow _tokenBorrow;

        private CancellationTokenRegistration _registration;

        private bool _hasRegistration;

        private CancellationTokenRegistration _ownerRegistration;

        private bool _hasOwnerRegistration;

        private bool _disposed;

        private NativeCancellationScope(CancelToken? token)
        {
            _token = token;
            _ownsToken = true;
            if (token != null)
                _tokenBorrow = token.Borrow();
        }

        // A shared token the caller already borrowed for the scope.
        private NativeCancellationScope(CancelToken shared, HandleBorrow borrow)
        {
            _token = shared;
            _ownsToken = false;
            _tokenBorrow = borrow;
        }

        /// <summary>The native token for a call inside this scope, or null without a token.</summary>
        public IntPtr Handle { get { return _tokenBorrow.Handle; } }

        public CancelToken? Token { get { return _token; } }

        public static Result<NativeCancellationScope> Create(CancellationToken cancellationToken, bool allocateForUncancelable)
        {
            if (!allocateForUncancelable && !cancellationToken.CanBeCanceled)
                return Result<NativeCancellationScope>.Success(new ResultInfo(StatusCode.Success, 0, string.Empty), new NativeCancellationScope(null));
            var tokenResult = CancelToken.Create();
            if (!tokenResult.IsSuccess)
                return Result<NativeCancellationScope>.Failure(tokenResult.Info);
            var scope = new NativeCancellationScope(tokenResult.Value);
            scope.Register(cancellationToken);
            return Result<NativeCancellationScope>.Success(tokenResult.Info, scope);
        }

        /// <summary>
        /// A scope for a call on <paramref name="endedBy"/> that may wait: always a native token,
        /// cancelled by the owner's Dispose, so disposing the object ends the call. Without a
        /// cancelable <paramref name="cancellationToken"/> it is the owner's shared
        /// <see cref="NativeHandleOwner.DisposeToken"/>; with one, a token of the call's own, also
        /// linked to <see cref="NativeHandleOwner.Disposing"/>.
        /// </summary>
        /// <exception cref="ObjectDisposedException">The owner is disposed.</exception>
        public static Result<NativeCancellationScope> Create(CancellationToken cancellationToken, NativeHandleOwner endedBy)
        {
            if (!cancellationToken.CanBeCanceled && endedBy.TryBorrowDisposeToken(out var shared, out var borrow))
                return Result<NativeCancellationScope>.Success(new ResultInfo(StatusCode.Success, 0, string.Empty), new NativeCancellationScope(shared!, borrow));
            endedBy.ThrowIfDisposed();
            return Create(cancellationToken, endedBy.Disposing);
        }

        /// <summary>
        /// A scope for a call that may wait: always a native token of the call's own, cancelled by
        /// <paramref name="cancellationToken"/> and by <paramref name="endedBy"/>, the Dispose of
        /// the object the call is made on.
        /// </summary>
        public static Result<NativeCancellationScope> Create(CancellationToken cancellationToken, CancellationToken endedBy)
        {
            var created = Create(cancellationToken, true);
            if (!created.IsSuccess)
                return created;
            var scope = created.Value;
            try
            {
                scope.RegisterOwner(endedBy);
            }
            catch
            {
                scope.Dispose();
                throw;
            }
            return created;
        }

        private void Register(CancellationToken cancellationToken)
        {
            if (cancellationToken.IsCancellationRequested)
            {
                _token!.Cancel();
                return;
            }
            if (cancellationToken.CanBeCanceled)
            {
                _registration = cancellationToken.Register(OnCanceled, _token);
                _hasRegistration = true;
            }
        }

        private void RegisterOwner(CancellationToken disposing)
        {
            if (disposing.IsCancellationRequested)
            {
                _token!.Cancel();
                return;
            }
            _ownerRegistration = disposing.Register(OnCanceled, _token);
            _hasOwnerRegistration = true;
        }

        private static void OnCanceled(object? state)
        {
            try
            {
                ((CancelToken)state!).Cancel();
            }
            catch (ObjectDisposedException)
            {
            }
        }

        public void Dispose()
        {
            if (_disposed)
                return;
            _disposed = true;
            if (_hasRegistration)
            {
                _registration.Dispose();
                _hasRegistration = false;
            }
            if (_hasOwnerRegistration)
            {
                _ownerRegistration.Dispose();
                _hasOwnerRegistration = false;
            }
            _tokenBorrow.Dispose();
            if (_token != null && _ownsToken)
                _token.Dispose();
        }
    }

    internal sealed class NativeStructureChain : IDisposable
    {
        private readonly NativeMemoryScope _scope = new NativeMemoryScope();
        public IntPtr Head { get; private set; }

        /// <summary>
        /// Copies one or more managed extension chains, in order, into one native
        /// header.next chain. Each argument is walked through its own Next links.
        /// </summary>
        public static NativeStructureChain From(params Structure?[] heads)
        {
            var chain = new NativeStructureChain();
            try
            {
                IntPtr previous = IntPtr.Zero;
                foreach (var head in heads)
                    previous = chain.Build(head, previous);
                return chain;
            }
            catch
            {
                // Frees what was copied and releases every borrowed handle; the caller never gets
                // the chain to dispose.
                chain.Dispose();
                throw;
            }
        }

        private IntPtr Build(Structure? head, IntPtr previous)
        {
            for (var current = head; current != null; current = current.Next)
            {
                var currentPtr = NativeExtensions.AllocateNativeCopy(current, _scope);
                if (previous == IntPtr.Zero)
                    Head = currentPtr;
                else
                    NativeExtensions.SetNextPointer(previous, currentPtr);
                previous = NativeExtensions.Tail(currentPtr);
            }
            return previous;
        }

        public void Dispose()
        {
            _scope.Dispose();
            Head = IntPtr.Zero;
        }
    }

    internal static class NativeExtensions
    {
        /// <summary>
        /// Allocates the exact C layout of one managed extension in
        /// <paramref name="scope"/>. Most map to one native struct with header.next zero; an
        /// <see cref="OnboardLocalApp"/> with a tag or a remote-app grant maps to two or three,
        /// already chained, so the caller
        /// links what follows after <see cref="Tail"/>.
        /// </summary>
        internal static IntPtr AllocateNativeCopy(Structure structure, NativeMemoryScope scope)
        {
            switch (structure)
            {
                case StreamParamsD3D11 d3d:
                    return scope.AllocStruct(new StreamParamsD3D11Native
                    {
                        Header = NativeDefaults.Header(StructureType.StreamD3D11ParamsExt),
                        D3D11Context = d3d.D3D11Context,
                        D3D11Device = d3d.D3D11Device,
                        ColorBindFlags = d3d.ColorBindFlags,
                        DepthBindFlags = d3d.DepthBindFlags,
                        AcquireKeyedMutexSync = d3d.AcquireKeyedMutexSync
                    });
                case StreamParamsVulkan vulkan:
                    return scope.AllocStruct(new StreamParamsVulkanNative
                    {
                        Header = NativeDefaults.Header(StructureType.StreamVulkanParamsExt),
                        PhysicalDevice = vulkan.PhysicalDevice,
                        Device = vulkan.Device
                    });
                case StreamParamsOpenGlEgl egl:
                    return scope.AllocStruct(new StreamParamsOpenGlEglNative
                    {
                        Header = NativeDefaults.Header(StructureType.StreamOpenGlEglParamsExt),
                        Display = egl.Display,
                        DrawSurface = egl.DrawSurface,
                        ReadSurface = egl.ReadSurface,
                        Context = egl.Context,
                        Callbacks = egl.Callbacks.ToNative()
                    });
                case StreamParamsOpenGlWgl wgl:
                    return scope.AllocStruct(new StreamParamsOpenGlWglNative
                    {
                        Header = NativeDefaults.Header(StructureType.StreamOpenGlWglParamsExt),
                        DeviceContext = wgl.DeviceContext,
                        Context = wgl.Context,
                        Callbacks = wgl.Callbacks.ToNative()
                    });
                case StreamParamsOpenGlGlx glx:
                    return scope.AllocStruct(new StreamParamsOpenGlGlxNative
                    {
                        Header = NativeDefaults.Header(StructureType.StreamOpenGlGlxParamsExt),
                        Display = glx.Display,
                        Drawable = glx.Drawable,
                        Context = glx.Context,
                        Callbacks = glx.Callbacks.ToNative()
                    });
                case RenderFrameShowViewOverrides overrides:
                    var native = new RenderFrameShowViewOverridesExtNative
                    {
                        Header = NativeDefaults.Header(StructureType.RenderingEndFrameViewOverridesExt),
                        ViewOverrides = NativeDefaults.RenderFrameViewOverrides(),
                        ViewOverridesCount = (UIntPtr)Math.Min((overrides.ViewOverrides ?? Array.Empty<RenderFrameViewOverride>()).Length, NativeConstants.MaxFrameViews)
                    };
                    NativeCopy.CopyToFixed(overrides.ViewOverrides ?? Array.Empty<RenderFrameViewOverride>(), native.ViewOverrides, NativeConstants.MaxFrameViews, v => v.ToNative());
                    return scope.AllocStruct(native);
                case OnboardCode onboardCode:
                    return scope.AllocStruct(new OnboardCodeExtNative
                    {
                        Header = NativeDefaults.Header(StructureType.RuntimeOnboardCodeExt),
                        Code = scope.AllocUtf8(onboardCode.Code)
                    });
                case OnboardHost onboardHost:
                    return scope.AllocStruct(new OnboardHostExtNative
                    {
                        Header = NativeDefaults.Header(StructureType.RuntimeOnboardHostExt),
                        Hostname = scope.AllocUtf8(onboardHost.Hostname),
                        Port = onboardHost.Port
                    });
                case OnboardInvite onboardInvite:
                    // The scope holds a reference on the invite until the native onboard is done.
                    return scope.AllocStruct(new OnboardInviteExtNative
                    {
                        Header = NativeDefaults.Header(StructureType.RuntimeOnboardInviteExt),
                        Invite = onboardInvite.Invite == null ? IntPtr.Zero : onboardInvite.Invite.BorrowHandle(scope)
                    });
                case OnboardPeerId onboardPeerId:
                    return scope.AllocStruct(new OnboardPeerIdExtNative
                    {
                        Header = NativeDefaults.Header(StructureType.RuntimeOnboardPeerIdExt),
                        PeerId = onboardPeerId.PeerId.ToNative()
                    });
                case OnboardLocalApp onboardLocalApp:
                    return scope.AllocStruct(new OnboardLocalAppExtNative
                    {
                        Header = NativeDefaults.Header(StructureType.RuntimeOnboardLocalAppExt, AllocateLocalAppTag(onboardLocalApp.Tag, AllocateRemoteAppGrant(onboardLocalApp.RemoteAppGrant, scope), scope)),
                        ApprovalTimeoutMs = (uint)Math.Min(Math.Max(onboardLocalApp.ApprovalTimeout.TotalMilliseconds, 0.0), uint.MaxValue)
                    });
                case GraphicsDeviceId graphicsDeviceId:
                    return scope.AllocStruct(graphicsDeviceId.ToNative());
                case PeerDeviceIdentity identity:
                    return scope.AllocStruct(new PeerDeviceIdentityNative
                    {
                        Header = NativeDefaults.Header(StructureType.PeerDeviceIdentity),
                        Category = identity.Category,
                        DeviceId = scope.AllocUtf8(identity.DeviceId),
                        DeviceFamily = scope.AllocUtf8(identity.DeviceFamily),
                        DeviceName = scope.AllocUtf8(identity.DeviceName)
                    });
                case RuntimeEphemeralStorage _:
                    return scope.AllocStruct(new RuntimeEphemeralStorageExtNative
                    {
                        Header = NativeDefaults.Header(StructureType.RuntimeEphemeralStorageExt)
                    });
                default:
                    throw new NotSupportedException("Unsupported QAR extension structure type: " + structure.Type);
            }
        }

        // QarOnboardLocalAppTagExt, chained right after QarOnboardLocalAppExt and followed by
        // `next`; none for the default tag, which then links `next` directly.
        private static IntPtr AllocateLocalAppTag(string? tag, IntPtr next, NativeMemoryScope scope)
        {
            if (tag == null)
                return next;
            return scope.AllocStruct(new OnboardLocalAppTagExtNative
            {
                Header = NativeDefaults.Header(StructureType.RuntimeOnboardLocalAppTagExt, next),
                AppTag = scope.AllocUtf8(tag)
            });
        }

        // QarOnboardRemoteAppGrantExt, chained after the local-app (and tag) ext; none without a
        // grant. The scope holds a reference on the invitation until the native call is done.
        private static IntPtr AllocateRemoteAppGrant(MemberHubInvitation? invitation, NativeMemoryScope scope)
        {
            if (invitation == null)
                return IntPtr.Zero;
            return scope.AllocStruct(new OnboardRemoteAppGrantExtNative
            {
                Header = NativeDefaults.Header(StructureType.RuntimeOnboardRemoteAppGrantExt),
                Invitation = invitation.BorrowHandle(scope)
            });
        }

        /// <summary>The last struct of the native chain starting at <paramref name="structurePtr"/>.</summary>
        internal static IntPtr Tail(IntPtr structurePtr)
        {
            var offset = Marshal.OffsetOf(typeof(StructureHeaderNative), nameof(StructureHeaderNative.Next)).ToInt32();
            var tail = structurePtr;
            for (var next = Marshal.ReadIntPtr(tail, offset); next != IntPtr.Zero; next = Marshal.ReadIntPtr(tail, offset))
                tail = next;
            return tail;
        }

        internal static void SetNextPointer(IntPtr structurePtr, IntPtr next)
        {
            var offset = Marshal.OffsetOf(typeof(StructureHeaderNative), nameof(StructureHeaderNative.Next)).ToInt32();
            Marshal.WriteIntPtr(structurePtr, offset, next);
        }
    }

    internal static class NativeCopy
    {
        internal static void CopyToFixed<TManaged, TNative>(TManaged[] source, TNative[] destination, int maxCount, Func<TManaged, TNative> convert)
        {
            var count = Math.Min(source.Length, maxCount);
            for (var i = 0; i < count; i++)
                destination[i] = convert(source[i]);
        }

        internal static TManaged[] CopyFromFixed<TNative, TManaged>(TNative[] source, UIntPtr countPtr, int maxCount, Func<TNative, TManaged> convert)
        {
            var count = Math.Min(NativeConvert.ToInt32(countPtr), maxCount);
            var result = new TManaged[count];
            for (var i = 0; i < count; i++)
                result[i] = convert(source[i]);
            return result;
        }
    }

    internal static class NativeConvert
    {
        internal static int ToInt32(UIntPtr value)
        {
            var asUlong = value.ToUInt64();
            if (asUlong > int.MaxValue)
                throw new OverflowException("Native size_t value does not fit into Int32.");
            return (int)asUlong;
        }
    }

    internal static class NativeIdArrays
    {
        internal static PeerIdNative[] CreatePeerArray(int count)
        {
            var result = new PeerIdNative[count];
            for (var i = 0; i < result.Length; i++)
                result[i] = PeerIdNative.CreateDefault();
            return result;
        }

        internal static PeerIdNative[] ToNative(PeerId[] ids)
        {
            var result = new PeerIdNative[ids.Length];
            for (var i = 0; i < ids.Length; i++)
                result[i] = ids[i].ToNative();
            return result;
        }

        internal static PeerId[] ToPublic(PeerIdNative[] ids, int count)
        {
            var result = new PeerId[count];
            for (var i = 0; i < count; i++)
                result[i] = PeerId.FromNative(ids[i]);
            return result;
        }
    }

    internal static class IdBytes
    {
        private static readonly byte[] Empty = new byte[NativeConstants.MaxIdLength];

        internal static byte[] Normalize(byte[]? value)
        {
            var result = new byte[NativeConstants.MaxIdLength];
            if (value != null)
                Array.Copy(value, result, Math.Min(value.Length, result.Length));
            return result;
        }

#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        internal static ReadOnlySpan<byte> AsSpan(byte[]? value)
        {
            return value ?? Empty;
        }

        internal static bool TryCopyTo(byte[]? value, Span<byte> destination)
        {
            if (destination.Length < NativeConstants.MaxIdLength)
                return false;
            for (var i = 0; i < NativeConstants.MaxIdLength; i++)
                destination[i] = value != null && i < value.Length ? value[i] : (byte)0;
            return true;
        }
#endif

        internal static byte[] Copy(byte[]? value)
        {
            return Normalize(value);
        }

        internal static bool TryParse(string text, out byte[] data)
        {
            data = new byte[NativeConstants.MaxIdLength];
            if (text == null)
                return false;
            using (var scope = new NativeMemoryScope())
            {
                var result = Results.ToInfo(NativeMethods.qar_impl_uuid_from_string(scope.AllocUtf8(text), data));
                return result.IsSuccess;
            }
        }

        internal static string ToAscii(byte[]? value)
        {
            var bytes = Normalize(value);
            var buffer = new byte[NativeConstants.MaxStringLength];
            Results.ToInfo(NativeMethods.qar_impl_uuid_to_string(bytes, buffer, (UIntPtr)buffer.Length)).EnsureSuccess();
            return NativeString.DecodeNullTerminated(buffer);
        }

        internal static bool Equals(byte[]? left, byte[]? right)
        {
            var leftBytes = Normalize(left);
            var rightBytes = Normalize(right);
            for (var i = 0; i < NativeConstants.MaxIdLength; i++)
            {
                if (leftBytes[i] != rightBytes[i])
                    return false;
            }
            return true;
        }

        internal static int GetHashCode(byte[]? value)
        {
            var bytes = Normalize(value);
            unchecked
            {
                var hash = 17;
                for (var i = 0; i < bytes.Length; i++)
                    hash = (hash * 31) + bytes[i];
                return hash;
            }
        }
    }
}
