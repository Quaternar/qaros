using System;

namespace Quaternar.Qaros
{
    public sealed class RenderStreamRequestEventArgs : EventArgs
    {
        public RenderStreamRequestEventArgs(RenderStreamRequest request)
        {
            Request = request;
        }

        public RenderStreamRequest Request { get; }
    }
}
