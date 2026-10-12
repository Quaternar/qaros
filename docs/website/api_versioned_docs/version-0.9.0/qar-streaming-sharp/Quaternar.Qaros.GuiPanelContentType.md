---
title: "GuiPanelContentType enum"
sidebar_label: "GuiPanelContentType"
description: "What a GUI panel shows. Fixed at creation: GuiPanelControl.NavigateToUri on a stream panel fails with StatusCode.GuiPanelChangeNotPermitted."
---

# GuiPanelContentType enum

What a GUI panel shows. Fixed at creation: [`GuiPanelControl.NavigateToUri`](Quaternar.Qaros.GuiPanelControl.md#navigatetouri) on a
stream panel fails with [`StatusCode.GuiPanelChangeNotPermitted`](Quaternar.Qaros.StatusCode.md#guipanelchangenotpermitted).

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L343)

```csharp
public enum GuiPanelContentType
```

## Values

| Name | Value | Description |
|---|---|---|
| <a id="websiteuri"></a>`WebsiteUri` | `0` |  |
| <a id="streamid"></a>`StreamId` | `1` | The video of a [`PrimarylessVideoSender`](Quaternar.Qaros.PrimarylessVideoSender.md) owned by the same session. |
