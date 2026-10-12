---
title: "StreamParamsOpenGlGlx class"
sidebar_label: "StreamParamsOpenGlGlx"
description: "Extension: adopt an existing GLX context."
---

# StreamParamsOpenGlGlx class

Extension: adopt an existing GLX context.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L883)

```csharp
public sealed class StreamParamsOpenGlGlx : StreamParamsOpenGl
```

**Inherits:** [Structure](Quaternar.Qaros.Structure.md) → [StreamParamsOpenGl](Quaternar.Qaros.StreamParamsOpenGl.md) → StreamParamsOpenGlGlx

## Constructors

### StreamParamsOpenGlGlx() {#ctor}

```csharp
public StreamParamsOpenGlGlx()
```

## Properties

### Display {#display}

X11 Display*.

```csharp
public nint Display { get; set; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)

### Drawable {#drawable}

GLXDrawable.

```csharp
public nuint Drawable { get; set; }
```

**Value:** [nuint](https://learn.microsoft.com/dotnet/api/system.uintptr)
