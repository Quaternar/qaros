## Joining as a local app

`OnboardMethod::LocalApp` waits for the Allow in the hub, at most `approval_timeout` (`None`: the hub's default). Progress reports arrive on the channel while the call blocks.

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:local_app}}
```

## App tags

`app_tag` is chained after the mode extension. An invalid tag fails with `StatusCode::ARGUMENT_NOT_SUPPORTED`; an interior NUL fails with `Error::InvalidArgument` before the call.

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:app_tag}}
```

## Co-process

Request the invite once per session and hand the same bytes to every start of the companion. Its `onboard` retries while your app or the hub is away: bound it with `BlockingOptions::cancel`.

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:co_process}}
```

## Remote app: through a member hub

`MemberHubInvitationConfig::app_grant_tag` chains the grant; `OnboardExtensions::remote_app_grant` redeems it next to `OnboardMethod::LocalApp`.

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:remote_app_grant}}
```

## Choosing the GPU: chaining the adapter

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:onboard_on_adapter}}
```

## Choosing the GPU: reading the session's adapter

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:session_adapter}}
```

## Onboarding without blocking

`Runtime::start_onboard` returns a `PendingOnboard`; `poll` never blocks, `wait` does, `cancel_handle` cancels from any thread.

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:onboard_async}}
```

## When the hub's session changes

`Session::subscribe_lifecycle` delivers the current state first, then every change, on a library thread; `Session::lifecycle` reads the current one. Dropping the returned `Subscription` ends it.

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:session_lifecycle}}
```
