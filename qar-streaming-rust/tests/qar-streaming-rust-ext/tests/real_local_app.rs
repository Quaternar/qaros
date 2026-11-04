//! `vulkan_cube` end to end as QAROS starts an app that follows "Launched by QAROS": the test
//! starts it with a piped stdin and no arguments, the cube joins the QAROS Hub on this PC as a
//! local app and renders for the targets it finds, and closing its stdin must stop it with exit
//! code 0. Unlike the other tests of this crate it talks to a running Hub, not to
//! `qar-streaming-c-ext`, because local-app onboarding is served by the Hub only.
//!
//! - `QAR_INSTANCE`: inherited by the cube, as QAROS sets it for the apps it starts: the QAROS
//!   instance whose Hub the cube joins and whose runtime it loads (a development hub's instance
//!   name); unset, the installed QAROS.
//! - `QAROS_E2E_APP`: the `vulkan_cube` executable (skipped when unset).
//! - `QAROS_E2E_START_BUDGET`: seconds until the cube has joined, default 180, long enough to
//!   click Allow in QAROS on its first run.
//! - `QAROS_E2E_REQUIRE_TARGET=1`: also require that it renders for a target (a Visualizer
//!   receiver or a headset in the Hub's session) and keeps streaming to it for
//!   `QAROS_E2E_STREAM_SECONDS` (default 10).
//!
//! `cargo test --release -p qar-streaming-rust-ext --features e2e --test real_local_app -- --nocapture`

#![cfg(feature = "e2e")]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

mod common;

use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::time::{Duration, Instant};

const EXIT_BUDGET: Duration = Duration::from_secs(30);
const POLL_INTERVAL: Duration = Duration::from_millis(100);
const JOINED_LINE: &str = "Connected. Waiting for rendering targets.";
const RENDERING_LINE: &str = "Rendering for target ";
const STREAM_ENDED_LINE: &str = "Stopped streaming to ";
const STOPPING_LINE: &str = "Stopping.";

#[test]
fn the_cube_joins_as_a_local_app_and_stops_when_stdin_closes() {
    let Some(program) = std::env::var_os("QAROS_E2E_APP").map(PathBuf::from) else {
        eprintln!("skipped: set QAROS_E2E_APP to the vulkan_cube executable");
        return;
    };
    let start_budget = Duration::from_secs(seconds_from_environment("QAROS_E2E_START_BUDGET", 180));
    let require_target = std::env::var("QAROS_E2E_REQUIRE_TARGET").is_ok_and(|value| value == "1");
    let stream_time = Duration::from_secs(seconds_from_environment("QAROS_E2E_STREAM_SECONDS", 10));
    let scratch = common::scratch_directory("local-app");
    eprintln!("scratch: {}", scratch.display());

    let started = Instant::now();
    let (app_child, app_stdin) = start_app(&program, &scratch);
    let mut app = ChildGuard(app_child);

    wait_for_line(&mut app.0, &scratch, JOINED_LINE, start_budget);
    eprintln!(
        "joined as a local app {:?} after the start",
        started.elapsed()
    );
    if require_target {
        wait_for_line(&mut app.0, &scratch, RENDERING_LINE, start_budget);
        eprintln!(
            "rendering for a target {:?} after the start",
            started.elapsed()
        );
        let streaming_until = Instant::now() + stream_time;
        while Instant::now() < streaming_until {
            assert_app_alive(&mut app.0, &scratch);
            std::thread::sleep(POLL_INTERVAL);
        }
    }

    // QAROS's Stop closes stdin and kills the app about 1.3 s later, so the time from here to
    // the exit is the margin that matters.
    let stdin_closed = Instant::now();
    drop(app_stdin);
    let status = wait_for_exit(&mut app.0, EXIT_BUDGET);
    eprintln!(
        "app exit: {status:?}, {:?} after stdin closed",
        stdin_closed.elapsed()
    );
    print_log_tail(&scratch);
    let stdout = read_log(&scratch, "app.stdout.log");
    assert!(
        stdout.lines().any(|line| line == STOPPING_LINE),
        "the cube took the end of stdin as its stop request"
    );
    if require_target {
        assert!(
            !stdout
                .lines()
                .any(|line| line.starts_with(STREAM_ENDED_LINE)),
            "no stream ended before the stop"
        );
    }
    assert_eq!(status, Some(0), "the cube exits cleanly when stdin closes");
}

fn seconds_from_environment(variable: &str, default_seconds: u64) -> u64 {
    std::env::var(variable)
        .ok()
        .map_or(default_seconds, |text| {
            text.parse()
                .unwrap_or_else(|error| panic!("{variable}: {error}"))
        })
}

/// The cube inherits this process's environment, `QAR_INSTANCE` included.
fn start_app(program: &Path, scratch: &Path) -> (Child, ChildStdin) {
    let mut app = Command::new(program)
        .current_dir(scratch)
        .stdin(Stdio::piped())
        .stdout(Stdio::from(
            std::fs::File::create(scratch.join("app.stdout.log")).expect("stdout log"),
        ))
        .stderr(Stdio::from(
            std::fs::File::create(scratch.join("app.stderr.log")).expect("stderr log"),
        ))
        .spawn()
        .expect("start the app");
    let stdin = app.stdin.take().expect("app stdin");
    (app, stdin)
}

/// Waits until the app's stdout has a line starting with `expected`; fails when the app exits
/// first or the budget runs out.
fn wait_for_line(app: &mut Child, scratch: &Path, expected: &str, budget: Duration) {
    let deadline = Instant::now() + budget;
    loop {
        let stdout = read_log(scratch, "app.stdout.log");
        if stdout.lines().any(|line| line.starts_with(expected)) {
            return;
        }
        assert_app_alive(app, scratch);
        if Instant::now() >= deadline {
            print_log_tail(scratch);
            panic!("no line {expected:?} within {budget:?}");
        }
        std::thread::sleep(POLL_INTERVAL);
    }
}

/// Kills the child when dropped, so a failing test leaves no process behind.
struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn assert_app_alive(app: &mut Child, scratch: &Path) {
    if let Some(status) = app.try_wait().expect("app state") {
        print_log_tail(scratch);
        panic!("the app exited early: {status}");
    }
}

fn wait_for_exit(app: &mut Child, budget: Duration) -> Option<i32> {
    let deadline = Instant::now() + budget;
    loop {
        if let Some(status) = app.try_wait().expect("app state") {
            return status.code();
        }
        if Instant::now() >= deadline {
            eprintln!("the app did not exit within {budget:?}; killing it");
            let _ = app.kill();
            let _ = app.wait();
            return None;
        }
        std::thread::sleep(POLL_INTERVAL);
    }
}

fn read_log(scratch: &Path, name: &str) -> String {
    std::fs::read_to_string(scratch.join(name))
        .unwrap_or_else(|error| panic!("{name} unreadable: {error}"))
}

fn print_log_tail(scratch: &Path) {
    for name in ["app.stderr.log", "app.stdout.log"] {
        let text = read_log(scratch, name);
        let lines: Vec<&str> = text.lines().collect();
        eprintln!("--- {name}, last lines ---");
        for line in &lines[lines.len().saturating_sub(25)..] {
            eprintln!("{line}");
        }
    }
}
