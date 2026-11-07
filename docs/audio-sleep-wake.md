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
name and WINEPREFIX are retained from process arguments; unrelated environment
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
