using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Qar.Native;

namespace Qar
{
    public sealed class RenderSender : IDisposable
    {
        private readonly RenderSenderSafeHandle _handle;

        internal RenderSender(RenderSenderSafeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
        }

        public Result<VideoFrameLayout> GetLayout()
        {
            ThrowIfDisposed();
            var native = NativeDefaults.VideoFrameLayout();
            var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_layout(_handle.DangerousGetHandle(), ref native));
            return info.IsSuccess ? Result<VideoFrameLayout>.Success(info, VideoFrameLayout.FromNative(native)) : Result<VideoFrameLayout>.Failure(info);
        }

        public Result<StreamId> GetStreamId()
        {
            ThrowIfDisposed();
            StreamIdNative native;
            var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_stream_id(_handle.DangerousGetHandle(), out native));
            return info.IsSuccess ? Result<StreamId>.Success(info, StreamId.FromNative(native)) : Result<StreamId>.Failure(info);
        }

        public Result<Unit> ChangeLayout(VideoFrameLayout layout)
        {
            if (layout == null)
                throw new ArgumentNullException(nameof(layout));
            ThrowIfDisposed();
            var native = layout.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_render_sender_change_layout(_handle.DangerousGetHandle(), ref native));
        }

        public Result<VideoFrameCpu> GetCpuFrame()
        {
            ThrowIfDisposed();
            var native = NativeDefaults.VideoFrameCpu();
            var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_frame_cpu(_handle.DangerousGetHandle(), ref native));
            return info.IsSuccess ? Result<VideoFrameCpu>.Success(info, VideoFrameCpu.FromNative(native)) : Result<VideoFrameCpu>.Failure(info);
        }

        public Result<VideoFrameD3D11> GetD3D11Frame()
        {
            ThrowIfDisposed();
            var native = NativeDefaults.VideoFrameD3D11();
            var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_frame_d3d11(_handle.DangerousGetHandle(), ref native));
            return info.IsSuccess ? Result<VideoFrameD3D11>.Success(info, VideoFrameD3D11.FromNative(native)) : Result<VideoFrameD3D11>.Failure(info);
        }

        /// <summary>
        /// Blocks until a new frame may be produced and returns its per-view poses
        /// and FOVs. The poses are in the bound app volume's app content space.
        /// </summary>
        /// <remarks>
        /// Fails with <see cref="StatusCode.RenderingProducerAppVolumeUnresolved"/>
        /// while the sender's app volume does not resolve (not known to the session
        /// yet, or removed): app content space is undefined, no frame is begun and
        /// nothing is handed out in room space instead. That is recoverable - keep
        /// calling, it succeeds once the volume resolves; a removed volume never
        /// does, so recreate it. Meanwhile <see cref="GetLastHands"/> reports both
        /// hands untracked.
        /// </remarks>
        public Result<RenderFrameInfo> BeginFrame(CancellationToken cancellationToken = default(CancellationToken))
        {
            ThrowIfDisposed();
            var cancellationResult = NativeCancellationScope.Create(cancellationToken, false);
            if (!cancellationResult.IsSuccess)
                return Result<RenderFrameInfo>.Failure(cancellationResult.Info);
            IntPtr handle;
            using (var cancellation = cancellationResult.Value)
            {
                var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_begin_frame(_handle.DangerousGetHandle(), cancellation.Handle, out handle));
                return Results.FromHandle(info, handle, h => new RenderFrameInfo(new RenderFrameInfoSafeHandle(h)));
            }
        }

        /// <summary>Async <see cref="BeginFrame"/>; fails the same way, including with <see cref="StatusCode.RenderingProducerAppVolumeUnresolved"/>.</summary>
        public Task<Result<RenderFrameInfo>> BeginFrameAsync(CancellationToken cancellationToken = default(CancellationToken))
        {
            ThrowIfDisposed();
            var state = new BeginFrameAsyncState();
            var cancelHandle = state.UseStandardCancellation(cancellationToken);
            if (!cancelHandle.IsSuccess)
            {
                state.Complete(Result<RenderFrameInfo>.Failure(cancelHandle.Info));
                return state.Task;
            }
            return BeginFrameAsync(state, cancelHandle.Value);
        }

        private Task<Result<RenderFrameInfo>> BeginFrameAsync(BeginFrameAsyncState state, IntPtr token)
        {
            try
            {
                var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_begin_frame_async(_handle.DangerousGetHandle(), state.Callback, state.UserState, token));
                if (!info.IsSuccess)
                    state.Complete(Result<RenderFrameInfo>.Failure(info));
            }
            catch (Exception ex)
            {
                state.Fault(ex);
            }
            return state.Task;
        }

        /// <summary>
        /// Non-blocking <see cref="BeginFrame"/>: starts the begin-frame wait on the
        /// first call and polls it on every call. The value is the frame info once
        /// the wait completed, or null while it is still pending - poll again later.
        /// A failure is a real failure, such as
        /// <see cref="StatusCode.RenderingProducerAppVolumeUnresolved"/> (recoverable).
        /// </summary>
        public Result<RenderFrameInfo?> TryBeginFrame()
        {
            ThrowIfDisposed();
            var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_try_begin_frame(_handle.DangerousGetHandle(), out var handle, out var ready));
            if (!info.IsSuccess)
                return Result<RenderFrameInfo?>.Failure(info);
            if (!ready)
                return Result<RenderFrameInfo?>.Success(info, null);
            return Results.FromHandle<RenderFrameInfo?>(info, handle, h => new RenderFrameInfo(new RenderFrameInfoSafeHandle(h)));
        }

        /// <summary>
        /// OpenGL-backed frame access; requires a library built with
        /// QAR_ENABLE_OPENGL (otherwise <see cref="EntryPointNotFoundException"/>).
        /// Readable before the first begin-frame, writable only inside a frame cycle.
        /// </summary>
        public Result<VideoFrameOpenGl> GetOpenGlFrame()
        {
            ThrowIfDisposed();
            var native = NativeDefaults.VideoFrameOpenGl();
            var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_frame_opengl(_handle.DangerousGetHandle(), ref native));
            return info.IsSuccess ? Result<VideoFrameOpenGl>.Success(info, VideoFrameOpenGl.FromNative(native)) : Result<VideoFrameOpenGl>.Failure(info);
        }

        /// <summary>
        /// Sender-owned Vulkan frame and its current image state; requires a library
        /// built with QAR_ENABLE_VULKAN (otherwise <see cref="EntryPointNotFoundException"/>).
        /// Valid until the next call or this sender's disposal. Inside a frame cycle,
        /// record final layouts, queue ownership and synchronization on it before
        /// <see cref="ShowFrame"/>.
        /// </summary>
        public Result<VideoFrameVulkan> GetVulkanFrame()
        {
            ThrowIfDisposed();
            var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_frame_vulkan(_handle.DangerousGetHandle(), out var frame));
            return Results.FromHandle(info, frame, VideoFrameVulkan.FromNativePointer);
        }

        /// <summary>Submits the rendered frame. <see cref="RenderFrameShow.RenderedNearFar"/> and any view-override pose are in app content space.</summary>
        public Result<Unit> ShowFrame(RenderFrameShow? frameShow = null)
        {
            ThrowIfDisposed();
            frameShow = frameShow ?? new RenderFrameShow();
            using (var chain = NativeStructureChain.From(frameShow.Extension))
            {
                var native = new RenderFrameShowNative
                {
                    Header = NativeDefaults.Header(StructureType.RenderingEndFrame, chain.Head),
                    RenderedNearFar = frameShow.RenderedNearFar.ToNative(),
                    DepthScale = frameShow.DepthScale
                };
                return Results.FromNative(NativeMethods.qar_impl_render_sender_show_frame(_handle.DangerousGetHandle(), ref native));
            }
        }

        /// <summary>
        /// The newest hands sample the target device sent since the previous call;
        /// poll once per frame.
        /// </summary>
        /// <remarks>
        /// Every pose (the aim pose and the joint poses), joint radius and joint
        /// linear/angular velocity is in the bound app volume's app content space,
        /// like the view poses: right-handed, +Y up, -Z forward, app units. Samples
        /// arrive only while the target peer is focused on the volume and not
        /// editing it. With nothing new, the last sample is returned again while it
        /// is younger than a second; after that, and while the app volume does not
        /// resolve, both hands are reported untracked. Fails once the hand stream
        /// has closed.
        /// </remarks>
        public Result<DeviceHandsWithJoints> GetLastHands()
        {
            ThrowIfDisposed();
            var native = NativeDefaults.DeviceHandsWithJoints();
            var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_last_hands(_handle.DangerousGetHandle(), ref native));
            return info.IsSuccess ? Result<DeviceHandsWithJoints>.Success(info, DeviceHandsWithJoints.FromNative(native)) : Result<DeviceHandsWithJoints>.Failure(info);
        }

        public void Dispose()
        {
            _handle.Dispose();
        }

        private void ThrowIfDisposed()
        {
            if (_handle.IsClosed || _handle.IsInvalid)
                throw new ObjectDisposedException(nameof(RenderSender));
        }
    }
}
