#pragma once
// What the member hub cube's server and client share: stopping, joining the
// hub of this PC as a local app, the GPU, and rendering a one-colored cube for
// the targets the app chooses.

#include "renderer.hpp"

#include <array>
#include <atomic>
#include <string>

// Set by Ctrl+C, SIGTERM or a closed stdin pipe (QAROS stops an app it started
// by closing its stdin).
extern std::atomic<bool> g_stopRequested;

// Ctrl+C, SIGTERM and, when stdin is a pipe, its end request a stop.
void WatchForStop();

// Prints the progress messages of a QAROS call.
void PrintProgress(QarActionSeverity severity, float percent, const char* message, void* state);

// Joins the hub of this PC as a local app. `tag` names the app (its peer id
// stays the same over restarts); `grant`, when given, redeems a remote-app
// grant instead, whose tag then applies. Ctrl+C cancels the call, which can
// wait minutes for an approval.
QarResult JoinAsLocalApp(
	QarRuntime* runtime,
	const char* displayName,
	const char* tag,
	const QarMemberHubInvitation* grant,
	QarSession** session
);

// Tells the user why joining failed and what to do; returns the exit code
// (3 to 7, else 1).
int ExplainJoinFailure(QarResult result);

// Prints "Joined QAROS as peer <id>".
void PrintMyPeerId(const QarSession* session);

// The session's GPU, in the form VulkanDevice::Create takes; empty for none.
std::string SessionAdapter(const QarSession* session);

struct CubeOptions
{
	// The cube's color, linear RGB.
	std::array<float, 3> color{};
	// Render only for target apps whose display name contains this text;
	// empty renders for every target app in the session.
	std::string targetFilter;
	// The app volume the cube sits in.
	const char* volumeName = nullptr;
	const char* volumeDisplayName = nullptr;
};

// Renders the cube for every running target app the filter admits until a
// stop. Returns whether it ran without a failure.
bool RenderCube(QarSession* session, VulkanDevice& gpu, const CubeOptions& options);
