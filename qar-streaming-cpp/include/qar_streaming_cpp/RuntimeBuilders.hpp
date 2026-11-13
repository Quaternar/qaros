#pragma once

// Builders of the runtime and onboarding inits (C module detail/onboarding.h).
// A builder owns every string and extension it chains; keep it alive until the
// call that reads it has started. Builders are not copyable: the chain points
// into them.

#include <qar_streaming_cpp/OnboardingInvite.hpp>
#include <qar_streaming_cpp/Values.hpp>

#include <cstdint>
#include <optional>
#include <string>

namespace qaros
{

/// The options of Runtime::Create.
class RuntimeBuilder
{
public:
	RuntimeBuilder() noexcept;
	RuntimeBuilder(const RuntimeBuilder&) = delete;
	RuntimeBuilder& operator=(const RuntimeBuilder&) = delete;

	/// The SDK `bin` folder the library was loaded from. Without it no peer
	/// can be invited.
	RuntimeBuilder& WithBinariesFolder(std::string path) noexcept;
	/// Where identities and session state persist.
	RuntimeBuilder& WithStorageFolder(std::string path) noexcept;

	[[nodiscard]]
	const native::RuntimeInit& Get() const noexcept;

private:
	void Sync() noexcept;

	native::RuntimeInit m_init;
	std::optional<std::string> m_binariesFolder;
	std::optional<std::string> m_storageFolder;
};

/// The peer presentation of an onboard. Unset fields stay null.
struct PresentationOptions
{
	std::optional<std::string> DisplayName;
	std::optional<std::string> AppVersion;
	std::optional<std::string> AppCustomPeerInfo;
};

/// The options of Runtime::Onboard. Exactly one mode: WithLocalApp, WithInvite
/// or WithCode; the later call replaces the earlier one.
class OnboardBuilder
{
public:
	OnboardBuilder() noexcept;
	OnboardBuilder(const OnboardBuilder&) = delete;
	OnboardBuilder& operator=(const OnboardBuilder&) = delete;

	/// Mode: join the QAROS hub installed on this PC as a local source app,
	/// with no code (QarOnboardLocalAppExt). The hub asks the user once to
	/// approve the app; `approval_timeout_ms` bounds that wait (0: the hub's
	/// default, 120 s). Stateless: the onboarding id is zero. Windows only.
	OnboardBuilder& WithLocalApp(uint32_t approval_timeout_ms = 0) noexcept;
	/// The app tag the hub remembers this instance's peer id by
	/// (QarOnboardLocalAppTagExt). Only with WithLocalApp; the C API refuses
	/// it with any other mode.
	OnboardBuilder& WithAppTag(std::string app_tag) noexcept;
	/// Mode: a full invite: a co-process invite from the app that started this
	/// process, or the launch invite QAROS hands an app it starts. Borrowed:
	/// keep `invite` alive until the call has started.
	OnboardBuilder& WithInvite(const OnboardingInvite& invite) noexcept;
	/// Mode: a pairing code minted for the hub's operators. Finds this user's
	/// hub by discovery. Code onboarding is deprecated in the C API
	/// (QarOnboardCodeExt): an app joins with WithLocalApp or WithInvite.
	OnboardBuilder& WithCode(std::string code) noexcept;
	/// Mode: a code for the hub at `hostname:port` (plain host or IP).
	/// Deprecated in the C API like WithCode (QarOnboardHostExt).
	OnboardBuilder& WithCode(std::string code, std::string hostname, uint16_t port) noexcept;

	/// Re-enrolls this persisted slot instead of creating a new identity
	/// (code onboarding only).
	OnboardBuilder& WithOnboardingId(const OnboardingId& id) noexcept;
	/// Joins under this peer id instead of a fresh one. Deprecated in the C API
	/// (QarOnboardPeerIdExt) and refused with WithLocalApp: a local app gets
	/// its peer id from the hub, per app tag.
	OnboardBuilder& WithPeerId(const PeerId& peer_id) noexcept;
	/// Creates the session's GPU devices on this adapter.
	OnboardBuilder& WithGraphicsDevice(const GraphicsDeviceId& device) noexcept;
	OnboardBuilder& WithDisplayName(std::string name) noexcept;
	OnboardBuilder& WithAppVersion(std::string version) noexcept;
	OnboardBuilder& WithAppCustomPeerInfo(std::string info) noexcept;

	[[nodiscard]]
	const native::OnboardInit& Get() const noexcept;

private:
	enum class Mode
	{
		None,
		LocalApp,
		Code,
		Invite,
	};

	void Sync() noexcept;

	native::OnboardInit m_init;
	Mode m_mode = Mode::None;
	native::OnboardCodeExt m_code;
	std::string m_codeText;
	std::optional<native::OnboardHostExt> m_host;
	std::string m_hostname;
	native::OnboardInviteExt m_invite;
	native::OnboardLocalAppExt m_localApp;
	std::optional<native::OnboardLocalAppTagExt> m_appTag;
	std::string m_appTagText;
	std::optional<native::OnboardPeerIdExt> m_peerId;
	std::optional<native::GraphicsDeviceId> m_graphicsDevice;
	PresentationOptions m_presentation;
};

} // namespace qaros

#include <qar_streaming_cpp/detail/RuntimeBuilders.inl>
