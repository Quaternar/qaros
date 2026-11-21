// qar-member-hub-client: the member hub's side of the member hub cube.
//
// First start (--join): asks the servers on the network (UDP multicast) for a
// member-hub invitation, makes the hub of this PC a member of the server's
// primary hub with it, and joins that hub as a local app by redeeming the
// invitation's remote-app grant, with no prompt. Later starts: joins as a local
// app, already approved. Then it renders a green cube for the target apps it
// picks. See README.md.

#include "cube_app.hpp"
#include "invite_channel.hpp"

#include <algorithm>
#include <charconv>
#include <chrono>
#include <cstdio>
#include <iostream>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

QAR_IMPLEMENT_DYNAMIC_LOADING()

namespace
{

constexpr const char* kUsage =
	"Usage: qar-member-hub-client [--join] [--group <address:port>] [--interface <IPv4>]\n"
	"                             [--wait <seconds>] [--target <text>]\n"
	"\n"
	"  --join              first start: ask a server on the network for an invitation\n"
	"                      and make the hub of this PC a member of its primary hub\n"
	"  --group <addr:port> the multicast group the server listens on\n"
	"                      (default 239.255.43.21:47123)\n"
	"  --interface <ip>    the address of the network interface to ask on\n"
	"  --wait <seconds>    how long to keep asking (default 600)\n"
	"  --target <text>     render only for target apps whose name contains <text>\n"
	"                      (default: every target app in the session)\n"
	"\n"
	"Without --join the app joins the hub of this PC, approved by an earlier first\n"
	"start. Stop with Ctrl+C, or by closing stdin when it is a pipe.\n";

struct ClientOptions
{
	bool join = false;
	ChannelOptions channel;
	std::chrono::seconds wait{ 600 };
	std::string target;
};

std::optional<ClientOptions>
ParseOptions(int argc, char** argv)
{
	ClientOptions options;
	for(int index = 1; index < argc; ++index)
	{
		const std::string_view name = argv[index];
		if(name == "--join")
		{
			options.join = true;
			continue;
		}
		if(index + 1 >= argc)
		{
			return std::nullopt;
		}
		const std::string_view value = argv[++index];
		if(name == "--group")
		{
			if(not ParseGroup(value, options.channel))
			{
				return std::nullopt;
			}
		}
		else if(name == "--interface")
		{
			options.channel.interfaceAddress = value;
		}
		else if(name == "--wait")
		{
			unsigned seconds = 0;
			const auto parsed = std::from_chars(value.data(), value.data() + value.size(), seconds);
			if(parsed.ec != std::errc() || parsed.ptr != value.data() + value.size())
			{
				return std::nullopt;
			}
			options.wait = std::chrono::seconds(seconds);
		}
		else if(name == "--target")
		{
			options.target = value;
		}
		else
		{
			return std::nullopt;
		}
	}
	return options;
}

// The invitation from a server, parsed; nullptr on failure (printed).
QarMemberHubInvitation*
ReceiveInvitation(const ClientOptions& options)
{
	std::cout << "Asking for an invitation on " << options.channel.group << ':'
			  << options.channel.port << '\n';
	std::optional<std::vector<uint8_t>> bytes =
		FetchInvitation(options.channel, options.wait, g_stopRequested);
	if(not bytes)
	{
		return nullptr;
	}
	QarMemberHubInvitation* invitation = nullptr;
	// Checks the format only; the join checks the signature.
	const QarResult parsed =
		qar_member_hub_invitation_deserialize(bytes->data(), bytes->size(), &invitation);
	// The handle keeps its own copy: clear the secret.
	std::fill(bytes->begin(), bytes->end(), uint8_t{ 0 });
	if(not Check(parsed))
	{
		std::cerr << "The server sent something that is no member-hub invitation.\n";
		return nullptr;
	}
	std::cout << "Received a member-hub invitation.\n";
	return invitation;
}

// Makes the hub of this PC a member of the invitation's primary hub. The hub
// restarts into the primary's session meanwhile; a timeout means the join is
// committed, and asking again returns once the hub is back.
bool
JoinHubAsMember(QarRuntime* runtime, const QarMemberHubInvitation* invitation)
{
	std::cout << "Joining the hub of this PC to the primary hub as a member.\n";
	QarJoinHubAsMemberInit init = qar_join_hub_as_member_init_default();
	init.invitation = invitation;
	QarResult joined =
		qar_runtime_join_hub_as_member(runtime, &init, PrintProgress, nullptr, nullptr);
	if(qar_result_has_code(joined, QAR_STATUS_TIMEOUT))
	{
		std::cout << "The hub is still restarting; asking again.\n";
		joined = qar_runtime_join_hub_as_member(runtime, &init, PrintProgress, nullptr, nullptr);
	}
	if(Check(joined))
	{
		std::cout << "The hub of this PC is a member of the primary hub.\n";
		return true;
	}
	if(qar_result_has_code(joined, QAR_STATUS_MEMBER_INVITATION_EXPIRED)
	   || qar_result_has_code(joined, QAR_STATUS_MEMBER_INVITATION_USED_UP)
	   || qar_result_has_code(joined, QAR_STATUS_MEMBER_INVITATION_REVOKED))
	{
		std::cerr
			<< "The invitation can no longer be used. Start the client again for a new one.\n";
	}
	else if(qar_result_has_code(joined, QAR_STATUS_MEMBER_INVITATION_PRIMARY_NOT_ALLOWED))
	{
		std::cerr << "This PC does not trust the server's primary hub: its hub already joined "
					 "another primary, or an administrator's allow-list leaves this one out.\n";
	}
	else if(qar_result_has_code(joined, QAR_STATUS_PRIMARY_UNREACHABLE))
	{
		std::cerr << "The hub of this PC cannot reach the primary hub. Check the network and the "
					 "firewall of the server's PC.\n";
	}
	return false;
}

int
RunClient(QarRuntime* runtime, const ClientOptions& options)
{
	QarMemberHubInvitation* invitation = nullptr;
	if(options.join)
	{
		invitation = ReceiveInvitation(options);
		if(invitation == nullptr)
		{
			return g_stopRequested.load() ? 0 : 8;
		}
		if(not JoinHubAsMember(runtime, invitation))
		{
			qar_member_hub_invitation_handle_destroy(invitation);
			return 9;
		}
	}
	// With the grant the primary approves this executable without a prompt and
	// remembers its peer id under the grant's tag; later starts need neither.
	QarSession* session = nullptr;
	const QarResult joined =
		JoinAsLocalApp(runtime, "Member Hub Cube Client", nullptr, invitation, &session);
	qar_member_hub_invitation_handle_destroy(invitation); // NULL is ignored
	if(not qar_result_is_success(joined))
	{
		return g_stopRequested.load() ? 0 : ExplainJoinFailure(joined);
	}
	PrintMyPeerId(session);
	VulkanDevice gpu; // Outlives every sender and camera.
	const bool rendered =
		gpu.Create(SessionAdapter(session))
		&& RenderCube(
			session,
			gpu,
			CubeOptions{ .color = { .12f, 1.f, .12f }, .targetFilter = options.target }
		);
	qar_session_handle_destroy(session);
	return rendered ? 0 : 1;
}

} // namespace

int
main(int argc, char** argv)
{
	// Every line as it happens, also into a pipe or a file.
	std::setvbuf(stdout, nullptr, _IONBF, 0);
	const std::optional<ClientOptions> options = ParseOptions(argc, argv);
	if(not options)
	{
		std::cerr << kUsage;
		return 1;
	}
	WatchForStop();
	if(not qar_library_load(nullptr))
	{
		std::cerr << "Could not load QAROS. Install QAROS on this machine, then start this app "
					 "again.\n";
		return 2;
	}
	auto libraryInit = qar_library_init_default();
	// Check() reports a failed call through the library's log.
	libraryInit.enable_console_logging = true;
	int exitCode = 1;
	if(Check(qar_library_init(&libraryInit)))
	{
		QarRuntime* runtime = nullptr;
		const auto runtimeInit = qar_runtime_init_default();
		if(Check(qar_runtime_create(&runtimeInit, &runtime)))
		{
			exitCode = RunClient(runtime, options.value());
			qar_runtime_handle_destroy(runtime);
		}
		Check(qar_library_destroy());
	}
	qar_library_unload();
	return exitCode;
}
