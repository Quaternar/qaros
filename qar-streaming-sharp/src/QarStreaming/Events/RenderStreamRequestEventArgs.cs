using System;

namespace Qar
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
