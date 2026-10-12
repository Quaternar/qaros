---
title: "StreamParamsOpenGlEgl class"
sidebar_label: "StreamParamsOpenGlEgl"
description: "Extension: adopt an existing EGL context."
---

# StreamParamsOpenGlEgl class

Extension: adopt an existing EGL context.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L853)

```csharp
public sealed class StreamParamsOpenGlEgl : StreamParamsOpenGl
```

**Inherits:** [Structure](Quaternar.Qaros.Structure.md) → [StreamParamsOpenGl](Quaternar.Qaros.StreamParamsOpenGl.md) → StreamParamsOpenGlEgl

## Constructors

### StreamParamsOpenGlEgl() {#ctor}

```csharp
public StreamParamsOpenGlEgl()
```

## Properties

| Member | Description |
|---|---|
| [Display](#display) | EGLDisplay. |
| [DrawSurface](#drawsurface) | EGLSurface used for drawing. |
| [ReadSurface](#readsurface) | EGLSurface used for reading. |

### Display {#display}

EGLDisplay.

```csharp
public nint Display { get; set; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)

### DrawSurface {#drawsurface}

EGLSurface used for drawing.

```csharp
public nuint DrawSurface { get; set; }
```

**Value:** [nuint](https://learn.microsoft.com/dotnet/api/system.uintptr)

### ReadSurface {#readsurface}

EGLSurface used for reading.

```csharp
public nuint ReadSurface { get; set; }
```

**Value:** [nuint](https://learn.microsoft.com/dotnet/api/system.uintptr)
