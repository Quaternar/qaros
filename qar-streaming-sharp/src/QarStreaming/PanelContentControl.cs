using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>
    /// Names one content bundle: the session that published it and serves its files, and its
    /// name, unique per publisher.
    /// </summary>
    public readonly struct PanelContentBundleId : IEquatable<PanelContentBundleId>
    {
        /// <summary>Longest bundle name, in bytes.</summary>
        public const int MaxNameLength = 64;

        public PanelContentBundleId(PeerId publisher, string name)
        {
            Publisher = publisher;
            Name = name ?? throw new ArgumentNullException(nameof(name));
        }

        /// <summary>The peer that published the bundle: the panel content's owner.</summary>
        public PeerId Publisher { get; }

        /// <summary>ASCII: A-Z, a-z, 0-9, '.', '_' and '-', not starting with '.', at most 64 characters.</summary>
        public string Name { get; }

        public bool Equals(PanelContentBundleId other) { return Publisher.Equals(other.Publisher) && string.Equals(Name, other.Name, StringComparison.Ordinal); }

        public override bool Equals(object? obj) { return obj is PanelContentBundleId other && Equals(other); }

        public override int GetHashCode() { return Publisher.GetHashCode() * 31 + (Name ?? string.Empty).GetHashCode(); }

        public override string ToString() { return Publisher + "/" + Name; }

        public static bool operator ==(PanelContentBundleId left, PanelContentBundleId right) { return left.Equals(right); }

        public static bool operator !=(PanelContentBundleId left, PanelContentBundleId right) { return !left.Equals(right); }

        /// <summary>
        /// The native id. A name longer than <see cref="MaxNameLength"/> bytes, or holding NUL,
        /// is passed so that the C API refuses it rather than cutting it.
        /// </summary>
        internal PanelContentBundleIdNative ToNative()
        {
            var native = PanelContentBundleIdNative.CreateDefault();
            native.Publisher = Publisher.ToNative();
            var bytes = Encoding.UTF8.GetBytes(Name ?? string.Empty);
            var length = Math.Min(bytes.Length, PanelContentBundleIdNative.NameCapacity);
            Array.Copy(bytes, native.Name, length);
            return native;
        }

        internal static PanelContentBundleId FromNative(PanelContentBundleIdNative native)
        {
            return new PanelContentBundleId(PeerId.FromNative(native.Publisher), NativeString.DecodeNullTerminated(native.Name));
        }
    }

    /// <summary>What a served GUI panel shows: one entry page of a content bundle.</summary>
    public sealed class GuiPanelServedContent
    {
        public GuiPanelServedContent(PanelContentBundleId bundle, string entry)
        {
            Bundle = bundle;
            Entry = entry ?? throw new ArgumentNullException(nameof(entry));
        }

        /// <summary>The bundle; its publisher owns the content.</summary>
        public PanelContentBundleId Bundle { get; }

        /// <summary>Path of the page inside the bundle, for example <c>inspector/index.html</c>.</summary>
        public string Entry { get; }
    }

    /// <summary>One file handed to <see cref="PanelContentControl.PublishFiles"/>.</summary>
    public sealed class PanelContentFile
    {
        public PanelContentFile(string path, byte[] data, string? mimeType = null)
        {
            Path = path ?? throw new ArgumentNullException(nameof(path));
            Data = data ?? throw new ArgumentNullException(nameof(data));
            MimeType = mimeType;
        }

        /// <summary>Relative, '/'-separated path inside the bundle.</summary>
        public string Path { get; }

        /// <summary>Null: taken from the path's extension. Otherwise it must be the type that extension is served as.</summary>
        public string? MimeType { get; }

        /// <summary>The file's bytes, copied when published.</summary>
        public byte[] Data { get; }
    }

    /// <summary>A message a served panel sent to its content owner, stamped by the mixer that shows it.</summary>
    public sealed class PanelContentMessage
    {
        internal PanelContentMessage(GuiPanelId panel, PeerId user, ulong sequence, string json)
        {
            Panel = panel;
            User = user;
            Sequence = sequence;
            Json = json;
        }

        /// <summary>The panel that sent it.</summary>
        public GuiPanelId Panel { get; }

        /// <summary>The user (the mixer's target peer) whose panel sent it.</summary>
        public PeerId User { get; }

        /// <summary>Counts up per sending mixer and panel, from 1.</summary>
        public ulong Sequence { get; }

        /// <summary>One JSON value, as the page passed it to <c>qaros.panel.send</c>.</summary>
        public string Json { get; }

        internal static PanelContentMessage FromNative(IntPtr message)
        {
            var native = Marshal.PtrToStructure<PanelContentMessageNative>(message);
            var length = NativeConvert.ToInt32(native.JsonSize);
            var bytes = new byte[length];
            if (length > 0)
                Marshal.Copy(native.Json, bytes, 0, length);
            return new PanelContentMessage(GuiPanelId.FromNative(native.Panel), PeerId.FromNative(native.User), native.Sequence, Encoding.UTF8.GetString(bytes));
        }
    }

    /// <summary>Carries one <see cref="PanelContentMessage"/>.</summary>
    public sealed class PanelContentMessageEventArgs : EventArgs
    {
        internal PanelContentMessageEventArgs(PanelContentMessage message) { Message = message; }

        public PanelContentMessage Message { get; }
    }

    /// <summary>
    /// Serves web pages to GUI panels from this session and exchanges JSON messages with them.
    /// Reached through <see cref="Session.PanelContent"/>.
    /// </summary>
    /// <remarks>
    /// <para>
    /// A served GUI panel shows a page of a content bundle: a named, versioned set of files this
    /// session publishes once. Every user's mixer loads the page in its own browser, fetching the
    /// files over the session, so each user has their own page state. The page sends with
    /// <c>qaros.panel.send(msg)</c> and receives with <c>qaros.panel.onMessage(cb)</c>.
    /// </para>
    /// <para>
    /// Publishing copies every byte: nothing on disk is read while serving, and an edit needs
    /// another publication, which reloads every panel showing the bundle. Only the publisher may
    /// put its bundle on a panel, receives the panels' messages and posts to them. Limits: 16 MiB
    /// per file, 64 MiB and 1024 files per bundle, 64 KiB per message.
    /// </para>
    /// </remarks>
    public sealed class PanelContentControl
    {
        private readonly SessionNativeContext _context;

        internal PanelContentControl(SessionNativeContext context)
        {
            _context = context ?? throw new ArgumentNullException(nameof(context));
        }

        /// <summary>
        /// Publishes every regular file under <paramref name="root"/> as bundle
        /// <paramref name="name"/>, replacing an earlier publication of that name. A link inside
        /// the folder is refused, as is every file <see cref="PublishFiles"/> refuses.
        /// </summary>
        public Result<PanelContentBundleId> PublishDirectory(string name, string root)
        {
            if (name == null)
                throw new ArgumentNullException(nameof(name));
            if (root == null)
                throw new ArgumentNullException(nameof(root));
            using (var scope = new NativeMemoryScope())
            using (var session = _context.Borrow())
            {
                var bundle = PanelContentBundleIdNative.CreateDefault();
                var info = Results.ToInfo(NativeMethods.qar_impl_panel_content_publish_directory(session.Handle, scope.AllocUtf8(name), scope.AllocUtf8(root), ref bundle));
                return info.IsSuccess ? Result<PanelContentBundleId>.Success(info, PanelContentBundleId.FromNative(bundle)) : Result<PanelContentBundleId>.Failure(info);
            }
        }

        /// <summary>
        /// Publishes <paramref name="files"/> as bundle <paramref name="name"/>, replacing an
        /// earlier publication of that name. Nothing is published when it fails: a bad name
        /// (<see cref="StatusCode.PanelContentBundleRefused"/>), a bad or duplicate path
        /// (<see cref="StatusCode.PanelContentPathRefused"/>), a type that is not served
        /// (<see cref="StatusCode.PanelContentMimeRefused"/>) or a limit exceeded
        /// (<see cref="StatusCode.PanelContentSizeRefused"/>).
        /// </summary>
        public Result<PanelContentBundleId> PublishFiles(string name, IReadOnlyList<PanelContentFile> files)
        {
            if (name == null)
                throw new ArgumentNullException(nameof(name));
            if (files == null)
                throw new ArgumentNullException(nameof(files));
            using (var scope = new NativeMemoryScope())
            using (var session = _context.Borrow())
            {
                var natives = new PanelContentFileNative[files.Count];
                for (var i = 0; i < files.Count; i++)
                {
                    var file = files[i] ?? throw new ArgumentException("A file is null.", nameof(files));
                    natives[i] = new PanelContentFileNative
                    {
                        Path = scope.AllocUtf8(file.Path),
                        MimeType = scope.AllocUtf8(file.MimeType),
                        Data = scope.AllocBytes(file.Data),
                        Size = (UIntPtr)file.Data.Length
                    };
                }
                var bundle = PanelContentBundleIdNative.CreateDefault();
                var info = Results.ToInfo(NativeMethods.qar_impl_panel_content_publish_files(session.Handle, scope.AllocUtf8(name), scope.AllocArray(natives), (UIntPtr)natives.Length, ref bundle));
                return info.IsSuccess ? Result<PanelContentBundleId>.Success(info, PanelContentBundleId.FromNative(bundle)) : Result<PanelContentBundleId>.Failure(info);
            }
        }

        /// <summary>Stops serving a bundle this session published. Panels showing it stay and show nothing.</summary>
        public Result<Unit> Remove(PanelContentBundleId bundle)
        {
            var native = bundle.ToNative();
            using var session = _context.Borrow();
            return Results.FromNative(NativeMethods.qar_impl_panel_content_remove(session.Handle, ref native));
        }

        /// <summary>
        /// Receives the messages of every panel showing a bundle this session published, each
        /// stamped with its panel and user. Disposing the result, cancelling the token, removing
        /// the bundle or closing the session stops delivery.
        /// </summary>
        /// <remarks>
        /// The handler runs on a library thread and an exception it throws is dropped. Validate
        /// each message before acting on it, and authorise per <see cref="PanelContentMessage.User"/>.
        /// </remarks>
        public Result<IDisposable> SubscribeMessages(PanelContentBundleId bundle, EventHandler<PanelContentMessageEventArgs> handler, CancellationToken cancellationToken = default(CancellationToken))
        {
            if (handler == null)
                throw new ArgumentNullException(nameof(handler));
            var native = bundle.ToNative();
            return _context.StartSubscription(
                cancellationToken,
                token => PanelContentMessageSubscription.Create(token, true, handler),
                (token, registration) => PanelContentMessageSubscription.Create(token, registration, handler),
                (subscription, session, token) => NativeMethods.qar_impl_panel_content_subscribe_messages(session, ref native, subscription.Callback, subscription.UserState, token));
        }

        /// <summary>
        /// Posts one JSON value to the panels showing a bundle this session published: to one
        /// panel or all (<paramref name="panel"/> null), for one user or all
        /// (<paramref name="user"/> null).
        /// </summary>
        public Result<Unit> PostMessage(PanelContentBundleId bundle, string json, GuiPanelId? panel = null, PeerId? user = null)
        {
            if (json == null)
                throw new ArgumentNullException(nameof(json));
            var native = bundle.ToNative();
            using (var scope = new NativeMemoryScope())
            using (var session = _context.Borrow())
            {
                var panelPointer = panel.HasValue ? scope.AllocStruct(panel.Value.ToNative()) : IntPtr.Zero;
                var userPointer = user.HasValue ? scope.AllocStruct(user.Value.ToNative()) : IntPtr.Zero;
                return Results.FromNative(NativeMethods.qar_impl_panel_content_post_message(session.Handle, ref native, panelPointer, userPointer, scope.AllocUtf8(json)));
            }
        }
    }

    internal sealed class PanelContentMessageSubscription : NativeSubscription
    {
        private readonly EventHandler<PanelContentMessageEventArgs> _handler;

        public PanelContentMessageCallback Callback { get; }

        private PanelContentMessageSubscription(CancelToken token, bool ownsToken, EventHandler<PanelContentMessageEventArgs> handler) : base(token, ownsToken)
        {
            _handler = handler;
            Callback = OnMessage;
        }

        private PanelContentMessageSubscription(CancelToken token, CancellationTokenRegistration standardCancellationRegistration, EventHandler<PanelContentMessageEventArgs> handler) : base(token, true, standardCancellationRegistration)
        {
            _handler = handler;
            Callback = OnMessage;
        }

        public static PanelContentMessageSubscription Create(CancelToken token, bool ownsToken, EventHandler<PanelContentMessageEventArgs> handler) { return new PanelContentMessageSubscription(token, ownsToken, handler); }

        public static PanelContentMessageSubscription Create(CancelToken token, CancellationTokenRegistration standardCancellationRegistration, EventHandler<PanelContentMessageEventArgs> handler) { return new PanelContentMessageSubscription(token, standardCancellationRegistration, handler); }

        private static void OnMessage(IntPtr message, IntPtr userState)
        {
            try
            {
                if (!NativeCallbackRegistry.TryGet(userState, out PanelContentMessageSubscription sub))
                    return;
                if (sub.IsDisposed || message == IntPtr.Zero)
                    return;
                // The message dies with this call, so it is copied here.
                using (sub.EnterCallback())
                    sub._handler(sub, new PanelContentMessageEventArgs(PanelContentMessage.FromNative(message)));
                GC.KeepAlive(sub);
            }
            catch
            {
            }
        }
    }
}
