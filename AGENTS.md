# Repository Guidelines

This fork is an audio-only macOS menu bar utility. Keep controller code, Windows
DLLs, bottle editors, network services and automatic updaters out of the app.
Preserve safe audio cleanup, helper isolation and sleep/wake recovery.

Write code, comments and documentation in English, except localized UI strings.
Keep English, Simplified Chinese and Japanese menu text in sync.

Use feature/<description>, hotfix/<description> and release/<version> branches,
and Conventional Commits (feat, fix, refactor, docs, test, build or chore).
Do not overwrite published release assets or rewrite existing branch history.

Use the workspace version for the application display version. Keep release
notes as English Markdown at docs/releases/v<version>.md. This project has no
updater feed or framework. Source builds must be signed, and must not reset TCC.

Run cargo fmt --check, cargo test --locked --workspace, strict Clippy,
sh tools/test-audio-engine.sh and sh tools/build-app.sh release for app changes.
Verify the complete bundle signature and ensure Frameworks and GuestDriver are
absent. Real-device playback and sleep/wake checks supplement automated tests.
