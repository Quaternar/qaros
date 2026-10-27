using System;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;
using Quaternar.Qaros;
using Quaternar.Qaros.Tests.Ext;
using Xunit;

namespace Quaternar.Qaros.Tests
{
    // The member-hub invitation part of qar-hub-test-ext (qar-common-lib/test/hub_test_ext): hubs
    // that serve local enrollment on an unregistered instance's pipe and restart as the launcher
    // does.
    internal static class HubInvitationExtNativeMethods
    {
        internal const uint ServeLocalEnrollment = 0x1;
        internal const int RolePrimary = 0;
        internal const int RoleMember = 1;

        [DllImport(HubTestExtNativeMethods.LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int qar_hub_test_ext_create_ex(byte[] rootUtf8, byte[] nameUtf8, uint flags, out IntPtr hub);

        [DllImport(HubTestExtNativeMethods.LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int qar_hub_test_ext_enrollment_instance(IntPtr hub, byte[]? buffer, UIntPtr bufferSize, out UIntPtr length);

        [DllImport(HubTestExtNativeMethods.LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int qar_hub_test_ext_approve_this_process(IntPtr hub, int maySponsorMembers, uint sponsorQuota);

        [DllImport(HubTestExtNativeMethods.LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int qar_hub_test_ext_role(IntPtr hub, out int role, [Out] byte[] primaryPeerId);

        [DllImport(HubTestExtNativeMethods.LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int qar_hub_test_ext_get_peer_id(IntPtr hub, [Out] byte[] peerId);

        [DllImport(HubTestExtNativeMethods.LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int qar_hub_test_ext_set_qar_instance(byte[]? instanceUtf8);

        [DllImport(HubTestExtNativeMethods.LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int qar_hub_test_ext_approval_count(IntPtr hub, out uint count);
    }

    /// <summary>A hub of qar-hub-test-ext that serves local enrollment, under a temp root of its own.</summary>
    public sealed class InvitationTestHub : IDisposable
    {
        private readonly string _root;
        private IntPtr _hub;

        public InvitationTestHub(string name)
        {
            _root = Path.Combine(Path.GetTempPath(), "qar-sharp-tests", "hub-invitation", Guid.NewGuid().ToString("N"));
            Directory.CreateDirectory(_root);
            Check(HubInvitationExtNativeMethods.qar_hub_test_ext_create_ex(
                Utf8(_root), Utf8(name), HubInvitationExtNativeMethods.ServeLocalEnrollment, out _hub), "create");
            Check(HubInvitationExtNativeMethods.qar_hub_test_ext_enrollment_instance(_hub, null, UIntPtr.Zero, out var length), "instance (length)");
            var buffer = new byte[(int)length.ToUInt64() + 1];
            Check(HubInvitationExtNativeMethods.qar_hub_test_ext_enrollment_instance(_hub, buffer, (UIntPtr)buffer.Length, out length), "instance");
            Instance = Encoding.UTF8.GetString(buffer, 0, (int)length.ToUInt64());
        }

        /// <summary>The QAROS instance whose pipe the hub serves.</summary>
        public string Instance { get; }

        /// <summary>The next native call reaches this hub as "the hub on this PC".</summary>
        public void UseAsHubOnThisPc()
        {
            Check(HubInvitationExtNativeMethods.qar_hub_test_ext_set_qar_instance(Utf8(Instance)), "set QAR_INSTANCE");
        }

        public void ApproveThisProcess(bool maySponsorMembers, uint quota)
        {
            Check(HubInvitationExtNativeMethods.qar_hub_test_ext_approve_this_process(_hub, maySponsorMembers ? 1 : 0, quota), "approve");
        }

        public byte[] PeerId()
        {
            var id = new byte[16];
            Check(HubInvitationExtNativeMethods.qar_hub_test_ext_get_peer_id(_hub, id), "peer id");
            return id;
        }

        public (int Role, byte[] Primary) Role()
        {
            var primary = new byte[16];
            Check(HubInvitationExtNativeMethods.qar_hub_test_ext_role(_hub, out var role, primary), "role");
            return (role, primary);
        }

        /// <summary>
        /// Whether the hub holds at least one app approval within <paramref name="budget"/> (scaled):
        /// a member receives its primary's approvals a moment after its restart.
        /// </summary>
        public bool HoldsApprovalWithin(TimeSpan budget)
        {
            var deadline = DateTime.UtcNow + ExtTime.Scaled(budget);
            while (true)
            {
                Check(HubInvitationExtNativeMethods.qar_hub_test_ext_approval_count(_hub, out var count), "approval count");
                if (count > 0)
                    return true;
                if (DateTime.UtcNow >= deadline)
                    return false;
                System.Threading.Thread.Sleep(50);
            }
        }

        public static void ClearHubOnThisPc()
        {
            Check(HubInvitationExtNativeMethods.qar_hub_test_ext_set_qar_instance(null), "clear QAR_INSTANCE");
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

        private static byte[] Utf8(string text)
        {
            return Encoding.UTF8.GetBytes(text + "\0");
        }

        private static void Check(int status, string what)
        {
            if (status != 0)
                throw new InvalidOperationException("qar-hub-test-ext " + what + " failed with status " + status + " (see the log)");
        }
    }

    /// <summary>
    /// Member-hub invitations through the C# binding against real hubs (GitHub #262): an app on
    /// the primary's PC creates an invitation, the hub on the joining PC joins by it through its
    /// restart, and the app then onboards there without a prompt. Both PCs are this process.
    /// </summary>
    [Collection("QAR Native")]
    public sealed class HubMemberInvitationTests : IClassFixture<QarNativeFixture>
    {
        public HubMemberInvitationTests(QarNativeFixture fixture)
        {
            _ = fixture;
        }

        [HubTestExtFact]
        public void AnInvitationJoinsTheHubOnThisPcAndTheAppOnboardsWithoutAPrompt()
        {
            var storageRoot = Path.Combine(Path.GetTempPath(), "qar-sharp-tests", Guid.NewGuid().ToString("N"));
            using var primary = new InvitationTestHub("primary");
            using var joining = new InvitationTestHub("joining");
            using var runtime = Runtime.Create(new RuntimeOptions { StorageFolderPath = storageRoot }).ValueOrThrow();
            try
            {
                primary.ApproveThisProcess(true, 2);
                primary.UseAsHubOnThisPc();
                byte[] bytes;
                using (var created = runtime.CreateMemberHubInvitation(new MemberHubInvitationOptions { Label = "C# test" }).ValueOrThrow())
                    bytes = created.Serialize().ValueOrThrow();
                using var invitation = MemberHubInvitation.Deserialize(bytes).ValueOrThrow();
                joining.UseAsHubOnThisPc();
                var options = new JoinHubAsMemberOptions(invitation) { RestartTimeout = ExtTime.Scaled(TimeSpan.FromSeconds(90)) };
                runtime.JoinHubAsMember(options).ValueOrThrow();
                var role = joining.Role();
                Assert.Equal(HubInvitationExtNativeMethods.RoleMember, role.Role);
                Assert.Equal(primary.PeerId(), role.Primary);
                // Again: already a member of that primary, success at once.
                runtime.JoinHubAsMember(options).ValueOrThrow();
                // The join did not onboard this app; onboarding it on the member needs no prompt. The
                // test host is signed, so the hub's covering check on the primary cannot match it to
                // the hash approval: wait until the member holds that approval itself.
                Assert.True(joining.HoldsApprovalWithin(TimeSpan.FromSeconds(30)), "the member never received the app's approval");
                var onboarded = runtime.Onboard(new OnboardOptions()
                    .UseLocalApp(new OnboardLocalApp { ApprovalTimeout = TimeSpan.FromSeconds(5) })
                    .WithPresentation(new PeerPresentationOptions { DisplayName = "QarStreamingSharpTests invitation" })).ValueOrThrow();
                using var session = onboarded.Session;
            }
            finally
            {
                InvitationTestHub.ClearHubOnThisPc();
            }
        }

        [HubTestExtFact]
        public void CreatingNeedsTheRightToBringInMemberHubs()
        {
            var storageRoot = Path.Combine(Path.GetTempPath(), "qar-sharp-tests", Guid.NewGuid().ToString("N"));
            using var primary = new InvitationTestHub("primary");
            using var runtime = Runtime.Create(new RuntimeOptions { StorageFolderPath = storageRoot }).ValueOrThrow();
            try
            {
                primary.ApproveThisProcess(false, 0);
                primary.UseAsHubOnThisPc();
                var created = runtime.CreateMemberHubInvitation(new MemberHubInvitationOptions());
                Assert.Equal(StatusCode.MemberSponsorNotAllowed, created.Info.StatusCode);
            }
            finally
            {
                InvitationTestHub.ClearHubOnThisPc();
            }
        }
    }
}
