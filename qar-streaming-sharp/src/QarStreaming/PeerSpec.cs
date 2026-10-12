using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Qar.Native;

namespace Qar
{
    /// <summary>
    /// An immutable snapshot of one peer, taken when it was read or delivered; safe to keep. It does
    /// not change: read a new one, or watch <see cref="PeerControl.SubscribeUpdates()"/>.
    /// </summary>
    public sealed class PeerSpec
    {
        internal PeerSpec(PeerId id, string displayName, string appVersion, string appCustomPeerInfo, AppState appState, string versionId, string roomTag, bool isTargetApp)
        {
            Id = id;
            DisplayName = displayName;
            AppVersion = appVersion;
            AppCustomPeerInfo = appCustomPeerInfo;
            AppState = appState;
            VersionId = versionId;
            RoomTag = roomTag;
            IsTargetApp = isTargetApp;
        }

        public PeerId Id { get; }

        public string DisplayName { get; }

        public string AppVersion { get; }

        /// <summary>Application-defined metadata the peer presented itself with.</summary>
        public string AppCustomPeerInfo { get; }

        /// <summary>
        /// Whether the peer is still starting, live or gone. A target app is ready to be rendered
        /// for once this is <see cref="AppState.Running"/>.
        /// </summary>
        public AppState AppState { get; }

        public string VersionId { get; }

        /// <summary>The physical room the peer is in, to group peers in multi-room deployments.</summary>
        public string RoomTag { get; }

        /// <summary>
        /// Whether this peer shows rendered content (a headset, the Android streamer, a visualizer
        /// receiver). A source application decides itself which target apps it renders for.
        /// </summary>
        public bool IsTargetApp { get; }

        internal static Result<PeerSpec> FromHandle(IntPtr handle, bool destroyHandle)
        {
            try
            {
                var id = PeerIdNative.CreateDefault();
                var idInfo = Results.ToInfo(NativeMethods.qar_impl_peer_spec_get_id(handle, ref id));
                if (!idInfo.IsSuccess)
                    return Result<PeerSpec>.Failure(idInfo);
                var state = AppState.Unknown;
                var stateInfo = Results.ToInfo(NativeMethods.qar_impl_peer_spec_get_app_state(handle, out state));
                if (!stateInfo.IsSuccess)
                    return Result<PeerSpec>.Failure(stateInfo);
                var displayName = NativeString.GetString((buffer, size) => NativeMethods.qar_impl_peer_spec_get_display_name(handle, buffer, size));
                if (!displayName.IsSuccess)
                    return Result<PeerSpec>.Failure(displayName.Info);
                var appVersion = NativeString.GetString((buffer, size) => NativeMethods.qar_impl_peer_spec_get_app_version(handle, buffer, size));
                if (!appVersion.IsSuccess)
                    return Result<PeerSpec>.Failure(appVersion.Info);
                var appCustomPeerInfo = NativeString.GetString((buffer, size) => NativeMethods.qar_impl_peer_spec_get_app_custom_peer_info(handle, buffer, size));
                if (!appCustomPeerInfo.IsSuccess)
                    return Result<PeerSpec>.Failure(appCustomPeerInfo.Info);
                var versionId = NativeString.GetString((buffer, size) => NativeMethods.qar_impl_peer_spec_get_version_id(handle, buffer, size));
                if (!versionId.IsSuccess)
                    return Result<PeerSpec>.Failure(versionId.Info);
                var roomTag = NativeString.GetString((buffer, size) => NativeMethods.qar_impl_peer_spec_get_room_tag(handle, buffer, size));
                if (!roomTag.IsSuccess)
                    return Result<PeerSpec>.Failure(roomTag.Info);
                var isTargetAppInfo = Results.ToInfo(NativeMethods.qar_impl_peer_spec_is_target_app(handle, out var isTargetApp));
                if (!isTargetAppInfo.IsSuccess)
                    return Result<PeerSpec>.Failure(isTargetAppInfo);
                return Result<PeerSpec>.Success(
                    idInfo,
                    new PeerSpec(
                        PeerId.FromNative(id),
                        displayName.Value,
                        appVersion.Value,
                        appCustomPeerInfo.Value,
                        state,
                        versionId.Value,
                        roomTag.Value,
                        isTargetApp));
            }
            finally
            {
                if (destroyHandle && handle != IntPtr.Zero)
                    NativeMethods.qar_impl_peer_spec_handle_destroy(handle);
            }
        }
    }
}
