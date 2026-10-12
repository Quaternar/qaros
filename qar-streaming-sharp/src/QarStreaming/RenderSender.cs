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
    /// Streams the application's rendered frames to one target peer, displayed in the bound app
    /// volume. Create one per target with <see cref="RenderSenderControl.Create"/>.
    /// </summary>
    /// <remarks>
    /// <para>
    /// Frame loop: <see cref="BeginFrame"/> (or <see cref="BeginFrameAsync(CancellationToken)"/> /
    /// <see cref="TryBeginFrame"/>) hands out the per-view poses and FOVs; render with exactly
    /// those into the frame from <see cref="GetCpuFrame"/>, <see cref="GetD3D11Frame"/>,
    /// <see cref="GetOpenGlFrame"/> or <see cref="GetVulkanFrame"/> (the one matching the
    /// sender's graphics API); then <see cref="ShowFrame"/> with the near/far actually used. The
    /// mixer re-projects the frame with that metadata and the depth, so wrong metadata shows as
    /// swimming content, not as an error. Only one frame cycle is open at a time; calling
    /// <see cref="ShowFrame"/> without a begun frame fails with
    /// <see cref="StatusCode.LogicError"/>.
    /// </para>
    /// <para>
    /// Everything the sender hands out (view poses, hands) is in the bound app volume's app
    /// content space, never room space. With auto-reconnects enabled, frame calls fail with
    /// <see cref="StatusCode.MediaStreamingStreamIsReconnecting"/> while the stream reconnects
    /// and recover on their own; keep the sender.
    /// <see cref="StatusCode.RenderingProducerStreamIsClosed"/> from begin-frame means the
    /// stream closed for good: dispose the sender and create a new one if the target is still
    /// to be served.
    /// </para>
    /// </remarks>
    public sealed class RenderSender : IDisposable
    {
        private readonly RenderSenderSafeHandle _handle;

        internal RenderSender(RenderSenderSafeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
        }

        /// <summary>
        /// The negotiated frame layout. Available as soon as the sender exists, before the first
        /// begin-frame and while reconnecting, so size lasting resources from it.
        /// </summary>
        /// <remarks>
        /// Its views come in the order declared in <see cref="RenderSenderOptions.FrameViews"/>,
        /// or after <see cref="ChangeLayout"/> in the order of that layout, and every frame
        /// matches it.
        /// </remarks>
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

        /// <summary>
        /// Requests a layout for subsequent frames; their views and textures keep the order given
        /// in <paramref name="layout"/>.
        /// </summary>
        /// <remarks>
        /// Not callable inside a frame cycle: between a successful begin-frame and its
        /// <see cref="ShowFrame"/> it fails with <see cref="StatusCode.LogicError"/> and changes
        /// nothing.
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="layout"/> is null.</exception>
        public Result<Unit> ChangeLayout(VideoFrameLayout layout)
        {
            if (layout == null)
                throw new ArgumentNullException(nameof(layout));
            ThrowIfDisposed();
            var native = layout.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_render_sender_change_layout(_handle.DangerousGetHandle(), ref native));
        }

        /// <summary>
        /// CPU-backed frame access. The frame exists from creation, so its textures, views and
        /// sizes are readable before the first begin-frame; its pixels may be written only inside
        /// a frame cycle.
        /// </summary>
        public Result<VideoFrameCpu> GetCpuFrame()
        {
            ThrowIfDisposed();
            var native = NativeDefaults.VideoFrameCpu();
            var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_frame_cpu(_handle.DangerousGetHandle(), ref native));
            return info.IsSuccess ? Result<VideoFrameCpu>.Success(info, VideoFrameCpu.FromNative(native)) : Result<VideoFrameCpu>.Failure(info);
        }

        /// <summary>
        /// D3D11-backed frame access: shared textures to render into directly. Readable before
        /// the first begin-frame, writable only inside a frame cycle.
        /// </summary>
        /// <remarks>
        /// The sender acquires and releases the textures' keyed mutexes itself unless
        /// <see cref="StreamParamsD3D11.AcquireKeyedMutexSync"/> was false at creation; then the
        /// application drives the keyed-mutex chain with the frame's synchronization data.
        /// </remarks>
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
        /// hands untracked. The call is held about 16 ms before failing, so a retry
        /// loop does not spin.
        /// <para>
        /// While the receiver has paused the stream (for example its display is not
        /// visible) the call waits instead of failing, and the frame begins once the
        /// receiver resumes. Cancel <paramref name="cancellationToken"/> to stop
        /// waiting; the call then fails. A frame still open from a previous
        /// begin-frame without <see cref="ShowFrame"/> is ended first.
        /// </para>
        /// <para>
        /// Read the returned <see cref="RenderFrameInfo"/> before <see cref="ShowFrame"/>
        /// and dispose it afterwards.
        /// </para>
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
        /// <remarks>
        /// Like <see cref="BeginFrame"/> it waits through a paused stream; cancel
        /// <paramref name="cancellationToken"/> to stop waiting.
        /// </remarks>
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
        /// <remarks>
        /// While the receiver has paused the stream the wait does not complete: the
        /// call keeps succeeding with null until the receiver resumes.
        /// </remarks>
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
        /// Valid until the next begin-frame or this sender's disposal. Inside a frame cycle,
        /// record final layouts, queue ownership and synchronization on it before
        /// <see cref="ShowFrame"/>.
        /// </summary>
        /// <remarks>
        /// Readable from creation, so resources can be created against the real images during
        /// initialization. Every begin-frame discards the previously returned frame: fetch it
        /// again in each frame cycle, and do not write through an instance from an earlier
        /// cycle. <see cref="ShowFrame"/> of a Vulkan sender fails with
        /// <see cref="StatusCode.LogicError"/> when this was not called in the current cycle.
        /// </remarks>
        public Result<VideoFrameVulkan> GetVulkanFrame()
        {
            ThrowIfDisposed();
            var info = Results.ToInfo(NativeMethods.qar_impl_render_sender_frame_vulkan(_handle.DangerousGetHandle(), out var frame));
            return Results.FromHandle(info, frame, VideoFrameVulkan.FromNativePointer);
        }

        /// <summary>Submits the rendered frame. <see cref="RenderFrameShow.RenderedNearFar"/> and any view-override pose are in app content space.</summary>
        /// <remarks>
        /// Report the near/far actually rendered with; the defaults (0.1 to 100) are used when
        /// <paramref name="frameShow"/> is null. When a view was rendered with a pose or FOV
        /// other than the one <see cref="RenderFrameInfo"/> handed out, declare it with
        /// <see cref="RenderFrameShow.WithViewOverrides"/>. Fails with
        /// <see cref="StatusCode.LogicError"/> when no frame was begun.
        /// </remarks>
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

        /// <summary>
        /// Ends the stream to the target. Every <see cref="RenderFrameInfo"/> this sender handed
        /// out becomes invalid.
        /// </summary>
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
