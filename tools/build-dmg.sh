#!/usr/bin/env sh
set -eu
root_dir="$(cd "$(dirname "$0")/.." && pwd)"
version="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$root_dir/Cargo.toml" | head -1)"
tag="${1:-v$version}"
case "$tag" in ''|*[!a-zA-Z0-9._-]*) echo "Invalid release tag." >&2; exit 2 ;; esac
sh "$root_dir/tools/build-app.sh" release
app="$root_dir/target/release/Crossover Sleep-Audio-fix.app"
architecture="$(uname -m)"
output_dir="$root_dir/target/dmg"
mkdir -p "$output_dir"
output="$output_dir/Crossover-Sleep-Audio-fix-$tag-macOS-$architecture.dmg"
if [ -e "$output" ]; then
  echo "Refusing to overwrite an existing DMG: $output" >&2
  exit 1
fi
staging="$(mktemp -d "$root_dir/target/.dmg.XXXXXX")"
trap 'rm -rf "$staging"' EXIT
ditto "$app" "$staging/Crossover Sleep-Audio-fix.app"
ln -s /Applications "$staging/Applications"
cp "$root_dir/README.md" "$root_dir/LICENSE" "$staging/"
codesign --verify --deep --strict "$staging/Crossover Sleep-Audio-fix.app"
hdiutil create -quiet -volname "Crossover Sleep-Audio-fix" \
  -srcfolder "$staging" -format UDZO "$output"
hdiutil verify "$output"
echo "$output"
