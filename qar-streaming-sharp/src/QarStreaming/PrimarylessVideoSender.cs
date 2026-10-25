using System;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// Application-owned video stream, such as GUI panel content, that exists before any receiver
    /// connects and is delivered to every receiver that requests its stream id.
    /// </summary>
    /// <remarks>
    /// <para>
    /// Unlike <see cref="RenderSender"/>, which answers one target's render request and hands
    /// out poses and FOVs to render with, this sender carries an arbitrary layout and no render
    /// metadata. Use <see cref="RenderSender"/> for remote-rendered content shown in an app
    /// volume, this one for video the application produces on its own.
    /// </para>
    /// <para>
    /// Frame cycle: <see cref="TryBeginFrame"/> until it returns true, write the frame
    /// (<see cref="GetCpuFrame"/>, <see cref="GetD3D11Frame"/>, <see cref="GetOpenGlFrame"/> or
    /// <see cref="GetVulkanFrame"/>, matching the sender's graphics API), then
    /// <see cref="ShowFrame"/>. The frame exists from creation and may be inspected outside a
    /// cycle; its contents and synchronization may be written only inside one. Its texture
    /// handles stay stable until a layout change, so they can be cached between cycles.
    /// </para>
    /// </remarks>
    public sealed class PrimarylessVideoSender : IDisposable
    {
        private readonly NativeHandleOwner _owner;

        internal PrimarylessVideoSender(VideoSenderSafeHandle handle)
        {
            _owner = new NativeHandleOwner(handle, nameof(PrimarylessVideoSender));
        }

        public Result<StreamId> GetStreamId()
        {
            ThrowIfDisposed();
            StreamIdNative native;
            using var sender = _owner.Borrow();
            var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_stream_id(sender.Handle, out native));
            return info.IsSuccess ? Result<StreamId>.Success(info, StreamId.FromNative(native)) : Result<StreamId>.Failure(info);
        }

        /// <summary>
        /// Non-blocking: true when a frame cycle was begun for the receivers ready at this
        /// moment; false when no cycle exists yet, so poll again later.
        /// </summary>
        /// <remarks>
        /// Receivers that become ready afterwards get the next cycle. Recoverable failures, to
        /// keep polling through: <see cref="StatusCode.MediaStreamingNoActiveTransfers"/> while
        /// no receiver is connected (the normal state before the first one requests the
        /// stream), <see cref="StatusCode.MediaStreamingStreamIsPaused"/> while every receiver
        /// has paused it, and <see cref="StatusCode.MediaStreamingStreamIsReconnecting"/> while
        /// every receiver is reconnecting. Calling it again before <see cref="ShowFrame"/> fails
        /// with <see cref="StatusCode.LogicError"/>: only one cycle may be open.
        /// </remarks>
        public Result<bool> TryBeginFrame()
        {
            ThrowIfDisposed();
            bool ready;
            using var sender = _owner.Borrow();
            var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_try_begin_frame(sender.Handle, out ready));
            return info.IsSuccess ? Result<bool>.Success(info, ready) : Result<bool>.Failure(info);
        }

        public Result<VideoFrameLayout> GetLayout()
        {
            ThrowIfDisposed();
            var native = NativeDefaults.VideoFrameLayout();
            using var sender = _owner.Borrow();
            var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_layout(sender.Handle, ref native));
            return info.IsSuccess ? Result<VideoFrameLayout>.Success(info, VideoFrameLayout.FromNative(native)) : Result<VideoFrameLayout>.Failure(info);
        }

        /// <summary>
        /// Replaces the frame layout. The new frame resources are created synchronously on the
        /// calling thread; if that fails, the previous layout and frame stay usable.
        /// </summary>
        /// <remarks>
        /// Texture handles cached from an earlier frame do not survive a successful change: fetch
        /// the frame again.
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="layout"/> is null.</exception>
        public Result<Unit> ChangeLayout(VideoFrameLayout layout)
        {
            if (layout == null)
                throw new ArgumentNullException(nameof(layout));
            ThrowIfDisposed();
            var native = layout.ToNative();
            using var sender = _owner.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_video_sender_change_layout(sender.Handle, ref native));
        }

        public Result<VideoFrameCpu> GetCpuFrame()
        {
            ThrowIfDisposed();
            var native = NativeDefaults.VideoFrameCpu();
            using var sender = _owner.Borrow();
            var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_frame_cpu(sender.Handle, ref native));
            return info.IsSuccess ? Result<VideoFrameCpu>.Success(info, VideoFrameCpu.FromNative(native)) : Result<VideoFrameCpu>.Failure(info);
        }

        public Result<VideoFrameD3D11> GetD3D11Frame()
        {
            ThrowIfDisposed();
            var native = NativeDefaults.VideoFrameD3D11();
            using var sender = _owner.Borrow();
            var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_frame_d3d11(sender.Handle, ref native));
            return info.IsSuccess ? Result<VideoFrameD3D11>.Success(info, VideoFrameD3D11.FromNative(native)) : Result<VideoFrameD3D11>.Failure(info);
        }

        /// <summary>Requires a library built with QAR_ENABLE_OPENGL (otherwise <see cref="EntryPointNotFoundException"/>).</summary>
        public Result<VideoFrameOpenGl> GetOpenGlFrame()
        {
            ThrowIfDisposed();
            var native = NativeDefaults.VideoFrameOpenGl();
            using var sender = _owner.Borrow();
            var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_frame_opengl(sender.Handle, ref native));
            return info.IsSuccess ? Result<VideoFrameOpenGl>.Success(info, VideoFrameOpenGl.FromNative(native)) : Result<VideoFrameOpenGl>.Failure(info);
        }

        /// <summary>
        /// Sender-owned Vulkan frame; requires a library built with QAR_ENABLE_VULKAN
        /// (otherwise <see cref="EntryPointNotFoundException"/>). Valid until the next
        /// call or this sender's disposal.
        /// </summary>
        /// <remarks>
        /// Inside a frame cycle, record on it the layout and queue family each image was left in
        /// (<see cref="VideoFrameVulkan.SetTextureState"/>) and the synchronization the rendering
        /// signals (<see cref="VideoFrameVulkan.SetSynchronization"/>) before
        /// <see cref="ShowFrame"/>. <see cref="ShowFrame"/> fails with
        /// <see cref="StatusCode.LogicError"/> when this was not called in the current cycle.
        /// </remarks>
        public Result<VideoFrameVulkan> GetVulkanFrame()
        {
            ThrowIfDisposed();
            using var sender = _owner.Borrow();
            var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_frame_vulkan(sender.Handle, out var frame));
            return Results.FromHandle(info, frame, VideoFrameVulkan.FromNativePointer);
        }

        /// <summary>
        /// Sends the written frame to the receivers reserved by <see cref="TryBeginFrame"/> and
        /// ends the cycle.
        /// </summary>
        /// <remarks>
        /// Succeeds when at least one reserved receiver got the frame; fails with
        /// <see cref="StatusCode.MediaStreamingNoActiveTransfers"/> when all of them were lost
        /// before delivery.
        /// </remarks>
        public Result<Unit> ShowFrame()
        {
            ThrowIfDisposed();
            using var sender = _owner.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_video_sender_show_frame(sender.Handle));
        }

        /// <summary>Closes the stream to every receiver, draining frames still being sent first.</summary>
        /// <remarks>
        /// Safe while another thread is inside a call on the sender: new calls are refused at once
        /// (<see cref="ObjectDisposedException"/>), Dispose waits until the running call returned
        /// and then destroys the native sender on the calling thread.
        /// </remarks>
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
