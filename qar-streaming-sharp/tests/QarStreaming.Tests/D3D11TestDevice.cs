#if NET8_0_OR_GREATER
using System;
using Vortice.Direct3D;
using Vortice.Direct3D11;
using Vortice.DXGI;
using static Vortice.Direct3D11.D3D11;

namespace Quaternar.Qaros.Tests
{
    /// <summary>
    /// An application D3D11 device. It is created on the adapter it is given, the session's
    /// (<see cref="Session.GetGraphicsDeviceId"/>): the sender's textures cannot be shared with a
    /// device on another GPU, and on a machine with several the default adapter is often not the
    /// session's.
    /// </summary>
    internal sealed class D3D11TestDevice : IDisposable
    {
        private D3D11TestDevice(ID3D11Device device, ID3D11DeviceContext context)
        {
            Device = device;
            Context = context;
        }

        public ID3D11Device Device { get; }

        public ID3D11DeviceContext Context { get; }

        /// <param name="adapter">The GPU to create the device on; null takes the default adapter, or WARP without one.</param>
        public static D3D11TestDevice Create(GraphicsDeviceId? adapter)
        {
            var featureLevels = Array.Empty<FeatureLevel>();
            ID3D11Device device;
            ID3D11DeviceContext context;
            if (adapter != null)
            {
                using var dxgiAdapter = FindAdapter(adapter);
                D3D11CreateDevice(dxgiAdapter, DriverType.Unknown, DeviceCreationFlags.None, featureLevels, out device, out context).CheckError();
                return new D3D11TestDevice(device, context);
            }
            var result = D3D11CreateDevice(
                (IDXGIAdapter?)null,
                DriverType.Hardware,
                DeviceCreationFlags.None,
                featureLevels,
                out device,
                out context);
            if (result.Failure)
            {
                result = D3D11CreateDevice(
                    (IDXGIAdapter?)null,
                    DriverType.Warp,
                    DeviceCreationFlags.None,
                    featureLevels,
                    out device,
                    out context);
            }
            result.CheckError();
            return new D3D11TestDevice(device, context);
        }

        public void Dispose()
        {
            Context.Dispose();
            Device.Dispose();
        }

        private static IDXGIAdapter1 FindAdapter(GraphicsDeviceId adapter)
        {
            if (adapter.IdType != GpuDeviceIdType.Luid)
                throw new InvalidOperationException("A D3D11 device is chosen by the adapter's LUID, and the session reports none.");
            using var factory = DXGI.CreateDXGIFactory1<IDXGIFactory1>();
            for (uint index = 0; factory.EnumAdapters1(index, out var candidate).Success; ++index)
            {
                var luid = candidate.Description1.Luid;
                if ((((long)luid.HighPart << 32) | luid.LowPart) == adapter.Luid)
                    return candidate;
                candidate.Dispose();
            }
            throw new InvalidOperationException("No DXGI adapter has the session's LUID.");
        }
    }
}
#endif
