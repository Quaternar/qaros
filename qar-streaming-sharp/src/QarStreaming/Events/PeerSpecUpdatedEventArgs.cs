using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Qar.Native;

namespace Qar
{
    public sealed class PeerSpecUpdatedEventArgs : EventArgs
    {
        public PeerSpecUpdatedEventArgs(PeerSpec peer) { Peer = peer; }

        public PeerSpec Peer { get; }
    }
}

