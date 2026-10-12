using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Qar.Native;

namespace Qar
{
    public sealed class AppVolume
    {
        internal AppVolume(AppVolumeId id, string displayName, Pose pose, AppVolumeSize size, AppVolumeLifetimeStatus lifetimeStatus, AppVolumeEditingStatus editingStatus, PeerId[] usedByPeers, Pose appPose, float appScale, AppWorldAnchor appWorldAnchor, AppVolumeGestureConfiguration gestureConfiguration, AppVolumeGestureState gestureState)
        {
            Id = id;
            DisplayName = displayName;
            Pose = pose;
            Size = size;
            LifetimeStatus = lifetimeStatus;
            EditingStatus = editingStatus;
            UsedByPeers = usedByPeers;
            AppPose = appPose;
            AppScale = appScale;
            AppWorldAnchor = appWorldAnchor;
            GestureConfiguration = gestureConfiguration;
            GestureState = gestureState;
        }

        public AppVolumeId Id { get; }

        public string DisplayName { get; }

        /// <summary>The cuboid centre's pose in the publishing peer's room space.</summary>
        public Pose Pose { get; }

        public AppVolumeSize Size { get; }

        public AppVolumeLifetimeStatus LifetimeStatus { get; }

        public AppVolumeEditingStatus EditingStatus { get; }

        public PeerId[] UsedByPeers { get; }

        /// <summary>Origin of the rendering application in app volume space, in volume metres; not scaled by <see cref="AppScale"/>.</summary>
        public Pose AppPose { get; }

        /// <summary>Room-metres per app-metre: the factor between app content space and app volume space.</summary>
        public float AppScale { get; }

        public AppWorldAnchor AppWorldAnchor { get; }

        /// <summary>The rules of gesture mode <see cref="AppVolumeGestureState.DefaultMode"/> and the allowed peers.</summary>
        public AppVolumeGestureConfiguration GestureConfiguration { get; }

        /// <summary>Gesture modes, the selected modes and the peer currently manipulating the app.</summary>
        public AppVolumeGestureState GestureState { get; }

        internal static Result<AppVolume> FromHandle(IntPtr handle, bool destroyHandle)
        {
            try
            {
                var id = AppVolumeIdNative.CreateDefault();
                var info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_id(handle, ref id));
                if (!info.IsSuccess)
                    return Result<AppVolume>.Failure(info);
                PoseNative pose;
                AppVolumeSizeNative size;
                AppVolumeLifetimeStatus lifetime;
                var editing = AppVolumeEditingStatusNative.CreateDefault();
                PoseNative appPose;
                float appScale;
                AppWorldAnchorNative appAnchor;
                var gestures = NativeDefaults.AppVolumeGestureConfiguration();
                var poseInfo = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_pose(handle, out pose));
                if (!poseInfo.IsSuccess)
                    return Result<AppVolume>.Failure(poseInfo);
                var sizeInfo = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_size(handle, out size));
                if (!sizeInfo.IsSuccess)
                    return Result<AppVolume>.Failure(sizeInfo);
                var lifetimeInfo = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_lifetime_status(handle, out lifetime));
                if (!lifetimeInfo.IsSuccess)
                    return Result<AppVolume>.Failure(lifetimeInfo);
                var editingInfo = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_editing_status(handle, ref editing));
                if (!editingInfo.IsSuccess)
                    return Result<AppVolume>.Failure(editingInfo);
                var appPoseInfo = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_app_pose(handle, out appPose));
                if (!appPoseInfo.IsSuccess)
                    return Result<AppVolume>.Failure(appPoseInfo);
                var appScaleInfo = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_app_scale(handle, out appScale));
                if (!appScaleInfo.IsSuccess)
                    return Result<AppVolume>.Failure(appScaleInfo);
                var appAnchorInfo = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_app_world_anchor(handle, out appAnchor));
                if (!appAnchorInfo.IsSuccess)
                    return Result<AppVolume>.Failure(appAnchorInfo);
                var gesturesInfo = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_gesture_configuration(handle, ref gestures));
                if (!gesturesInfo.IsSuccess)
                    return Result<AppVolume>.Failure(gesturesInfo);
                var displayName = NativeString.GetString((buffer, count) => NativeMethods.qar_impl_app_volume_get_display_name(handle, buffer, count));
                if (!displayName.IsSuccess)
                    return Result<AppVolume>.Failure(displayName.Info);
                var usedByPeers = GetUsedByPeers(handle);
                if (!usedByPeers.IsSuccess)
                    return Result<AppVolume>.Failure(usedByPeers.Info);
                var gestureState = AppVolumeGestureState.FromHandle(handle, usedByPeers.Value);
                if (!gestureState.IsSuccess)
                    return Result<AppVolume>.Failure(gestureState.Info);
                var gestureConfiguration = AppVolumeGestureConfiguration.FromNative(gestures);
                gestureConfiguration.AllowedPeers = gestureState.Value.AllowedPeers;
                return Result<AppVolume>.Success(
                    info,
                    new AppVolume(
                        AppVolumeId.FromNative(id),
                        displayName.Value,
                        Pose.FromNative(pose),
                        AppVolumeSize.FromNative(size),
                        lifetime,
                        AppVolumeEditingStatus.FromNative(editing),
                        usedByPeers.Value,
                        Pose.FromNative(appPose),
                        appScale,
                        AppWorldAnchor.FromNative(appAnchor),
                        gestureConfiguration,
                        gestureState.Value));
            }
            finally
            {
                if (destroyHandle && handle != IntPtr.Zero)
                    NativeMethods.qar_impl_app_volume_handle_destroy(handle);
            }
        }

        private static Result<PeerId[]> GetUsedByPeers(IntPtr handle)
        {
            UIntPtr count;
            var info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_used_by_peers_count(handle, out count));
            if (!info.IsSuccess)
                return Result<PeerId[]>.Failure(info);
            var peers = NativeIdArrays.CreatePeerArray(NativeConvert.ToInt32(count));
            UIntPtr written;
            info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_used_by_peers(handle, peers, count, out written));
            return info.IsSuccess ? Result<PeerId[]>.Success(info, NativeIdArrays.ToPublic(peers, NativeConvert.ToInt32(written))) : Result<PeerId[]>.Failure(info);
        }
    }
    /// <summary>
    /// An app volume's gesture modes and who is using them, read with the
    /// volume snapshot.
    /// </summary>
    public sealed class AppVolumeGestureState
    {
        /// <summary>The mode every volume has and cannot remove, and the one every peer is in until switched.</summary>
        public const uint DefaultMode = 0;

        internal AppVolumeGestureState(PeerId[] allowedPeers, PeerId? holder, IReadOnlyDictionary<uint, AppVolumeGestureConfiguration> modes, uint selectedDefaultModeId, IReadOnlyDictionary<PeerId, uint> activeModeByPeer)
        {
            AllowedPeers = allowedPeers;
            Holder = holder;
            Modes = modes;
            SelectedDefaultModeId = selectedDefaultModeId;
            ActiveModeByPeer = activeModeByPeer;
        }

        /// <summary>The peers whose gestures may move or scale the app; empty means every peer's.</summary>
        public PeerId[] AllowedPeers { get; }

        /// <summary>The one peer currently manipulating the app with a gesture, or null when none holds it.</summary>
        public PeerId? Holder { get; }

        /// <summary>Every mode the volume defines, by id, with its rules; always contains <see cref="DefaultMode"/>.</summary>
        public IReadOnlyDictionary<uint, AppVolumeGestureConfiguration> Modes { get; }

        /// <summary>The mode every peer without its own mode follows.</summary>
        public uint SelectedDefaultModeId { get; }

        /// <summary>The mode each peer using the volume maps its next gesture through.</summary>
        public IReadOnlyDictionary<PeerId, uint> ActiveModeByPeer { get; }

        internal static Result<AppVolumeGestureState> FromHandle(IntPtr handle, PeerId[] usedByPeers)
        {
            var allowedPeers = ReadAllowedPeers(handle);
            if (!allowedPeers.IsSuccess)
                return Result<AppVolumeGestureState>.Failure(allowedPeers.Info);
            var holderId = PeerIdNative.CreateDefault();
            var info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_gesture_holder(handle, ref holderId, out var isHeld));
            if (!info.IsSuccess)
                return Result<AppVolumeGestureState>.Failure(info);
            var modes = ReadModes(handle);
            if (!modes.IsSuccess)
                return Result<AppVolumeGestureState>.Failure(modes.Info);
            info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_active_gesture_mode(handle, IntPtr.Zero, out var selectedDefault));
            if (!info.IsSuccess)
                return Result<AppVolumeGestureState>.Failure(info);
            var activeModeByPeer = ReadActiveModes(handle, usedByPeers);
            if (!activeModeByPeer.IsSuccess)
                return Result<AppVolumeGestureState>.Failure(activeModeByPeer.Info);
            PeerId? holder = isHeld ? PeerId.FromNative(holderId) : (PeerId?)null;
            return Result<AppVolumeGestureState>.Success(info, new AppVolumeGestureState(allowedPeers.Value, holder, modes.Value, selectedDefault, activeModeByPeer.Value));
        }

        private static Result<PeerId[]> ReadAllowedPeers(IntPtr handle)
        {
            var info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_gesture_allowed_peers(handle, null, UIntPtr.Zero, out var count));
            if (!info.IsSuccess)
                return Result<PeerId[]>.Failure(info);
            var peers = NativeIdArrays.CreatePeerArray(NativeConvert.ToInt32(count));
            if (peers.Length == 0)
                return Result<PeerId[]>.Success(info, Array.Empty<PeerId>());
            info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_gesture_allowed_peers(handle, peers, count, out var written));
            if (!info.IsSuccess)
                return Result<PeerId[]>.Failure(info);
            return Result<PeerId[]>.Success(info, NativeIdArrays.ToPublic(peers, Math.Min(peers.Length, NativeConvert.ToInt32(written))));
        }

        private static Result<IReadOnlyDictionary<uint, AppVolumeGestureConfiguration>> ReadModes(IntPtr handle)
        {
            var info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_gesture_mode_ids(handle, null, UIntPtr.Zero, out var count));
            if (!info.IsSuccess)
                return Result<IReadOnlyDictionary<uint, AppVolumeGestureConfiguration>>.Failure(info);
            var ids = new uint[NativeConvert.ToInt32(count)];
            if (ids.Length > 0)
            {
                info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_gesture_mode_ids(handle, ids, count, out var written));
                if (!info.IsSuccess)
                    return Result<IReadOnlyDictionary<uint, AppVolumeGestureConfiguration>>.Failure(info);
                Array.Resize(ref ids, Math.Min(ids.Length, NativeConvert.ToInt32(written)));
            }
            var modes = new Dictionary<uint, AppVolumeGestureConfiguration>();
            foreach (var id in ids)
            {
                var config = NativeDefaults.AppVolumeGestureConfiguration();
                info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_gesture_mode(handle, id, ref config));
                if (!info.IsSuccess)
                    return Result<IReadOnlyDictionary<uint, AppVolumeGestureConfiguration>>.Failure(info);
                modes[id] = AppVolumeGestureConfiguration.FromNative(config);
            }
            return Result<IReadOnlyDictionary<uint, AppVolumeGestureConfiguration>>.Success(info, modes);
        }

        private static Result<IReadOnlyDictionary<PeerId, uint>> ReadActiveModes(IntPtr handle, PeerId[] peers)
        {
            var result = new Dictionary<PeerId, uint>();
            var info = new ResultInfo(StatusCode.Success, 0, string.Empty);
            using (var scope = new NativeMemoryScope())
            {
                foreach (var peer in peers)
                {
                    info = Results.ToInfo(NativeMethods.qar_impl_app_volume_get_active_gesture_mode(handle, scope.AllocStruct(peer.ToNative()), out var modeId));
                    if (!info.IsSuccess)
                        return Result<IReadOnlyDictionary<PeerId, uint>>.Failure(info);
                    result[peer] = modeId;
                }
            }
            return Result<IReadOnlyDictionary<PeerId, uint>>.Success(info, result);
        }
    }
}
