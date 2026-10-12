---
title: "OpenGlContextCallbacks struct"
sidebar_label: "OpenGlContextCallbacks"
description: "Raw C function pointers overriding how the library binds an OpenGL context; see QarOpenGlContextCallbacks in types.h. Every one is optional and called on the library's internal graphics thread; zero u"
---

# OpenGlContextCallbacks struct

Raw C function pointers overriding how the library binds an OpenGL
context; see `QarOpenGlContextCallbacks` in types.h. Every one is
optional and called on the library's internal graphics thread; zero uses
the library's own platform implementation for that step.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L791)

```csharp
public readonly struct OpenGlContextCallbacks
```

## Remarks

A pointer obtained with `Marshal.GetFunctionPointerForDelegate` is valid only while
its delegate is reachable: keep the delegates alive for as long as the sender exists.

## Constructors

### OpenGlContextCallbacks(nint, nint, nint, nint, nint) {#ctor}

```csharp
public OpenGlContextCallbacks(nint userState, nint getProcAddress, nint makeCurrent, nint captureCurrent, nint restoreCurrent)
```

**Parameters**

| Name | Type | Description |
|---|---|---|
| `userState` | [nint](https://learn.microsoft.com/dotnet/api/system.intptr) |  |
| `getProcAddress` | [nint](https://learn.microsoft.com/dotnet/api/system.intptr) |  |
| `makeCurrent` | [nint](https://learn.microsoft.com/dotnet/api/system.intptr) |  |
| `captureCurrent` | [nint](https://learn.microsoft.com/dotnet/api/system.intptr) |  |
| `restoreCurrent` | [nint](https://learn.microsoft.com/dotnet/api/system.intptr) |  |

## Properties

| Member | Description |
|---|---|
| [UserState](#userstate) | Passed unchanged to every callback; may be zero. |
| [GetProcAddress](#getprocaddress) | `void* (void* userState, const char* name)`: resolves a GL entry point, as eglGetProcAddress, wglGetProcAddress or glXGetProcAddressARB do. Zero uses the platform loader. |
| [MakeCurrent](#makecurrent) | `bool (void* userState)`: binds the extension's context to the calling thread with the handles passed in the extension. |
| [CaptureCurrent](#capturecurrent) | `QarOpenGlContextThreadState (void* userState)`: reads the thread's current binding before [`OpenGlContextCallbacks.MakeCurrent`](#makecurrent) replaces it. |
| [RestoreCurrent](#restorecurrent) | `bool (void* userState, const QarOpenGlContextThreadState* state)`: re-binds what [`OpenGlContextCallbacks.CaptureCurrent`](#capturecurrent) read once the library is done. When the state's `has_context` is false the thread had nothing bound, so bind nothing. |

### UserState {#userstate}

Passed unchanged to every callback; may be zero.

```csharp
public nint UserState { get; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)

### GetProcAddress {#getprocaddress}

`void* (void* userState, const char* name)`: resolves a GL entry point, as
eglGetProcAddress, wglGetProcAddress or glXGetProcAddressARB do. Zero uses the
platform loader.

```csharp
public nint GetProcAddress { get; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)

### MakeCurrent {#makecurrent}

`bool (void* userState)`: binds the extension's context to the calling thread with
the handles passed in the extension.

```csharp
public nint MakeCurrent { get; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)

### CaptureCurrent {#capturecurrent}

`QarOpenGlContextThreadState (void* userState)`: reads the thread's current binding
before [`OpenGlContextCallbacks.MakeCurrent`](#makecurrent) replaces it.

```csharp
public nint CaptureCurrent { get; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)

### RestoreCurrent {#restorecurrent}

`bool (void* userState, const QarOpenGlContextThreadState* state)`: re-binds what
[`OpenGlContextCallbacks.CaptureCurrent`](#capturecurrent) read once the library is done. When the state's
`has_context` is false the thread had nothing bound, so bind nothing.

```csharp
public nint RestoreCurrent { get; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)
