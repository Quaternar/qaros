---
title: "StructureType enum"
sidebar_label: "StructureType"
description: "Native structure type tag of a Structure. Each extension class sets its own in its constructor."
---

# StructureType enum

Native structure type tag of a [`Structure`](Quaternar.Qaros.Structure.md). Each extension class sets its own
in its constructor.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L292)

```csharp
public enum StructureType
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="unknown"></a>`Unknown` | `0` |  |
| <a id="libraryinit"></a>`LibraryInit` | `1` |  |
| <a id="runtimeinit"></a>`RuntimeInit` | `4096` |  |
| <a id="runtimerejoininit"></a>`RuntimeRejoinInit` | `4098` |  |
| <a id="runtimeonboardinit"></a>`RuntimeOnboardInit` | `4099` |  |
| <a id="runtimeforgetinit"></a>`RuntimeForgetInit` | `4101` |  |
| <a id="runtimeonboardhostext"></a>`RuntimeOnboardHostExt` | `4102` |  |
| <a id="runtimeonboardcodeext"></a>`RuntimeOnboardCodeExt` | `4103` |  |
| <a id="runtimeonboardinviteext"></a>`RuntimeOnboardInviteExt` | `4104` |  |
| <a id="runtimeonboardpeeridext"></a>`RuntimeOnboardPeerIdExt` | `4105` |  |
| <a id="sessiongraphicsdeviceid"></a>`SessionGraphicsDeviceId` | `8196` |  |
| <a id="sessionrequestinviteinit"></a>`SessionRequestInviteInit` | `8197` |  |
| <a id="peerpresentation"></a>`PeerPresentation` | `8198` |  |
| <a id="sessioninvitetargetappinit"></a>`SessionInviteTargetAppInit` | `8199` |  |
| <a id="sessionrequestonboardingcodesinit"></a>`SessionRequestOnboardingCodesInit` | `8200` |  |
| <a id="renderingstreamsenderinit"></a>`RenderingStreamSenderInit` | `12288` |  |
| <a id="renderingbeginframe"></a>`RenderingBeginFrame` | `12289` |  |
| <a id="renderingendframe"></a>`RenderingEndFrame` | `12290` |  |
| <a id="renderingendframeviewoverridesext"></a>`RenderingEndFrameViewOverridesExt` | `12292` |  |
| <a id="videosenderinit"></a>`VideoSenderInit` | `12293` |  |
| <a id="streamd3d11paramsext"></a>`StreamD3D11ParamsExt` | `16384` |  |
| <a id="streamvulkanparamsext"></a>`StreamVulkanParamsExt` | `16386` |  |
| <a id="streamopengleglparamsext"></a>`StreamOpenGlEglParamsExt` | `16387` |  |
| <a id="streamopenglwglparamsext"></a>`StreamOpenGlWglParamsExt` | `16388` |  |
| <a id="streamopenglglxparamsext"></a>`StreamOpenGlGlxParamsExt` | `16389` |  |
| <a id="guipanelinit"></a>`GuiPanelInit` | `20481` |  |
| <a id="guipanelstreamidext"></a>`GuiPanelStreamIdExt` | `20482` |  |
| <a id="guipanelurlext"></a>`GuiPanelUrlExt` | `20483` |  |
| <a id="appvolumeinit"></a>`AppVolumeInit` | `21761` |  |
| <a id="appvolumegesturemappingrule"></a>`AppVolumeGestureMappingRule` | `21762` |  |
| <a id="appvolumegestureconfiguration"></a>`AppVolumeGestureConfiguration` | `21763` |  |
| <a id="appvolumegestureallowedpeersext"></a>`AppVolumeGestureAllowedPeersExt` | `21764` |  |
