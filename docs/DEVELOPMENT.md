# Developer guide

Install Rust and Apple's Command Line Tools on macOS 14.2 or later:

```sh
sh tools/build-app.sh release
open "target/release/Crossover Sleep-Audio-fix.app"
```

Only the macOS menu and native audio helper are built. No Windows toolchain,
updater framework or controller driver is required. The build is locally signed
by default. Set SLEEP_AUDIO_FIX_SIGN_IDENTITY for Developer ID signing and
SLEEP_AUDIO_FIX_BUILD_VERSION for a numeric build override. The previous
CROSSPUCK_APP_SIGN_IDENTITY and CROSSPUCK_BUILD_VERSION variables remain accepted.

Quit the running old app before installing. Replace the complete bundle rather
than overlaying it, and keep a backup if rollback is needed. The renamed bundle
is /Applications/Crossover Sleep-Audio-fix.app. Do not leave CrossPuck.app running
alongside it. Local-signature changes can require renewed audio permission.

Verification:

```sh
cargo fmt --check
cargo test --locked --workspace
cargo clippy --locked --workspace --all-targets -- -D warnings
sh tools/test-audio-engine.sh
sh tools/build-app.sh release
```

Read-only process inventory (creates no tap):

```sh
"/Applications/Crossover Sleep-Audio-fix.app/Contents/MacOS/SleepAudioHelper" --list
```

Use SLEEP_AUDIO_FIX_LOG_LEVEL=debug for detailed logs. CROSSPUCK_LOG_LEVEL remains
accepted. The legacy bundle/signing identifiers, log subsystem
com.github.scryner.crosspuck and routing lease name are deliberately retained to
preserve preferences and avoid simultaneous old/new audio services. macOS may
still ask for audio permission again when a local signature or path changes.

The source package is crossover-sleep-audio-fix, with a SleepAudioFix menu binary
and SleepAudioHelper native helper. PCM never crosses their control channel.
Automated tests do not replace real-device playback and repeated sleep/wake
checks. The existing audio engine and generated Menu Helper discovery are kept.
