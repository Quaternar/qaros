---
title: "StreamParamsOpenGlWgl class"
sidebar_label: "StreamParamsOpenGlWgl"
description: "Extension: adopt an existing WGL context."
---

# StreamParamsOpenGlWgl class

Extension: adopt an existing WGL context.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L871)

```csharp
public sealed class StreamParamsOpenGlWgl : StreamParamsOpenGl
```

**Inherits:** [Structure](Quaternar.Qaros.Structure.md) → [StreamParamsOpenGl](Quaternar.Qaros.StreamParamsOpenGl.md) → StreamParamsOpenGlWgl

## Constructors

### StreamParamsOpenGlWgl() {#ctor}

```csharp
public StreamParamsOpenGlWgl()
```

## Properties

### DeviceContext {#devicecontext}

HDC the context was created against.

```csharp
public nint DeviceContext { get; set; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)
