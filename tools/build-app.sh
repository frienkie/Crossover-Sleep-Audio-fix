#!/usr/bin/env sh
set -eu
root_dir="$(cd "$(dirname "$0")/.." && pwd)"
profile="${1:-release}"
case "$profile" in
  release) cargo_profile=release ;;
  debug) cargo_profile=dev ;;
  *) echo "Usage: $0 [release|debug]" >&2; exit 2 ;;
esac
export MACOSX_DEPLOYMENT_TARGET=14.2
cargo build --locked --manifest-path "$root_dir/Cargo.toml" \
  --target-dir "$root_dir/target" -p crossover-sleep-audio-fix --profile "$cargo_profile"
# The helper needs no Rust runtime; keep its entry point native and small.
xcrun clang -fobjc-arc -std=gnu11 -Os -Wall -Wextra -Werror \
  -mmacosx-version-min=14.2 \
  "$root_dir/crates/crossover-sleep-audio-fix/src/audio/helper.m" \
  "$root_dir/crates/crossover-sleep-audio-fix/src/audio/router.m" \
  -framework Foundation -framework AppKit -framework CoreAudio \
  -o "$root_dir/target/$profile/SleepAudioHelper"
app_dir="$root_dir/target/$profile/Crossover Sleep-Audio-fix.app"
staging="$(mktemp -d "$root_dir/target/$profile/.bundle.XXXXXX")"
trap 'rm -rf "$staging"' EXIT
bundle="$staging/Crossover Sleep-Audio-fix.app"
mkdir -p "$bundle/Contents/MacOS" "$bundle/Contents/Resources"
for binary in SleepAudioFix SleepAudioHelper; do
  cp "$root_dir/target/$profile/$binary" "$bundle/Contents/MacOS/$binary"
done
cp "$root_dir/crates/crossover-sleep-audio-fix/Info.plist" "$bundle/Contents/Info.plist"
cp -R "$root_dir/crates/crossover-sleep-audio-fix/Resources/." "$bundle/Contents/Resources/"
cp "$root_dir/LICENSE" "$root_dir/THIRD-PARTY-NOTICES.md" "$bundle/Contents/Resources/"
version="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$root_dir/Cargo.toml" | head -1)"
build_version="${SLEEP_AUDIO_FIX_BUILD_VERSION:-${CROSSPUCK_BUILD_VERSION:-$version}}"
case "$build_version" in ''|*[!0-9.]*) echo "Build version must be numeric." >&2; exit 2 ;; esac
/usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $version" "$bundle/Contents/Info.plist"
/usr/libexec/PlistBuddy -c "Set :CFBundleVersion $build_version" "$bundle/Contents/Info.plist"
commit="$(git -C "$root_dir" rev-parse HEAD 2>/dev/null || true)"
if [ -n "$commit" ]; then
  /usr/libexec/PlistBuddy -c "Add :SleepAudioFixBuildCommit string $commit" "$bundle/Contents/Info.plist"
fi
identity="${SLEEP_AUDIO_FIX_SIGN_IDENTITY:-${CROSSPUCK_APP_SIGN_IDENTITY:--}}"
if [ "$identity" = "-" ]; then
  codesign --force --sign - --identifier com.github.scryner.crosspuck.audio "$bundle/Contents/MacOS/SleepAudioHelper"
  codesign --force --sign - --identifier com.github.scryner.crosspuck "$bundle"
else
  codesign --force --timestamp --options runtime --sign "$identity" \
    --identifier com.github.scryner.crosspuck.audio "$bundle/Contents/MacOS/SleepAudioHelper"
  codesign --force --timestamp --options runtime --sign "$identity" \
    --identifier com.github.scryner.crosspuck "$bundle"
fi
codesign --verify --deep --strict "$bundle"
rm -rf "$app_dir"
mv "$bundle" "$app_dir"
echo "$app_dir"
