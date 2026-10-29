using System;
using System.Linq;
using System.Runtime.InteropServices;
using Quaternar.Qaros;
using Quaternar.Qaros.Interop;
using Xunit;
using Quaternar.Qaros.Mathematics;

namespace Quaternar.Qaros.Tests
{
    public sealed class NativeLayoutTests
    {
        private const string CanonicalId = "00112233-4455-6677-8899-aabbccddeeff";

        // Expected values are the x64 C layout of qar_streaming_c/v0/types.h
        // (sizeof / offsetof printed by a C program compiled against the header
        // with QAR_ENABLE_OPENGL and QAR_ENABLE_VULKAN). Update them together
        // with the header.
        public static TheoryData<Type, int> NativeSizes = new TheoryData<Type, int>
        {
            { typeof(ResultNative), 8 },
            { typeof(StructureHeaderNative), 16 },
            { typeof(PeerIdNative), 16 },
            { typeof(MemberHubInvitationInitNative), 40 },
            { typeof(JoinHubAsMemberInitNative), 32 },
            { typeof(TimePointNative), 8 },
            { typeof(PoseNative), 28 },
            { typeof(FovNative), 16 },
            { typeof(HandJointLocationNative), 48 },
            { typeof(HandJointVelocityNative), 40 },
            { typeof(HandJointsNative), 2328 },
            { typeof(DeviceHandsWithJointsNative), 4680 },
            { typeof(RenderFrameViewNative), 12 },
            { typeof(TextureSizeNative), 16 },
            { typeof(VideoFrameViewNative), 36 },
            { typeof(VideoFrameLayoutNative), 368 },
            { typeof(VideoTextureCpuNative), 40 },
            { typeof(VideoFrameCpuNative), 464 },
            { typeof(VideoTextureD3D11Native), 24 },
            { typeof(DXGIKeyedMutexSyncNative), 16 },
            { typeof(SyncFrameD3D11Native), 72 },
            { typeof(VideoFrameD3D11Native), 472 },
            { typeof(VideoTextureOpenGlNative), 20 },
            { typeof(VideoFrameOpenGlNative), 384 },
            { typeof(VideoTextureVulkanNative), 48 },
            { typeof(SyncFrameVulkanNative), 16 },
            { typeof(VideoFrameVulkanNative), 512 },
            { typeof(GuiPanelSizeNative), 12 },
            { typeof(GuiPanelPointNative), 8 },
            { typeof(GeoAnchorFrameNative), 112 },
            { typeof(AppWorldAnchorNative), 120 },
            { typeof(AppVolumeSizeNative), 12 },
            { typeof(AppVolumeEditingStatusNative), 17 },
            { typeof(AppVolumeControllerRayStyleNative), 8 },
            { typeof(AppVolumeGestureMappingRuleNative), 56 },
            { typeof(AppVolumeGestureConfigurationNative), 920 },
            { typeof(AppVolumeGestureAllowedPeersExtNative), 32 },
            { typeof(LibraryInitNative), 40 },
            { typeof(RuntimeInitNative), 32 },
            { typeof(PeerPresentationNative), 40 },
            { typeof(RejoinInitNative), 72 },
            { typeof(OnboardInitNative), 72 },
            { typeof(OnboardCodeExtNative), 24 },
            { typeof(OnboardHostExtNative), 32 },
            { typeof(PeerDeviceIdentityNative), 48 },
            { typeof(OnboardInviteExtNative), 24 },
            { typeof(OnboardPeerIdExtNative), 32 },
            { typeof(OnboardLocalAppExtNative), 24 },
            { typeof(OnboardLocalAppTagExtNative), 24 },
            { typeof(RuntimeEphemeralStorageExtNative), 16 },
            { typeof(OnboardRemoteAppGrantExtNative), 24 },
            { typeof(MemberHubInvitationAppGrantExtNative), 24 },
            { typeof(RuntimeInfoNative), 24 },
            { typeof(RequestInviteInitNative), 24 },
            { typeof(SessionInviteTargetAppInitNative), 24 },
            { typeof(ForgetInitNative), 32 },
            { typeof(LuidNative), 8 },
            { typeof(GraphicsDeviceIdNative), 48 },
            { typeof(SessionLifecycleInfoNative), 72 },
            { typeof(GuiPanelInitNative), 88 },
            { typeof(GuiPanelStreamIdExtNative), 32 },
            { typeof(GuiPanelUrlExtNative), 24 },
            { typeof(AppVolumeInitNative), 248 },
            { typeof(RenderSenderInitNative), 168 },
            { typeof(StreamParamsD3D11Native), 48 },
            { typeof(OpenGlContextThreadStateNative), 40 },
            { typeof(OpenGlContextCallbacksNative), 40 },
            { typeof(StreamParamsOpenGlEglNative), 88 },
            { typeof(StreamParamsOpenGlWglNative), 72 },
            { typeof(StreamParamsOpenGlGlxNative), 80 },
            { typeof(StreamParamsVulkanNative), 32 },
            { typeof(VideoSenderInitNative), 408 },
            { typeof(RenderFrameShowNative), 32 },
            { typeof(RenderFrameViewOverrideNative), 56 },
            { typeof(RenderFrameShowViewOverridesExtNative), 472 },
        };

        public static TheoryData<Type, string, int> NativeOffsets = new TheoryData<Type, string, int>
        {
            { typeof(HandJointLocationNative), nameof(HandJointLocationNative.JointId), 0 },
            { typeof(HandJointLocationNative), nameof(HandJointLocationNative.LocationFlags), 8 },
            { typeof(HandJointLocationNative), nameof(HandJointLocationNative.Pose), 16 },
            { typeof(HandJointLocationNative), nameof(HandJointLocationNative.Radius), 44 },
            { typeof(HandJointVelocityNative), nameof(HandJointVelocityNative.Flags), 8 },
            { typeof(HandJointVelocityNative), nameof(HandJointVelocityNative.LinearVelocity), 16 },
            { typeof(HandJointVelocityNative), nameof(HandJointVelocityNative.AngularVelocity), 28 },
            { typeof(HandJointsNative), nameof(HandJointsNative.IsTracked), 0 },
            { typeof(HandJointsNative), nameof(HandJointsNative.IsActive), 1 },
            { typeof(HandJointsNative), nameof(HandJointsNative.Pose), 4 },
            { typeof(HandJointsNative), nameof(HandJointsNative.JointLocations), 32 },
            { typeof(HandJointsNative), nameof(HandJointsNative.HasVelocity), 1280 },
            { typeof(HandJointsNative), nameof(HandJointsNative.JointVelocities), 1288 },
            { typeof(DeviceHandsWithJointsNative), nameof(DeviceHandsWithJointsNative.DeviceId), 0 },
            { typeof(DeviceHandsWithJointsNative), nameof(DeviceHandsWithJointsNative.DeviceTimestamp), 16 },
            { typeof(DeviceHandsWithJointsNative), nameof(DeviceHandsWithJointsNative.LeftHand), 24 },
            { typeof(DeviceHandsWithJointsNative), nameof(DeviceHandsWithJointsNative.RightHand), 2352 },
            { typeof(VideoFrameViewNative), nameof(VideoFrameViewNative.TextureIndex), 16 },
            { typeof(VideoFrameViewNative), nameof(VideoFrameViewNative.ArrayLayerIndex), 20 },
            { typeof(VideoFrameViewNative), nameof(VideoFrameViewNative.TextureFormat), 24 },
            { typeof(VideoFrameViewNative), nameof(VideoFrameViewNative.DataType), 28 },
            { typeof(VideoFrameViewNative), nameof(VideoFrameViewNative.Eye), 32 },
            { typeof(VideoFrameLayoutNative), nameof(VideoFrameLayoutNative.ViewsCount), 288 },
            { typeof(VideoFrameLayoutNative), nameof(VideoFrameLayoutNative.Textures), 296 },
            { typeof(VideoFrameLayoutNative), nameof(VideoFrameLayoutNative.TexturesCount), 360 },
            { typeof(VideoTextureCpuNative), nameof(VideoTextureCpuNative.Pitch), 16 },
            { typeof(VideoTextureCpuNative), nameof(VideoTextureCpuNative.TextureData), 24 },
            { typeof(VideoTextureCpuNative), nameof(VideoTextureCpuNative.TextureDataSize), 32 },
            { typeof(VideoFrameCpuNative), nameof(VideoFrameCpuNative.TextureViewsCount), 288 },
            { typeof(VideoFrameCpuNative), nameof(VideoFrameCpuNative.Textures), 296 },
            { typeof(VideoFrameCpuNative), nameof(VideoFrameCpuNative.TexturesCount), 456 },
            { typeof(VideoFrameD3D11Native), nameof(VideoFrameD3D11Native.Textures), 296 },
            { typeof(VideoFrameD3D11Native), nameof(VideoFrameD3D11Native.TexturesCount), 392 },
            { typeof(VideoFrameD3D11Native), nameof(VideoFrameD3D11Native.Synchronization), 400 },
            { typeof(VideoFrameOpenGlNative), nameof(VideoFrameOpenGlNative.Textures), 296 },
            { typeof(VideoFrameOpenGlNative), nameof(VideoFrameOpenGlNative.TexturesCount), 376 },
            { typeof(VideoTextureVulkanNative), nameof(VideoTextureVulkanNative.Image), 16 },
            { typeof(VideoTextureVulkanNative), nameof(VideoTextureVulkanNative.ImageMemory), 24 },
            { typeof(VideoTextureVulkanNative), nameof(VideoTextureVulkanNative.FormatFeatures), 32 },
            { typeof(VideoTextureVulkanNative), nameof(VideoTextureVulkanNative.Layout), 36 },
            { typeof(VideoTextureVulkanNative), nameof(VideoTextureVulkanNative.QueueFamilyIndex), 40 },
            { typeof(VideoFrameVulkanNative), nameof(VideoFrameVulkanNative.Textures), 296 },
            { typeof(VideoFrameVulkanNative), nameof(VideoFrameVulkanNative.TexturesCount), 488 },
            { typeof(VideoFrameVulkanNative), nameof(VideoFrameVulkanNative.Synchronization), 496 },
            { typeof(GeoAnchorFrameNative), nameof(GeoAnchorFrameNative.OriginWorld), 8 },
            { typeof(GeoAnchorFrameNative), nameof(GeoAnchorFrameNative.Handedness), 104 },
            { typeof(AppWorldAnchorNative), nameof(AppWorldAnchorNative.Anchor), 8 },
            { typeof(AppVolumeEditingStatusNative), nameof(AppVolumeEditingStatusNative.EditorPeer), 1 },
            { typeof(AppVolumeGestureMappingRuleNative), nameof(AppVolumeGestureMappingRuleNative.GestureKind), 16 },
            { typeof(AppVolumeGestureMappingRuleNative), nameof(AppVolumeGestureMappingRuleNative.Enabled), 20 },
            { typeof(AppVolumeGestureMappingRuleNative), nameof(AppVolumeGestureMappingRuleNative.Precision), 24 },
            { typeof(AppVolumeGestureMappingRuleNative), nameof(AppVolumeGestureMappingRuleNative.EnableGesturesFromOutsideOfVolume), 40 },
            { typeof(AppVolumeGestureMappingRuleNative), nameof(AppVolumeGestureMappingRuleNative.ControllerRayStyle), 44 },
            { typeof(AppVolumeGestureMappingRuleNative), nameof(AppVolumeGestureMappingRuleNative.AppTransformMapping), 52 },
            { typeof(AppVolumeGestureConfigurationNative), nameof(AppVolumeGestureConfigurationNative.MappingRules), 16 },
            { typeof(AppVolumeGestureConfigurationNative), nameof(AppVolumeGestureConfigurationNative.MappingRuleCount), 912 },
            { typeof(AppVolumeGestureAllowedPeersExtNative), nameof(AppVolumeGestureAllowedPeersExtNative.AllowedPeers), 16 },
            { typeof(AppVolumeGestureAllowedPeersExtNative), nameof(AppVolumeGestureAllowedPeersExtNative.AllowedPeerCount), 24 },
            { typeof(LibraryInitNative), nameof(LibraryInitNative.EnableConsoleLogging), 16 },
            { typeof(LibraryInitNative), nameof(LibraryInitNative.LogFolderPath), 24 },
            { typeof(LibraryInitNative), nameof(LibraryInitNative.LogSeverity), 32 },
            { typeof(RejoinInitNative), nameof(RejoinInitNative.OnboardingId), 16 },
            { typeof(RejoinInitNative), nameof(RejoinInitNative.Presentation), 32 },
            { typeof(OnboardHostExtNative), nameof(OnboardHostExtNative.Port), 24 },
            { typeof(PeerDeviceIdentityNative), nameof(PeerDeviceIdentityNative.Category), 16 },
            { typeof(PeerDeviceIdentityNative), nameof(PeerDeviceIdentityNative.DeviceId), 24 },
            { typeof(PeerDeviceIdentityNative), nameof(PeerDeviceIdentityNative.DeviceFamily), 32 },
            { typeof(PeerDeviceIdentityNative), nameof(PeerDeviceIdentityNative.DeviceName), 40 },
            { typeof(OnboardPeerIdExtNative), nameof(OnboardPeerIdExtNative.PeerId), 16 },
            { typeof(OnboardLocalAppExtNative), nameof(OnboardLocalAppExtNative.ApprovalTimeoutMs), 16 },
            { typeof(OnboardLocalAppTagExtNative), nameof(OnboardLocalAppTagExtNative.AppTag), 16 },
            { typeof(OnboardRemoteAppGrantExtNative), nameof(OnboardRemoteAppGrantExtNative.Invitation), 16 },
            { typeof(MemberHubInvitationAppGrantExtNative), nameof(MemberHubInvitationAppGrantExtNative.AppTag), 16 },
            { typeof(RequestInviteInitNative), nameof(RequestInviteInitNative.AppTag), 16 },
            { typeof(RuntimeInfoNative), nameof(RuntimeInfoNative.Flavor), 8 },
            { typeof(RuntimeInfoNative), nameof(RuntimeInfoNative.CApi), 16 },
            { typeof(SessionInviteTargetAppInitNative), nameof(SessionInviteTargetAppInitNative.ConnectionString), 16 },
            { typeof(GraphicsDeviceIdNative), nameof(GraphicsDeviceIdNative.IdType), 16 },
            { typeof(GraphicsDeviceIdNative), nameof(GraphicsDeviceIdNative.Luid), 20 },
            { typeof(GraphicsDeviceIdNative), nameof(GraphicsDeviceIdNative.Uuid), 28 },
            { typeof(SessionLifecycleInfoNative), nameof(SessionLifecycleInfoNative.State), 16 },
            { typeof(SessionLifecycleInfoNative), nameof(SessionLifecycleInfoNative.Cause), 20 },
            { typeof(SessionLifecycleInfoNative), nameof(SessionLifecycleInfoNative.SessionId), 24 },
            { typeof(SessionLifecycleInfoNative), nameof(SessionLifecycleInfoNative.PreviousSessionId), 40 },
            { typeof(SessionLifecycleInfoNative), nameof(SessionLifecycleInfoNative.HasPreviousSession), 56 },
            { typeof(SessionLifecycleInfoNative), nameof(SessionLifecycleInfoNative.Reason), 64 },
            { typeof(GuiPanelInitNative), nameof(GuiPanelInitNative.CommonName), 16 },
            { typeof(GuiPanelInitNative), nameof(GuiPanelInitNative.DisplayName), 24 },
            { typeof(GuiPanelInitNative), nameof(GuiPanelInitNative.Pose), 32 },
            { typeof(GuiPanelInitNative), nameof(GuiPanelInitNative.Size), 60 },
            { typeof(GuiPanelInitNative), nameof(GuiPanelInitNative.VisibleToPeers), 72 },
            { typeof(GuiPanelInitNative), nameof(GuiPanelInitNative.VisibleToPeerCount), 80 },
            { typeof(AppVolumeInitNative), nameof(AppVolumeInitNative.CommonName), 16 },
            { typeof(AppVolumeInitNative), nameof(AppVolumeInitNative.Pose), 32 },
            { typeof(AppVolumeInitNative), nameof(AppVolumeInitNative.Size), 60 },
            { typeof(AppVolumeInitNative), nameof(AppVolumeInitNative.InitialPeers), 72 },
            { typeof(AppVolumeInitNative), nameof(AppVolumeInitNative.InitialPeerCount), 80 },
            { typeof(AppVolumeInitNative), nameof(AppVolumeInitNative.AppPose), 88 },
            { typeof(AppVolumeInitNative), nameof(AppVolumeInitNative.AppScale), 116 },
            { typeof(AppVolumeInitNative), nameof(AppVolumeInitNative.HasAppWorldAnchor), 120 },
            { typeof(AppVolumeInitNative), nameof(AppVolumeInitNative.AppWorldAnchor), 128 },
            { typeof(AppVolumeInitNative), nameof(AppVolumeInitNative.GestureConfiguration), 240 },
            { typeof(RenderSenderInitNative), nameof(RenderSenderInitNative.PeerId), 16 },
            { typeof(RenderSenderInitNative), nameof(RenderSenderInitNative.EnableAutoReconnects), 32 },
            { typeof(RenderSenderInitNative), nameof(RenderSenderInitNative.TextureLayout), 36 },
            { typeof(RenderSenderInitNative), nameof(RenderSenderInitNative.FrameViews), 40 },
            { typeof(RenderSenderInitNative), nameof(RenderSenderInitNative.FrameViewsCount), 136 },
            { typeof(RenderSenderInitNative), nameof(RenderSenderInitNative.AppVolumeId), 144 },
            { typeof(RenderSenderInitNative), nameof(RenderSenderInitNative.ColorFormat), 152 },
            { typeof(RenderSenderInitNative), nameof(RenderSenderInitNative.DepthFormat), 156 },
            { typeof(RenderSenderInitNative), nameof(RenderSenderInitNative.GraphicsAPI), 160 },
            { typeof(StreamParamsD3D11Native), nameof(StreamParamsD3D11Native.ColorBindFlags), 32 },
            { typeof(StreamParamsD3D11Native), nameof(StreamParamsD3D11Native.AcquireKeyedMutexSync), 40 },
            { typeof(OpenGlContextThreadStateNative), nameof(OpenGlContextThreadStateNative.HasContext), 32 },
            { typeof(StreamParamsOpenGlEglNative), nameof(StreamParamsOpenGlEglNative.Context), 40 },
            { typeof(StreamParamsOpenGlEglNative), nameof(StreamParamsOpenGlEglNative.Callbacks), 48 },
            { typeof(StreamParamsOpenGlWglNative), nameof(StreamParamsOpenGlWglNative.Context), 24 },
            { typeof(StreamParamsOpenGlWglNative), nameof(StreamParamsOpenGlWglNative.Callbacks), 32 },
            { typeof(StreamParamsOpenGlGlxNative), nameof(StreamParamsOpenGlGlxNative.Context), 32 },
            { typeof(StreamParamsOpenGlGlxNative), nameof(StreamParamsOpenGlGlxNative.Callbacks), 40 },
            { typeof(VideoSenderInitNative), nameof(VideoSenderInitNative.Layout), 16 },
            { typeof(VideoSenderInitNative), nameof(VideoSenderInitNative.EnableReconnects), 384 },
            { typeof(VideoSenderInitNative), nameof(VideoSenderInitNative.ReconnectTimeoutMs), 392 },
            { typeof(VideoSenderInitNative), nameof(VideoSenderInitNative.GraphicsApi), 400 },
            { typeof(RenderFrameShowNative), nameof(RenderFrameShowNative.RenderedNearFar), 16 },
            { typeof(RenderFrameShowNative), nameof(RenderFrameShowNative.DepthScale), 24 },
            { typeof(RenderFrameViewOverrideNative), nameof(RenderFrameViewOverrideNative.OverridePose), 8 },
            { typeof(RenderFrameViewOverrideNative), nameof(RenderFrameViewOverrideNative.OverrideFov), 9 },
            { typeof(RenderFrameViewOverrideNative), nameof(RenderFrameViewOverrideNative.Pose), 12 },
            { typeof(RenderFrameViewOverrideNative), nameof(RenderFrameViewOverrideNative.Fov), 40 },
            { typeof(RenderFrameShowViewOverridesExtNative), nameof(RenderFrameShowViewOverridesExtNative.ViewOverrides), 16 },
            { typeof(RenderFrameShowViewOverridesExtNative), nameof(RenderFrameShowViewOverridesExtNative.ViewOverridesCount), 464 },
            { typeof(MemberHubInvitationInitNative), nameof(MemberHubInvitationInitNative.LifetimeSeconds), 16 },
            { typeof(MemberHubInvitationInitNative), nameof(MemberHubInvitationInitNative.MaxUses), 20 },
            { typeof(MemberHubInvitationInitNative), nameof(MemberHubInvitationInitNative.Permissions), 24 },
            { typeof(MemberHubInvitationInitNative), nameof(MemberHubInvitationInitNative.Label), 32 },
            { typeof(JoinHubAsMemberInitNative), nameof(JoinHubAsMemberInitNative.Invitation), 16 },
            { typeof(JoinHubAsMemberInitNative), nameof(JoinHubAsMemberInitNative.HubStartTimeoutMs), 24 },
            { typeof(JoinHubAsMemberInitNative), nameof(JoinHubAsMemberInitNative.RestartTimeoutMs), 28 },
        };

        [Theory]
        [MemberData(nameof(NativeSizes))]
        public void NativeStructSizeMatchesC(Type type, int expectedSize)
        {
            Assert.Equal(expectedSize, Marshal.SizeOf(type));
        }

        [Theory]
        [MemberData(nameof(NativeOffsets))]
        public void NativeFieldOffsetMatchesC(Type type, string field, int expectedOffset)
        {
            Assert.Equal(expectedOffset, Marshal.OffsetOf(type, field).ToInt32());
        }

        [Fact]
        public void HandJointCountMatchesC()
        {
            Assert.Equal(26, NativeConstants.HandJointCount);
            Assert.Equal(26, Enum.GetValues(typeof(HandJoint)).Length);
            Assert.Equal(25, (int)HandJoint.LittleTip);
        }

        [Fact]
        public void StatusCodesMirrorC()
        {
            Assert.Equal(805, (int)StatusCode.RenderingProducerAppVolumeUnresolved);
            Assert.Equal(804, (int)StatusCode.RenderingProducerStreamIsClosed);
            Assert.Equal(327, (int)StatusCode.AppVolumeGestureModeUnknown);
            Assert.Equal(1803, (int)StatusCode.OnboardingSessionStillActive);
            Assert.Equal(1805, (int)StatusCode.OnboardingApprovalDenied);
            Assert.Equal(1806, (int)StatusCode.OnboardingApprovalTimeout);
            Assert.Equal(1807, (int)StatusCode.OnboardingHubNotAuthentic);
            Assert.Equal(1808, (int)StatusCode.OnboardingAppIdentityMismatch);
            Assert.Equal(1809, (int)StatusCode.OnboardingRePairRequired);
            Assert.Equal(1810, (int)StatusCode.OnboardingRenewalPossessionUnproven);
            Assert.Equal(1812, (int)StatusCode.OnboardingInviteRefused);
            Assert.Equal(3100, (int)StatusCode.HubTargetAppDeviceNotInvitable);
            Assert.Equal(3101, (int)StatusCode.PeerDeviceCategoryNotAllowed);
            Assert.Equal(3300, (int)StatusCode.HubLifecycleOperationRunning);
            Assert.Equal(602, (int)StatusCode.MediaStreamingStreamIsPaused);
            Assert.Equal(3400, (int)StatusCode.MemberJoinPending);
            Assert.Equal(3401, (int)StatusCode.MemberJoinDenied);
            Assert.Equal(3402, (int)StatusCode.MemberJoinRefused);
            Assert.Equal(3403, (int)StatusCode.MemberJoinRedirected);
            Assert.Equal(3404, (int)StatusCode.MemberRemoved);
            Assert.Equal(3405, (int)StatusCode.PrimaryUnreachable);
            Assert.Equal(3406, (int)StatusCode.SessionWideActionOnMember);
            Assert.Equal(3407, (int)StatusCode.MemberSponsorNotAllowed);
            Assert.Equal(3408, (int)StatusCode.MemberSponsorQuotaExceeded);
            Assert.Equal(3409, (int)StatusCode.AuditLogCorrupt);
            Assert.Equal(3500, (int)StatusCode.SecureStorageError);
            Assert.Equal(3501, (int)StatusCode.SecureStorageNotFound);
            Assert.Equal(3502, (int)StatusCode.SecureStorageTampered);
            Assert.Equal(3503, (int)StatusCode.SecureStorageProtectorMismatch);
            Assert.Equal(3504, (int)StatusCode.SecureStorageProtectorUnavailable);
            Assert.Equal(3505, (int)StatusCode.SecureStorageInvalidKey);
            Assert.Equal(3506, (int)StatusCode.SecureStorageLocked);
            Assert.Equal(3600, (int)StatusCode.MemberInvitationInvalid);
            Assert.Equal(3601, (int)StatusCode.MemberInvitationExpired);
            Assert.Equal(3602, (int)StatusCode.MemberInvitationUsedUp);
            Assert.Equal(3603, (int)StatusCode.MemberInvitationRevoked);
            Assert.Equal(3604, (int)StatusCode.MemberInvitationPrimaryNotAllowed);
            Assert.Equal(3605, (int)StatusCode.MemberInvitationPolicyNotProtected);
            Assert.False(Enum.IsDefined(typeof(StatusCode), 601));
            Assert.Equal(58, Enum.GetValues(typeof(StatusCode)).Length);
        }

        [Fact]
        public void StructureTypesMirrorC()
        {
            Assert.Equal(0x1009, (int)StructureType.RuntimeOnboardPeerIdExt);
            Assert.Equal(0x100A, (int)StructureType.RuntimeOnboardLocalAppExt);
            Assert.Equal(0x100D, (int)StructureType.RuntimeCreateMemberHubInvitationInit);
            Assert.Equal(0x100E, (int)StructureType.RuntimeJoinHubAsMemberInit);
            Assert.Equal(0x100F, (int)StructureType.RuntimeOnboardLocalAppTagExt);
            Assert.Equal(0x1010, (int)StructureType.RuntimeEphemeralStorageExt);
            Assert.Equal(0x1011, (int)StructureType.RuntimeMemberHubInvitationAppGrantExt);
            Assert.Equal(0x1012, (int)StructureType.RuntimeOnboardRemoteAppGrantExt);
            Assert.False(Enum.IsDefined(typeof(StructureType), 0x100B));
            Assert.False(Enum.IsDefined(typeof(StructureType), 0x100C));
            // The untagged request-invite init and the removed onboarding-codes init are retired.
            Assert.Equal(0x200A, (int)StructureType.SessionRequestInviteInit);
            Assert.False(Enum.IsDefined(typeof(StructureType), 0x2005));
            Assert.False(Enum.IsDefined(typeof(StructureType), 0x2008));
            Assert.Equal(0x2004, (int)StructureType.SessionGraphicsDeviceId);
            Assert.Equal(0x2007, (int)StructureType.SessionInviteTargetAppInit);
            Assert.Equal(0x2009, (int)StructureType.PeerDeviceIdentity);
            Assert.Equal(0x200B, (int)StructureType.SessionLifecycleInfo);
            Assert.Equal(0x4002, (int)StructureType.StreamVulkanParamsExt);
            Assert.Equal(0x4003, (int)StructureType.StreamOpenGlEglParamsExt);
            Assert.Equal(0x4004, (int)StructureType.StreamOpenGlWglParamsExt);
            Assert.Equal(0x4005, (int)StructureType.StreamOpenGlGlxParamsExt);
            Assert.Equal(0x5504, (int)StructureType.AppVolumeGestureAllowedPeersExt);
        }

        [Fact]
        public void EveryStructureTypeIsUnique()
        {
            var values = Enum.GetValues(typeof(StructureType)).Cast<StructureType>().Select(v => (int)v).ToList();
            Assert.Equal(values.Count, values.Distinct().Count());
            Assert.Equal(Enum.GetNames(typeof(StructureType)).Length, values.Distinct().Count());
        }

        [Fact]
        public void EnumValuesMirrorC()
        {
            Assert.Equal(6, (int)GraphicsAPI.Vulkan);
            Assert.Equal(7, (int)GraphicsAPI.OpenGL);
            Assert.Equal(0, (int)Handedness.Unspecified);
            Assert.Equal(1, (int)Handedness.Left);
            Assert.Equal(2, (int)Handedness.Right);
            Assert.Equal(3, (int)GuiPanelGestureKind.Drag);
            Assert.Equal(1, (int)GpuDeviceIdType.Uuid);
            Assert.Equal(56, (int)PixelFormat.B8G8R8X8);
            Assert.Equal(20, (int)VideoFrameViewEye.Right);
            Assert.Equal(0x8UL, (ulong)HandJointLocationFlags.PositionTracked);
            Assert.Equal(0, (int)PeerPresence.Unknown);
            Assert.Equal(1, (int)PeerPresence.Online);
            Assert.Equal(2, (int)PeerPresence.Offline);
            Assert.Equal(3, (int)PeerPresence.Leaving);
            Assert.Equal(4, (int)PeerPresence.Retired);
            Assert.Equal(0, (int)SessionLifecycleState.Connected);
            Assert.Equal(1, (int)SessionLifecycleState.Migrating);
            Assert.Equal(2, (int)SessionLifecycleState.HubUnreachable);
            Assert.Equal(3, (int)SessionLifecycleState.RePairRequired);
            Assert.Equal(0, (int)SessionChangeCause.None);
            Assert.Equal(1, (int)SessionChangeCause.Rotated);
            Assert.Equal(2, (int)SessionChangeCause.JoinedPrimary);
            Assert.Equal(5, Enum.GetValues(typeof(PeerPresence)).Length);
        }

        [Fact]
        public void GraphicsDeviceIdSplitsLuidIntoLowAndHighParts()
        {
            var native = GraphicsDeviceId.FromLuid(0x0000000700000005L).ToNative();
            Assert.Equal(StructureType.SessionGraphicsDeviceId, native.Header.Type);
            Assert.Equal(GpuDeviceIdType.Luid, native.IdType);
            Assert.Equal(5u, native.Luid.LowPart);
            Assert.Equal(7, native.Luid.HighPart);
            Assert.Equal(16, native.Uuid.Length);
        }

        [Fact]
        public void DefaultGestureConfigurationMirrorsC()
        {
            var rules = new AppVolumeGestureConfiguration().MappingRules;
            Assert.Equal(new[] { GestureKind.DualPointerTranslateDistance, GestureKind.SinglePointer6Dof, GestureKind.Hover, GestureKind.Click }, Array.ConvertAll(rules, r => r.GestureKind));
            Assert.Equal(AppVolumeAxisFlags.None, rules[0].RotationAxes);
            Assert.Equal(AppVolumeAxisFlags.None, rules[1].RotationAxes);
            Assert.Equal(1.0f, rules[0].Precision);
            Assert.Equal(AppVolumeControllerRayLineStyle.Dotted, rules[2].ControllerRayStyle.LineStyle);
        }

        [Fact]
        public void AllowedPeersAreChainedOnlyWhenPresent()
        {
            using (var scope = new NativeMemoryScope())
            {
                var config = new AppVolumeGestureConfiguration();
                Assert.Equal(IntPtr.Zero, config.ToNative(scope).Header.Next);
                config.AllowedPeers = new[] { PeerId.Default, PeerId.Default };
                var next = config.ToNative(scope).Header.Next;
                Assert.NotEqual(IntPtr.Zero, next);
                var extension = Marshal.PtrToStructure<AppVolumeGestureAllowedPeersExtNative>(next);
                Assert.Equal(StructureType.AppVolumeGestureAllowedPeersExt, extension.Header.Type);
                Assert.Equal((UIntPtr)2, extension.AllowedPeerCount);
            }
        }

        // The application reports the state it left an image in by writing into the sender-owned
        // frame; only that texture's layout and queue family may change.
        [Fact]
        public void VulkanTextureStateIsWrittenIntoTheNativeFrame()
        {
            var native = new VideoFrameVulkanNative
            {
                TextureViews = new VideoFrameViewNative[NativeConstants.MaxFrameViews],
                Textures = new VideoTextureVulkanNative[NativeConstants.MaxFrameTextures]
            };
            native.TexturesCount = (UIntPtr)2;
            native.Textures[0].Image = 0x10;
            native.Textures[1].Image = 0x20;
            native.Synchronization.Semaphore = 0x30;
            var frame = Marshal.AllocHGlobal(Marshal.SizeOf(typeof(VideoFrameVulkanNative)));
            try
            {
                Marshal.StructureToPtr(native, frame, false);
                var access = VideoFrameVulkan.FromNativePointer(frame);
                access.SetTextureState(1, VulkanConstants.ImageLayoutTransferSrcOptimal, VulkanConstants.QueueFamilyExternal);
                Assert.Throws<ArgumentOutOfRangeException>(() => access.SetTextureState(2, 0, 0));
                var written = VideoFrameVulkan.FromNativePointer(frame);
                Assert.Equal(2, written.Textures.Length);
                Assert.Equal(VulkanConstants.ImageLayoutUndefined, written.Textures[0].Layout);
                Assert.Equal(0u, written.Textures[0].QueueFamilyIndex);
                Assert.Equal(0x20UL, written.Textures[1].Image);
                Assert.Equal(VulkanConstants.ImageLayoutTransferSrcOptimal, written.Textures[1].Layout);
                Assert.Equal(VulkanConstants.QueueFamilyExternal, written.Textures[1].QueueFamilyIndex);
                Assert.Equal(0x30UL, written.Semaphore);
            }
            finally
            {
                Marshal.FreeHGlobal(frame);
            }
        }

        [Fact]
        public void RenderSenderAlphaModeExtMatchesNativeLayout()
        {
            int pointerSize = IntPtr.Size;
            Assert.Equal(pointerSize * 2, Marshal.OffsetOf(typeof(RenderSenderAlphaModeExtNative), nameof(RenderSenderAlphaModeExtNative.AlphaMode)).ToInt32());
            Assert.Equal(pointerSize * 3, Marshal.SizeOf(typeof(RenderSenderAlphaModeExtNative)));
        }

        [Fact]
        public void RenderSenderChainsTheAlphaModeOnlyWhenDeclared()
        {
            using (var scope = new NativeMemoryScope())
            {
                var options = new RenderSenderOptions(PeerId.Parse(CanonicalId), GraphicsAPI.Cpu);
                Assert.Equal(ColorAlphaMode.Unspecified, options.AlphaMode);
                Assert.Equal(IntPtr.Zero, NativeInit.RenderSender(options, scope).Header.Next);

                options.AlphaMode = ColorAlphaMode.Premultiplied;
                var native = NativeInit.RenderSender(options, scope);
                Assert.NotEqual(IntPtr.Zero, native.Header.Next);
                var extension = Marshal.PtrToStructure<RenderSenderAlphaModeExtNative>(native.Header.Next);
                Assert.Equal(StructureType.RenderSenderAlphaModeExt, extension.Header.Type);
                Assert.Equal(ColorAlphaMode.Premultiplied, extension.AlphaMode);
                Assert.Equal(IntPtr.Zero, extension.Header.Next);
            }
        }

        [Fact]
        public void RenderSenderColorEncodingExtMatchesNativeLayout()
        {
            int pointerSize = IntPtr.Size;
            Assert.Equal(pointerSize * 2, Marshal.OffsetOf(typeof(RenderSenderColorEncodingExtNative), nameof(RenderSenderColorEncodingExtNative.ColorEncoding)).ToInt32());
            Assert.Equal(pointerSize * 3, Marshal.SizeOf(typeof(RenderSenderColorEncodingExtNative)));
        }

        [Fact]
        public void RenderSenderChainsTheColorEncodingOnlyWhenLinear()
        {
            using (var scope = new NativeMemoryScope())
            {
                var options = new RenderSenderOptions(PeerId.Parse(CanonicalId), GraphicsAPI.Cpu);
                Assert.Equal(ColorEncoding.Srgb, options.ColorEncoding);
                Assert.Equal(IntPtr.Zero, NativeInit.RenderSender(options, scope).Header.Next);

                options.ColorEncoding = ColorEncoding.Linear;
                var native = NativeInit.RenderSender(options, scope);
                Assert.NotEqual(IntPtr.Zero, native.Header.Next);
                var extension = Marshal.PtrToStructure<RenderSenderColorEncodingExtNative>(native.Header.Next);
                Assert.Equal(StructureType.RenderSenderColorEncodingExt, extension.Header.Type);
                Assert.Equal(ColorEncoding.Linear, extension.ColorEncoding);
                Assert.Equal(IntPtr.Zero, extension.Header.Next);
            }
        }

        [Fact]
        public void VulkanRenderSenderChainsItsDeviceAndRequiresIt()
        {
            using (var scope = new NativeMemoryScope())
            {
                var options = new RenderSenderOptions(PeerId.Parse(CanonicalId), GraphicsAPI.Vulkan);
                Assert.Throws<ArgumentException>(() => NativeInit.RenderSender(options, scope));
                options.StreamParamsVulkan = new StreamParamsVulkan { PhysicalDevice = new IntPtr(0x1234), Device = new IntPtr(0x5678) };
                var native = NativeInit.RenderSender(options, scope);
                Assert.Equal(GraphicsAPI.Vulkan, native.GraphicsAPI);
                var extension = Marshal.PtrToStructure<StreamParamsVulkanNative>(native.Header.Next);
                Assert.Equal(StructureType.StreamVulkanParamsExt, extension.Header.Type);
                Assert.Equal(new IntPtr(0x1234), extension.PhysicalDevice);
                Assert.Equal(new IntPtr(0x5678), extension.Device);
                Assert.Equal(IntPtr.Zero, extension.Header.Next);
                options.GraphicsAPI = GraphicsAPI.Cpu;
                Assert.Throws<ArgumentException>(() => NativeInit.RenderSender(options, scope));
            }
        }

        [Fact]
        public void OnboardExtensionsAreChainedAfterTheMode()
        {
            var options = new OnboardOptions().UseCode("1234").WithHost("hub", 19121).WithGraphicsDevice(GraphicsDeviceId.FromLuid(1));
            using (var chain = NativeStructureChain.From(options.Mode, options.Extension))
            {
                var types = new System.Collections.Generic.List<StructureType>();
                for (var current = chain.Head; current != IntPtr.Zero;)
                {
                    var header = Marshal.PtrToStructure<StructureHeaderNative>(current);
                    types.Add(header.Type);
                    current = header.Next;
                }
                Assert.Equal(new[] { StructureType.RuntimeOnboardCodeExt, StructureType.RuntimeOnboardHostExt, StructureType.SessionGraphicsDeviceId }, types.ToArray());
            }
        }

        [Fact]
        public void PeerIdUsesFixedSixteenByteStorage()
        {
            var id = PeerId.Parse(CanonicalId);
            Assert.Equal(16, id.ToByteArray().Length);
            Assert.Equal(CanonicalId, id.ToString());
        }

        [Fact]
        public void OnboardingIdUsesFixedSixteenByteStorage()
        {
            var id = OnboardingId.Parse(CanonicalId);
            Assert.Equal(16, id.ToByteArray().Length);
            Assert.Equal(CanonicalId, id.ToString());
        }

#if NETSTANDARD2_1_OR_GREATER || NET8_0_OR_GREATER
        [Fact]
        public void PeerIdCanCopyToSpan()
        {
            var id = PeerId.Parse(CanonicalId);
            Span<byte> destination = stackalloc byte[16];
            Assert.True(id.TryCopyTo(destination));
            Assert.Equal(16, id.AsSpan().Length);
            Assert.Equal(16, destination.Length);
        }

        [Fact]
        public void CpuTextureExposesBorrowedSpan()
        {
            var pointer = Marshal.AllocHGlobal(3);
            try
            {
                Marshal.WriteByte(pointer, 0, 1);
                Marshal.WriteByte(pointer, 1, 2);
                Marshal.WriteByte(pointer, 2, 3);
                var texture = new VideoTextureCpu(default(TextureSize), 0, pointer, (UIntPtr)3);
                Assert.Equal(new byte[] { 1, 2, 3 }, texture.TextureDataSpan.ToArray());
            }
            finally
            {
                Marshal.FreeHGlobal(pointer);
            }
        }
#endif
    }
}
