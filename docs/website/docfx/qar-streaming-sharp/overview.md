<!--
  Hand-written intro of the C# API landing page (/api/qar-streaming-sharp/).
  scripts/run-docfx.js puts it under the page title and appends the generated
  type tables (grouped by areas.json) after it. Link types as `Qar.<Type>.md`
  and guide pages as `/docs/...`.
-->

`QarStreamingSharp` is the .NET binding of the QAROS streaming C API. It targets
netstandard2.0, netstandard2.1 and net8.0, loads the same native `qar-streaming-c`
library, and is the C# surface Unity uses too.

- **`Qar`** (this reference) is the managed API: `IDisposable` handles, `Result<T>`
  return values, C# events and `...Async` variants. Every public type is listed below.
- **`Qar.Native`** is the internal P/Invoke layer the managed API is built on. It is not
  part of the public surface and is not documented here.

Concepts, threading rules and usage snippets live in the
[Developer Guide](/docs/developer-guide/language-bindings), next to the C examples.

## Entry points

1. [`Library.Init`](Qar.Library.md) loads the native library, once per process.
2. [`Runtime.Create`](Qar.Runtime.md) creates the runtime. Its `Onboard` joins a session
   for the first time and returns an [`OnboardSession`](Qar.OnboardSession.md); `Rejoin`
   returns to a session joined before.
3. A [`Session`](Qar.Session.md) exposes one control per area:
   [`Peers`](Qar.PeerControl.md), [`GuiPanels`](Qar.GuiPanelControl.md),
   [`AppVolumes`](Qar.AppVolumeControl.md), [`RenderSenders`](Qar.RenderSenderControl.md)
   and [`PrimarylessVideoSenders`](Qar.PrimarylessVideoSenderControl.md).
4. Calls return a [`Result<T>`](Qar.Result-1.md): check `IsSuccess`, or use `ValueOrThrow()`.
   Failures carry a [`StatusCode`](Qar.StatusCode.md).
