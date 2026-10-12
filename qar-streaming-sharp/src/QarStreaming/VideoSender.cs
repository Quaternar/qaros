using System;
using Qar.Native;

namespace Qar
{
    public sealed class PrimarylessVideoSenderControl
    {
        private readonly SessionNativeContext _context;

        internal PrimarylessVideoSenderControl(SessionNativeContext context)
        {
            _context = context ?? throw new ArgumentNullException(nameof(context));
        }

        /// <summary>Creates a CPU video sender; <see cref="PrimarylessVideoSenderOptions.GraphicsAPI"/> must be <see cref="GraphicsAPI.Cpu"/>.</summary>
        public Result<PrimarylessVideoSender> CreateCpu(PrimarylessVideoSenderOptions options)
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            if (options.GraphicsAPI != GraphicsAPI.Cpu)
                throw new ArgumentException("CreateCpu requires GraphicsAPI.Cpu; use Create for GPU senders.", nameof(options));
            return Create(options);
        }

        /// <summary>
        /// Creates a producer-owned logical video sender for
        /// <see cref="PrimarylessVideoSenderOptions.GraphicsAPI"/>, with the matching
        /// stream-parameter extension for a GPU API.
        /// </summary>
        public Result<PrimarylessVideoSender> Create(PrimarylessVideoSenderOptions options)
        {
            if (options == null)
                throw new ArgumentNullException(nameof(options));
            if (options.Layout == null)
                throw new ArgumentNullException(nameof(options.Layout));
            if (options.ReconnectTimeout <= TimeSpan.Zero)
                throw new ArgumentOutOfRangeException(nameof(options.ReconnectTimeout));
            if (options.ReconnectTimeout.TotalMilliseconds > ulong.MaxValue)
                throw new ArgumentOutOfRangeException(nameof(options.ReconnectTimeout));
            _context.ThrowIfDisposed();
            using (var scope = new NativeMemoryScope())
            {
                var native = NativeInit.VideoSender(options, scope);
                IntPtr sender;
                var info = Results.ToInfo(NativeMethods.qar_impl_video_sender_create(_context.DangerousHandle, ref native, out sender));
                return Results.FromHandle(info, sender, h => new PrimarylessVideoSender(new VideoSenderSafeHandle(h)));
            }
        }
    }
}
