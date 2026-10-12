<!--
  Hand-written intro of the C# API landing page (/api/qar-streaming-sharp/).
  scripts/run-docfx.js puts it under the page title and appends the generated
  type tables (grouped by areas.json) after it. Link types as `Quaternar.Qaros.<Type>.md`
  and guide pages as `/docs/...`.
-->

`Quaternar.Qaros` is the .NET binding of the QAROS streaming C API. It targets
netstandard2.0, netstandard2.1 and net8.0, loads the same native `qar-streaming-c`
library, and is the C# surface Unity uses too.

- **`Quaternar.Qaros`** (this reference) is the managed API: `IDisposable` handles, `Result<T>`
  return values, C# events and `...Async` variants. Every public type is listed below.
- **`Quaternar.Qaros.Mathematics`** holds `Vector3`, `Vector3d`, `Quaternion` and `Pose`, apart
  from the main namespace so they do not clash with the `UnityEngine` or `System.Numerics` types
  of the same names. Add `using Quaternar.Qaros.Mathematics;` where you use them.
- **`Quaternar.Qaros.Interop`** is the internal P/Invoke layer the managed API is built on. It is not
  part of the public surface and is not documented here.

Concepts, threading rules and usage snippets live in the
[Developer Guide](/docs/developer-guide/language-bindings), next to the C examples.

## Entry points

1. [`Library.Init`](Quaternar.Qaros.Library.md) loads the native library, once per process.
2. [`Runtime.Create`](Quaternar.Qaros.Runtime.md) creates the runtime. Its `Onboard` joins a session
   for the first time and returns an [`OnboardSession`](Quaternar.Qaros.OnboardSession.md); `Rejoin`
   returns to a session joined before.
3. A [`Session`](Quaternar.Qaros.Session.md) exposes one control per area:
   [`Peers`](Quaternar.Qaros.PeerControl.md), [`GuiPanels`](Quaternar.Qaros.GuiPanelControl.md),
   [`AppVolumes`](Quaternar.Qaros.AppVolumeControl.md), [`RenderSenders`](Quaternar.Qaros.RenderSenderControl.md)
   and [`PrimarylessVideoSenders`](Quaternar.Qaros.PrimarylessVideoSenderControl.md).
4. Calls return a [`Result<T>`](Quaternar.Qaros.Result-1.md): check `IsSuccess`, or use `ValueOrThrow()`.
   Failures carry a [`StatusCode`](Quaternar.Qaros.StatusCode.md).
