using System;
using System.IO;
using System.Runtime.InteropServices;
using Quaternar.Qaros;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    public sealed class InstalledRuntimeTests
    {
        [Fact]
        public void BinFolderIsNullOrTheInstallBinFolder()
        {
            var folder = InstalledRuntime.TryGetBinFolder();
            if (folder == null)
                return;
            Assert.True(Path.IsPathRooted(folder), folder);
            Assert.Equal("bin", Path.GetFileName(folder));
        }

        // Step 1 wins over the installed QAROS: the test output folder holds the
        // runtime under test, so that is the copy the process runs against.
        [Fact]
        public void TheRuntimeBesideTheBindingIsLoadedFirst()
        {
            if (!RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
                return;
            var beside = Path.Combine(
                Path.GetDirectoryName(typeof(InstalledRuntime).Assembly.Location)!,
                InstalledRuntime.LibraryFileName);
            if (!File.Exists(beside))
                return;
            var loaded = InstalledRuntime.Load();
            Assert.True(loaded.IsSuccess, loaded.Info.Message);
            Assert.Equal(Path.GetFullPath(beside), loaded.Value, StringComparer.OrdinalIgnoreCase);
        }

        [Fact]
        public void TheInstanceEnvironmentVariableIsRead()
        {
            var name = InstalledRuntime.InstanceEnvironmentVariable;
            var previous = Environment.GetEnvironmentVariable(name);
            try
            {
                Environment.SetEnvironmentVariable(name, "dev-0123456789abcdef");
                Assert.Equal("dev-0123456789abcdef", InstalledRuntime.TryGetRequestedInstance());
                Environment.SetEnvironmentVariable(name, null);
                Assert.Null(InstalledRuntime.TryGetRequestedInstance());
            }
            finally
            {
                Environment.SetEnvironmentVariable(name, previous);
            }
        }

        [Fact]
        public void InstanceNamesAreShortPortableTokens()
        {
            Assert.True(InstalledRuntime.IsValidInstanceName("default"));
            Assert.True(InstalledRuntime.IsValidInstanceName("dev-0123456789abcdef"));
            Assert.True(InstalledRuntime.IsValidInstanceName(new string('a', 64)));
            Assert.False(InstalledRuntime.IsValidInstanceName(new string('a', 65)));
            Assert.False(InstalledRuntime.IsValidInstanceName(""));
            Assert.False(InstalledRuntime.IsValidInstanceName("with space"));
            Assert.False(InstalledRuntime.IsValidInstanceName("..\\escape"));
        }

        // The record a development hub writes (HKCU ...\Instances\<name>, value RuntimeFolder) is
        // the folder step 2 loads from. The name is unique to this run and removed afterwards.
        [Fact]
        public void ARecordedInstanceRuntimeFolderIsResolved()
        {
            if (!RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
                return;
            var instance = UniqueInstanceName();
            var folder = Path.Combine(Path.GetTempPath(), "qaros-instance-" + Guid.NewGuid().ToString("N"));
            try
            {
                Assert.Equal(0, TestRegistry.SetRuntimeFolder(instance, folder));

                Assert.Equal(folder, InstalledRuntime.TryGetInstanceRuntimeFolder(instance));
                var library = InstalledRuntime.ResolveInstanceLibrary(instance);
                Assert.True(library.IsSuccess, library.Info.Message);
                Assert.Equal(Path.Combine(folder, InstalledRuntime.LibraryFileName), library.Value, StringComparer.OrdinalIgnoreCase);
            }
            finally
            {
                TestRegistry.DeleteInstance(instance);
            }
        }

        // A relative folder would load against the current directory: it counts as no record.
        [Fact]
        public void ARelativeInstanceRuntimeFolderIsIgnored()
        {
            if (!RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
                return;
            var instance = UniqueInstanceName();
            try
            {
                Assert.Equal(0, TestRegistry.SetRuntimeFolder(instance, "relative\\bin"));

                Assert.Null(InstalledRuntime.TryGetInstanceRuntimeFolder(instance));
                Assert.False(InstalledRuntime.ResolveInstanceLibrary(instance).IsSuccess);
            }
            finally
            {
                TestRegistry.DeleteInstance(instance);
            }
        }

        // A registered named instance (InstallFolder) without a RuntimeFolder resolves to its bin.
        [Fact]
        public void ARegisteredInstanceWithoutRuntimeFolderUsesItsBin()
        {
            if (!RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
                return;
            var instance = UniqueInstanceName();
            var installFolder = Path.Combine(Path.GetTempPath(), "qaros-instance-" + Guid.NewGuid().ToString("N"));
            try
            {
                Assert.Equal(0, TestRegistry.SetValue(instance, "InstallFolder", installFolder));

                Assert.Equal(Path.Combine(installFolder, "bin"), InstalledRuntime.TryGetInstanceRuntimeFolder(instance));
            }
            finally
            {
                TestRegistry.DeleteInstance(instance);
            }
        }

        // An instance nobody recorded is an error naming it, never the installed runtime.
        [Fact]
        public void AnUnknownInstanceIsAnErrorNotTheInstalledRuntime()
        {
            if (!RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
                return;
            var instance = UniqueInstanceName();

            Assert.Null(InstalledRuntime.TryGetInstanceRuntimeFolder(instance));
            var library = InstalledRuntime.ResolveInstanceLibrary(instance);
            Assert.False(library.IsSuccess);
            Assert.Equal(StatusCode.LogicError, library.Info.StatusCode);
            Assert.Contains(instance, library.Info.Message);
        }

        [Fact]
        public void AnInvalidInstanceIsAnError()
        {
            var library = InstalledRuntime.ResolveInstanceLibrary("not a name");
            Assert.False(library.IsSuccess);
            Assert.Null(InstalledRuntime.TryGetInstanceRuntimeFolder("not a name"));
        }

        [Fact]
        public void TheDefaultInstanceIsTheInstalledQaros()
        {
            Assert.Equal(InstalledRuntime.TryGetBinFolder(), InstalledRuntime.TryGetInstanceRuntimeFolder(InstalledRuntime.DefaultInstanceName));
        }

        private static string UniqueInstanceName()
        {
            return "cs-test-" + System.Diagnostics.Process.GetCurrentProcess().Id + "-" + Guid.NewGuid().ToString("N").Substring(0, 12);
        }

        private static class TestRegistry
        {
            private const string InstancesKey = @"SOFTWARE\Quaternar\QAROS\Instances\";
            private static readonly IntPtr CurrentUser = new IntPtr(unchecked((int)0x80000001));
            private const uint RegSz = 1;

            internal static int SetRuntimeFolder(string instance, string folder)
            {
                return SetValue(instance, "RuntimeFolder", folder);
            }

            internal static int SetValue(string instance, string valueName, string value)
            {
                var data = value + "\0";
                return RegSetKeyValueW(CurrentUser, InstancesKey + instance, valueName, RegSz, data, (uint)(data.Length * 2));
            }

            internal static void DeleteInstance(string instance)
            {
                RegDeleteTreeW(CurrentUser, InstancesKey + instance);
            }

            [DllImport("advapi32.dll", CharSet = CharSet.Unicode, ExactSpelling = true)]
            private static extern int RegSetKeyValueW(IntPtr key, string subKey, string valueName, uint type, string data, uint dataSize);

            [DllImport("advapi32.dll", CharSet = CharSet.Unicode, ExactSpelling = true)]
            private static extern int RegDeleteTreeW(IntPtr key, string subKey);
        }

        [Fact]
        public void UseLocalAppSelectsLocalAppModeWithDefaults()
        {
            var options = new OnboardOptions().UseLocalApp();
            var mode = Assert.IsType<OnboardLocalApp>(options.Mode);
            Assert.Equal(StructureType.RuntimeOnboardLocalAppExt, mode.Type);
            Assert.Equal(TimeSpan.Zero, mode.ApprovalTimeout);
            Assert.Null(mode.Tag);
        }

        [Fact]
        public void UseLocalAppKeepsCallerSettings()
        {
            var local = new OnboardLocalApp { ApprovalTimeout = TimeSpan.FromSeconds(30) };
            var options = new OnboardOptions().UseLocalApp(local);
            Assert.Same(local, options.Mode);
        }
    }
}
