#pragma once
// The example's own channel for the member-hub invitation, from the server
// (the primary hub's PC) to a client (another PC) on the same network. QAROS
// does not carry the invitation: your application brings it over a channel of
// its own.
//
// UDP multicast, so neither side needs the other's address:
//
//   client -> group:port   request  "QMHR" | nonce (16 bytes)
//   server -> client       answer   "QMHI" | nonce | invitation
//
// The client sends its request once a second until an answer with its nonce
// arrives; the server answers by unicast to the address the request came
// from, the same invitation for every repeat of one request. (Windows
// Firewall lets a unicast answer to a multicast request in for 3 s; the
// server's PC must let the multicast port in.)
//
// The invitation is a bearer secret, and this channel has no encryption and
// no authentication but the client's address (--allow). It is fit for a demo
// on a network you trust, nothing more. A real application sends the
// invitation only over an encrypted, authenticated channel of its own.

#include <atomic>
#include <chrono>
#include <cstdint>
#include <functional>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

// A serialized member-hub invitation is at most 16 KiB.
constexpr std::uint32_t kMaxInvitationBytes = 16U * 1024U;

struct ChannelOptions
{
	// The multicast group and port the server listens on.
	std::string group = "239.255.43.21";
	std::uint16_t port = 47123;
	// The IPv4 address of the network interface to use; empty lets the
	// system pick (the interface of the default route).
	std::string interfaceAddress;
};

// "address:port", an IPv4 address and a port 1..65535.
bool ParseGroup(std::string_view text, ChannelOptions& options);

// Creates the invitation for a client (its IPv4 address); empty bytes refuse
// it.
using InvitationFactory = std::function<std::vector<std::uint8_t>(const std::string& client)>;

// The server's side: answers every request from an allowed address (every
// address when `allowed` is empty) until `stop`. Returns false when the group
// cannot be joined; the reason is printed.
bool ServeInvitations(
	const ChannelOptions& options,
	const std::vector<std::string>& allowed,
	const InvitationFactory& create,
	const std::atomic<bool>& stop
);

// The client's side: asks the group for an invitation until one arrives,
// `wait` passes or `stop` is set. Empty on failure; the reason is printed.
std::optional<std::vector<std::uint8_t>> FetchInvitation(
	const ChannelOptions& options, std::chrono::seconds wait, const std::atomic<bool>& stop
);
