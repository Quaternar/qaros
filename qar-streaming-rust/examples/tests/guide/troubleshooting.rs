use qaros::{BlockingOptions, OnboardMethod, PeerPresentation, Runtime, StatusCode};

fn onboard_local_app(runtime: &Runtime) {
    // ANCHOR: onboard_local_app
    match runtime.onboard(
        &OnboardMethod::LocalApp {
            approval_timeout: None,
            app_tag: None,
        },
        &PeerPresentation::default(),
        &BlockingOptions::default(),
    ) {
        Ok(onboarded) => {
            // use the session; dropping it releases the handle
            drop(onboarded.session);
        }
        Err(error) if error.has_code(StatusCode::ONBOARDING_APPROVAL_TIMEOUT) => {
            // nobody answered the prompt in QAROS: ask the user to start the app again
        }
        Err(error) => {
            // Display prints the C function, the status code and its message
            eprintln!("{error}");
        }
    }
    // ANCHOR_END: onboard_local_app
}
