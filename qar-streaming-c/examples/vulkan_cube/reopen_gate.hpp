#pragma once
#include <chrono>
#include <optional>
#include <qar_streaming.h>

// What the cube does about a target it serves but has no sender for: its first
// sender, or a new one after the old one ended (a stream closed for good
// included).
enum class ReopenAction
{
	// Not yet: the target is not ONLINE (OFFLINE, LEAVING or UNKNOWN), or the
	// wait after the last sender ended still runs. A target that is away may
	// come back under its id.
	Wait,
	// Create the new sender now.
	Create,
	// The target retired, or the peer list no longer lists it: it will not
	// come back, so the cube forgets it.
	Forget,
};

// Decides from the target's presence in the peer list right now (empty: not
// listed) and the earliest time of the next sender. Only an ONLINE target gets
// a sender, so a stream that closed while its target was away is reopened only
// once the target is back.
inline ReopenAction
DecideReopen(
	const std::optional<QarPeerPresence> presence,
	const std::chrono::steady_clock::time_point now,
	const std::chrono::steady_clock::time_point notBefore
)
{
	if(not presence.has_value() || presence.value() == QAR_PEER_PRESENCE_RETIRED)
	{
		return ReopenAction::Forget;
	}
	if(presence.value() != QAR_PEER_PRESENCE_ONLINE || now < notBefore)
	{
		return ReopenAction::Wait;
	}
	return ReopenAction::Create;
}
