---
title: "StreamParamsVulkan class"
sidebar_label: "StreamParamsVulkan"
description: "Extension: adopt an existing Vulkan device. Requires a library built with QAR_ENABLE_VULKAN."
---

# StreamParamsVulkan class

Extension: adopt an existing Vulkan device. Requires a library built with QAR_ENABLE_VULKAN.

**Namespace:** [Quaternar.Qaros](index.md) · **Assembly:** Quaternar.Qaros.dll · **Source:** [QarTypes.cs](https://github.com/Quaternar/qaros/blob/a516d778e65141d0f99ef705a96ec3ea8a9360ba/qar-streaming-sharp/src/QarStreaming/QarTypes.cs#L898)

```csharp
public sealed class StreamParamsVulkan : Structure
```

**Inherits:** [Structure](Quaternar.Qaros.Structure.md) → StreamParamsVulkan

## Constructors

### StreamParamsVulkan() {#ctor}

```csharp
public StreamParamsVulkan()
```

## Properties

### PhysicalDevice {#physicaldevice}

VkPhysicalDevice.

```csharp
public nint PhysicalDevice { get; set; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)

### Device {#device}

VkDevice.

```csharp
public nint Device { get; set; }
```

**Value:** [nint](https://learn.microsoft.com/dotnet/api/system.intptr)
