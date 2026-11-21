// qar-member-hub-server: the primary hub's side of the member hub cube.
//
// Joins the hub of this PC as a local app and listens on a UDP multicast group
// for clients on the network. Each client that asks (from an allowed address)
// gets a fresh member-hub invitation carrying a remote-app grant for the
// client app. Meanwhile it renders a red cube for the target apps it picks.
// See README.md.

#include "cube_app.hpp"
#include "invite_channel.hpp"

#include <algorithm>
#include <cstdio>
#include <iostream>
#include <optional>
#include <string>
#include <string_view>
#include <thread>
#include <vector>

QAR_IMPLEMENT_DYNAMIC_LOADING()

namespace
{

// The server's own tag on the primary, and the tag its grant admits the client
// under. One grant tag admits one executable: the first client executable that
// redeems it is the only one the primary accepts under that tag afterwards.
constexpr const char* kServerTag = "server.member-hub-cube.examples.qaros";
constexpr const char* kClientTag = "client.member-hub-cube.examples.qaros";
// An invitation only has to live until the client used it.
constexpr uint32_t kInvitationLifetimeSeconds = 600;

constexpr const char* kUsage =
	"Usage: qar-member-hub-server [--allow <client IPv4> ...] [--group <address:port>]\n"
	"                             [--interface <IPv4>] [--target <text>]\n"
	"\n"
	"  --allow <ip>        answer only this client PC; repeat for more\n"
	"                      (default: every PC on the network that asks)\n"
	"  --group <addr:port> the multicast group clients ask on (default 239.255.43.21:47123)\n"
	"  --interface <ip>    the address of the network interface to listen on\n"
	"  --target <text>     render only for target apps whose name contains <text>\n"
	"                      (default: every target app in the session)\n"
	"\n"
	"Run it on the PC of the primary hub. Its approval in QAROS must allow it to\n"
	"bring in member hubs. Stop with Ctrl+C, or by closing stdin when it is a pipe.\n";

struct ServerOptions
{
	std::vector<std::string> allowed;
	ChannelOptions channel;
	std::string target;
};

std::optional<ServerOptions>
ParseOptions(int argc, char** argv)
{
	ServerOptions options;
	for(int index = 1; index + 1 < argc; index += 2)
	{
		const std::string_view name = argv[index];
		const std::string_view value = argv[index + 1];
		if(name == "--allow")
		{
			options.allowed.emplace_back(value);
		}
		else if(name == "--group")
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
		else if(name == "--target")
		{
			options.target = value;
		}
		else
		{
			return std::nullopt;
		}
	}
	if(argc % 2 == 0)
	{
		return std::nullopt;
	}
	return options;
}

// A member-hub invitation carrying a remote-app grant for the client app,
// serialized. Empty when the hub refused it; the reason is printed.
std::vector<uint8_t>
CreateInvitation(QarRuntime* runtime, const std::string& client)
{
	const std::string label = "Member hub cube client " + client;
	QarMemberHubInvitationInit init = qar_member_hub_invitation_init_default();
	// Shown to the primary's operator, who can revoke the invitation there.
	init.label = label.c_str();
	init.lifetime_seconds = kInvitationLifetimeSeconds;
	init.max_uses = 1;
	QarMemberHubInvitationAppGrantExt grant = qar_member_hub_invitation_app_grant_ext_default();
	grant.app_tag = kClientTag;
	init.header.next = &grant.header;
	QarMemberHubInvitation* invitation = nullptr;
	const QarResult created =
		qar_runtime_create_member_hub_invitation(runtime, &init, nullptr, &invitation);
	if(not Check(created))
	{
		if(qar_result_has_code(created, QAR_STATUS_MEMBER_SPONSOR_NOT_ALLOWED))
		{
			std::cerr
				<< "This app may not bring in member hubs. Revoke it in the QAROS Visualizer "
				   "(Source Applications, Approved apps), start it again and tick \"May bring "
				   "in member hubs\" in the approval prompt.\n";
		}
		else if(qar_result_has_code(created, QAR_STATUS_MEMBER_SPONSOR_QUOTA_EXCEEDED))
		{
			std::cerr << "This app has brought in as many member hubs as its approval allows.\n";
		}
		else if(qar_result_has_code(created, QAR_STATUS_SESSION_WIDE_ACTION_ON_MEMBER))
		{
			std::cerr << "The hub of this PC is a member itself: run the server on the PC of "
						 "the primary hub.\n";
		}
		return {};
	}
	size_t size = 0;
	std::vector<uint8_t> bytes;
	if(Check(qar_member_hub_invitation_serialized_size(invitation, &size)) && size > 0
	   && size <= kMaxInvitationBytes)
	{
		bytes.resize(size);
		size_t written = 0;
		if(not Check(qar_member_hub_invitation_serialize(invitation, bytes.data(), size, &written))
		   || written != size)
		{
			bytes.clear();
		}
	}
	qar_member_hub_invitation_handle_destroy(invitation);
	return bytes;
}

// The channel thread: answers clients until a stop. A group it cannot join
// stops the app.
void
ServeClients(QarRuntime* runtime, const ServerOptions* options, std::atomic<bool>* failed)
{
	const InvitationFactory create = [runtime](const std::string& client)
	{
		return CreateInvitation(runtime, client);
	};
	if(not ServeInvitations(options->channel, options->allowed, create, g_stopRequested))
	{
		failed->store(true);
		g_stopRequested.store(true);
	}
}

int
RunServer(QarRuntime* runtime, const ServerOptions& options)
{
	std::cout << "Joining the QAROS on this machine. On the first run, tick \"May bring in "
				 "member hubs\" and click Allow in the QAROS approval prompt.\n";
	QarSession* session = nullptr;
	const QarResult joined =
		JoinAsLocalApp(runtime, "Member Hub Cube Server", kServerTag, nullptr, &session);
	if(not qar_result_is_success(joined))
	{
		return g_stopRequested.load() ? 0 : ExplainJoinFailure(joined);
	}
	PrintMyPeerId(session);
	VulkanDevice gpu; // Outlives every sender and camera.
	std::atomic<bool> channelFailed = false;
	std::thread channel(ServeClients, runtime, &options, &channelFailed);
	const bool rendered =
		gpu.Create(SessionAdapter(session))
		&& RenderCube(
			session,
			gpu,
			CubeOptions{ .color = { 1.f, .12f, .12f }, .targetFilter = options.target }
		);
	g_stopRequested.store(true);
	channel.join();
	qar_session_handle_destroy(session);
	if(channelFailed.load())
	{
		return 8;
	}
	return rendered ? 0 : 1;
}

} // namespace

int
main(int argc, char** argv)
{
	// Every line as it happens, also into a pipe or a file.
	std::setvbuf(stdout, nullptr, _IONBF, 0);
	const std::optional<ServerOptions> options = ParseOptions(argc, argv);
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
			exitCode = RunServer(runtime, options.value());
			qar_runtime_handle_destroy(runtime);
		}
		Check(qar_library_destroy());
	}
	qar_library_unload();
	return exitCode;
}
