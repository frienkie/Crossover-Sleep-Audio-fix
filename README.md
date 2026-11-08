# CrossPuck Audio

A lightweight macOS menu bar app that keeps CrossOver audio following the
system output, including recovery after sleep and wake. Steam is not required.
This fork contains only the audio service and its menu; controller support,
Windows drivers, bottle editing, profiling tools and the updater are removed.

## Use

Requires macOS 14.2 or later. Open CrossPuck.app and allow System Audio Recording
when requested. Enable **Follow macOS Audio Output** in the speaker menu.

- **Monitoring · Output**: watching the system output. Programs already using
  that output continue playing directly; no extra capture is needed.
- **Rerouting · Output**: forwarding eligible CrossOver audio from a different
  device to this output.
- **Language**: switch instantly between English, 简体中文 and 日本語. The choice
  persists; first launch follows the system's preferred language.
- **Quit**: stop the service and release its private audio route.

The service handles direct Wine clients and CrossOver-generated Menu Helper
shortcuts, including multiple programs in one bottle. Audio stays in memory;
no recording is saved or uploaded. The app has no update checks or network
service. Input Monitoring permission is not used.

## Build

Install Rust and Apple's Command Line Tools, then run:

```sh
sh tools/build-app.sh release
open target/release/CrossPuck.app
```

The script builds only the macOS app and audio helper. No Windows toolchain,
third-party framework download or guest-driver installation is needed. Builds
are locally signed by default. Set `CROSSPUCK_APP_SIGN_IDENTITY` for Developer ID
signing. Source changes can require renewed macOS audio permission after a
local-signature change; use a consistent signing identity for distribution.

To install, quit the running CrossPuck and replace `/Applications/CrossPuck.app`
with the built bundle. Keep a copy of the old bundle if you need to roll back.
Replacing a previous full bundle must remove its old Frameworks and GuestDriver
resources as well; do not overlay the audio-only build onto it.

## Verify and diagnose

```sh
cargo test --locked --workspace
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo fmt --check
sh tools/test-audio-engine.sh
```

The Rust tests cover the helper lifecycle, retries and localization. Native
ASan/UBSan regressions cover audio buffers, process discovery, shortcut identity
verification and sleep/wake invalidation. Physical speaker playback and repeated
sleep/wake recovery still require hardware testing.

Read-only process inventory (does not create a tap):

```sh
/Applications/CrossPuck.app/Contents/MacOS/CrossPuckAudio --list
```

Set `CROSSPUCK_LOG_LEVEL=debug` when launching the app for detailed logs. The
system log subsystem remains `com.github.scryner.crosspuck`, and existing audio
preferences are preserved. See [audio design](docs/audio-routing.md) for routing,
recovery and process-selection details.
