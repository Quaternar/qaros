using System;
using System.IO;
using System.Runtime.CompilerServices;
using Quaternar.Qaros;

namespace Quaternar.Qaros.Tests
{
    /// <summary>
    /// Binds the suite to the qar-streaming-c.dll copied beside the test assembly (the build
    /// under test, see Directory.Build.targets) before any QAR call. Without it the binding
    /// prefers an installed QAROS and the tests would exercise that instead.
    /// </summary>
    internal static class BuildUnderTestRuntime
    {
        [ModuleInitializer]
        internal static void Load()
        {
            var library = Path.Combine(AppContext.BaseDirectory, InstalledRuntime.LibraryFileName);
            if (!File.Exists(library))
                return;
            var loaded = InstalledRuntime.Load(library);
            if (!loaded.IsSuccess)
                throw new InvalidOperationException("Loading the qar-streaming-c build under test failed: " + loaded.Info);
        }
    }
}

#if !NET5_0_OR_GREATER
namespace System.Runtime.CompilerServices
{
    [AttributeUsage(AttributeTargets.Method, Inherited = false)]
    internal sealed class ModuleInitializerAttribute : Attribute
    {
    }
}
#endif
