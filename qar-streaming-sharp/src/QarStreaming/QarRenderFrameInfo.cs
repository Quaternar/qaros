using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Qar.Native;

namespace Qar
{
    /// <summary>Per-frame information from a successful begin-frame. Everything it hands out is in the bound app volume's app content space.</summary>
    public sealed class RenderFrameInfo : IDisposable
    {
        private readonly RenderFrameInfoSafeHandle _handle;

        internal RenderFrameInfo(RenderFrameInfoSafeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
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
            var info = Results.ToInfo(NativeMethods.qar_impl_render_frame_info_get_view_pose(_handle.DangerousGetHandle(), (UIntPtr)viewIndex, out pose));
            return info.IsSuccess ? Result<Pose>.Success(info, Pose.FromNative(pose)) : Result<Pose>.Failure(info);
        }

        /// <summary>The field of view of one view, in view space, in radians. A view of eye <see cref="VideoFrameViewEye.None"/> gets the left eye's.</summary>
        public Result<Fov> GetViewFov(int viewIndex)
        {
            ThrowIfDisposed();
            FovNative fov;
            var info = Results.ToInfo(NativeMethods.qar_impl_render_frame_info_get_view_fov(_handle.DangerousGetHandle(), (UIntPtr)viewIndex, out fov));
            return info.IsSuccess ? Result<Fov>.Success(info, Fov.FromNative(fov)) : Result<Fov>.Failure(info);
        }

        public void Dispose()
        {
            _handle.Dispose();
        }

        private void ThrowIfDisposed()
        {
            if (_handle.IsClosed || _handle.IsInvalid)
                throw new ObjectDisposedException(nameof(RenderFrameInfo));
        }
    }
}

