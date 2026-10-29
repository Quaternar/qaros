use qaros::{BlockingOptions, OnboardingId, PeerPresentation, Runtime, StatusCode};

fn rejoin_or_onboard(runtime: &Runtime, onboarding_id: OnboardingId) {
    // ANCHOR: rejoin_or_onboard
    match runtime.rejoin(
        onboarding_id,
        &PeerPresentation::default(),
        &BlockingOptions::default(),
    ) {
        Ok(session) => {
            // use the session; dropping it releases the handle
            drop(session);
        }
        Err(error) if error.has_code(StatusCode::ONBOARDING_SESSION_NOT_FOUND) => {
            // expected first-run: fall back to runtime.onboard
        }
        Err(error) => {
            // Display prints the C function, the status code and its message
            eprintln!("{error}");
        }
    }
    // ANCHOR_END: rejoin_or_onboard
}
