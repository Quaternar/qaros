## Joining as a local app

`OnboardMethod::LocalApp` waits for the Allow in the hub, at most `approval_timeout` (`None`: the hub's default). Progress reports arrive on the channel while the call blocks.

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:local_app}}
```

## Rejoin, else onboard

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:obtain_session}}
```

## Sibling onboarding: the onboarded instance

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:mint_invite}}
```

## Sibling onboarding: the sibling instance

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:onboard_sibling}}
```

## One-time codes

One-time codes are raw only: call `qar_session_request_onboarding_codes` through `library.raw_api()` with the session's `as_raw()`.

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:one_time_codes}}
```

## Choosing the GPU: chaining the adapter

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:onboard_on_adapter}}
```

## Choosing the GPU: reading the session's adapter

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:session_adapter}}
```

## Platforms without local enrollment

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:launch_invite}}
```

## Teardown: destroy vs. forget

```rust
{{#include ../tests/guide/onboarding_and_sessions.rs:teardown}}
```
