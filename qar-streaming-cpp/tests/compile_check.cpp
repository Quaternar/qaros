// Compile check of qar-streaming-cpp: every public header, the dynamic-loading implementation of
// the C header and the onboarding builders in one translation unit. `python ci/run.py cpp-headers`
// compiles it with warnings as errors (MSVC: /W4 /WX, C++17 and C++20, with and without
// /permissive-), so a header that uses an alternative token (not, and, or) or calls a C function
// the header marks deprecated fails. It must include every header of include/qar_streaming_cpp.

#include <qar_streaming_cpp/AppVolumeBuilders.hpp>
#include <qar_streaming_cpp/AppVolumes.hpp>
#include <qar_streaming_cpp/Async.hpp>
#include <qar_streaming_cpp/CancelToken.hpp>
#include <qar_streaming_cpp/Config.hpp>
#include <qar_streaming_cpp/Enums.hpp>
#include <qar_streaming_cpp/GuiPanelBuilder.hpp>
#include <qar_streaming_cpp/GuiPanels.hpp>
#include <qar_streaming_cpp/Handle.hpp>
#include <qar_streaming_cpp/Library.hpp>
#include <qar_streaming_cpp/OnboardingInvite.hpp>
#include <qar_streaming_cpp/Peers.hpp>
#include <qar_streaming_cpp/RenderSender.hpp>
#include <qar_streaming_cpp/Runtime.hpp>
#include <qar_streaming_cpp/RuntimeBuilders.hpp>
#include <qar_streaming_cpp/SenderBuilders.hpp>
#include <qar_streaming_cpp/Session.hpp>
#include <qar_streaming_cpp/Status.hpp>
#include <qar_streaming_cpp/Subscription.hpp>
#include <qar_streaming_cpp/Types.hpp>
#include <qar_streaming_cpp/Values.hpp>
#include <qar_streaming_cpp/VideoSender.hpp>

QAR_IMPLEMENT_DYNAMIC_LOADING()

namespace
{

[[maybe_unused]]
qaros::Result<qaros::OnboardSession>
JoinAsLocalApp(const qaros::Runtime& runtime)
{
	qaros::OnboardBuilder builder;
	builder.WithLocalApp().WithAppTag("compile-check.qaros").WithDisplayName("Compile check");
	return runtime.Onboard(builder);
}

[[maybe_unused]]
qaros::Result<qaros::OnboardSession>
JoinAsCoProcess(const qaros::Runtime& runtime, const qaros::OnboardingInvite& invite)
{
	qaros::OnboardBuilder builder;
	builder.WithInvite(invite);
	return runtime.Onboard(builder);
}

[[maybe_unused]]
bool
IsHere(const qaros::PeerSpec& peer)
{
	return peer.Presence == qaros::PeerPresence::Online
		   || peer.Presence == qaros::PeerPresence::Unknown;
}

} // namespace

int
main()
{
	return 0;
}
