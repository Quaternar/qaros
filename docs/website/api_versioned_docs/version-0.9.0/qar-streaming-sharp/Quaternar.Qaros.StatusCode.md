---
title: "StatusCode enum"
sidebar_label: "StatusCode"
description: "Status codes the C API exposes (QarStatusCode), with the same values. Any internal code outside this set reaches callers as StatusCode.Unclassified."
---

# StatusCode enum

Status codes the C API exposes (QarStatusCode), with the same values.
Any internal code outside this set reaches callers as
[`StatusCode.Unclassified`](#unclassified).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarResult.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarResult.cs#L16)

```csharp
public enum StatusCode
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="success"></a>`Success` | `0` |  |
| <a id="unclassified"></a>`Unclassified` | `1` |  |
| <a id="notimplemented"></a>`NotImplemented` | `4` |  |
| <a id="argumentnotsupported"></a>`ArgumentNotSupported` | `5` |  |
| <a id="timeout"></a>`Timeout` | `6` |  |
| <a id="logicerror"></a>`LogicError` | `7` | A programming error, such as an invalid handle or a missing required argument; the binding also reports a native call that succeeded without its output this way. |
| <a id="guipanelchangenotpermitted"></a>`GuiPanelChangeNotPermitted` | `304` |  |
| <a id="guipanelinvalidid"></a>`GuiPanelInvalidId` | `305` |  |
| <a id="appvolumeinvalidid"></a>`AppVolumeInvalidId` | `325` |  |
| <a id="appvolumegestureconfigurationinvalid"></a>`AppVolumeGestureConfigurationInvalid` | `326` | A gesture configuration was refused because a rule in it could never work. The volume keeps its previous configuration; the message names the rule. |
| <a id="appvolumegesturemodeunknown"></a>`AppVolumeGestureModeUnknown` | `327` | A gesture mode was named that the volume does not define. Nothing was changed. |
| <a id="mediastreamingstreamispaused"></a>`MediaStreamingStreamIsPaused` | `602` | The receiver paused this stream (its display is not visible); keep the sender and try again later. |
| <a id="mediastreamingstreamisreconnecting"></a>`MediaStreamingStreamIsReconnecting` | `603` |  |
| <a id="mediastreamingnoactivetransfers"></a>`MediaStreamingNoActiveTransfers` | `604` |  |
| <a id="mediastreamingdestinationreservationfailed"></a>`MediaStreamingDestinationReservationFailed` | `605` |  |
| <a id="mediastreamingsendpipelinefailed"></a>`MediaStreamingSendPipelineFailed` | `606` |  |
| <a id="mediastreaminglayoutchangefailed"></a>`MediaStreamingLayoutChangeFailed` | `607` |  |
| <a id="renderingproducerunabletodobeginframe"></a>`RenderingProducerUnableToDoBeginFrame` | `803` |  |
| <a id="renderingproducerstreamisclosed"></a>`RenderingProducerStreamIsClosed` | `804` |  |
| <a id="renderingproducerappvolumeunresolved"></a>`RenderingProducerAppVolumeUnresolved` | `805` | The render sender's app volume does not resolve - it is not known to this session yet, or it was removed - so app content space is undefined and no frame was begun. Nothing is handed out in room space instead. Recoverable: keep calling begin-frame, it succeeds once the volume resolves. A volume that was removed never does; recreate it. |
| <a id="pakeerror"></a>`PakeError` | `1600` | Code-based onboarding handshake rejected, expired or timed out. Re-check the code shown on the hub and retry. |
| <a id="onboardingsessionnotfound"></a>`OnboardingSessionNotFound` | `1800` | No persisted state for the onboarding id; fall back to a full onboard. |
| <a id="onboardingfailed"></a>`OnboardingFailed` | `1801` | Generic onboarding failure; the message has details. |
| <a id="onboardinghubunreachable"></a>`OnboardingHubUnreachable` | `1802` | Discovery could not reach a hub; verify the hub is running (and the host/port) and retry. |
| <a id="onboardingsessionstillactive"></a>`OnboardingSessionStillActive` | `1803` | Forget was called while the slot still has an active session; dispose the session first. |
