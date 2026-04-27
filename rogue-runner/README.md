<!-- Unlicense — public domain — cochranblock.org -->
<!-- Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->
# Rogue Runner

1000-level endless runner. Procedural generation, offline, cross-platform.

## Targets

| Platform | Build | Notes |
|----------|-------|-------|
| **Web** | `scripts/build-web.sh` (from repo root) | wasm32. Serves at `/apps/rogue-runner-wasm` |
| **Windows** | `scripts/build-windows.sh` (from repo root) | x86_64-pc-windows-gnu. Needs mingw-w64 on Linux |
| **iOS Simulator** | `scripts/build-ios-sim.sh` (from repo root) | macOS only. x86_64-apple-ios |
| **Android** | `scripts/build-android.sh` (from repo root) | Docker + cargo-quad-apk |

## Web

```sh
# From repo root:
scripts/build-web.sh
# Wasm copied to rogue-repo assets. Run rogue-repo, open /apps/rogue-runner-wasm
```

## Native (host)

```sh
cargo run -p rogue-runner
```
