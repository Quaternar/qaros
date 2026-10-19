using System;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// Finds and loads the QAROS runtime, <c>qar-streaming-c.dll</c>, the same way as the C
    /// header's <c>qar_library_load(NULL)</c> and every other QAROS binding (ADR-0339). The first
    /// match wins:
    /// <list type="number">
    /// <item>(step 0) a <c>qar-streaming-c.dll</c> already loaded in the process: nothing is
    /// loaded;</item>
    /// <item>beside this binding: the folder of the <c>Quaternar.Qaros</c> assembly, then the
    /// application's base directory (a test project's output folder, a development ZIP);</item>
    /// <item>the folder named by the <c>QAROS_RUNTIME_DIR</c> environment variable; when it is
    /// set, a runtime missing there is an error;</item>
    /// <item>the installed QAROS: <c>&lt;InstallFolder&gt;\bin</c>, read from <c>HKLM</c>
    /// <c>SOFTWARE\Quaternar\QAROS</c>, value <c>InstallFolder</c>; an installed copy that fails
    /// to load is an error;</item>
    /// <item>every folder on <c>PATH</c>, in order. The current directory is never searched.</item>
    /// </list>
    /// Each loads from its absolute path, its dependencies from its own folder and System32 only.
    /// <see cref="Library.Init"/> loads it on first use, so calling <see cref="Load()"/> yourself
    /// is only needed to learn about a missing runtime early. Windows only; elsewhere the
    /// binding's <c>DllImport</c> probing finds the library.
    /// </summary>
    public static class InstalledRuntime
    {
        /// <summary>File name of the C runtime library inside the installed <c>bin</c> folder.</summary>
        public const string LibraryFileName = "qar-streaming-c.dll";

        private const string RegistryKey = @"SOFTWARE\Quaternar\QAROS";
        private const string InstallFolderValue = "InstallFolder";
        /// <summary>Environment variable naming a folder that holds the runtime (step 2).</summary>
        public const string RuntimeDirEnvironmentVariable = "QAROS_RUNTIME_DIR";

        private const string NotFoundMessage =
            @"qar-streaming-c.dll is not beside the application, QAROS_RUNTIME_DIR is not set, QAROS is not installed (no InstallFolder under HKLM\SOFTWARE\Quaternar\QAROS), and it is not on PATH. Install QAROS to run this application.";

        private static readonly object s_lock = new object();
        private static IntPtr s_handle = IntPtr.Zero;
        private static string? s_loadedPath;

        /// <summary>
        /// Full path of the loaded <c>qar-streaming-c.dll</c>, or null before a successful
        /// <see cref="Load()"/>.
        /// </summary>
        public static string? LoadedLibraryPath
        {
            get { lock (s_lock) { return s_loadedPath; } }
        }

        /// <summary>
        /// <c>&lt;InstallFolder&gt;\bin</c> of the installed QAROS, read from HKLM (never
        /// HKCU, which every process of the user can write). Null when QAROS is not installed
        /// or the platform is not Windows.
        /// </summary>
        public static string? TryGetBinFolder()
        {
            if (!IsWindows())
                return null;
            var installFolder = ReadInstallFolder(RegistryRoots.LocalMachine);
            if (string.IsNullOrEmpty(installFolder))
                return null;
            return Path.Combine(installFolder, "bin");
        }

        /// <summary>
        /// Loads the runtime by the lookup described on <see cref="InstalledRuntime"/>: already
        /// loaded, else beside the binding, else <c>QAROS_RUNTIME_DIR</c>, else the installed QAROS,
        /// else on <c>PATH</c>. Returns the loaded file's path. Fails with
        /// <see cref="StatusCode.LogicError"/> when nothing is found, or when the copy that
        /// <c>QAROS_RUNTIME_DIR</c> or the installation names is missing or does not load.
        /// </summary>
        public static Result<string> Load()
        {
            lock (s_lock)
            {
                return LoadDefaultLocked();
            }
        }

        /// <summary>
        /// Loads <c>qar-streaming-c.dll</c> from <paramref name="libraryPath"/> instead of the
        /// installation. For our own development builds; a distributed application uses
        /// <see cref="Load()"/>. Must run before the first QAR call.
        /// </summary>
        public static Result<string> Load(string libraryPath)
        {
            if (libraryPath == null)
                throw new ArgumentNullException(nameof(libraryPath));
            lock (s_lock)
            {
                if (s_handle != IntPtr.Zero)
                {
                    if (string.Equals(s_loadedPath, Path.GetFullPath(libraryPath), StringComparison.OrdinalIgnoreCase))
                        return Result<string>.Success(SuccessInfo(), s_loadedPath!);
                    return Result<string>.Failure(new ResultInfo(StatusCode.LogicError, 0,
                        "qar-streaming-c is already loaded from '" + s_loadedPath + "'; it cannot be replaced by '" + libraryPath + "'."));
                }
                return LoadLocked(Path.GetFullPath(libraryPath));
            }
        }

        /// <summary>Used by <see cref="Library.Init"/>: <see cref="Load()"/>.</summary>
        internal static Result<string> EnsureLoaded()
        {
            return Load();
        }

        private static Result<string> LoadDefaultLocked()
        {
            if (s_handle != IntPtr.Zero)
                return Result<string>.Success(SuccessInfo(), s_loadedPath!);
            if (!IsWindows())
                return Result<string>.Success(SuccessInfo(), LibraryFileName);
            // 1. Already in the process (a host, or a load outside this class).
            var existing = Kernel32.GetModuleHandleW(LibraryFileName);
            if (existing != IntPtr.Zero)
            {
                s_handle = existing;
                s_loadedPath = ModulePath(existing) ?? LibraryFileName;
                RegisterResolver();
                return Result<string>.Success(SuccessInfo(), s_loadedPath);
            }
            // 1. Beside this binding.
            var beside = FirstExisting(BindingFolders());
            if (beside != null)
                return LoadLocked(beside);
            // 2. QAROS_RUNTIME_DIR; when set, it is the only copy considered.
            var envFolder = TryGetRuntimeDirFromEnvironment();
            if (envFolder != null)
                return LoadLocked(Path.GetFullPath(Path.Combine(envFolder, LibraryFileName)));
            // 3. The installed QAROS; an installed copy that fails to load is an error.
            var binFolder = TryGetBinFolder();
            if (binFolder != null)
                return LoadLocked(Path.GetFullPath(Path.Combine(binFolder, LibraryFileName)));
            // 4. Each folder on PATH.
            var onPath = FirstExisting(PathFolders());
            if (onPath != null)
                return LoadLocked(onPath);
            return Result<string>.Failure(new ResultInfo(StatusCode.LogicError, 0, NotFoundMessage));
        }

        /// <summary>
        /// The folder <c>QAROS_RUNTIME_DIR</c> names, or null when it is unset or empty.
        /// </summary>
        public static string? TryGetRuntimeDirFromEnvironment()
        {
            var folder = Environment.GetEnvironmentVariable(RuntimeDirEnvironmentVariable);
            return string.IsNullOrWhiteSpace(folder) ? null : folder!.Trim().Trim('"');
        }

        private static string? FirstExisting(System.Collections.Generic.IEnumerable<string> folders)
        {
            foreach (var folder in folders)
            {
                var candidate = Path.Combine(folder, LibraryFileName);
                if (File.Exists(candidate))
                    return Path.GetFullPath(candidate);
            }
            return null;
        }

        /// <summary>
        /// Step 1: the folder of the <c>Quaternar.Qaros</c> assembly, then the application's base
        /// directory. Empty for an assembly loaded from memory.
        /// </summary>
        private static System.Collections.Generic.IEnumerable<string> BindingFolders()
        {
            var assembly = typeof(InstalledRuntime).Assembly.Location;
            if (!string.IsNullOrEmpty(assembly))
            {
                var folder = Path.GetDirectoryName(assembly);
                if (!string.IsNullOrEmpty(folder))
                    yield return folder!;
            }
            if (!string.IsNullOrEmpty(AppContext.BaseDirectory))
                yield return AppContext.BaseDirectory;
        }

        /// <summary>Step 4: every rooted folder on <c>PATH</c>. Never the current directory.</summary>
        private static System.Collections.Generic.IEnumerable<string> PathFolders()
        {
            var path = Environment.GetEnvironmentVariable("PATH") ?? string.Empty;
            foreach (var raw in path.Split(Path.PathSeparator))
            {
                var entry = raw.Trim().Trim('"');
                if (entry.Length > 0 && Path.IsPathRooted(entry))
                    yield return entry;
            }
        }

        private static Result<string> LoadLocked(string fullPath)
        {
            if (!IsWindows())
                return Result<string>.Failure(new ResultInfo(StatusCode.NotImplemented, 0, "Loading the installed QAROS runtime is supported on Windows only."));
            if (!File.Exists(fullPath))
                return Result<string>.Failure(new ResultInfo(StatusCode.LogicError, 0,
                    "QAROS runtime library not found at '" + fullPath + "'. Check QAROS_RUNTIME_DIR, or repair or reinstall QAROS."));
            // Absolute path; dependencies from the library's own folder and System32 only,
            // never from the application's folder (same rule as the C loader).
            var handle = Kernel32.LoadLibraryExW(fullPath, IntPtr.Zero, Kernel32.LoadLibrarySearchDllLoadDir | Kernel32.LoadLibrarySearchSystem32);
            if (handle == IntPtr.Zero)
            {
                var error = Marshal.GetLastWin32Error();
                return Result<string>.Failure(new ResultInfo(StatusCode.LogicError, 0,
                    "Loading the QAROS runtime '" + fullPath + "' failed: Win32 error " + error + "."));
            }
            s_handle = handle;
            s_loadedPath = fullPath;
            RegisterResolver();
            return Result<string>.Success(SuccessInfo(), fullPath);
        }

        // On Windows a bare DllImport name binds to an already-loaded module of the same
        // name, which the preload above provides. .NET 8 additionally gets an explicit
        // resolver so the binding never depends on the default search.
        private static void RegisterResolver()
        {
#if NET8_0_OR_GREATER
            if (s_resolverRegistered)
                return;
            s_resolverRegistered = true;
            NativeLibrary.SetDllImportResolver(typeof(InstalledRuntime).Assembly, ResolveImport);
#endif
        }

#if NET8_0_OR_GREATER
        private static bool s_resolverRegistered;

        private static IntPtr ResolveImport(string libraryName, System.Reflection.Assembly assembly, DllImportSearchPath? searchPath)
        {
            if (libraryName != NativeMethods.LibraryName)
                return IntPtr.Zero;
            lock (s_lock)
            {
                return s_handle;
            }
        }
#endif

        private static ResultInfo SuccessInfo()
        {
            return new ResultInfo(StatusCode.Success, 0, string.Empty);
        }

        private static bool IsWindows()
        {
            return RuntimeInformation.IsOSPlatform(OSPlatform.Windows);
        }

        private static string? ReadInstallFolder(IntPtr root)
        {
            uint size = 0;
            uint type;
            var status = Advapi32.RegGetValueW(root, RegistryKey, InstallFolderValue, Advapi32.RrfRtRegSz, out type, null, ref size);
            if (status != 0 || size == 0)
                return null;
            var buffer = new char[(size / 2) + 1];
            status = Advapi32.RegGetValueW(root, RegistryKey, InstallFolderValue, Advapi32.RrfRtRegSz, out type, buffer, ref size);
            if (status != 0)
                return null;
            var text = new string(buffer);
            var end = text.IndexOf('\0');
            return end >= 0 ? text.Substring(0, end) : text;
        }

        private static string? ModulePath(IntPtr module)
        {
            var buffer = new StringBuilder(1024);
            var length = Kernel32.GetModuleFileNameW(module, buffer, (uint)buffer.Capacity);
            return length == 0 ? null : buffer.ToString(0, (int)length);
        }

        private static class RegistryRoots
        {
            internal static readonly IntPtr LocalMachine = new IntPtr(unchecked((int)0x80000002));
        }

        private static class Advapi32
        {
            internal const uint RrfRtRegSz = 0x00000002;
            [DllImport("advapi32.dll", CharSet = CharSet.Unicode, ExactSpelling = true)]
            internal static extern int RegGetValueW(IntPtr hkey, string subKey, string value, uint flags, out uint type, [Out] char[]? data, ref uint dataSize);
        }

        private static class Kernel32
        {
            internal const uint LoadLibrarySearchDllLoadDir = 0x00000100;
            internal const uint LoadLibrarySearchSystem32 = 0x00000800;
            [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true, ExactSpelling = true)]
            internal static extern IntPtr LoadLibraryExW(string fileName, IntPtr file, uint flags);
            [DllImport("kernel32.dll", CharSet = CharSet.Unicode, ExactSpelling = true)]
            internal static extern IntPtr GetModuleHandleW(string moduleName);
            [DllImport("kernel32.dll", CharSet = CharSet.Unicode, ExactSpelling = true)]
            internal static extern uint GetModuleFileNameW(IntPtr module, StringBuilder fileName, uint size);
        }
    }

    /// <summary>What the loaded runtime reports about itself (<see cref="Library.GetRuntimeInfo"/>).</summary>
    public sealed class RuntimeInfo
    {
        internal RuntimeInfo(string version, string flavor, string cApi)
        {
            Version = version;
            Flavor = flavor;
            CApi = cApi;
        }

        /// <summary>Product version, e.g. "0.9.0" or "0.9.0-rc.2".</summary>
        public string Version { get; }

        /// <summary>"release" or "demo".</summary>
        public string Flavor { get; }

        /// <summary>C API generation, "v0".</summary>
        public string CApi { get; }
    }
}
