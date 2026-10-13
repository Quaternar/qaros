using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// Process-wide library state. <see cref="Init"/> comes first, before any other call of this
    /// API; then <see cref="Runtime.Create"/>, then <see cref="Runtime.Onboard"/> or
    /// <see cref="Runtime.Rejoin"/> for a <see cref="Session"/>.
    /// </summary>
    public static class Library
    {
        /// <summary>
        /// Loads the QAROS runtime installed on this PC (<see cref="InstalledRuntime"/>) and
        /// initializes the library once per process, from a single thread. Calling it again without
        /// a <see cref="Destroy"/> in between may fail. Fails with a "QAROS is not installed"
        /// result when there is no installation, instead of a <see cref="DllNotFoundException"/>.
        /// </summary>
        public static Result<Unit> Init(LibraryOptions? options = null)
        {
            var loaded = InstalledRuntime.EnsureLoaded();
            if (!loaded.IsSuccess)
                return Result<Unit>.Failure(loaded.Info);
            options = options ?? new LibraryOptions();
            using (var scope = new NativeMemoryScope())
            using (var chain = NativeStructureChain.From(options.Extension))
            {
                var native = new LibraryInitNative
                {
                    Header = NativeDefaults.Header(StructureType.LibraryInit, chain.Head),
                    EnableConsoleLogging = options.EnableConsoleLogging,
                    LogFolderPath = scope.AllocUtf8(options.LogFolderPath),
                    LogSeverity = options.LogSeverity
                };
                return Results.FromNative(NativeMethods.qar_impl_library_init(ref native));
            }
        }

        /// <summary>
        /// Frees process-wide state. No other call of this API is allowed afterwards until the next
        /// <see cref="Init"/>.
        /// </summary>
        /// <remarks>
        /// It ends every session and subscription still alive, waiting for a subscription handler
        /// running on another thread. Dispose your sessions and the runtime first, and never call it
        /// from a QAROS callback, event handler or progress report.
        /// </remarks>
        public static Result<Unit> Destroy()
        {
            return Results.FromNative(NativeMethods.qar_impl_library_destroy());
        }

        /// <summary>
        /// Version, flavor and C API generation of the loaded runtime. Loads the installed
        /// runtime first if needed; works before <see cref="Init"/>.
        /// </summary>
        public static Result<RuntimeInfo> GetRuntimeInfo()
        {
            var loaded = InstalledRuntime.EnsureLoaded();
            if (!loaded.IsSuccess)
                return Result<RuntimeInfo>.Failure(loaded.Info);
            RuntimeInfoNative native;
            var info = Results.ToInfo(NativeMethods.qar_impl_runtime_info(out native));
            if (!info.IsSuccess)
                return Result<RuntimeInfo>.Failure(info);
            return Result<RuntimeInfo>.Success(info, new RuntimeInfo(
                NativeString.DecodeNullTerminated(native.Version) ?? string.Empty,
                NativeString.DecodeNullTerminated(native.Flavor) ?? string.Empty,
                NativeString.DecodeNullTerminated(native.CApi) ?? string.Empty));
        }
    }
}
