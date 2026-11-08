# Audio routing and recovery

The menu starts a supervised SleepAudioHelper child process. Core Audio calls,
permission waits and PCM processing stay in that helper. Its stdin pipe ties
its lifetime to the menu app. Quit, disable, a crash or a watchdog timeout
releases the private tap, restoring the original application output. Retrying
uses bounded backoff. A per-user lease prevents duplicate routing helpers.

The engine monitors the macOS default output, scans active audio clients about
once per second and checks an active route's device health every 200 ms. Both
workspace sleep and wake notifications invalidate the current route, including
same-device-ID recovery. Route creation pauses while sleeping, and wake causes
client rediscovery and a new tap/aggregate when output differs. The menu has no
periodic refresh timer; its state refreshes when opened or changed.

Clients must match the CrossOver or CrossOverHelper bundle prefix and a verified
CrossOver bottle. Direct Wine processes use WINEPREFIX. Generated Menu Helper
shortcuts can lack that variable: the actual executable and its bundle must
match the Core Audio identity, then CXHelperAppBottleName and the colon-separated
CX_BOTTLE_PATH locate a bottle with system.reg and cxbottle.conf. Search order
matches CrossOver. The selected controller/Steam bottle is irrelevant; neither
controller support nor Steam integration exists in this fork.

When native output already matches, no route is created, preserving native
volume. A mismatch creates a private stereo process tap and private aggregate
on the default device. Physical inputs are disabled and verified before IO.
Float32 sample copying and a short gain ramp run in the callback without locks,
allocation or HAL reconfiguration. Unsupported formats or callback failure
release the route and retry. Output changes can briefly interrupt playback.

The monitoring state also covers no active eligible clients. It is not proof
that an excluded application is using the correct device. Routing progress
means callback frames are advancing, including valid digital silence; it does
not measure audible speaker output.

## Read-only inventory

`SleepAudioHelper --list` includes all Core Audio clients except itself. Process
arguments retain only the command name and WINEPREFIX or CX_BOTTLE_PATH;
unrelated environment values are discarded. Nothing is uploaded.

| selection_reason | Meaning |
| --- | --- |
| bundle_id_mismatch | Not a recognized CrossOver bundle ID. |
| unverified_bottle | Process environment and bundle do not identify a valid bottle. |
| inactive | No active output reported. |
| no_reported_output_devices | Empty/unavailable device metadata; output is unknown. |
| already_default | All reported output devices match the default. |
| routing_target | Eligible client with a different output. |

Property-read errors currently become empty/zero metadata. An empty list is not
proof of native/default playback. Helper fallback is marked
`identity_source: crossover_helper_bundle`. Absolute bottle names in generated
helper metadata and managed-bottle lookup are not implemented by that fallback;
direct WINEPREFIX-based discovery remains available.

## Languages and permissions

The menu supports English, Simplified Chinese and Japanese, with a persistent
MenuLanguage preference. Native permission descriptions are localized through
InfoPlist.strings and follow macOS's application language selection. Menu
language changes do not replace the operating system's own dialog language.

Only System Audio Recording is needed. No microphone or HID input is captured.
There are no network requests, listening sockets, updater frameworks, Windows
DLLs, input monitoring or controller-driven power assertions.
