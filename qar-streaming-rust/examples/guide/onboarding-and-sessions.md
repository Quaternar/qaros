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

## Rejoin, else onboard

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:obtain_session}}
```

## Co-process

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:co_process}}
```

## Remote app: through a member hub

`MemberHubInvitationConfig::app_grant_tag` chains the grant; `OnboardExtensions::remote_app_grant` redeems it next to `OnboardMethod::LocalApp`. The member-hub calls exist where the bindings declare them (cfg `qaros_member_hubs`).

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

## Re-enrolling a slot

`OnboardExtensions::onboarding_id` sets `QarOnboardInit::onboarding_id`. The zero id, and a local app, are refused with `Error::InvalidArgument` before the call.

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:re_enroll}}
```

## Onboarding without blocking

`Runtime::start_onboard` (and `start_rejoin`) returns a `PendingOnboard`; `poll` never blocks, `wait` does, `cancel_handle` cancels from any thread.

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:onboard_async}}
```

## Platforms without local enrollment

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:launch_invite}}
```

## When the hub's session changes

`Session::subscribe_lifecycle` delivers the current state first, then every change, on a library thread; `Session::lifecycle` reads the current one. Dropping the returned `Subscription` ends it.

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:session_lifecycle}}
```

## Teardown: destroy vs. forget

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:teardown}}
```
