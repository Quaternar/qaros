using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Qar;

namespace Qar.Native
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

        public void Dispose()
        {
            for (var i = _allocations.Count - 1; i >= 0; i--)
                Marshal.FreeHGlobal(_allocations[i]);
            _allocations.Clear();
        }
    }

    internal sealed class NativeCancellationScope : IDisposable
    {
        private readonly CancelToken? _token;

        private CancellationTokenRegistration _registration;

        private bool _hasRegistration;

        private bool _detached;

        private NativeCancellationScope(CancelToken? token)
        {
            _token = token;
        }

        public IntPtr Handle { get { return _token == null ? IntPtr.Zero : _token.DangerousHandle; } }

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
            if (_hasRegistration)
            {
                _registration.Dispose();
                _hasRegistration = false;
            }
            if (_token != null && !_detached)
                _token.Dispose();
        }

        public void Detach()
        {
            _detached = true;
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
            IntPtr previous = IntPtr.Zero;
            foreach (var head in heads)
                previous = chain.Build(head, previous);
            return chain;
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
                previous = currentPtr;
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
        /// <paramref name="scope"/>, with header.next zero.
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
                    return scope.AllocStruct(new OnboardInviteExtNative
                    {
                        Header = NativeDefaults.Header(StructureType.RuntimeOnboardInviteExt),
                        Invite = onboardInvite.Invite == null ? IntPtr.Zero : onboardInvite.Invite.DangerousHandle
                    });
                case OnboardPeerId onboardPeerId:
                    return scope.AllocStruct(new OnboardPeerIdExtNative
                    {
                        Header = NativeDefaults.Header(StructureType.RuntimeOnboardPeerIdExt),
                        PeerId = onboardPeerId.PeerId.ToNative()
                    });
                case GraphicsDeviceId graphicsDeviceId:
                    return scope.AllocStruct(graphicsDeviceId.ToNative());
                default:
                    throw new NotSupportedException("Unsupported QAR extension structure type: " + structure.Type);
            }
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
