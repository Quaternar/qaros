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

        // Step 1 wins over the installed QAROS (ADR-0339): the test output folder holds the
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
        public void TheRuntimeDirEnvironmentVariableIsRead()
        {
            var name = InstalledRuntime.RuntimeDirEnvironmentVariable;
            var previous = Environment.GetEnvironmentVariable(name);
            try
            {
                Environment.SetEnvironmentVariable(name, "\"C:\\Some\\Runtime Folder\"");
                Assert.Equal("C:\\Some\\Runtime Folder", InstalledRuntime.TryGetRuntimeDirFromEnvironment());
                Environment.SetEnvironmentVariable(name, "   ");
                Assert.Null(InstalledRuntime.TryGetRuntimeDirFromEnvironment());
            }
            finally
            {
                Environment.SetEnvironmentVariable(name, previous);
            }
        }

        [Fact]
        public void UseLocalAppSelectsLocalAppModeWithDefaults()
        {
            var options = new OnboardOptions().UseLocalApp();
            var mode = Assert.IsType<OnboardLocalApp>(options.Mode);
            Assert.Equal(StructureType.RuntimeOnboardLocalAppExt, mode.Type);
            Assert.Equal(TimeSpan.Zero, mode.ApprovalTimeout);
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
