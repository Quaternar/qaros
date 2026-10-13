using System;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// Creates <see cref="PrimarylessVideoSender"/>s for a session; reached through
    /// <see cref="Session.PrimarylessVideoSenders"/>.
    /// </summary>
    public sealed class PrimarylessVideoSenderControl
    {
        private readonly SessionNativeContext _context;

        internal PrimarylessVideoSenderControl(SessionNativeContext context)
        {
            _context = context ?? throw new ArgumentNullException(nameof(context));
        }

        /// <summary>Creates a CPU video sender; <see cref="PrimarylessVideoSenderOptions.GraphicsAPI"/> must be <see cref="GraphicsAPI.Cpu"/>.</summary>
        /// <exception cref="ArgumentNullException"><paramref name="options"/> or its layout is null.</exception>
        /// <exception cref="ArgumentException">The graphics API is not <see cref="GraphicsAPI.Cpu"/>.</exception>
        /// <exception cref="ArgumentOutOfRangeException">The reconnect timeout is not positive.</exception>
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
        /// <remarks>
        /// Synchronous: it negotiates nothing and needs no receiver, and creates the frame
        /// resources on the calling thread. The library generates the stream id; publish
        /// <see cref="PrimarylessVideoSender.GetStreamId"/> (for example as
        /// <see cref="GuiPanelOptions.ContentStreamId"/>) so receivers can request the stream.
        /// </remarks>
        /// <exception cref="ArgumentNullException"><paramref name="options"/> or its layout is null.</exception>
        /// <exception cref="ArgumentOutOfRangeException">
        /// The reconnect timeout is not positive (checked even with reconnects disabled) or does
        /// not fit in milliseconds.
        /// </exception>
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
