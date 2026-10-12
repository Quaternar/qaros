using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Quaternar.Qaros.Interop;

namespace Quaternar.Qaros
{
    /// <summary>One GUI panel update; <see cref="Panel"/> is a snapshot, safe to keep.</summary>
    public sealed class GuiPanelUpdatedEventArgs : EventArgs
    {
        public GuiPanelUpdatedEventArgs(GuiPanel panel) { Panel = panel; }

        public GuiPanel Panel { get; }
    }

    public sealed class GuiPanelGestureEventArgs : EventArgs
    {
        public GuiPanelGestureEventArgs(GuiPanelGestureEvent gestureEvent) { GestureEvent = gestureEvent; }

        public GuiPanelGestureEvent GestureEvent { get; }
    }
}

