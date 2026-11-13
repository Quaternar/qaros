# qar-streaming-cpp

A header-only C++17 wrapper over the QAROS C API (`qar-streaming-c/include/qar_streaming.h`):
owning handles, result types, builders and optional asio completion tokens. It has no binding of
its own: it only calls the `qar_*` functions of the C header, so it works with the dynamic loader
and with a static link alike.

## Using it from CMake

```cmake
project(MyApp LANGUAGES CXX)            # C is not needed
add_subdirectory(qaros/qar-streaming-c)
add_subdirectory(qaros/qar-streaming-cpp)
target_link_libraries(my_app PRIVATE qar-streaming-cpp)
```

- `qar-streaming-cpp` needs C++17 (`cxx_std_17`) and links `qar-streaming-c-headers`.
- `qar-streaming-c-headers` sets no C standard and needs no C language. It adds the compile
  definition `QAR_ENABLE_DYNAMIC_LOADING` to every target that links it: the program loads the
  runtime with `qar_library_load`, and exactly one of its source files expands
  `QAR_IMPLEMENT_DYNAMIC_LOADING()`. A program that links the runtime's import library instead
  includes the header without this target.
- The C examples build only when qaros is the top-level project. `QAROS_BUILD_EXAMPLES` turns them
  on or off; the earlier variable `BUILD_CORE_EXAMPLES`, when set before the first configure, still
  decides its default.
- Asio completion tokens: define `QAR_STREAMING_CPP_WITH_ASIO` (standalone asio) or
  `QAR_STREAMING_CPP_WITH_BOOST_ASIO` on the consuming target and link asio.
- `QAR_STREAMING_CPP_C_HEADER` names another C header to include instead of `<qar_streaming.h>`.

## Compilers

The headers use no alternative tokens (`not`, `and`, `or`), so MSVC compiles them in C++17 and
C++20 with or without `/permissive-`. They call no C function the header marks deprecated and
compile warning-free with `/W4 /WX`. `python ci/run.py cpp` checks both (MSVC, Windows).

## Joining a session

```cpp
QAR_IMPLEMENT_DYNAMIC_LOADING() // in exactly one source file

qaros::OnboardBuilder builder;
builder.WithLocalApp()                      // a source app on the PC of the QAROS hub, no code
    .WithAppTag("left.viewer.example.com")  // optional: tells instances of one app apart
    .WithDisplayName("My viewer");
qaros::Result<qaros::OnboardSession> joined = runtime.Onboard(builder);
```

- `WithLocalApp(approval_timeout_ms)` joins the hub installed on this PC
  (`QarOnboardLocalAppExt`). The hub asks the user once to approve the app; later starts join
  silently. Nothing is persisted: the onboarding id is zero. Windows only.
- `WithInvite(invite)` joins as a co-process with an invite from the app that started it
  (`qar_session_request_onboarding_invite`), or with the launch invite QAROS hands an app it starts.
- `WithCode` and `WithPeerId` remain for applications that use them; the C API marks code
  onboarding and caller-chosen peer ids deprecated.

## Migrating from Rejoin and Forget

`Runtime::Rejoin`, `Runtime::RejoinAsync`, `RejoinBuilder` and `Runtime::Forget` are removed, with
the type aliases `native::RejoinInit` and `native::ForgetInit`. They wrapped `qar_runtime_rejoin`,
`qar_runtime_rejoin_async` and `qar_runtime_forget`, which the C header marks deprecated: only the
deprecated code onboarding persists an identity slot to rejoin or forget.

| Before | Now |
|---|---|
| `Onboard(builder.WithCode(code))` once, then `Rejoin(RejoinBuilder(id))` on every start | `Onboard(builder.WithLocalApp())` on every start; the hub remembers the approval and, per app tag, the peer id |
| A co-process paired with its own code | The parent calls `qar_session_request_onboarding_invite`; the co-process onboards with `WithInvite` |
| `Forget(id)` to erase the stored pairing | Nothing to erase: a local app and a co-process store nothing. The user revokes a local app's approval in the QAROS Visualizer |
| `PeerSpec::AppState` (`qaros::AppState`) | `PeerSpec::Presence` (`qaros::PeerPresence`): pick a peer that is `Online`, or `Unknown` while the hub has not reconciled it |

`PeerSpec::AppState` and the enum `qaros::AppState` are removed with them: the C header marks
`QarAppState` and `qar_peer_spec_get_app_state` deprecated in favour of the presence. A program
that still needs a deprecated call calls the C function itself.
