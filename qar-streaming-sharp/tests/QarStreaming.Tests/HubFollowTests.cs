using System;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using Quaternar.Qaros;
using Quaternar.Qaros.Tests.Ext;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    // P/Invoke layer of qar-hub-test-ext: a real secured primary hub that rotates its session,
    // never shipped (Quaternar repository, qar-common-lib/test/hub_test_ext/README.md). The plain
    // hub of qar-streaming-c-ext cannot rotate.
    internal static class HubTestExtNativeMethods
    {
        internal const string LibraryName = "qar-hub-test-ext";

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int qar_hub_test_ext_create(byte[] rootUtf8, out IntPtr hub);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int qar_hub_test_ext_mint_invite(IntPtr hub, byte[]? buffer, UIntPtr bufferSize, out UIntPtr length);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int qar_hub_test_ext_rotate_session(IntPtr hub, [Out] byte[] sessionId);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void qar_hub_test_ext_destroy(IntPtr hub);
    }

    /// <summary>A secured primary hub that rotates, under a temp root of its own.</summary>
    public sealed class HubTestExt : IDisposable
    {
        private readonly string _root;
        private IntPtr _hub;

        public HubTestExt()
        {
            _root = Path.Combine(Path.GetTempPath(), "qar-sharp-tests", "hub-follow", Guid.NewGuid().ToString("N"));
            Directory.CreateDirectory(_root);
            Check(HubTestExtNativeMethods.qar_hub_test_ext_create(Encoding.UTF8.GetBytes(_root + "\0"), out _hub), "create");
        }

        /// <summary>Whether qar-hub-test-ext.dll sits beside the tests (only in the Quaternar repository).</summary>
        public static bool IsAvailable
        {
            get { return File.Exists(Path.Combine(AppContext.BaseDirectory, HubTestExtNativeMethods.LibraryName + ".dll")); }
        }

        /// <summary>A shared-password invite wire string for one device.</summary>
        public string MintInvite()
        {
            Check(HubTestExtNativeMethods.qar_hub_test_ext_mint_invite(_hub, null, UIntPtr.Zero, out var length), "mint invite (length)");
            var buffer = new byte[(int)length.ToUInt64() + 1];
            Check(HubTestExtNativeMethods.qar_hub_test_ext_mint_invite(_hub, buffer, (UIntPtr)buffer.Length, out length), "mint invite");
            return Encoding.UTF8.GetString(buffer, 0, (int)length.ToUInt64());
        }

        /// <summary>Rotates the hub's session; the 16 bytes of the new session id.</summary>
        public byte[] RotateSession()
        {
            var next = new byte[16];
            Check(HubTestExtNativeMethods.qar_hub_test_ext_rotate_session(_hub, next), "rotate session");
            return next;
        }

        public void Dispose()
        {
            if (_hub != IntPtr.Zero)
            {
                HubTestExtNativeMethods.qar_hub_test_ext_destroy(_hub);
                _hub = IntPtr.Zero;
            }
            try
            {
                Directory.Delete(_root, true);
            }
            catch (IOException)
            {
            }
            catch (UnauthorizedAccessException)
            {
            }
        }

        private static void Check(int status, string what)
        {
            if (status != 0)
                throw new InvalidOperationException("qar-hub-test-ext " + what + " failed with status " + status + " (see the log)");
        }
    }

    /// <summary>A fact that is skipped where qar-hub-test-ext.dll is not built (a QAROS package drop).</summary>
    public sealed class HubTestExtFactAttribute : FactAttribute
    {
        public HubTestExtFactAttribute()
        {
            if (!HubTestExt.IsAvailable)
                Skip = "qar-hub-test-ext.dll is not beside the tests; build qar-hub-test-ext_gtests in the Quaternar repository.";
        }
    }

    [Collection("QAR Native")]
    public sealed class HubFollowTests : IClassFixture<QarNativeFixture>
    {
        public HubFollowTests(QarNativeFixture fixture)
        {
            _ = fixture;
        }

        // A move waits for the fallback's 6 s when no notice arrives, then a probe, a renewal and
        // the reconnect.
        private static TimeSpan FollowBudget { get { return ExtTime.Scaled(TimeSpan.FromSeconds(60)); } }

        // docs/hub-topology D9, D11: the same Session follows its hub's rotation, and GetId returns
        // the new session id once it has moved.
        [HubTestExtFact]
        public void GetIdReturnsTheNewIdAfterARealRotation()
        {
            var storageRoot = Path.Combine(Path.GetTempPath(), "qar-sharp-tests", Guid.NewGuid().ToString("N"));
            using var hub = new HubTestExt();
            using var runtime = Runtime.Create(new RuntimeOptions { StorageFolderPath = storageRoot }).ValueOrThrow();
            using var invite = OnboardingInvite.Deserialize(Encoding.UTF8.GetBytes(hub.MintInvite())).ValueOrThrow();
            var onboarded = runtime.Onboard(new OnboardOptions()
                .UseInvite(invite)
                .WithPresentation(new PeerPresentationOptions { DisplayName = "QarStreamingSharpTests follow" })).ValueOrThrow();
            using var session = onboarded.Session;

            var next = hub.RotateSession();
            var deadline = DateTime.UtcNow + FollowBudget;
            while (DateTime.UtcNow < deadline && !IsIn(session, next))
                Thread.Sleep(20);

            Assert.Equal(next, session.GetId().ValueOrThrow().ToByteArray());

            try
            {
                Directory.Delete(storageRoot, true);
            }
            catch (IOException)
            {
            }
            catch (UnauthorizedAccessException)
            {
            }
        }

        private static bool IsIn(Session session, byte[] next)
        {
            var id = session.GetId();
            return id.IsSuccess && id.Value.ToByteArray().SequenceEqual(next);
        }
    }
}
