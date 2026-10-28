//! Async onboard and rejoin, re-enrollment, declared devices and member hubs against the stub,
//! which follows the header's rules for them (exactly one callback, `QAR_STATUS_TIMEOUT` on
//! cancel, a runtime destroy that waits for the callbacks).

use std::sync::mpsc;

use qaros::{OnboardingId, PendingOnboard, Runtime};

use super::*;

fn counting_wake() -> (Arc<AtomicUsize>, impl Fn() + Send + Sync + 'static) {
    let wakes = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&wakes);
    (wakes, move || {
        counter.fetch_add(1, Ordering::SeqCst);
    })
}

fn start_code_onboard(runtime: &Runtime, timeout: Option<Duration>) -> PendingOnboard {
    runtime
        .start_onboard(
            &code("ABCD-EFGH"),
            &PeerPresentation::default(),
            &OnboardExtensions::default(),
            None,
            timeout,
            || {},
        )
        .expect("start")
}

#[test]
fn an_async_onboard_delivers_its_session_and_progress_and_an_async_rejoin_echoes_the_id() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    fixture.control.set_onboard_async_delay(20);
    let (progress, reports) = mpsc::channel();
    let (wakes, wake) = counting_wake();
    let mut pending = runtime
        .start_onboard(
            &code("ABCD-EFGH"),
            &PeerPresentation::default(),
            &OnboardExtensions::default(),
            Some(progress),
            None,
            wake,
        )
        .expect("start");
    assert!(pending.poll().is_none(), "the call is still running");
    let onboarded = pending.wait().expect("onboarded");
    assert!(!onboarded.onboarding_id.is_zero());
    assert_eq!(wakes.load(Ordering::SeqCst), 1);
    let report = reports.try_recv().expect("a progress report");
    assert_eq!(report.message, "stub async call");
    let onboarding_id = onboarded.onboarding_id;
    drop(onboarded.session);

    let mut rejoin = runtime
        .start_rejoin(
            onboarding_id,
            &PeerPresentation::default(),
            None,
            None,
            None,
            || {},
        )
        .expect("start rejoin");
    let rejoined = loop {
        if let Some(outcome) = rejoin.poll() {
            break outcome.expect("rejoined");
        }
        std::thread::sleep(Duration::from_millis(1));
    };
    assert_eq!(rejoined.onboarding_id, onboarding_id);
    assert!(
        matches!(rejoin.poll(), Some(Err(Error::InvalidArgument(_)))),
        "the outcome is taken once"
    );
    drop(rejoined);
    let zero = runtime.start_rejoin(
        OnboardingId::ZERO,
        &PeerPresentation::default(),
        None,
        None,
        None,
        || {},
    );
    assert!(matches!(zero, Err(Error::InvalidArgument(_))));
    drop(runtime);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    drop(library);
    assert_eq!(fixture.control.live_handle_count(), 0);
}

#[test]
fn a_cancelled_or_timed_out_async_onboard_reports_a_timeout() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    fixture.control.set_onboard_async_delay(5_000);
    let pending = start_code_onboard(&runtime, None);
    let cancel = pending.cancel_handle().expect("a running call");
    std::thread::spawn(move || cancel.cancel())
        .join()
        .expect("cancel thread")
        .expect("cancelled");
    let cancelled = pending.wait().expect_err("cancelled");
    assert!(cancelled.has_code(StatusCode::TIMEOUT), "{cancelled}");

    let started = Instant::now();
    let timed_out = start_code_onboard(&runtime, Some(Duration::from_millis(20)))
        .wait()
        .expect_err("timed out");
    assert!(timed_out.has_code(StatusCode::TIMEOUT), "{timed_out}");
    assert!(started.elapsed() < Duration::from_secs(4));
    drop(runtime);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn a_dropped_async_onboard_is_cancelled_and_its_runtime_destroy_waits_for_the_callback() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    fixture.control.set_onboard_async_delay(5_000);
    let started = Instant::now();
    drop(start_code_onboard(&runtime, None));
    drop(runtime);
    assert!(
        started.elapsed() < Duration::from_secs(4),
        "nothing waited for the delay"
    );
    let calls = fixture.control.calls();
    assert!(
        position_of(&calls, "onboard_async called back") < position_of(&calls, "runtime destroyed"),
        "{calls:?}"
    );
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    drop(library);
    assert_eq!(fixture.control.live_handle_count(), 0);
}

#[test]
fn a_session_delivered_after_its_onboard_was_dropped_is_destroyed_by_the_runtime() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    fixture.control.set_onboard_async_delay(30);
    fixture.control.set_onboard_async_ignores_cancel(true);
    drop(start_code_onboard(&runtime, None));
    // The call finishes despite the cancel and delivers a session nobody holds.
    let deadline = Instant::now() + Duration::from_secs(5);
    while fixture
        .control
        .entries_starting_with("onboard_async called back")
        .is_empty()
    {
        assert!(Instant::now() < deadline, "the call never called back");
        std::thread::sleep(Duration::from_millis(1));
    }
    // The runtime's next call releases it, before its own session exists.
    let onboarded = runtime
        .onboard(
            &code("ABCD-EFGH"),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("onboard");
    let calls = fixture.control.calls();
    assert_eq!(
        calls
            .iter()
            .filter(|call| *call == "session_handle_destroy")
            .count(),
        1,
        "{calls:?}"
    );
    drop(onboarded);
    drop(runtime);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    drop(library);
    assert_eq!(fixture.control.live_handle_count(), 0);
}

#[test]
fn an_async_start_that_fails_never_calls_back() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let (wakes, wake) = counting_wake();
    let rejected = runtime
        .start_onboard(
            &code("WRONG"),
            &PeerPresentation::default(),
            &OnboardExtensions::default(),
            None,
            None,
            wake,
        )
        .expect_err("checked synchronously");
    assert!(rejected.has_code(StatusCode::PAKE_ERROR), "{rejected}");
    fixture.control.fail_next(
        "runtime_onboard_async",
        StatusCode::ONBOARDING_FAILED.value(),
    );
    let injected = runtime
        .start_onboard(
            &code("ABCD-EFGH"),
            &PeerPresentation::default(),
            &OnboardExtensions::default(),
            None,
            None,
            || {},
        )
        .expect_err("injected");
    assert!(
        injected.has_code(StatusCode::ONBOARDING_FAILED),
        "{injected}"
    );
    assert_eq!(wakes.load(Ordering::SeqCst), 0);
    assert_eq!(
        fixture
            .control
            .entries_starting_with("onboard_async called"),
        Vec::<String>::new()
    );
    drop(runtime);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn a_slot_is_re_enrolled_under_its_own_id() {
    let fixture = Fixture::new();
    let library = load_stub();
    let runtime = library
        .create_runtime(&fixture.runtime_config())
        .expect("runtime");
    let first = runtime
        .onboard(
            &code("ABCD-EFGH"),
            &PeerPresentation::default(),
            &BlockingOptions::default(),
        )
        .expect("onboard");
    let slot = first.onboarding_id;
    let re_enroll = OnboardExtensions {
        onboarding_id: Some(slot),
        ..OnboardExtensions::default()
    };
    let still_active = runtime
        .onboard_with(
            &code("ABCD-EFGH"),
            &PeerPresentation::default(),
            &re_enroll,
            &BlockingOptions::default(),
        )
        .expect_err("the slot's session is alive");
    assert!(
        still_active.has_code(StatusCode::ONBOARDING_SESSION_STILL_ACTIVE),
        "{still_active}"
    );
    drop(first);
    let again = runtime
        .onboard_with(
            &code("ABCD-EFGH"),
            &PeerPresentation::default(),
            &re_enroll,
            &BlockingOptions::default(),
        )
        .expect("re-enrolled");
    assert_eq!(again.onboarding_id, slot);
    assert_eq!(
        fixture
            .control
            .entries_starting_with("re-enrolled slot")
            .len(),
        1
    );
    drop(again);

    for (method, onboarding_id) in [(code("ABCD-EFGH"), OnboardingId::ZERO), (local_app(), slot)] {
        let refused = runtime
            .onboard_with(
                &method,
                &PeerPresentation::default(),
                &OnboardExtensions {
                    onboarding_id: Some(onboarding_id),
                    ..OnboardExtensions::default()
                },
                &BlockingOptions::default(),
            )
            .expect_err("refused before the call");
        assert!(matches!(refused, Error::InvalidArgument(_)), "{refused}");
    }
    let ephemeral = library
        .create_runtime(&RuntimeConfig {
            ephemeral_storage: true,
            ..fixture.runtime_config()
        })
        .expect("ephemeral runtime");
    let refused = ephemeral
        .onboard_with(
            &code("ABCD-EFGH"),
            &PeerPresentation::default(),
            &re_enroll,
            &BlockingOptions::default(),
        )
        .expect_err("an ephemeral runtime keeps no slot");
    assert!(
        refused.has_code(StatusCode::ARGUMENT_NOT_SUPPORTED),
        "{refused}"
    );
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[cfg(qaros_device_identity)]
mod device_identity {
    use qaros::{DeviceIdentity, SourceDeviceCategory};

    use super::*;

    fn camera() -> DeviceIdentity {
        DeviceIdentity {
            category: SourceDeviceCategory::Camera,
            device_id: "LuxonisOakD".to_string(),
            device_family: Some("Luxonis".to_string()),
            device_name: Some("OAK-D".to_string()),
        }
    }

    #[test]
    fn a_declared_device_is_chained_on_the_presentation_and_updated_on_the_session() {
        let fixture = Fixture::new();
        let library = load_stub();
        let runtime = library
            .create_runtime(&fixture.runtime_config())
            .expect("runtime");
        let presentation = PeerPresentation {
            device_identity: Some(camera()),
            ..PeerPresentation::default()
        };
        let onboarded = runtime
            .onboard(
                &code("ABCD-EFGH"),
                &presentation,
                &BlockingOptions::default(),
            )
            .expect("onboard as a camera");
        let declared = "declared device category=5 id=LuxonisOakD family=Luxonis name=OAK-D";
        assert_eq!(
            fixture.control.entries_starting_with("declared device"),
            vec![declared.to_string()]
        );
        onboarded
            .session
            .update_device_identity(&DeviceIdentity {
                category: SourceDeviceCategory::Server,
                device_id: "RenderNode".to_string(),
                ..DeviceIdentity::default()
            })
            .expect("update");
        assert_eq!(
            fixture.control.entries_starting_with("updated device"),
            vec!["updated device category=2 id=RenderNode family= name=".to_string()]
        );
        let empty_id = onboarded
            .session
            .update_device_identity(&DeviceIdentity::default())
            .expect_err("an empty device id");
        assert!(
            empty_id.has_code(StatusCode::ARGUMENT_NOT_SUPPORTED),
            "{empty_id}"
        );
        let interior_nul = onboarded
            .session
            .update_device_identity(&DeviceIdentity {
                device_id: "a\0b".to_string(),
                ..DeviceIdentity::default()
            })
            .expect_err("an interior NUL");
        assert!(matches!(interior_nul, Error::InvalidArgument(_)));
        let onboarding_id = onboarded.onboarding_id;
        drop(onboarded);

        let rejoined = runtime
            .rejoin(onboarding_id, &presentation, &BlockingOptions::default())
            .expect("rejoin as a camera");
        assert_eq!(
            fixture
                .control
                .entries_starting_with("declared device")
                .len(),
            2
        );
        drop(rejoined);
        assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    }
}

#[cfg(qaros_member_hubs)]
mod member_hubs {
    use std::num::NonZeroU32;

    use qaros::{
        JoinHubAsMemberConfig, MemberHubInvitation, MemberHubInvitationConfig, MemberHubPermissions,
    };

    use super::*;

    fn granting(tag: &str) -> MemberHubInvitationConfig {
        MemberHubInvitationConfig {
            label: Some("Simulation clients".to_string()),
            app_grant_tag: Some(tag.to_string()),
            ..MemberHubInvitationConfig::default()
        }
    }

    #[test]
    fn an_invitation_crosses_to_the_joining_pc_and_its_app_grant_is_redeemed() {
        let fixture = Fixture::new();
        let library = load_stub();
        let primary = library
            .create_runtime(&fixture.runtime_config())
            .expect("primary runtime");
        let invitation = primary
            .create_member_hub_invitation(&granting("left.member.qaros"), None)
            .expect("invitation");
        assert_eq!(
            fixture.control.entries_starting_with("member invitation"),
            vec![
                "member invitation lifetime=86400 max_uses=1 permissions=0xb app_grant=left.member.qaros"
                    .to_string()
            ]
        );
        let wire_bytes = invitation.serialize().expect("serialize");
        drop(invitation);

        let joining = library
            .create_runtime(&RuntimeConfig {
                storage_directory: Some(fixture.storage.join("joining")),
                ..fixture.runtime_config()
            })
            .expect("joining runtime");
        let received = library
            .deserialize_member_hub_invitation(&wire_bytes)
            .expect("deserialize");
        let (progress, reports) = mpsc::channel();
        joining
            .join_hub_as_member(
                &received,
                &JoinHubAsMemberConfig {
                    restart_timeout: Some(Duration::from_secs(30)),
                    ..JoinHubAsMemberConfig::default()
                },
                &BlockingOptions {
                    progress: Some(progress),
                    cancel: None,
                },
            )
            .expect("joined");
        assert_eq!(
            fixture.control.entries_starting_with("joined as member"),
            vec!["joined as member hub_start_timeout_ms=0 restart_timeout_ms=30000".to_string()]
        );
        assert!(reports.try_recv().is_ok(), "a progress report");

        let onboarded = joining
            .onboard_with(
                &local_app(),
                &PeerPresentation::default(),
                &OnboardExtensions {
                    remote_app_grant: Some(&received),
                    ..OnboardExtensions::default()
                },
                &BlockingOptions::default(),
            )
            .expect("redeemed");
        assert_eq!(
            fixture
                .control
                .entries_starting_with("local app remote app grant"),
            vec!["local app remote app grant tag=left.member.qaros".to_string()]
        );
        let refused = joining
            .onboard_with(
                &code("ABCD-EFGH"),
                &PeerPresentation::default(),
                &OnboardExtensions {
                    remote_app_grant: Some(&received),
                    ..OnboardExtensions::default()
                },
                &BlockingOptions::default(),
            )
            .expect_err("a grant needs the local-app mode");
        assert!(matches!(refused, Error::InvalidArgument(_)), "{refused}");
        drop(onboarded);
        drop(received);
        drop(joining);
        drop(primary);
        assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
        drop(library);
        assert_eq!(fixture.control.live_handle_count(), 0);
    }

    #[test]
    fn a_hub_that_is_not_a_member_ignores_the_grant() {
        let fixture = Fixture::new();
        let library = load_stub();
        let runtime = library
            .create_runtime(&fixture.runtime_config())
            .expect("runtime");
        let invitation = runtime
            .create_member_hub_invitation(&granting("left.member.qaros"), None)
            .expect("invitation");
        let onboarded = runtime
            .onboard_with(
                &local_app(),
                &PeerPresentation::default(),
                &OnboardExtensions {
                    remote_app_grant: Some(&invitation),
                    ..OnboardExtensions::default()
                },
                &BlockingOptions::default(),
            )
            .expect("admitted as any local app");
        assert_eq!(
            fixture.control.entries_starting_with("local app remote app grant"),
            vec!["local app remote app grant ignored: this hub is not a member (tag=left.member.qaros)".to_string()]
        );
        drop(onboarded);
        drop(invitation);
        assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    }

    #[test]
    fn an_invitation_out_of_range_is_refused_where_the_header_refuses_it() {
        let fixture = Fixture::new();
        let library = load_stub();
        let runtime = library
            .create_runtime(&fixture.runtime_config())
            .expect("runtime");
        let before_the_call = [
            MemberHubInvitationConfig {
                lifetime: Some(Duration::from_millis(500)),
                ..MemberHubInvitationConfig::default()
            },
            MemberHubInvitationConfig {
                permissions: Some(MemberHubPermissions::default()),
                ..MemberHubInvitationConfig::default()
            },
            MemberHubInvitationConfig {
                label: Some("a\0b".to_string()),
                ..MemberHubInvitationConfig::default()
            },
        ];
        for config in &before_the_call {
            let refused = runtime
                .create_member_hub_invitation(config, None)
                .expect_err("refused before the call");
            assert!(matches!(refused, Error::InvalidArgument(_)), "{refused}");
        }
        let by_the_library = [
            MemberHubInvitationConfig {
                lifetime: Some(Duration::from_secs(8 * 24 * 3600)),
                ..MemberHubInvitationConfig::default()
            },
            MemberHubInvitationConfig {
                max_uses: NonZeroU32::new(65),
                ..MemberHubInvitationConfig::default()
            },
            MemberHubInvitationConfig {
                label: Some("x".repeat(65)),
                ..MemberHubInvitationConfig::default()
            },
            granting("member/left"),
        ];
        for config in &by_the_library {
            let refused = runtime
                .create_member_hub_invitation(config, None)
                .expect_err("refused by the library");
            assert!(
                refused.has_code(StatusCode::ARGUMENT_NOT_SUPPORTED),
                "{refused}"
            );
        }
        let all = MemberHubPermissions::JOIN
            | MemberHubPermissions::REJOIN
            | MemberHubPermissions::READMIT
            | MemberHubPermissions::BRING_DEVICES;
        assert!(all.contains(MemberHubPermissions::READMIT));
        let invitation = runtime
            .create_member_hub_invitation(
                &MemberHubInvitationConfig {
                    lifetime: Some(Duration::from_secs(3600)),
                    max_uses: NonZeroU32::new(3),
                    permissions: Some(all),
                    ..MemberHubInvitationConfig::default()
                },
                None,
            )
            .expect("invitation");
        assert_eq!(
            fixture.control.entries_starting_with("member invitation"),
            vec![
                "member invitation lifetime=3600 max_uses=3 permissions=0xf app_grant=-"
                    .to_string()
            ]
        );
        drop(invitation);
        assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    }

    #[test]
    fn a_damaged_or_expired_invitation_is_refused() {
        let fixture = Fixture::new();
        let library = load_stub();
        let runtime = library
            .create_runtime(&fixture.runtime_config())
            .expect("runtime");
        let garbage = library
            .deserialize_member_hub_invitation(b"not an invitation")
            .expect_err("garbage");
        assert!(
            garbage.has_code(StatusCode::MEMBER_INVITATION_INVALID),
            "{garbage}"
        );

        let invitation = runtime
            .create_member_hub_invitation(&MemberHubInvitationConfig::default(), None)
            .expect("invitation");
        let wire = String::from_utf8(invitation.serialize().expect("serialize")).expect("UTF-8");
        // The stub's format: the expiry is the second line; move it into the past.
        let mut lines: Vec<&str> = wire.split('\n').collect();
        lines[1] = "1";
        let expired: MemberHubInvitation = library
            .deserialize_member_hub_invitation(lines.join("\n").as_bytes())
            .expect("deserialize");
        let refused = runtime
            .join_hub_as_member(
                &expired,
                &JoinHubAsMemberConfig::default(),
                &BlockingOptions::default(),
            )
            .expect_err("expired");
        assert!(
            refused.has_code(StatusCode::MEMBER_INVITATION_EXPIRED),
            "{refused}"
        );
        let zero_timeout = runtime
            .join_hub_as_member(
                &invitation,
                &JoinHubAsMemberConfig {
                    hub_start_timeout: Some(Duration::ZERO),
                    ..JoinHubAsMemberConfig::default()
                },
                &BlockingOptions::default(),
            )
            .expect_err("a zero timeout");
        assert!(matches!(zero_timeout, Error::InvalidArgument(_)));
        fixture.control.fail_next(
            "runtime_join_hub_as_member",
            StatusCode::MEMBER_INVITATION_REVOKED.value(),
        );
        let revoked = runtime
            .join_hub_as_member(
                &invitation,
                &JoinHubAsMemberConfig::default(),
                &BlockingOptions::default(),
            )
            .expect_err("injected");
        assert!(
            revoked.has_code(StatusCode::MEMBER_INVITATION_REVOKED),
            "{revoked}"
        );
        drop(expired);
        drop(invitation);
        assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    }
}
