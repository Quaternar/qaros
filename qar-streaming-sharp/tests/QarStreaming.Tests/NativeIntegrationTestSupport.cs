using System;
using System.Collections.Generic;
using System.IO;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros;
using Quaternar.Qaros.Interop;
using Quaternar.Qaros.Tests.Ext;
using Xunit;
using Quaternar.Qaros.Mathematics;

namespace Quaternar.Qaros.Tests
{
    [CollectionDefinition("QAR Native", DisableParallelization = true)]
    public sealed class QarNativeCollection
    {
    }

    public sealed class QarNativeFixture : IDisposable
    {
        public QarNativeFixture()
        {
            Library.Init().EnsureSuccess();
        }

        public SessionTestScope CreateSessionScope()
        {
            return SessionTestScope.Create();
        }

        public void Dispose()
        {
            Library.Destroy().EnsureSuccess();
        }
    }

    /// <summary>
    /// One session of the API under test, onboarded into an in-process hub of
    /// qar-streaming-c-ext: no running QAROS needed. Peers to stream to come
    /// from the same hub.
    /// </summary>
    public sealed class SessionTestScope : IDisposable
    {
        private readonly List<ExtPeer> _peers = new List<ExtPeer>();
        private readonly List<ExtRenderTarget> _targets = new List<ExtRenderTarget>();
        private readonly string _storageRoot;

        private SessionTestScope(string storageRoot, Runtime runtime, ExtHub hub, OnboardSession onboarded)
        {
            _storageRoot = storageRoot;
            Runtime = runtime;
            Hub = hub;
            Session = onboarded.Session;
            OnboardingId = onboarded.OnboardingId;
        }

        public Runtime Runtime { get; }

        public ExtHub Hub { get; }

        public Session Session { get; }

        public OnboardingId OnboardingId { get; }

        /// <summary>A visualizer peer in the session, kept until the scope ends.</summary>
        public ExtPeer CreatePeer(string displayName = "QarStreamingSharpTests peer")
        {
            var peer = Hub.CreatePeer(displayName);
            _peers.Add(peer);
            return peer;
        }

        /// <summary>A visualizer peer render senders can target, once the session lists it.</summary>
        public PeerId InviteVisualizerPeer(int timeoutMs = 15000)
        {
            var peer = CreatePeer("QarStreamingSharpTests visualizer");
            QarNativeAssert.WaitForPeerInQuery(Session, peer.Id, (int)ExtTime.Scaled(TimeSpan.FromMilliseconds(timeoutMs)).TotalMilliseconds);
            return peer.Id;
        }

        /// <summary>
        /// A peer that receives what a render sender renders for it: an ext
        /// render receiver whose loop requests and collects frames until the
        /// scope ends. Use its PeerId as RenderSenderOptions.PeerId.
        /// </summary>
        public ExtRenderTarget StartRenderTarget(uint eyeWidth = 64, uint eyeHeight = 64)
        {
            var target = new ExtRenderTarget(CreatePeer("QarStreamingSharpTests render target"), ExtLayouts.SideBySide(eyeWidth, eyeHeight));
            _targets.Add(target);
            return target;
        }

        /// <summary>The app volume every render sender needs, created through the API under test.</summary>
        public AppVolumeId CreateAppVolume(string commonName = "qar-sharp-tests-volume")
        {
            return Session.AppVolumes.GetOrCreate(new AppVolumeOptions(commonName) { DisplayName = commonName }).ValueOrThrow();
        }

        public static SessionTestScope Create(string displayName = "QarStreamingSharpTests")
        {
            var storageRoot = Path.Combine(Path.GetTempPath(), "qar-sharp-tests", Guid.NewGuid().ToString("N"));
            var runtime = Runtime.Create(new RuntimeOptions { StorageFolderPath = storageRoot }).ValueOrThrow();
            var hub = ExtHub.Create();
            using var invite = OnboardingInvite.Deserialize(hub.MintInvite()).ValueOrThrow();
            var options = new OnboardOptions()
                .UseInvite(invite)
                .WithPresentation(new PeerPresentationOptions { DisplayName = displayName });
            var onboarded = runtime.Onboard(options).ValueOrThrow();
            return new SessionTestScope(storageRoot, runtime, hub, onboarded);
        }

        public void Dispose()
        {
            foreach (var target in _targets)
                target.Dispose();
            foreach (var peer in _peers)
                peer.Dispose();
            Session.Dispose();
            Hub.Dispose();
            Runtime.Dispose();
            try
            {
                Directory.Delete(_storageRoot, true);
            }
            catch (IOException)
            {
            }
            catch (UnauthorizedAccessException)
            {
            }
        }
    }

    /// <summary>
    /// A headset stand-in: requests frames for its peer and lends CPU buffers
    /// they are written into, on a background loop.
    /// </summary>
    public sealed class ExtRenderTarget : IDisposable
    {
        private readonly ExtRenderReceiver _receiver;
        private readonly ExtCpuFrame _frame;
        private readonly CancellationTokenSource _stop = new CancellationTokenSource();
        private readonly Task _loop;
        private int _framesReceived;
        private byte[]? _lastColor;

        internal ExtRenderTarget(ExtPeer peer, VideoFrameLayout layout)
        {
            PeerId = peer.Id;
            Layout = layout;
            _receiver = peer.CreateRenderReceiver(layout);
            _frame = new ExtCpuFrame(layout);
            _loop = Task.Run(() => Receive(_stop.Token));
        }

        public PeerId PeerId { get; }

        public VideoFrameLayout Layout { get; }

        public int FramesReceived { get { return Volatile.Read(ref _framesReceived); } }

        /// <summary>Colour bytes of the last valid frame, row pitch width x 4; null before one arrived.</summary>
        public byte[]? LastColor { get { return Volatile.Read(ref _lastColor); } }

        public bool WaitForFrames(int count, TimeSpan timeout)
        {
            var deadline = DateTime.UtcNow + ExtTime.Scaled(timeout);
            while (DateTime.UtcNow < deadline)
            {
                if (FramesReceived >= count)
                    return true;
                Thread.Sleep(20);
            }
            return FramesReceived >= count;
        }

        private void Receive(CancellationToken stop)
        {
            var video = _receiver.Video;
            while (!stop.IsCancellationRequested && !video.WaitConnected(TimeSpan.FromMilliseconds(200)))
            {
            }
            var lent = false;
            while (!stop.IsCancellationRequested)
            {
                if (!_receiver.RequestRender(Pose.Identity).IsSuccess)
                    return;
                lent = lent || video.TryProvideFrame(_frame);
                var info = video.ReceiveFrame(TimeSpan.FromMilliseconds(200));
                if (info == null)
                    continue;
                lent = false;
                if (!info.Value.ValidFrame)
                    continue;
                Volatile.Write(ref _lastColor, (byte[])_frame.Textures[0].Clone());
                Interlocked.Increment(ref _framesReceived);
            }
        }

        public void Dispose()
        {
            _stop.Cancel();
            try
            {
                _loop.Wait(TimeSpan.FromSeconds(10));
            }
            catch (AggregateException)
            {
            }
            _receiver.Dispose();
            _frame.Dispose();
            _stop.Dispose();
        }
    }

    internal static class QarNativeAssert
    {
        /// <summary>Budget of every eventually-consistent wait, scaled by QAR_TIME_SCALE.</summary>
        public static TimeSpan Budget { get { return ExtTime.Scaled(TimeSpan.FromSeconds(15)); } }

        /// <summary>
        /// Applies <paramref name="action"/> until <paramref name="seen"/> completes: an update
        /// published before a fresh subscription is live is otherwise lost.
        /// </summary>
        public static bool ApplyUntil(Task seen, Action action)
        {
            var deadline = DateTime.UtcNow + Budget;
            while (DateTime.UtcNow < deadline)
            {
                action();
                if (seen.Wait(TimeSpan.FromSeconds(1)))
                    return true;
            }
            return seen.IsCompleted;
        }
        public static GuiPanel SinglePanel(Session session, GuiPanelId id)
        {
            var panels = session.GuiPanels.GetAll().ValueOrThrow();
            return Assert.Single(panels, panel => panel.Id == id);
        }

        public static AppVolume SingleVolume(Session session, AppVolumeId id)
        {
            var volumes = session.AppVolumes.GetAll().ValueOrThrow();
            return Assert.Single(volumes, volume => volume.Id == id);
        }

        public static GuiPanel WaitForGuiPanelUpdate(Session session, Predicate<GuiPanel> predicate, Action action)
        {
#if NET8_0_OR_GREATER
            var tcs = new TaskCompletionSource<GuiPanel>(TaskCreationOptions.RunContinuationsAsynchronously);
#else
            var tcs = new TaskCompletionSource<GuiPanel>();
#endif
            using var subscription = session.GuiPanels.SubscribeUpdates((sender, args) =>
            {
                if (predicate(args.Panel))
                    tcs.TrySetResult(args.Panel);
            }).ValueOrThrow();
            Assert.True(ApplyUntil(tcs.Task, action), "Timed out waiting for gui panel update.");
            return tcs.Task.GetAwaiter().GetResult();
        }

        public static GuiPanel WaitForGuiPanelUpdate(Session session, GuiPanelId id, Action action)
        {
#if NET8_0_OR_GREATER
            var tcs = new TaskCompletionSource<GuiPanel>(TaskCreationOptions.RunContinuationsAsynchronously);
#else
            var tcs = new TaskCompletionSource<GuiPanel>();
#endif
            using var subscription = session.GuiPanels.SubscribeUpdates(id, (sender, args) => tcs.TrySetResult(args.Panel)).ValueOrThrow();
            Assert.True(ApplyUntil(tcs.Task, action), "Timed out waiting for gui panel update.");
            return tcs.Task.GetAwaiter().GetResult();
        }

        public static AppVolume WaitForAppVolumeUpdate(Session session, Predicate<AppVolume> predicate, Action action)
        {
#if NET8_0_OR_GREATER
            var tcs = new TaskCompletionSource<AppVolume>(TaskCreationOptions.RunContinuationsAsynchronously);
#else
            var tcs = new TaskCompletionSource<AppVolume>();
#endif
            using var subscription = session.AppVolumes.SubscribeUpdates((sender, args) =>
            {
                if (predicate(args.Volume))
                    tcs.TrySetResult(args.Volume);
            }).ValueOrThrow();
            Assert.True(ApplyUntil(tcs.Task, action), "Timed out waiting for app volume update.");
            return tcs.Task.GetAwaiter().GetResult();
        }

        public static T WaitUntil<T>(Func<T?> probe, Predicate<T> predicate, string description, int timeoutMs = 15000, int intervalMs = 50)
            where T : class
        {
            var deadline = DateTime.UtcNow.AddMilliseconds(timeoutMs);
            while (DateTime.UtcNow < deadline)
            {
                var value = probe();
                if (value != null && predicate(value))
                    return value;
                Thread.Sleep(intervalMs);
            }
            throw new Xunit.Sdk.XunitException("Timed out waiting for " + description + ".");
        }

        public static void WaitUntil(Func<bool> predicate, string description, int timeoutMs = 15000, int intervalMs = 50)
        {
            var deadline = DateTime.UtcNow.AddMilliseconds(timeoutMs);
            while (DateTime.UtcNow < deadline)
            {
                if (predicate())
                    return;
                Thread.Sleep(intervalMs);
            }
            throw new Xunit.Sdk.XunitException("Timed out waiting for " + description + ".");
        }

        public static GuiPanel WaitForPanelInQuery(Session session, GuiPanelId id)
        {
            return WaitUntil(
                () =>
                {
                    var result = session.GuiPanels.GetAll();
                    if (!result.IsSuccess)
                        return null;
                    foreach (var panel in result.Value)
                    {
                        if (panel.Id == id)
                            return panel;
                    }
                    return null;
                },
                panel => panel.Id == id,
                "gui panel query snapshot");
        }

        public static AppVolume WaitForVolumeInQuery(Session session, AppVolumeId id)
        {
            return WaitUntil(
                () =>
                {
                    var result = session.AppVolumes.GetAll();
                    if (!result.IsSuccess)
                        return null;
                    foreach (var volume in result.Value)
                    {
                        if (volume.Id == id)
                            return volume;
                    }
                    return null;
                },
                volume => volume.Id == id,
                "app volume query snapshot");
        }

        public static PeerSpec WaitForPeerInQuery(Session session, PeerId id, int timeoutMs = 15000)
        {
            return WaitUntil(
                () =>
                {
                    var result = session.Peers.GetAll();
                    if (!result.IsSuccess)
                        return null;
                    foreach (var peer in result.Value)
                    {
                        if (peer.Id == id)
                            return peer;
                    }
                    return null;
                },
                peer => peer.Id == id,
                "peer query snapshot",
                timeoutMs);
        }

#if NET8_0_OR_GREATER
        public static async Task<T> ReadSingleAsync<T>(IAsyncEnumerable<T> source, Action trigger)
        {
            await using var enumerator = source.GetAsyncEnumerator();
            trigger();
            Assert.True(await enumerator.MoveNextAsync(), "Expected one async item.");
            return enumerator.Current;
        }

        public static async Task<T> ReadSingleAsync<T>(IAsyncEnumerable<T> source, Predicate<T> predicate, Action trigger)
        {
            await using var enumerator = source.GetAsyncEnumerator();
            trigger();
            var deadline = DateTime.UtcNow.AddSeconds(5);
            while (DateTime.UtcNow < deadline)
            {
                Assert.True(await enumerator.MoveNextAsync(), "Expected one async item.");
                if (predicate(enumerator.Current))
                    return enumerator.Current;
            }
            throw new Xunit.Sdk.XunitException("Timed out waiting for matching async item.");
        }
#endif

        public static void AssertFailure(Result<Unit> result, StatusCode expectedStatus)
        {
            Assert.False(result.IsSuccess);
            Assert.Equal(expectedStatus, result.Info.StatusCode);
        }

        public static void AssertFailure<T>(Result<T> result, StatusCode expectedStatus)
        {
            Assert.False(result.IsSuccess);
            Assert.Equal(expectedStatus, result.Info.StatusCode);
        }

        public static void PoseEqual(Pose expected, Pose actual)
        {
            QuaternionEqual(expected.Orientation, actual.Orientation);
            VectorEqual(expected.Position, actual.Position);
        }

        public static void PosePositionEqual(Pose expected, Pose actual)
        {
            VectorEqual(expected.Position, actual.Position);
        }

        public static void GuiPanelSizeEqual(GuiPanelSize expected, GuiPanelSize actual)
        {
            Assert.Equal(expected.WidthMeters, actual.WidthMeters, 5);
            Assert.Equal(expected.HeightMeters, actual.HeightMeters, 5);
            Assert.Equal(expected.ContentScale, actual.ContentScale, 5);
        }

        public static void AppVolumeSizeEqual(AppVolumeSize expected, AppVolumeSize actual)
        {
            Assert.Equal(expected.WidthMeters, actual.WidthMeters, 5);
            Assert.Equal(expected.LengthMeters, actual.LengthMeters, 5);
            Assert.Equal(expected.HeightMeters, actual.HeightMeters, 5);
        }

        public static void AnchorEqual(GeoAnchorFrame expected, GeoAnchorFrame actual)
        {
            Assert.Equal(expected.WorldReferenceSystem, actual.WorldReferenceSystem);
            Assert.Equal(expected.Handedness, actual.Handedness);
            Vector3dEqual(expected.OriginWorld, actual.OriginWorld);
            Vector3dEqual(expected.AxisXWorld, actual.AxisXWorld);
            Vector3dEqual(expected.AxisYWorld, actual.AxisYWorld);
            Vector3dEqual(expected.AxisZWorld, actual.AxisZWorld);
        }

        public static void GestureRuleEqual(AppVolumeGestureMappingRule expected, AppVolumeGestureMappingRule actual)
        {
            Assert.Equal(expected.GestureKind, actual.GestureKind);
            Assert.Equal(expected.Enabled, actual.Enabled);
            Assert.Equal(expected.Precision, actual.Precision, 5);
            Assert.Equal(expected.AppScaleSensitivityMode, actual.AppScaleSensitivityMode);
            Assert.Equal(expected.TranslationAxes, actual.TranslationAxes);
            Assert.Equal(expected.RotationAxes, actual.RotationAxes);
            Assert.Equal(expected.EnableGesturesFromOutsideOfVolume, actual.EnableGesturesFromOutsideOfVolume);
            Assert.Equal(expected.AppTransformMapping, actual.AppTransformMapping);
            Assert.Equal(expected.ControllerRayStyle.LineStyle, actual.ControllerRayStyle.LineStyle);
            Assert.Equal(expected.ControllerRayStyle.ColorR, actual.ControllerRayStyle.ColorR);
            Assert.Equal(expected.ControllerRayStyle.ColorG, actual.ControllerRayStyle.ColorG);
            Assert.Equal(expected.ControllerRayStyle.ColorB, actual.ControllerRayStyle.ColorB);
            Assert.Equal(expected.ControllerRayStyle.ColorA, actual.ControllerRayStyle.ColorA);
        }

        public static AppVolumeGestureMappingRule CreateRule(int index)
        {
            return new AppVolumeGestureMappingRule
            {
                GestureKind = index % 2 == 0 ? GestureKind.SinglePointer6Dof : GestureKind.DualPointerTranslateDistance,
                Enabled = index % 3 != 0,
                Precision = 0.1f + index,
                AppScaleSensitivityMode = index % 2 == 0 ? AppScaleSensitivityMode.Constant : AppScaleSensitivityMode.InverseToAppScale,
                TranslationAxes = index % 2 == 0 ? AppVolumeAxisFlags.X | AppVolumeAxisFlags.Y : AppVolumeAxisFlags.All,
                RotationAxes = index % 2 == 0 ? AppVolumeAxisFlags.Z : AppVolumeAxisFlags.None,
                EnableGesturesFromOutsideOfVolume = index % 2 == 0,
                ControllerRayStyle = new AppVolumeControllerRayStyle(AppVolumeControllerRayLineStyle.Dotted, (byte)index, (byte)(index + 10), (byte)(index + 20), (byte)(index + 30)),
                AppTransformMapping = index % 2 == 0 ? GestureAppTransformMapping.AppPose : GestureAppTransformMapping.AppScale
            };
        }

        private static void VectorEqual(Vector3 expected, Vector3 actual)
        {
            Assert.Equal(expected.X, actual.X, 5);
            Assert.Equal(expected.Y, actual.Y, 5);
            Assert.Equal(expected.Z, actual.Z, 5);
        }

        private static void Vector3dEqual(Vector3d expected, Vector3d actual)
        {
            Assert.Equal(expected.X, actual.X, 10);
            Assert.Equal(expected.Y, actual.Y, 10);
            Assert.Equal(expected.Z, actual.Z, 10);
        }

        private static void QuaternionEqual(Quaternion expected, Quaternion actual)
        {
            Assert.Equal(expected.X, actual.X, 5);
            Assert.Equal(expected.Y, actual.Y, 5);
            Assert.Equal(expected.Z, actual.Z, 5);
            Assert.Equal(expected.W, actual.W, 5);
        }
    }
}
