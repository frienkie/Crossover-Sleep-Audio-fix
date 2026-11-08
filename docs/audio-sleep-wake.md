# Audio sleep/wake recovery and process diagnostics

The helper observes workspace sleep and wake notifications. Both invalidate the
current route even if the default device ID remains unchanged. The control loop
tears down an active route after observing sleep, defers new routes until wake,
then rediscovers clients and rebuilds the tap and aggregate. Notification handlers
only update atomic state and wake the control loop; they never call HAL.

This is recovery hardening, not a verified fix for every post-wake routing
failure. It cannot correct stale process-device metadata or identify a silent
tap merely from continuing IO callbacks. Real-device sleep/wake testing is still
required.

## Understanding the menu

`Following <output>` means the default output is known and no mismatched,
eligible audio clients were found at the latest scan. It does not establish that
all audible programs have been routed. `Playing through <output>` means the
route's IO callback has made progress; it does not measure audible sound.

Audio routing does not require Steam. Each audio process is checked separately,
including multiple programs in the same bottle and their audio-producing child
processes. The controller's selected bottle does not restrict audio discovery.

## Read-only inventory

While the affected programs are playing, run:

```sh
"/Applications/CrossPuck.app/Contents/MacOS/CrossPuckAudio" --list
```

Use the executable from the newly built bundle to obtain the expanded inventory.
The command creates no tap and changes no output settings. It includes all
Core Audio process objects except the helper itself, including excluded clients.
`selection_reason` explains the first exclusion or identifies a routing target:

| Reason | Meaning and next investigation |
| --- | --- |
| `bundle_id_mismatch` | Bundle ID does not match the CrossOver prefix. Verify the actual audio process identity before adjusting the allowlist. |
| `unverified_bottle` | WINEPREFIX could not be verified against a bottle containing cxbottle.conf. Inspect the actual launch environment and bottle path. |
| `inactive` | Core Audio does not report active output. Compare before and after wake during continuous playback. |
| `no_reported_output_devices` | Device metadata is empty or unavailable. This is unknown output, not proof that playback matches the default. |
| `already_default` | All reported device IDs match the default. Compare with the actual audible output if this seems wrong. |
| `routing_target` | Eligible active process reports an output different from the default. The next control scan should attempt routing. |

The original inventory filtered out bundle and bottle mismatches, so their
absence from an old helper's inventory is inconclusive. Automatic capture
eligibility is unchanged by the expanded diagnostics. Only the parsed command
name, WINEPREFIX or CX_BOTTLE_PATH are retained from process arguments; unrelated environment
values are discarded. Property-read failures are still represented as zero or
empty metadata, so the inventory does not distinguish every HAL error.

## Validation

Run `sh tools/test-audio-engine.sh`. The native sanitizer suite covers same-ID
sleep/wake invalidation and notifications arriving before and during a control
wait. These tests call the notification handler directly and do not establish
real workspace notification delivery or hardware recovery.

The native suite, a standalone optimized warnings-as-errors compilation of
router.m, the locked Rust release build of crosspuck-app, and all 78 application
tests passed for this change. The application tests require permission to open
local loopback connections. Physical sleep/wake validation remains outstanding.
Test simultaneous programs in one
bottle, independent program restarts, and at least three real sleep/wake cycles,
comparing per-process inventory with audible output each time.

## CrossOver-generated application shortcuts

An active audio client launched through a generated application shortcut can
report a `com.codeweavers.CrossOverHelper.*` bundle ID and run as `Menu Helper`.
The previous filter accepted only `com.codeweavers.CrossOver.*`. The helper can
also lack WINEPREFIX in its initial process arguments, while providing the
colon-separated CX_BOTTLE_PATH search path. These are independent exclusions:
accepting the bundle ID alone still fails the original bottle verification.

Both bundle prefixes are now accepted. Direct Wine clients still use verified
WINEPREFIX. For a generated helper without that variable, the engine checks the
actual executable path returned by proc_pidpath against its application's
CFBundleExecutable and matches the bundle ID with the Core Audio identity.
It reads CXHelperAppBottleName from that bundle and follows CX_BOTTLE_PATH
search order, requiring system.reg and a regular cxbottle.conf file. Relative
search roots, invalid bottle names, mismatched executables and bundle IDs are
rejected. It does not execute the shortcut command or alter the bottle.

The inventory marks this fallback as
`identity_source: crossover_helper_bundle`. Native sanitizer tests cover helper
argument parsing, explicit WINEPREFIX precedence, matching and mismatched
identities, paths with spaces, missing bottle files, and duplicate bottle-name
search precedence. A live read-only check confirmed a previously excluded
active helper is now a routing target. Audible playback and repeated sleep/wake
recovery still require validation after installation.
