---
sidebar_position: 4
title: Working with Peers
description: Enumerate session participants, read their presentation metadata, and react to peers joining and leaving.
---

# Working with Peers

Every participant in the session — devices and applications alike — is a **peer**. The peer API lets your application answer "who is here?", present itself well, and react to arrivals and departures. (Concept: [The QAROS World](/docs/user-guide/qaros-world#devices-and-peers).)

## Presenting your application

Your own peer's metadata is set at onboarding time via `QarPeerPresentation` (display name, app version, custom info string) and can be updated live:

```c
qar_peer_update_display_name(session, "My CAD Tool — Assembly B");
```

The `app_custom_peer_info` field is yours: put a JSON blob or tag there to coordinate between instances of your own application across the session.

## Enumerating peers

Count-then-fetch, like every enumeration in the API:

```c
size_t count = 0;
qar_result_log_if_error(qar_query_peer_specs_count(session, &count));

QarPeerSpec** specs = malloc(count * sizeof(*specs));
qar_result_log_if_error(
    qar_query_peer_specs(session, specs, count, &count));

for (size_t i = 0; i < count; ++i)
{
    char name[128];
    qar_peer_spec_get_display_name(specs[i], name, sizeof(name));
    /* also: _get_app_version, _get_app_custom_peer_info,
             _get_app_state, _get_version_id, _get_room_tag,
             _get_id */
    qar_peer_spec_handle_destroy(specs[i]);
}
free(specs);
```

Useful fields:

- **app state** — `QAR_APP_STATE_INITIALIZING / RUNNING / SHUTTING_DOWN / UNKNOWN`: distinguish a peer that is present-but-loading from one that is live.
- **room tag** — which physical room the peer is in; lets multi-room deployments group peers.
- **`qar_session_get_my_spec`** — your own peer's spec (e.g. to learn your assigned peer ID).

## Reacting to changes

```c
static void on_peer_update(/* peer update info */ ..., void* state)
{
    /* peer joined / left / changed presentation or app state */
}

qar_peer_subscribe_updates(session, on_peer_update, NULL);
```

Typical uses: refresh a "participants" panel, start a [render stream](/docs/developer-guide/rendering-streams) when a target device appears, or pause expensive rendering when the last viewer leaves. Callback threading rules apply as everywhere: see [API Conventions](/docs/developer-guide/api-conventions#4-blocking--async-pairs).
