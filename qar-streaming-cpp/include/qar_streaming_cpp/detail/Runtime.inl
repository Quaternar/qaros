#pragma once

// Implementation of Runtime.hpp.

#include <qar_streaming_cpp/detail/Completion.hpp>
#include <qar_streaming_cpp/detail/Native.hpp>

#include <utility>

namespace qaros
{

namespace detail
{

inline void
OnOnboarded(
	NativeResult status,
	const native::OnboardingId* onboarding_id,
	SessionHandle* session,
	void* user_state
) noexcept
{
	const Status result(status);
	if(!result.Ok())
	{
		Finish<OnboardSession>(user_state, Result<OnboardSession>(result));
		return;
	}
	OnboardSession value;
	value.OnboardingId = FromNative(*onboarding_id);
	value.Session = Session(SessionPtr(session));
	Finish<OnboardSession>(user_state, std::move(value));
}

} // namespace detail

inline Runtime::Runtime(RuntimePtr owned) noexcept
	: m_handle(std::move(owned))
{
}

inline Result<Runtime>
Runtime::Create(const RuntimeBuilder& builder) noexcept
{
	RuntimePtr runtime;
	const Status status(qar_runtime_create(&builder.Get(), OutPtr(runtime)));
	if(!status.Ok())
	{
		return Result<Runtime>(status);
	}
	return Runtime(std::move(runtime));
}

inline RuntimeHandle*
Runtime::Get() const noexcept
{
	return m_handle.get();
}

inline Runtime::
operator bool() const noexcept
{
	return m_handle != nullptr;
}

inline RuntimeHandle*
Runtime::Release() noexcept
{
	return m_handle.release();
}

inline Result<OnboardSession>
Runtime::Onboard(const OnboardBuilder& builder, const CallOptions& options) const noexcept
{
	OnboardSession value;
	SessionPtr session;
	const Status status(qar_runtime_onboard(
		m_handle.get(),
		&builder.Get(),
		detail::BlockingProgressCallback(options),
		detail::BlockingProgressState(options),
		options.Cancel,
		detail::OutValue(&value.OnboardingId),
		OutPtr(session)
	));
	if(!status.Ok())
	{
		return Result<OnboardSession>(status);
	}
	value.Session = Session(std::move(session));
	return value;
}

template<typename Token>
auto
Runtime::OnboardAsync(
	const OnboardBuilder& builder, CallOptions options, Token&& token
) const noexcept
{
	using V = OnboardSession;
	const qar_progress_callback_t progress = detail::AsyncProgressCallback<V>(options);
	const auto start = [runtime = m_handle.get(),
						init = &builder.Get(),
						progress](void* user_state, CancelTokenHandle* cancel)
	{
		return qar_runtime_onboard_async(
			runtime, init, &detail::OnOnboarded, progress, user_state, cancel
		);
	};
	return detail::Initiate<V>(start, std::move(options), nullptr, std::forward<Token>(token));
}

} // namespace qaros
