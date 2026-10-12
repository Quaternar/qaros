using System;
using System.Runtime.InteropServices;

namespace Qar.Native
{
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    internal delegate void ProgressCallback(ActionSeverity severity, float percent, IntPtr message, IntPtr userState);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    internal delegate void RuntimeOnboardResultCallback(ResultNative status, IntPtr onboardingId, IntPtr session, IntPtr userState);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    internal delegate void SessionRequestInviteResultCallback(ResultNative status, IntPtr invite, IntPtr userState);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    internal delegate void RenderRequestCallback(IntPtr request, IntPtr userState);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    internal delegate void PeerUpdateCallback(IntPtr handle, IntPtr userState);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    internal delegate void RenderSenderCreateCallback(ResultNative status, IntPtr outStream, IntPtr userState);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    internal delegate void RenderSenderBeginFrameCallback(ResultNative status, IntPtr frameInfo, IntPtr userState);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    internal delegate void GuiPanelUpdateCallback(IntPtr handle, IntPtr userState);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    internal delegate void AppVolumeUpdateCallback(IntPtr handle, IntPtr userState);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    internal delegate void AppVolumeGestureEventCallback(IntPtr gestureEvent, IntPtr userState);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    internal delegate void GuiPanelGestureEventCallback(IntPtr gesture, IntPtr userState);

    /// <remarks>Shape of both qar_target_app_invite_result_callback_t and qar_target_app_kill_result_callback_t.</remarks>
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    internal delegate void TargetAppResultCallback(ResultNative status, IntPtr peerId, IntPtr userState);
}
