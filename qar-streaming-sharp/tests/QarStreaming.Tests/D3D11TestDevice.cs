#if NET8_0_OR_GREATER
using System;
using Vortice.Direct3D;
using Vortice.Direct3D11;
using Vortice.DXGI;
using static Vortice.Direct3D11.D3D11;

namespace Quaternar.Qaros.Tests
{
    internal sealed class D3D11TestDevice : IDisposable
    {
        private D3D11TestDevice(ID3D11Device device, ID3D11DeviceContext context)
        {
            Device = device;
            Context = context;
        }

        public ID3D11Device Device { get; }

        public ID3D11DeviceContext Context { get; }

        /// <summary>
        /// A device on the GPU the session runs on (<see cref="Session.GetGraphicsDeviceId"/>), as an
        /// application creates it: the sender refuses a device on another GPU, and a machine with an
        /// integrated and a discrete GPU may make either one the default adapter. Null takes the
        /// default adapter.
        /// </summary>
        public static D3D11TestDevice Create(GraphicsDeviceId? adapter)
        {
            if (adapter != null)
                return CreateOnAdapter(adapter);
            var featureLevels = Array.Empty<FeatureLevel>();
            ID3D11Device device;
            ID3D11DeviceContext context;
            var result = D3D11CreateDevice(
                (Vortice.DXGI.IDXGIAdapter?)null,
                DriverType.Hardware,
                DeviceCreationFlags.None,
                featureLevels,
                out device,
                out context);
            if (result.Failure)
            {
                result = D3D11CreateDevice(
                    (Vortice.DXGI.IDXGIAdapter?)null,
                    DriverType.Warp,
                    DeviceCreationFlags.None,
                    featureLevels,
                    out device,
                    out context);
            }
            result.CheckError();
            return new D3D11TestDevice(device, context);
        }

        private static D3D11TestDevice CreateOnAdapter(GraphicsDeviceId adapter)
        {
            if (adapter.IdType != GpuDeviceIdType.Luid)
                throw new NotSupportedException("The test devices select an adapter by LUID only.");
            using var factory = DXGI.CreateDXGIFactory1<IDXGIFactory1>();
            for (uint index = 0; factory.EnumAdapters1(index, out IDXGIAdapter1? candidate).Success; index++)
            {
                using (candidate)
                {
                    if ((long)candidate!.Description1.Luid != adapter.Luid)
                        continue;
                    // An explicit adapter takes DriverType.Unknown.
                    D3D11CreateDevice(
                        candidate,
                        DriverType.Unknown,
                        DeviceCreationFlags.None,
                        Array.Empty<FeatureLevel>(),
                        out ID3D11Device device,
                        out ID3D11DeviceContext context).CheckError();
                    return new D3D11TestDevice(device, context);
                }
            }
            throw new InvalidOperationException($"No DXGI adapter with LUID 0x{adapter.Luid:x16}.");
        }

        public void Dispose()
        {
            Context.Dispose();
            Device.Dispose();
        }
    }
}
#endif
