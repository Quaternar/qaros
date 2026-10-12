using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>One app volume update; <see cref="Volume"/> is a snapshot, safe to keep.</summary>
    public sealed class AppVolumeUpdatedEventArgs : EventArgs
    {
        public AppVolumeUpdatedEventArgs(AppVolume volume) { Volume = volume; }

        public AppVolume Volume { get; }
    }

    public sealed class AppVolumeGestureEventArgs : EventArgs
    {
        public AppVolumeGestureEventArgs(AppVolumeGestureEvent gestureEvent) { GestureEvent = gestureEvent; }

        public AppVolumeGestureEvent GestureEvent { get; }
    }
}

