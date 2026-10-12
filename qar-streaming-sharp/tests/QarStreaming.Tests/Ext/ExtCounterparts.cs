using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using Qar;
using Qar.Native;

namespace QarStreaming.Tests.Ext
{
    // Managed counterparts of the C API for tests: an in-process hub to onboard
    // into, peers to stream to, receivers that check pixels. Each wraps one
    // qar-streaming-c-ext handle; disposal releases it. Results are base
    // QarResults and read like every other binding call.

    internal static class ExtResults
    {
        internal static void Ensure(ResultNative result)
        {
            Results.FromNative(result).EnsureSuccess();
        }

        internal static Result<Unit> From(ResultNative result)
        {
            return Results.FromNative(result);
        }

        internal static string DecodeText(byte[] text)
        {
            return NativeString.DecodeNullTerminated(text);
        }
    }

    public static class ExtTime
    {
        /// <summary>Scales a test budget by QAR_TIME_SCALE, like the native suites do.</summary>
        public static TimeSpan Scaled(TimeSpan budget)
        {
            return TimeSpan.FromMilliseconds(ExtNativeMethods.qar_impl_ext_scale_ms((ulong)budget.TotalMilliseconds));
        }
    }

    public static class ExtLayouts
    {
        /// <summary>Stereo colour + depth, each eye <paramref name="eyeWidth"/> wide.</summary>
        public static VideoFrameLayout SideBySide(uint eyeWidth, uint eyeHeight)
        {
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_layout_side_by_side(eyeWidth, eyeHeight, out var native));
            return VideoFrameLayout.FromNative(native);
        }
    }

    public sealed class ExtHub : IDisposable
    {
        private IntPtr _handle;

        private ExtHub(IntPtr handle)
        {
            _handle = handle;
        }

        public static ExtHub Create(bool enablePasswordOnboarding = true)
        {
            var init = new ExtHubInitNative
            {
                Header = NativeDefaults.Header(ExtStructureType.HubInit),
                EnablePasswordOnboarding = enablePasswordOnboarding,
            };
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_hub_create(ref init, out var handle));
            return new ExtHub(handle);
        }

        internal IntPtr Handle { get { ThrowIfDisposed(); return _handle; } }

        public PeerId PeerId
        {
            get
            {
                ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_hub_get_peer_id(Handle, out var id));
                return PeerId.FromNative(id);
            }
        }

        /// <summary>A fresh invite wire string into this hub.</summary>
        public byte[] MintInvite(ExtInviteMethod method = ExtInviteMethod.Pake)
        {
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_hub_mint_invite(Handle, method, null, UIntPtr.Zero, out var size));
            var buffer = new byte[(int)size];
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_hub_mint_invite(Handle, method, buffer, (UIntPtr)buffer.Length, out size));
            var length = Array.IndexOf(buffer, (byte)0);
            if (length < 0)
                return buffer;
            var wire = new byte[length];
            Array.Copy(buffer, wire, length);
            return wire;
        }

        /// <summary>The display name the hub lists for <paramref name="peer"/>; null while it does not list it.</summary>
        public string? FindPeerDisplayName(PeerId peer)
        {
            var native = peer.ToNative();
            var buffer = new byte[NativeConstants.MaxStringLength];
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_hub_find_peer_display_name(Handle, ref native, buffer, (UIntPtr)buffer.Length, out _, out var found));
            return found ? ExtResults.DecodeText(buffer) : null;
        }

        public ExtPeer CreatePeer(string? displayName = null)
        {
            return ExtPeer.Create(this, displayName);
        }

        public ExtTargetAppResponder StartTargetAppResponder(string? machineId = null)
        {
            return ExtTargetAppResponder.Create(this, machineId);
        }

        public IReadOnlyList<ExtMediaStreamRecord> QueryMediaStreams()
        {
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_hub_query_media_streams(Handle, null, UIntPtr.Zero, out var count));
            var records = new ExtMediaStreamRecordNative[(int)count];
            for (var i = 0; i < records.Length; ++i)
                records[i].SourceLayout = NativeDefaults.VideoFrameLayout();
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_hub_query_media_streams(Handle, records, (UIntPtr)records.Length, out count));
            var result = new List<ExtMediaStreamRecord>();
            for (var i = 0; i < Math.Min(records.Length, (int)count); ++i)
                result.Add(new ExtMediaStreamRecord(records[i]));
            return result;
        }

        public void Dispose()
        {
            if (_handle == IntPtr.Zero)
                return;
            ExtNativeMethods.qar_impl_ext_hub_handle_destroy(_handle);
            _handle = IntPtr.Zero;
        }

        private void ThrowIfDisposed()
        {
            if (_handle == IntPtr.Zero)
                throw new ObjectDisposedException(nameof(ExtHub));
        }
    }

    public sealed class ExtMediaStreamRecord
    {
        internal ExtMediaStreamRecord(ExtMediaStreamRecordNative native)
        {
            Id = StreamId.FromNative(native.Id);
            HasVideo = native.HasVideo;
            SenderPeer = PeerId.FromNative(native.SenderPeer);
            ReceiverCount = (int)native.ReceiverCount;
            SourceLayout = native.HasSourceLayout ? VideoFrameLayout.FromNative(native.SourceLayout) : null;
        }

        public StreamId Id { get; }
        public bool HasVideo { get; }
        public PeerId SenderPeer { get; }
        public int ReceiverCount { get; }
        public VideoFrameLayout? SourceLayout { get; }
    }

    public sealed class ExtPeer : IDisposable
    {
        private IntPtr _handle;

        private ExtPeer(IntPtr handle)
        {
            _handle = handle;
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_peer_get_id(handle, out var id));
            Id = PeerId.FromNative(id);
        }

        internal static ExtPeer Create(ExtHub hub, string? displayName)
        {
            using (var scope = new NativeMemoryScope())
            {
                var init = new ExtPeerInitNative
                {
                    Header = NativeDefaults.Header(ExtStructureType.PeerInit),
                    DisplayName = scope.AllocString(displayName),
                };
                ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_peer_create(hub.Handle, ref init, out var handle));
                return new ExtPeer(handle);
            }
        }

        public PeerId Id { get; }

        internal IntPtr Handle { get { ThrowIfDisposed(); return _handle; } }

        /// <summary>A receiver for a stream rendered for this peer; it connects once a sender answers.</summary>
        public ExtRenderReceiver CreateRenderReceiver(VideoFrameLayout layout)
        {
            var init = new ExtRenderReceiverInitNative
            {
                Header = NativeDefaults.Header(ExtStructureType.RenderReceiverInit),
                OptimalLayout = layout.ToNative(),
            };
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_render_receiver_create(Handle, ref init, out var handle));
            return new ExtRenderReceiver(handle);
        }

        /// <summary>A plain receiver, e.g. of a video sender's stream id.</summary>
        public ExtVideoReceiver CreateVideoReceiver(VideoFrameLayout layout, StreamId? streamId = null, ExtVideoReceiverRole role = ExtVideoReceiverRole.Auto, bool startPaused = false)
        {
            var init = new ExtVideoReceiverInitNative
            {
                Header = NativeDefaults.Header(ExtStructureType.VideoReceiverInit),
                OptimalLayout = layout.ToNative(),
                HasStreamId = streamId.HasValue,
                StreamId = (streamId ?? StreamId.Default).ToNative(),
                Role = role,
                StartPaused = startPaused,
            };
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_video_receiver_create(Handle, ref init, out var handle));
            return new ExtVideoReceiver(handle, owned: true);
        }

        public Result<Unit> SendDevicePose(Pose headPose)
        {
            var native = headPose.ToNative();
            return ExtResults.From(ExtNativeMethods.qar_impl_ext_peer_send_device_pose(Handle, ref native));
        }

        /// <summary>Publishes a left hand as tracked at its default pose: enough for a sender to report it.</summary>
        public Result<Unit> SendTrackedLeftHand()
        {
            var hands = NativeDefaults.DeviceHandsWithJoints();
            hands.LeftHand.IsTracked = true;
            return ExtResults.From(ExtNativeMethods.qar_impl_ext_peer_send_hand_joints(Handle, ref hands));
        }

        public Result<Unit> TapGuiPanel(GuiPanelId panel, int x, int y)
        {
            var gesture = new ExtGuiPanelGestureNative
            {
                Panel = panel.ToNative(),
                Kind = (int)GuiPanelGestureKind.Tap,
                Phase = (int)GesturePhase.Instant,
                Hand = (int)Hand.Right,
                X = x,
                Y = y,
                StartX = x,
                StartY = y,
                TapCount = 1,
            };
            return ExtResults.From(ExtNativeMethods.qar_impl_ext_peer_gui_panel_gesture(Handle, ref gesture));
        }

        public Result<Unit> AddPeerToAppVolume(AppVolumeId volume, PeerId added)
        {
            var nativeVolume = volume.ToNative();
            var nativePeer = added.ToNative();
            return ExtResults.From(ExtNativeMethods.qar_impl_ext_peer_app_volume_add_peer(Handle, ref nativeVolume, ref nativePeer));
        }

        public Result<Unit> RemovePeerFromAppVolume(AppVolumeId volume, PeerId removed)
        {
            var nativeVolume = volume.ToNative();
            var nativePeer = removed.ToNative();
            return ExtResults.From(ExtNativeMethods.qar_impl_ext_peer_app_volume_remove_peer(Handle, ref nativeVolume, ref nativePeer));
        }

        public Result<Unit> StartGesturing(AppVolumeId volume)
        {
            var nativeVolume = volume.ToNative();
            return ExtResults.From(ExtNativeMethods.qar_impl_ext_peer_app_volume_start_gesturing(Handle, ref nativeVolume));
        }

        public Result<Unit> StopGesturing(AppVolumeId volume)
        {
            var nativeVolume = volume.ToNative();
            return ExtResults.From(ExtNativeMethods.qar_impl_ext_peer_app_volume_stop_gesturing(Handle, ref nativeVolume));
        }

        /// <summary>A render sender hands its app the hands of a peer only while it is focused on the sender's volume.</summary>
        public Result<Unit> SetFocused(AppVolumeId volume, bool focused)
        {
            var nativeVolume = volume.ToNative();
            return ExtResults.From(ExtNativeMethods.qar_impl_ext_peer_app_volume_set_focused(Handle, ref nativeVolume, focused));
        }

        public void Dispose()
        {
            if (_handle == IntPtr.Zero)
                return;
            ExtNativeMethods.qar_impl_ext_peer_handle_destroy(_handle);
            _handle = IntPtr.Zero;
        }

        private void ThrowIfDisposed()
        {
            if (_handle == IntPtr.Zero)
                throw new ObjectDisposedException(nameof(ExtPeer));
        }
    }

    /// <summary>CPU buffers lent to a receiver; pinned until disposed.</summary>
    public sealed class ExtCpuFrame : IDisposable
    {
        private readonly GCHandle[] _pins;

        public ExtCpuFrame(VideoFrameLayout layout)
        {
            Textures = new byte[layout.Textures.Length][];
            Pitches = new uint[layout.Textures.Length];
            _pins = new GCHandle[layout.Textures.Length];
            Native = new VideoFrameCpuNative
            {
                TextureViews = new VideoFrameViewNative[NativeConstants.MaxFrameViews],
                Textures = new VideoTextureCpuNative[NativeConstants.MaxFrameTextures],
            };
            var layoutNative = layout.ToNative();
            Array.Copy(layoutNative.Views, Native.TextureViews, NativeConstants.MaxFrameViews);
            Native.TextureViewsCount = layoutNative.ViewsCount;
            for (var i = 0; i < layout.Textures.Length; ++i)
            {
                var size = layout.Textures[i];
                Pitches[i] = size.Width * 4;
                Textures[i] = new byte[Pitches[i] * size.Height];
                _pins[i] = GCHandle.Alloc(Textures[i], GCHandleType.Pinned);
                Native.Textures[i] = new VideoTextureCpuNative
                {
                    Size = layoutNative.Textures[i],
                    Pitch = Pitches[i],
                    TextureData = _pins[i].AddrOfPinnedObject(),
                    TextureDataSize = (UIntPtr)Textures[i].Length,
                };
            }
            Native.TexturesCount = (UIntPtr)layout.Textures.Length;
        }

        public byte[][] Textures { get; }
        public uint[] Pitches { get; }
        internal VideoFrameCpuNative Native;

        public void Dispose()
        {
            foreach (var pin in _pins)
            {
                if (pin.IsAllocated)
                    pin.Free();
            }
        }
    }

    public readonly struct ExtFrameInfo
    {
        internal ExtFrameInfo(ExtVideoFrameInfoNative native)
        {
            ValidFrame = native.ValidFrame;
            SequenceNumber = native.SequenceNumber;
        }

        public bool ValidFrame { get; }
        public ulong SequenceNumber { get; }
    }

    public sealed class ExtVideoReceiver : IDisposable
    {
        private readonly bool _owned;
        private IntPtr _handle;

        internal ExtVideoReceiver(IntPtr handle, bool owned)
        {
            _handle = handle;
            _owned = owned;
        }

        internal IntPtr Handle { get { ThrowIfDisposed(); return _handle; } }

        public bool WaitConnected(TimeSpan timeout)
        {
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_video_receiver_wait_connected(Handle, (uint)timeout.TotalMilliseconds, out var connected));
            return connected;
        }

        public StreamId GetStreamId()
        {
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_video_receiver_stream_id(Handle, out var id));
            return StreamId.FromNative(id);
        }

        /// <summary>Lends <paramref name="frame"/>; false while not connected, paused or a frame is lent.</summary>
        public bool TryProvideFrame(ExtCpuFrame frame)
        {
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_video_receiver_try_provide_frame_cpu(Handle, ref frame.Native, out var accepted));
            return accepted;
        }

        /// <summary>The lent frame's metadata once written; null on timeout.</summary>
        public ExtFrameInfo? ReceiveFrame(TimeSpan timeout)
        {
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_video_receiver_receive_frame(Handle, (uint)timeout.TotalMilliseconds, out var info, out var received));
            return received ? new ExtFrameInfo(info) : (ExtFrameInfo?)null;
        }

        public Result<Unit> Pause()
        {
            return ExtResults.From(ExtNativeMethods.qar_impl_ext_video_receiver_pause(Handle));
        }

        public Result<Unit> Resume()
        {
            return ExtResults.From(ExtNativeMethods.qar_impl_ext_video_receiver_resume(Handle));
        }

        public void Dispose()
        {
            if (_handle == IntPtr.Zero)
                return;
            if (_owned)
                ExtNativeMethods.qar_impl_ext_video_receiver_handle_destroy(_handle);
            _handle = IntPtr.Zero;
        }

        private void ThrowIfDisposed()
        {
            if (_handle == IntPtr.Zero)
                throw new ObjectDisposedException(nameof(ExtVideoReceiver));
        }
    }

    public sealed class ExtRenderReceiver : IDisposable
    {
        private IntPtr _handle;

        internal ExtRenderReceiver(IntPtr handle)
        {
            _handle = handle;
            ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_render_receiver_video(handle, out var video));
            Video = new ExtVideoReceiver(video, owned: false);
        }

        /// <summary>Where frames arrive; owned by this render receiver.</summary>
        public ExtVideoReceiver Video { get; }

        internal IntPtr Handle { get { ThrowIfDisposed(); return _handle; } }

        public Result<Unit> RequestRender(Pose headPose, ulong frameCount = ulong.MaxValue)
        {
            var request = new ExtRenderRequestNative { HeadPose = headPose.ToNative(), FrameCount = frameCount };
            return ExtResults.From(ExtNativeMethods.qar_impl_ext_render_receiver_request_render(Handle, ref request));
        }

        public Result<Unit> SendPose(Pose headPose)
        {
            var native = headPose.ToNative();
            return ExtResults.From(ExtNativeMethods.qar_impl_ext_render_receiver_send_pose(Handle, ref native));
        }

        public void Dispose()
        {
            if (_handle == IntPtr.Zero)
                return;
            Video.Dispose();
            ExtNativeMethods.qar_impl_ext_render_receiver_handle_destroy(_handle);
            _handle = IntPtr.Zero;
        }

        private void ThrowIfDisposed()
        {
            if (_handle == IntPtr.Zero)
                throw new ObjectDisposedException(nameof(ExtRenderReceiver));
        }
    }

    public sealed class ExtTargetAppInvite
    {
        internal ExtTargetAppInvite(ExtTargetAppInviteRecordNative native)
        {
            Producer = PeerId.FromNative(native.Producer);
            Hostname = ExtResults.DecodeText(native.Hostname);
            Device = ExtResults.DecodeText(native.Device);
            Answered = native.Answered;
            Target = PeerId.FromNative(native.Target);
        }

        public PeerId Producer { get; }
        public string Hostname { get; }
        public string Device { get; }
        public bool Answered { get; }
        public PeerId Target { get; }
    }

    public sealed class ExtTargetAppShutdown
    {
        internal ExtTargetAppShutdown(ExtTargetAppShutdownRecordNative native)
        {
            PeerToShutdown = PeerId.FromNative(native.PeerToShutdown);
            Answered = native.Answered;
        }

        public PeerId PeerToShutdown { get; }
        public bool Answered { get; }
    }

    public sealed class ExtTargetAppResponder : IDisposable
    {
        private IntPtr _handle;

        private ExtTargetAppResponder(IntPtr handle)
        {
            _handle = handle;
        }

        internal static ExtTargetAppResponder Create(ExtHub hub, string? machineId)
        {
            using (var scope = new NativeMemoryScope())
            {
                var init = new ExtTargetAppResponderInitNative
                {
                    Header = NativeDefaults.Header(ExtStructureType.TargetAppResponderInit),
                    MachineId = scope.AllocString(machineId),
                };
                ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_target_app_responder_create(hub.Handle, ref init, out var handle));
                return new ExtTargetAppResponder(handle);
            }
        }

        public IReadOnlyList<ExtTargetAppInvite> Invites
        {
            get
            {
                ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_target_app_responder_invite_count(Handle, out var count));
                var result = new List<ExtTargetAppInvite>();
                for (var i = 0; i < (int)count; ++i)
                {
                    ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_target_app_responder_get_invite(Handle, (UIntPtr)i, out var record));
                    result.Add(new ExtTargetAppInvite(record));
                }
                return result;
            }
        }

        public IReadOnlyList<ExtTargetAppShutdown> Shutdowns
        {
            get
            {
                ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_target_app_responder_shutdown_count(Handle, out var count));
                var result = new List<ExtTargetAppShutdown>();
                for (var i = 0; i < (int)count; ++i)
                {
                    ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_target_app_responder_get_shutdown(Handle, (UIntPtr)i, out var record));
                    result.Add(new ExtTargetAppShutdown(record));
                }
                return result;
            }
        }

        public int RunningCount
        {
            get
            {
                ExtResults.Ensure(ExtNativeMethods.qar_impl_ext_target_app_responder_running_count(Handle, out var count));
                return (int)count;
            }
        }

        internal IntPtr Handle { get { ThrowIfDisposed(); return _handle; } }

        public void Dispose()
        {
            if (_handle == IntPtr.Zero)
                return;
            ExtNativeMethods.qar_impl_ext_target_app_responder_handle_destroy(_handle);
            _handle = IntPtr.Zero;
        }

        private void ThrowIfDisposed()
        {
            if (_handle == IntPtr.Zero)
                throw new ObjectDisposedException(nameof(ExtTargetAppResponder));
        }
    }
}
