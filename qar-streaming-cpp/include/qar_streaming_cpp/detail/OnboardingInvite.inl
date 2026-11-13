#pragma once

// Implementation of OnboardingInvite.hpp.

#include <qar_streaming_cpp/detail/Native.hpp>

#include <utility>

namespace qaros
{

inline OnboardingInvite::OnboardingInvite(OnboardingInvitePtr owned) noexcept
	: m_handle(std::move(owned))
{
}

inline Result<OnboardingInvite>
OnboardingInvite::Deserialize(const uint8_t* data, size_t size) noexcept
{
	OnboardingInvitePtr invite;
	const Status status(qar_onboarding_invite_deserialize(data, size, OutPtr(invite)));
	if(not status.Ok())
	{
		return Result<OnboardingInvite>(status);
	}
	return OnboardingInvite(std::move(invite));
}

inline Result<OnboardingInvite>
OnboardingInvite::Deserialize(const std::vector<uint8_t>& wire) noexcept
{
	return Deserialize(wire.data(), wire.size());
}

inline OnboardingInviteHandle*
OnboardingInvite::Get() const noexcept
{
	return m_handle.get();
}

inline OnboardingInvite::
operator bool() const noexcept
{
	return m_handle != nullptr;
}

inline Result<std::vector<uint8_t>>
OnboardingInvite::Serialize() const noexcept
{
	size_t size = 0;
	const Status sized(qar_onboarding_invite_serialized_size(m_handle.get(), &size));
	if(not sized.Ok())
	{
		return Result<std::vector<uint8_t>>(sized);
	}
	std::vector<uint8_t> wire(size);
	size_t written = 0;
	const Status serialized(
		qar_onboarding_invite_serialize(m_handle.get(), wire.data(), wire.size(), &written)
	);
	if(not serialized.Ok())
	{
		return Result<std::vector<uint8_t>>(serialized);
	}
	wire.resize(written);
	return wire;
}

} // namespace qaros
