using System;
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
            { typeof(TimePointNative), 16 },
            { typeof(PoseNative), 28 },
            { typeof(FovNative), 16 },
            { typeof(HandJointLocationNative), 48 },
            { typeof(HandJointVelocityNative), 40 },
            { typeof(HandJointsNative), 2328 },
            { typeof(DeviceHandsWithJointsNative), 4688 },
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
            { typeof(OnboardInviteExtNative), 24 },
            { typeof(OnboardPeerIdExtNative), 32 },
            { typeof(RequestInviteInitNative), 16 },
            { typeof(SessionInviteTargetAppInitNative), 24 },
            { typeof(ForgetInitNative), 32 },
            { typeof(LuidNative), 8 },
            { typeof(GraphicsDeviceIdNative), 48 },
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
            { typeof(DeviceHandsWithJointsNative), nameof(DeviceHandsWithJointsNative.LeftHand), 32 },
            { typeof(DeviceHandsWithJointsNative), nameof(DeviceHandsWithJointsNative.RightHand), 2360 },
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
            { typeof(OnboardPeerIdExtNative), nameof(OnboardPeerIdExtNative.PeerId), 16 },
            { typeof(SessionInviteTargetAppInitNative), nameof(SessionInviteTargetAppInitNative.ConnectionString), 16 },
            { typeof(GraphicsDeviceIdNative), nameof(GraphicsDeviceIdNative.IdType), 16 },
            { typeof(GraphicsDeviceIdNative), nameof(GraphicsDeviceIdNative.Luid), 20 },
            { typeof(GraphicsDeviceIdNative), nameof(GraphicsDeviceIdNative.Uuid), 28 },
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
            Assert.Equal(602, (int)StatusCode.MediaStreamingStreamIsPaused);
            Assert.False(Enum.IsDefined(typeof(StatusCode), 601));
            Assert.Equal(25, Enum.GetValues(typeof(StatusCode)).Length);
        }

        [Fact]
        public void StructureTypesMirrorC()
        {
            Assert.Equal(0x1009, (int)StructureType.RuntimeOnboardPeerIdExt);
            Assert.Equal(0x2004, (int)StructureType.SessionGraphicsDeviceId);
            Assert.Equal(0x2007, (int)StructureType.SessionInviteTargetAppInit);
            Assert.Equal(0x4002, (int)StructureType.StreamVulkanParamsExt);
            Assert.Equal(0x4003, (int)StructureType.StreamOpenGlEglParamsExt);
            Assert.Equal(0x4004, (int)StructureType.StreamOpenGlWglParamsExt);
            Assert.Equal(0x4005, (int)StructureType.StreamOpenGlGlxParamsExt);
            Assert.Equal(0x5504, (int)StructureType.AppVolumeGestureAllowedPeersExt);
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
