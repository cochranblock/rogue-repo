# Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved.
# Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#!/bin/bash
# Build rogue-runner APK for Android (requires Docker + cargo-quad-apk)
# docker pull notfl3/cargo-apk
# cargo install cargo-quad-apk
set -e
cd "$(dirname "$0")/.."
docker run --rm -v "$(pwd):/root/src" -w /root/src notfl3/cargo-apk cargo quad-apk build --release
echo "APK: target/android-artifacts/release/apk/"
