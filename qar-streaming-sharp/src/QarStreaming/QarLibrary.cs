using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Qar.Native;

namespace Qar
{
    public static class Library
    {
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
                    LogFolderPath = scope.AllocString(options.LogFolderPath),
                    LogSeverity = options.LogSeverity
                };
                return Results.FromNative(NativeMethods.qar_impl_library_init(ref native));
            }
        }

        public static Result<Unit> Destroy()
        {
            return Results.FromNative(NativeMethods.qar_impl_library_destroy());
        }

    }
}
