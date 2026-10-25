using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros.Interop;
using Quaternar.Qaros.Mathematics;

namespace Quaternar.Qaros
{
    /// <summary>Per-frame information from a successful begin-frame. Everything it hands out is in the bound app volume's app content space.</summary>
    /// <remarks>
    /// Valid only for the frame it was begun with: read the poses and FOVs before
    /// <see cref="RenderSender.ShowFrame"/>, which ends that frame, then dispose it. Read after
    /// that, the getters fail with <see cref="StatusCode.LogicError"/>, or return the next
    /// frame's values once that one is begun; disposing the sender invalidates it. A view index
    /// is a position in the sender's layout (<see cref="RenderSender.GetLayout"/>), in the order
    /// of <see cref="RenderSenderOptions.FrameViews"/>; one out of range fails with
    /// <see cref="StatusCode.LogicError"/>.
    /// </remarks>
    public sealed class RenderFrameInfo : IDisposable
    {
        private readonly NativeHandleOwner _owner;

        internal RenderFrameInfo(RenderFrameInfoSafeHandle handle)
        {
            _owner = new NativeHandleOwner(handle, nameof(RenderFrameInfo));
        }

        /// <summary>
        /// The pose to render one view with, in the bound app volume's app content
        /// space (right-handed, +Y up, -Z forward, app units). The sender has
        /// already taken the viewer's room-space pose through the volume's pose, app
        /// pose and app scale, so the application renders its scene in its own
        /// coordinates. A view of eye <see cref="VideoFrameViewEye.None"/> gets the
        /// head pose.
        /// </summary>
        public Result<Pose> GetViewPose(int viewIndex)
        {
            ThrowIfDisposed();
            PoseNative pose;
            using var frame = _owner.Borrow();
            var info = Results.ToInfo(NativeMethods.qar_impl_render_frame_info_get_view_pose(frame.Handle, (UIntPtr)viewIndex, out pose));
            return info.IsSuccess ? Result<Pose>.Success(info, Pose.FromNative(pose)) : Result<Pose>.Failure(info);
        }

        /// <summary>The field of view of one view, in view space, in radians. A view of eye <see cref="VideoFrameViewEye.None"/> gets the left eye's.</summary>
        public Result<Fov> GetViewFov(int viewIndex)
        {
            ThrowIfDisposed();
            FovNative fov;
            using var frame = _owner.Borrow();
            var info = Results.ToInfo(NativeMethods.qar_impl_render_frame_info_get_view_fov(frame.Handle, (UIntPtr)viewIndex, out fov));
            return info.IsSuccess ? Result<Fov>.Success(info, Fov.FromNative(fov)) : Result<Fov>.Failure(info);
        }

        public void Dispose()
        {
            _owner.Dispose();
        }

        private void ThrowIfDisposed()
        {
            _owner.ThrowIfDisposed();
        }
    }
}

