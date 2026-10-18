using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// An immutable snapshot of one peer, taken when it was read or delivered; safe to keep. It does
    /// not change: read a new one, or watch <see cref="PeerControl.SubscribeUpdates()"/>.
    /// </summary>
    public sealed class PeerSpec
    {
        internal PeerSpec(
            PeerId id,
            string displayName,
            string appVersion,
            string appCustomPeerInfo,
            AppState appState,
            string versionId,
            string roomTag,
            PeerRole role,
            DeviceCategory category = DeviceCategory.Unknown,
            string deviceId = "",
            string deviceFamily = "",
            string deviceName = "")
        {
            Id = id;
            DisplayName = displayName;
            AppVersion = appVersion;
            AppCustomPeerInfo = appCustomPeerInfo;
            AppState = appState;
            VersionId = versionId;
            RoomTag = roomTag;
            Role = role;
            Category = category;
            DeviceId = deviceId;
            DeviceFamily = deviceFamily;
            DeviceName = deviceName;
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
        /// What the peer is in the streaming graph. A source application renders for the peers
        /// whose role is <see cref="PeerRole.Target"/>; this process is always
        /// <see cref="PeerRole.Source"/>.
        /// </summary>
        public PeerRole Role { get; }

        /// <summary>
        /// Whether this peer shows rendered content (a headset's serving app, a Visualizer
        /// receiver, a camera's receiver slot): <see cref="Role"/> is <see cref="PeerRole.Target"/>.
        /// A source application decides itself which target apps it renders for.
        /// </summary>
        public bool IsTargetApp => Role == PeerRole.Target;

        /// <summary>
        /// What kind of device the peer is; the fallback for a <see cref="DeviceId"/> and
        /// <see cref="DeviceFamily"/> the application does not know.
        /// </summary>
        public DeviceCategory Category { get; }

        /// <summary>
        /// Open string naming the exact device or app ("MetaQuest3", "HoloLens2", "Visualizer").
        /// It can change while the peer runs, once its serving app knows the model.
        /// </summary>
        public string DeviceId { get; }

        /// <summary>Open string naming the device family ("MetaQuest"); empty when there is none.</summary>
        public string DeviceFamily { get; }

        /// <summary>Human name of the device ("Meta Quest 3"); empty when nothing better than the id is known.</summary>
        public string DeviceName { get; }

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
                var roleInfo = Results.ToInfo(NativeMethods.qar_impl_peer_spec_get_role(handle, out var role));
                if (!roleInfo.IsSuccess)
                    return Result<PeerSpec>.Failure(roleInfo);
                var categoryInfo = Results.ToInfo(NativeMethods.qar_impl_peer_spec_get_category(handle, out var category));
                if (!categoryInfo.IsSuccess)
                    return Result<PeerSpec>.Failure(categoryInfo);
                var deviceId = NativeString.GetString((buffer, size) => NativeMethods.qar_impl_peer_spec_get_device_id(handle, buffer, size));
                if (!deviceId.IsSuccess)
                    return Result<PeerSpec>.Failure(deviceId.Info);
                var deviceFamily = NativeString.GetString((buffer, size) => NativeMethods.qar_impl_peer_spec_get_device_family(handle, buffer, size));
                if (!deviceFamily.IsSuccess)
                    return Result<PeerSpec>.Failure(deviceFamily.Info);
                var deviceName = NativeString.GetString((buffer, size) => NativeMethods.qar_impl_peer_spec_get_device_name(handle, buffer, size));
                if (!deviceName.IsSuccess)
                    return Result<PeerSpec>.Failure(deviceName.Info);
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
                        role,
                        category,
                        deviceId.Value,
                        deviceFamily.Value,
                        deviceName.Value));
            }
            finally
            {
                if (destroyHandle && handle != IntPtr.Zero)
                    NativeMethods.qar_impl_peer_spec_handle_destroy(handle);
            }
        }
    }
}
