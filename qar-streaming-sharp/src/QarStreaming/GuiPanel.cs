using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Qar.Native;

namespace Qar
{
    public sealed class GuiPanel
    {
        private GuiPanel(GuiPanelId id, string displayName, Pose pose, GuiPanelSize size, string contentUri, GuiPanelState state, PeerId[] visibleToPeers)
        {
            Id = id;
            DisplayName = displayName;
            Pose = pose;
            Size = size;
            ContentType = GuiPanelContentType.WebsiteUri;
            ContentUri = contentUri ?? throw new ArgumentNullException(nameof(contentUri));
            State = state;
            VisibleToPeers = visibleToPeers;
        }

        private GuiPanel(GuiPanelId id, string displayName, Pose pose, GuiPanelSize size, StreamId contentStreamId, GuiPanelState state, PeerId[] visibleToPeers)
        {
            Id = id;
            DisplayName = displayName;
            Pose = pose;
            Size = size;
            ContentType = GuiPanelContentType.StreamId;
            ContentStreamId = contentStreamId;
            State = state;
            VisibleToPeers = visibleToPeers;
        }

        internal static GuiPanel CreateWithUri(GuiPanelId id, string displayName, Pose pose, GuiPanelSize size, string contentUri, GuiPanelState state, PeerId[] visibleToPeers)
        {
            return new GuiPanel(id, displayName, pose, size, contentUri, state, visibleToPeers);
        }

        internal static GuiPanel CreateWithStreamId(GuiPanelId id, string displayName, Pose pose, GuiPanelSize size, StreamId contentStreamId, GuiPanelState state, PeerId[] visibleToPeers)
        {
            return new GuiPanel(id, displayName, pose, size, contentStreamId, state, visibleToPeers);
        }

        public GuiPanelId Id { get; }

        public string DisplayName { get; }

        /// <summary>The panel's pose in the publishing peer's room space: the midpoint of its top edge, front normal along -Z.</summary>
        public Pose Pose { get; }

        public GuiPanelSize Size { get; }

        public GuiPanelContentType ContentType { get; }

        public string? ContentUri { get; }

        public StreamId? ContentStreamId { get; }

        public GuiPanelState State { get; }

        /// <summary>The peers the panel is narrowed to. Empty means visible to every peer, not to none.</summary>
        public PeerId[] VisibleToPeers { get; }

        internal static Result<GuiPanel> FromHandle(IntPtr handle, bool destroyHandle)
        {
            try
            {
                var id = GuiPanelIdNative.CreateDefault();
                var info = Results.ToInfo(NativeMethods.qar_impl_gui_panel_get_id(handle, ref id));
                if (!info.IsSuccess)
                    return Result<GuiPanel>.Failure(info);
                PoseNative pose;
                GuiPanelSizeNative size;
                GuiPanelState state;
                var poseInfo = Results.ToInfo(NativeMethods.qar_impl_gui_panel_get_pose(handle, out pose));
                if (!poseInfo.IsSuccess)
                    return Result<GuiPanel>.Failure(poseInfo);
                var sizeInfo = Results.ToInfo(NativeMethods.qar_impl_gui_panel_get_size(handle, out size));
                if (!sizeInfo.IsSuccess)
                    return Result<GuiPanel>.Failure(sizeInfo);
                var stateInfo = Results.ToInfo(NativeMethods.qar_impl_gui_panel_get_state(handle, out state));
                if (!stateInfo.IsSuccess)
                    return Result<GuiPanel>.Failure(stateInfo);
                var displayName = NativeString.GetString((buffer, count) => NativeMethods.qar_impl_gui_panel_get_display_name(handle, buffer, count));
                if (!displayName.IsSuccess)
                    return Result<GuiPanel>.Failure(displayName.Info);
                GuiPanelContentType contentType;
                var contentTypeInfo = Results.ToInfo(NativeMethods.qar_impl_gui_panel_get_content_type(handle, out contentType));
                if (!contentTypeInfo.IsSuccess)
                    return Result<GuiPanel>.Failure(contentTypeInfo);
                var visibleToPeers = GetVisibleToPeers(handle);
                if (!visibleToPeers.IsSuccess)
                    return Result<GuiPanel>.Failure(visibleToPeers.Info);
                GuiPanel panel;
                if (contentType == GuiPanelContentType.WebsiteUri)
                {
                    var contentUriResult = NativeString.GetString((buffer, count) => NativeMethods.qar_impl_gui_panel_get_content_uri(handle, buffer, count));
                    if (!contentUriResult.IsSuccess)
                        return Result<GuiPanel>.Failure(contentUriResult.Info);
                    panel = CreateWithUri(
                        GuiPanelId.FromNative(id),
                        displayName.Value,
                        Pose.FromNative(pose),
                        GuiPanelSize.FromNative(size),
                        contentUriResult.Value,
                        state,
                        visibleToPeers.Value);
                }
                else if (contentType == GuiPanelContentType.StreamId)
                {
                    StreamIdNative contentStreamIdNative;
                    var contentStreamIdInfo = Results.ToInfo(NativeMethods.qar_impl_gui_panel_get_content_stream_id(handle, out contentStreamIdNative));
                    if (!contentStreamIdInfo.IsSuccess)
                        return Result<GuiPanel>.Failure(contentStreamIdInfo);
                    panel = CreateWithStreamId(
                        GuiPanelId.FromNative(id),
                        displayName.Value,
                        Pose.FromNative(pose),
                        GuiPanelSize.FromNative(size),
                        StreamId.FromNative(contentStreamIdNative),
                        state,
                        visibleToPeers.Value);
                }
                else
                {
                    throw new NotSupportedException("Unsupported GUI panel content type: " + contentType);
                }
                return Result<GuiPanel>.Success(info, panel);
            }
            finally
            {
                if (destroyHandle && handle != IntPtr.Zero)
                    NativeMethods.qar_impl_gui_panel_handle_destroy(handle);
            }
        }

        private static Result<PeerId[]> GetVisibleToPeers(IntPtr handle)
        {
            UIntPtr count;
            var info = Results.ToInfo(NativeMethods.qar_impl_gui_panel_get_visible_to_peers_count(handle, out count));
            if (!info.IsSuccess)
                return Result<PeerId[]>.Failure(info);
            var peers = NativeIdArrays.CreatePeerArray(NativeConvert.ToInt32(count));
            UIntPtr written;
            info = Results.ToInfo(NativeMethods.qar_impl_gui_panel_get_visible_to_peers(handle, peers, count, out written));
            return info.IsSuccess ? Result<PeerId[]>.Success(info, NativeIdArrays.ToPublic(peers, NativeConvert.ToInt32(written))) : Result<PeerId[]>.Failure(info);
        }
    }
}
