using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros;
using Quaternar.Qaros.Mathematics;

namespace Quaternar.Qaros.Interop
{
    internal static class NativeDefaults
    {
        internal static StructureHeaderNative Header(StructureType type, IntPtr next = default(IntPtr))
        {
            return new StructureHeaderNative { Type = type, Next = next };
        }

        internal static VideoFrameViewNative VideoFrameView()
        {
            return new VideoFrameViewNative { TextureFormat = PixelFormat.B8G8R8A8, DataType = VideoFrameViewType.Color, Eye = VideoFrameViewEye.None };
        }

        internal static TextureSizeNative TextureSize()
        {
            return new TextureSizeNative { Format = PixelFormat.B8G8R8A8, ArrayLayers = 1 };
        }

        internal static VideoFrameLayoutNative VideoFrameLayout()
        {
            var views = new VideoFrameViewNative[NativeConstants.MaxFrameViews];
            for (var i = 0; i < views.Length; i++)
                views[i] = VideoFrameView();
            var textures = new TextureSizeNative[NativeConstants.MaxFrameTextures];
            for (var i = 0; i < textures.Length; i++)
                textures[i] = TextureSize();
            return new VideoFrameLayoutNative { Views = views, Textures = textures };
        }

        internal static VideoTextureCpuNative VideoTextureCpu()
        {
            return new VideoTextureCpuNative { Size = TextureSize() };
        }

        internal static VideoFrameCpuNative VideoFrameCpu()
        {
            var views = new VideoFrameViewNative[NativeConstants.MaxFrameViews];
            for (var i = 0; i < views.Length; i++)
                views[i] = VideoFrameView();
            var textures = new VideoTextureCpuNative[NativeConstants.MaxFrameTextures];
            for (var i = 0; i < textures.Length; i++)
                textures[i] = VideoTextureCpu();
            return new VideoFrameCpuNative { TextureViews = views, Textures = textures };
        }

        internal static VideoTextureD3D11Native VideoTextureD3D11()
        {
            return new VideoTextureD3D11Native { Size = TextureSize() };
        }

        internal static SyncFrameD3D11Native SyncFrameD3D11()
        {
            return new SyncFrameD3D11Native { TextureSync = new DXGIKeyedMutexSyncNative[NativeConstants.MaxFrameTextures] };
        }

        internal static VideoFrameD3D11Native VideoFrameD3D11()
        {
            var views = new VideoFrameViewNative[NativeConstants.MaxFrameViews];
            for (var i = 0; i < views.Length; i++)
                views[i] = VideoFrameView();
            var textures = new VideoTextureD3D11Native[NativeConstants.MaxFrameTextures];
            for (var i = 0; i < textures.Length; i++)
                textures[i] = VideoTextureD3D11();
            return new VideoFrameD3D11Native { TextureViews = views, Textures = textures, Synchronization = SyncFrameD3D11() };
        }

        internal static VideoFrameOpenGlNative VideoFrameOpenGl()
        {
            var views = new VideoFrameViewNative[NativeConstants.MaxFrameViews];
            for (var i = 0; i < views.Length; i++)
                views[i] = VideoFrameView();
            var textures = new VideoTextureOpenGlNative[NativeConstants.MaxFrameTextures];
            for (var i = 0; i < textures.Length; i++)
                textures[i] = new VideoTextureOpenGlNative { Size = TextureSize() };
            return new VideoFrameOpenGlNative { TextureViews = views, Textures = textures };
        }

        internal static HandJointsNative HandJoints()
        {
            return new HandJointsNative
            {
                Pose = Pose.Identity.ToNative(),
                JointLocations = new HandJointLocationNative[NativeConstants.HandJointCount],
                JointVelocities = new HandJointVelocityNative[NativeConstants.HandJointCount]
            };
        }

        internal static DeviceHandsWithJointsNative DeviceHandsWithJoints()
        {
            return new DeviceHandsWithJointsNative
            {
                DeviceId = PeerIdNative.CreateDefault(),
                LeftHand = HandJoints(),
                RightHand = HandJoints()
            };
        }

        internal static AppVolumeGestureMappingRuleNative AppVolumeGestureMappingRule()
        {
            return new AppVolumeGestureMappingRule().ToNative();
        }

        internal static AppVolumeGestureConfigurationNative AppVolumeGestureConfiguration()
        {
            return new AppVolumeGestureConfigurationNative
            {
                Header = Header(StructureType.AppVolumeGestureConfiguration),
                MappingRules = new AppVolumeGestureMappingRuleNative[NativeConstants.MaxAppVolumeGestureMappingRules],
                MappingRuleCount = UIntPtr.Zero
            };
        }

        internal static RenderFrameViewOverrideNative[] RenderFrameViewOverrides()
        {
            return new RenderFrameViewOverrideNative[NativeConstants.MaxFrameViews];
        }
    }

    internal static class NativeInit
    {
        internal static PeerPresentationNative PeerPresentation(PeerPresentationOptions options, NativeMemoryScope scope, IntPtr next)
        {
            return new PeerPresentationNative
            {
                Header = NativeDefaults.Header(StructureType.PeerPresentation, next),
                DisplayName = scope.AllocUtf8(options.DisplayName),
                AppVersion = scope.AllocUtf8(options.AppVersion),
                AppCustomPeerInfo = scope.AllocUtf8(options.AppCustomPeerInfo)
            };
        }

        internal static RejoinInitNative Rejoin(RejoinOptions options, NativeMemoryScope scope, IntPtr next, IntPtr presentationNext)
        {
            return new RejoinInitNative
            {
                Header = NativeDefaults.Header(StructureType.RuntimeRejoinInit, next),
                OnboardingId = options.OnboardingId.ToNative(),
                Presentation = PeerPresentation(options.Presentation ?? new PeerPresentationOptions(), scope, presentationNext)
            };
        }

        internal static OnboardInitNative Onboard(OnboardOptions options, NativeMemoryScope scope, IntPtr next, IntPtr presentationNext)
        {
            return new OnboardInitNative
            {
                Header = NativeDefaults.Header(StructureType.RuntimeOnboardInit, next),
                OnboardingId = options.OnboardingId.HasValue ? options.OnboardingId.Value.ToNative() : OnboardingIdNative.CreateDefault(),
                Presentation = PeerPresentation(options.Presentation ?? new PeerPresentationOptions(), scope, presentationNext)
            };
        }

        internal static RequestInviteInitNative RequestInvite(RequestOnboardingInviteOptions options, NativeMemoryScope scope, IntPtr next)
        {
            return new RequestInviteInitNative
            {
                Header = NativeDefaults.Header(StructureType.SessionRequestInviteInit, next),
                AppTag = scope.AllocUtf8(options.AppTag)
            };
        }

        internal static ForgetInitNative Forget(ForgetOptions options, IntPtr next)
        {
            return new ForgetInitNative
            {
                Header = NativeDefaults.Header(StructureType.RuntimeForgetInit, next),
                OnboardingId = options.OnboardingId.ToNative()
            };
        }

        internal static RenderSenderInitNative RenderSender(RenderSenderOptions options, NativeMemoryScope scope)
        {
            var next = GraphicsExtension(options.GraphicsAPI, options.StreamParamsD3D11, options.StreamParamsVulkan, options.StreamParamsOpenGl, scope, nameof(RenderSenderOptions));
            var native = new RenderSenderInitNative
            {
                Header = NativeDefaults.Header(StructureType.RenderingStreamSenderInit, next),
                PeerId = options.PeerId.ToNative(),
                EnableAutoReconnects = options.EnableAutoReconnects,
                TextureLayout = options.TextureLayout,
                FrameViews = new RenderFrameViewNative[NativeConstants.MaxFrameViews],
                FrameViewsCount = (UIntPtr)Math.Min((options.FrameViews ?? Array.Empty<RenderFrameView>()).Length, NativeConstants.MaxFrameViews),
                AppVolumeId = options.AppVolumeId.HasValue ? scope.AllocStruct(options.AppVolumeId.Value.ToNative()) : IntPtr.Zero,
                ColorFormat = options.ColorFormat,
                DepthFormat = options.DepthFormat,
                GraphicsAPI = options.GraphicsAPI
            };
            NativeCopy.CopyToFixed(options.FrameViews ?? Array.Empty<RenderFrameView>(), native.FrameViews, NativeConstants.MaxFrameViews, v => v.ToNative());
            return native;
        }

        internal static VideoSenderInitNative VideoSender(PrimarylessVideoSenderOptions options, NativeMemoryScope scope)
        {
            var next = GraphicsExtension(options.GraphicsAPI, options.StreamParamsD3D11, options.StreamParamsVulkan, options.StreamParamsOpenGl, scope, nameof(PrimarylessVideoSenderOptions));
            return new VideoSenderInitNative
            {
                Header = NativeDefaults.Header(StructureType.VideoSenderInit, next),
                Layout = options.Layout.ToNative(),
                EnableReconnects = options.EnableReconnects,
                ReconnectTimeoutMs = checked((ulong)options.ReconnectTimeout.TotalMilliseconds),
                GraphicsApi = options.GraphicsAPI
            };
        }

        /// <summary>
        /// The one stream-parameter extension a GPU graphics API requires, or zero
        /// for the CPU path. Mirrors the C rule: exactly the matching extension,
        /// none for CPU.
        /// </summary>
        private static IntPtr GraphicsExtension(GraphicsAPI api, StreamParamsD3D11? d3d11, StreamParamsVulkan? vulkan, StreamParamsOpenGl? openGl, NativeMemoryScope scope, string optionsName)
        {
            RequireOnlyFor(api, GraphicsAPI.D3D11, d3d11, optionsName + ".StreamParamsD3D11");
            RequireOnlyFor(api, GraphicsAPI.Vulkan, vulkan, optionsName + ".StreamParamsVulkan");
            RequireOnlyFor(api, GraphicsAPI.OpenGL, openGl, optionsName + ".StreamParamsOpenGl");
            Structure? extension = api == GraphicsAPI.D3D11 ? d3d11 : api == GraphicsAPI.Vulkan ? (Structure?)vulkan : api == GraphicsAPI.OpenGL ? openGl : null;
            return extension == null ? IntPtr.Zero : NativeExtensions.AllocateNativeCopy(extension, scope);
        }

        private static void RequireOnlyFor(GraphicsAPI api, GraphicsAPI owner, Structure? extension, string name)
        {
            if (api == owner && extension == null)
                throw new ArgumentException(name + " must be provided when GraphicsAPI is " + owner + ".");
            if (api != owner && extension != null)
                throw new ArgumentException(name + " can only be set when GraphicsAPI is " + owner + ".");
        }

        internal static SessionInviteTargetAppInitNative InviteTargetApp(string connectionString, NativeMemoryScope scope)
        {
            return new SessionInviteTargetAppInitNative
            {
                Header = NativeDefaults.Header(StructureType.SessionInviteTargetAppInit),
                ConnectionString = scope.AllocUtf8(connectionString)
            };
        }

        internal static GuiPanelInitNative GuiPanel(GuiPanelOptions options, NativeMemoryScope scope, IntPtr next)
        {
            var peers = NativeIdArrays.ToNative(options.VisibleToPeers ?? Array.Empty<PeerId>());
            return new GuiPanelInitNative
            {
                Header = NativeDefaults.Header(StructureType.GuiPanelInit, next),
                CommonName = scope.AllocUtf8(options.CommonName),
                DisplayName = scope.AllocUtf8(options.DisplayName),
                Pose = options.Pose.ToNative(),
                Size = options.Size.ToNative(),
                VisibleToPeers = scope.AllocArray(peers),
                VisibleToPeerCount = (UIntPtr)peers.Length
            };
        }

        internal static AppVolumeInitNative AppVolume(AppVolumeOptions options, NativeMemoryScope scope, IntPtr next)
        {
            IntPtr gestureConfiguration = IntPtr.Zero;
            if (options.GestureConfiguration != null)
                gestureConfiguration = scope.AllocStruct(options.GestureConfiguration.ToNative(scope));
            return new AppVolumeInitNative
            {
                Header = NativeDefaults.Header(StructureType.AppVolumeInit, next),
                CommonName = scope.AllocUtf8(options.CommonName),
                DisplayName = scope.AllocUtf8(options.DisplayName),
                Pose = options.Pose.ToNative(),
                Size = options.Size.ToNative(),
                InitialPeers = scope.AllocPointerArray(options.InitialPeers ?? Array.Empty<PeerId>()),
                InitialPeerCount = (UIntPtr)(options.InitialPeers == null ? 0 : options.InitialPeers.Length),
                AppPose = options.AppPose.ToNative(),
                AppScale = options.AppScale,
                HasAppWorldAnchor = options.AppWorldAnchor.HasValue,
                AppWorldAnchor = options.AppWorldAnchor.HasValue ? options.AppWorldAnchor.Value.ToNative() : default(GeoAnchorFrameNative),
                GestureConfiguration = gestureConfiguration
            };
        }
    }
}
