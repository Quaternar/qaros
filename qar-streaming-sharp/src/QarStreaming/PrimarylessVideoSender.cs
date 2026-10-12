using System;
using Qar.Native;

namespace Qar
{
    public sealed class PrimarylessVideoSender : IDisposable
    {
        private readonly VideoSenderSafeHandle _handle;

        internal PrimarylessVideoSender(VideoSenderSafeHandle handle)
        {
            _handle = handle ?? throw new ArgumentNullException(nameof(handle));
        }

        public Result<StreamId> GetStreamId()
        {
            ThrowIfDisposed();
            StreamIdNative native;
            var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_stream_id(_handle.DangerousGetHandle(), out native));
            return info.IsSuccess ? Result<StreamId>.Success(info, StreamId.FromNative(native)) : Result<StreamId>.Failure(info);
        }

        public Result<bool> TryBeginFrame()
        {
            ThrowIfDisposed();
            bool ready;
            var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_try_begin_frame(_handle.DangerousGetHandle(), out ready));
            return info.IsSuccess ? Result<bool>.Success(info, ready) : Result<bool>.Failure(info);
        }

        public Result<VideoFrameLayout> GetLayout()
        {
            ThrowIfDisposed();
            var native = NativeDefaults.VideoFrameLayout();
            var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_layout(_handle.DangerousGetHandle(), ref native));
            return info.IsSuccess ? Result<VideoFrameLayout>.Success(info, VideoFrameLayout.FromNative(native)) : Result<VideoFrameLayout>.Failure(info);
        }

        public Result<Unit> ChangeLayout(VideoFrameLayout layout)
        {
            if (layout == null)
                throw new ArgumentNullException(nameof(layout));
            ThrowIfDisposed();
            var native = layout.ToNative();
            return Results.FromNative(NativeMethods.qar_impl_video_sender_change_layout(_handle.DangerousGetHandle(), ref native));
        }

        public Result<VideoFrameCpu> GetCpuFrame()
        {
            ThrowIfDisposed();
            var native = NativeDefaults.VideoFrameCpu();
            var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_frame_cpu(_handle.DangerousGetHandle(), ref native));
            return info.IsSuccess ? Result<VideoFrameCpu>.Success(info, VideoFrameCpu.FromNative(native)) : Result<VideoFrameCpu>.Failure(info);
        }

        public Result<VideoFrameD3D11> GetD3D11Frame()
        {
            ThrowIfDisposed();
            var native = NativeDefaults.VideoFrameD3D11();
            var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_frame_d3d11(_handle.DangerousGetHandle(), ref native));
            return info.IsSuccess ? Result<VideoFrameD3D11>.Success(info, VideoFrameD3D11.FromNative(native)) : Result<VideoFrameD3D11>.Failure(info);
        }

        /// <summary>Requires a library built with QAR_ENABLE_OPENGL (otherwise <see cref="EntryPointNotFoundException"/>).</summary>
        public Result<VideoFrameOpenGl> GetOpenGlFrame()
        {
            ThrowIfDisposed();
            var native = NativeDefaults.VideoFrameOpenGl();
            var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_frame_opengl(_handle.DangerousGetHandle(), ref native));
            return info.IsSuccess ? Result<VideoFrameOpenGl>.Success(info, VideoFrameOpenGl.FromNative(native)) : Result<VideoFrameOpenGl>.Failure(info);
        }

        /// <summary>
        /// Sender-owned Vulkan frame; requires a library built with QAR_ENABLE_VULKAN
        /// (otherwise <see cref="EntryPointNotFoundException"/>). Valid until the next
        /// call or this sender's disposal.
        /// </summary>
        public Result<VideoFrameVulkan> GetVulkanFrame()
        {
            ThrowIfDisposed();
            var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_frame_vulkan(_handle.DangerousGetHandle(), out var frame));
            return Results.FromHandle(info, frame, VideoFrameVulkan.FromNativePointer);
        }

        public Result<Unit> ShowFrame()
        {
            ThrowIfDisposed();
            return Results.FromNative(NativeMethods.qar_impl_video_sender_show_frame(_handle.DangerousGetHandle()));
        }

        public void Dispose()
        {
            _handle.Dispose();
        }

        private void ThrowIfDisposed()
        {
            if (_handle.IsClosed || _handle.IsInvalid)
                throw new ObjectDisposedException(nameof(PrimarylessVideoSender));
        }
    }
}
