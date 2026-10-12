#if NET8_0_OR_GREATER
using System;
using Vortice.Direct3D;
using Vortice.Direct3D11;
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

        public static D3D11TestDevice Create()
        {
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

        public void Dispose()
        {
            Context.Dispose();
            Device.Dispose();
        }
    }
}
#endif
