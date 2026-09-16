#!/usr/bin/env bash
# ensure_javap.sh — idempotent local javap via Temurin JDK 21 in /tmp (task165).
# Sandbox JRE has no javap and no sudo; JDK tarball is fetched once and cached.
set -euo pipefail
if [ -x /tmp/jdk21/bin/javap ]; then echo "javap ready: /tmp/jdk21/bin/javap"; exit 0; fi
if ls /usr/lib/jvm/*/bin/javap >/dev/null 2>&1; then echo "javap present in system JVM"; exit 0; fi
mkdir -p /tmp/jdk21-dl && cd /tmp/jdk21-dl
[ -s jdk21.tar.gz ] || curl -sL -o jdk21.tar.gz \
  "https://api.adoptium.net/v3/binary/latest/21/ga/linux/x64/jdk/hotspot/normal/eclipse"
tar xzf jdk21.tar.gz
DIR="$(find /tmp/jdk21-dl -maxdepth 1 -name 'jdk-21*' -type d | head -1)"
rm -rf /tmp/jdk21 && mv "$DIR" /tmp/jdk21
/tmp/jdk21/bin/javap -version && echo "javap ready: /tmp/jdk21/bin/javap"
