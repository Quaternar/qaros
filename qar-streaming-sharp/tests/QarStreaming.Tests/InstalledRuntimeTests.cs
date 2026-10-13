using System;
using System.IO;
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
