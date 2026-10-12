using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Qar.Native;

namespace Qar
{
    /// <summary>
    /// Process-wide library state. <see cref="Init"/> comes first, before any other call of this
    /// API; then <see cref="Runtime.Create"/>, then <see cref="Runtime.Onboard"/> or
    /// <see cref="Runtime.Rejoin"/> for a <see cref="Session"/>.
    /// </summary>
    public static class Library
    {
        /// <summary>
        /// Initializes the library once per process, from a single thread. Calling it again without
        /// a <see cref="Destroy"/> in between may fail.
        /// </summary>
        public static Result<Unit> Init(LibraryOptions? options = null)
        {
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

    }
}
