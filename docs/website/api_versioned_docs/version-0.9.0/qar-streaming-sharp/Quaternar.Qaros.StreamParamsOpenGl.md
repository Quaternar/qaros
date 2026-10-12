---
title: "StreamParamsOpenGl class"
sidebar_label: "StreamParamsOpenGl"
description: "Base of the three OpenGL stream-parameter extensions. Pick the one matching the context the application owns; requires a library built with QAR_ENABLE_OPENGL."
---

# StreamParamsOpenGl class

Base of the three OpenGL stream-parameter extensions. Pick the one matching
the context the application owns; requires a library built with
QAR_ENABLE_OPENGL.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L838)

```csharp
public abstract class StreamParamsOpenGl : Structure
```

**Inherits:** [Structure](Quaternar.Qaros.Structure.md) → StreamParamsOpenGl  
**Derived:** [StreamParamsOpenGlEgl](Quaternar.Qaros.StreamParamsOpenGlEgl.md), [StreamParamsOpenGlGlx](Quaternar.Qaros.StreamParamsOpenGlGlx.md), [StreamParamsOpenGlWgl](Quaternar.Qaros.StreamParamsOpenGlWgl.md)

## Properties

### Context {#context}

Required. EGLContext, HGLRC or GLXContext; a null context is rejected.

```csharp
public nint Context { get; set; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)

### Callbacks {#callbacks}

Optional overrides of how the library binds the context; the default uses the library's own.

```csharp
public OpenGlContextCallbacks Callbacks { get; set; }
```

**Value:** [OpenGlContextCallbacks](Quaternar.Qaros.OpenGlContextCallbacks.md)
