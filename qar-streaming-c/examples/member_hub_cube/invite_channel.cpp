#include "invite_channel.hpp"

#include <asio.hpp>

#include <algorithm>
#include <array>
#include <charconv>
#include <cstring>
#include <iostream>
#include <map>
#include <random>
#include <set>

namespace
{

using Clock = std::chrono::steady_clock;
using udp = asio::ip::udp;

constexpr std::array<char, 4> kRequestMagic{ 'Q', 'M', 'H', 'R' };
constexpr std::array<char, 4> kAnswerMagic{ 'Q', 'M', 'H', 'I' };
constexpr size_t kNonceBytes = 16;
constexpr size_t kHeaderBytes = kRequestMagic.size() + kNonceBytes;
constexpr std::chrono::milliseconds kRequestInterval{ 1000 };
// How long the server answers repeats of one request with the same invitation.
constexpr std::chrono::minutes kAnswerLifetime{ 10 };

using Nonce = std::array<std::uint8_t, kNonceBytes>;

bool
HasMagic(const std::uint8_t* bytes, const std::array<char, 4>& magic)
{
	return std::memcmp(bytes, magic.data(), magic.size()) == 0;
}

// Clears an invitation's bytes: it is a bearer secret.
void
Wipe(std::vector<std::uint8_t>& bytes)
{
	std::fill(bytes.begin(), bytes.end(), std::uint8_t{ 0 });
	bytes.clear();
}

std::optional<asio::ip::address_v4>
ParseAddress(const std::string& text)
{
	asio::error_code error;
	const asio::ip::address_v4 address = asio::ip::make_address_v4(text, error);
	if(error)
	{
		return std::nullopt;
	}
	return address;
}

// The server's socket: joined to the group, answering requests.
class Responder
{
public:
	Responder(
		asio::io_context& io,
		const std::vector<std::string>& allowed,
		const InvitationFactory& create
	)
		: m_socket(io)
		, m_allowed(&allowed)
		, m_create(&create)
	{
	}
	Responder(const Responder&) = delete;
	Responder& operator=(const Responder&) = delete;
	~Responder()
	{
		for(auto& [key, answer] : m_answers)
		{
			Wipe(answer.invitation);
		}
	}

	bool Open(const ChannelOptions& options)
	{
		const std::optional<asio::ip::address_v4> group = ParseAddress(options.group);
		if(not group || not group->is_multicast())
		{
			std::cerr << options.group << " is no IPv4 multicast address.\n";
			return false;
		}
		asio::error_code error;
		m_socket.open(udp::v4(), error);
		if(not error)
		{
			m_socket.set_option(asio::socket_base::reuse_address(true), error);
		}
		if(not error)
		{
			m_socket.bind(udp::endpoint(asio::ip::address_v4::any(), options.port), error);
		}
		if(not error)
		{
			const std::optional<asio::ip::address_v4> local =
				options.interfaceAddress.empty() ? std::nullopt
												 : ParseAddress(options.interfaceAddress);
			m_socket.set_option(
				local ? asio::ip::multicast::join_group(*group, *local)
					  : asio::ip::multicast::join_group(*group),
				error
			);
		}
		if(error)
		{
			std::cerr << "Could not join " << options.group << ':' << options.port << ": "
					  << error.message() << '\n';
			return false;
		}
		std::cout << "Waiting for clients on " << options.group << ':' << options.port << '\n';
		if(m_allowed->empty())
		{
			std::cout << "No --allow: any PC on the network that asks gets an invitation.\n";
		}
		Receive();
		return true;
	}

private:
	struct Answer
	{
		std::vector<std::uint8_t> invitation;
		Clock::time_point created;
	};

	void Receive()
	{
		m_socket.async_receive_from(
			asio::buffer(m_request),
			m_sender,
			[this](const asio::error_code& error, size_t size)
			{
				OnRequest(error, size);
			}
		);
	}

	void OnRequest(const asio::error_code& error, size_t size)
	{
		if(error == asio::error::operation_aborted)
		{
			return;
		}
		if(not error && size == kHeaderBytes && HasMagic(m_request.data(), kRequestMagic))
		{
			Respond(m_sender);
		}
		Receive();
	}

	bool Allowed(const std::string& client)
	{
		if(m_allowed->empty()
		   || std::find(m_allowed->begin(), m_allowed->end(), client) != m_allowed->end())
		{
			return true;
		}
		if(m_refused.insert(client).second)
		{
			std::cout << "Refused a client at " << client << ": not in --allow.\n";
		}
		return false;
	}

	void Respond(const udp::endpoint& sender)
	{
		const std::string client = sender.address().to_string();
		if(not Allowed(client))
		{
			return;
		}
		Forget(Clock::now() - kAnswerLifetime);
		const std::string key =
			client + '/'
			+ std::string(
				reinterpret_cast<const char*>(m_request.data()) + kRequestMagic.size(), kNonceBytes
			);
		auto found = m_answers.find(key);
		if(found == m_answers.end())
		{
			std::cout << "Client " << client
					  << " asks to join. Creating a member-hub invitation.\n";
			found = m_answers.emplace(key, Answer{ (*m_create)(client), Clock::now() }).first;
			if(found->second.invitation.empty()
			   || found->second.invitation.size() > kMaxInvitationBytes)
			{
				// Refused or unusable: repeats of this request get no answer either.
				Wipe(found->second.invitation);
				return;
			}
		}
		if(found->second.invitation.empty())
		{
			return;
		}
		std::vector<std::uint8_t> datagram(kHeaderBytes);
		std::memcpy(datagram.data(), kAnswerMagic.data(), kAnswerMagic.size());
		std::memcpy(
			datagram.data() + kAnswerMagic.size(),
			m_request.data() + kRequestMagic.size(),
			kNonceBytes
		);
		datagram.insert(
			datagram.end(), found->second.invitation.begin(), found->second.invitation.end()
		);
		asio::error_code error;
		m_socket.send_to(asio::buffer(datagram), sender, 0, error);
		Wipe(datagram);
		if(error)
		{
			std::cerr << "Could not send the invitation to " << client << ": " << error.message()
					  << '\n';
		}
		else if(m_sent.insert(key).second)
		{
			std::cout << "Sent a member-hub invitation to " << client << '\n';
		}
	}

	void Forget(Clock::time_point before)
	{
		for(auto it = m_answers.begin(); it != m_answers.end();)
		{
			if(it->second.created >= before)
			{
				++it;
				continue;
			}
			Wipe(it->second.invitation);
			m_sent.erase(it->first);
			it = m_answers.erase(it);
		}
	}

	udp::socket m_socket;
	const std::vector<std::string>* m_allowed;
	const InvitationFactory* m_create;
	std::array<std::uint8_t, kHeaderBytes> m_request{};
	udp::endpoint m_sender;
	std::map<std::string, Answer> m_answers;
	std::set<std::string> m_sent;
	std::set<std::string> m_refused;
};

Nonce
NewNonce()
{
	std::random_device random;
	Nonce nonce{};
	for(std::uint8_t& byte : nonce)
	{
		byte = static_cast<std::uint8_t>(random() & 0xFF);
	}
	return nonce;
}

} // namespace

bool
ParseGroup(std::string_view text, ChannelOptions& options)
{
	const size_t colon = text.rfind(':');
	if(colon == std::string_view::npos || colon == 0)
	{
		return false;
	}
	const std::string_view portText = text.substr(colon + 1);
	unsigned port = 0;
	const auto parsed = std::from_chars(portText.data(), portText.data() + portText.size(), port);
	const std::optional<asio::ip::address_v4> group =
		ParseAddress(std::string(text.substr(0, colon)));
	if(parsed.ec != std::errc() || parsed.ptr != portText.data() + portText.size() || port == 0
	   || port > 65535 || not group || not group->is_multicast())
	{
		return false;
	}
	options.group = std::string(text.substr(0, colon));
	options.port = static_cast<std::uint16_t>(port);
	return true;
}

bool
ServeInvitations(
	const ChannelOptions& options,
	const std::vector<std::string>& allowed,
	const InvitationFactory& create,
	const std::atomic<bool>& stop
)
{
	asio::io_context io;
	Responder responder(io, allowed, create);
	if(not responder.Open(options))
	{
		return false;
	}
	// Short slices, so a stop is seen within 200 ms.
	while(not stop.load())
	{
		io.run_for(std::chrono::milliseconds(200));
		io.restart();
	}
	return true;
}

std::optional<std::vector<std::uint8_t>>
FetchInvitation(
	const ChannelOptions& options, std::chrono::seconds wait, const std::atomic<bool>& stop
)
{
	const std::optional<asio::ip::address_v4> group = ParseAddress(options.group);
	if(not group)
	{
		std::cerr << options.group << " is no IPv4 address.\n";
		return std::nullopt;
	}
	asio::io_context io;
	udp::socket socket(io);
	asio::error_code error;
	socket.open(udp::v4(), error);
	if(not error && not options.interfaceAddress.empty())
	{
		const std::optional<asio::ip::address_v4> local = ParseAddress(options.interfaceAddress);
		if(local)
		{
			socket.set_option(asio::ip::multicast::outbound_interface(*local), error);
		}
	}
	if(not error)
	{
		socket.bind(udp::endpoint(asio::ip::address_v4::any(), 0), error);
	}
	if(error)
	{
		std::cerr << "Could not open a UDP socket: " << error.message() << '\n';
		return std::nullopt;
	}
	const Nonce nonce = NewNonce();
	std::array<std::uint8_t, kHeaderBytes> request{};
	std::memcpy(request.data(), kRequestMagic.data(), kRequestMagic.size());
	std::memcpy(request.data() + kRequestMagic.size(), nonce.data(), nonce.size());
	const udp::endpoint target(*group, options.port);
	std::vector<std::uint8_t> answer(kHeaderBytes + kMaxInvitationBytes);
	udp::endpoint sender;
	const Clock::time_point deadline = Clock::now() + wait;
	std::optional<std::vector<std::uint8_t>> invitation;
	while(not invitation && not stop.load() && Clock::now() < deadline)
	{
		socket.send_to(asio::buffer(request), target, 0, error);
		if(error)
		{
			std::cerr << "Could not send to " << options.group << ':' << options.port << ": "
					  << error.message() << '\n';
			return std::nullopt;
		}
		size_t received = 0;
		bool done = false;
		socket.async_receive_from(
			asio::buffer(answer),
			sender,
			[&received, &done](const asio::error_code& result, size_t size)
			{
				done = true;
				received = result ? 0 : size;
			}
		);
		io.restart();
		io.run_for(kRequestInterval);
		if(not done)
		{
			// Ends the receive; its handler runs with operation_aborted.
			socket.cancel(error);
			io.restart();
			io.run();
		}
		const bool ours =
			received > kHeaderBytes && HasMagic(answer.data(), kAnswerMagic)
			&& std::memcmp(answer.data() + kAnswerMagic.size(), nonce.data(), nonce.size()) == 0;
		if(ours)
		{
			invitation.emplace(
				answer.begin() + kHeaderBytes,
				answer.begin() + static_cast<std::ptrdiff_t>(received)
			);
			std::cout << "Answer from the server at " << sender.address().to_string() << '\n';
		}
	}
	Wipe(answer);
	if(not invitation && not stop.load())
	{
		std::cerr << "No server answered on " << options.group << ':' << options.port
				  << ". Is it running, does this PC's address pass its --allow, and does the "
					 "server PC's firewall let UDP port "
				  << options.port << " in?\n";
	}
	return invitation;
}
