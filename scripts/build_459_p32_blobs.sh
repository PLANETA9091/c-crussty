#!/usr/bin/env bash
# build_459_p32_blobs.sh — TASK-459-57 (idea ID-P32 SNAP sidecar, закон 11
# тик-459): rebuild the inside_snap CHM-plane bridge (source of the Snap
# package-private nested type) + the NEW InsideSnapRegistryOps sidecar —
# ONE javac pass (lesson ×93: inter-source deps resolved by javac itself).
#
# Full cp = kernel round-396-a + fastutil + paper-api 1.21.10 +
# adventure-api/key 4.24.0 (mandate /home/z/tools, NEVER round-j2b-jar),
# --release 21 (major 65), NESTED path installed FIRST (include_bytes!
# contract) then flat legacy copy, then flat==nested byte-equality gate.
#
# Usage: scripts/build_459_p32_blobs.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${KERNEL_JAR:-research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL" ] || KERNEL="/home/z/tools/patched-kernel.jar"
[ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }

CP="$KERNEL:entityinside/build"
for j in /home/z/tools/fastutil.jar /home/z/tools/paper-api-1.21.10.jar \
         /home/z/tools/adventure-api-4.24.0.jar /home/z/tools/adventure-key-4.24.0.jar; do
  [ -f "$j" ] && CP="$CP:$j"
done

ALL_BUILD=$(mktemp -d)
trap 'rm -rf "$ALL_BUILD"' EXIT

"$JAVAC" --release 21 -nowarn -cp "$CP" -d "$ALL_BUILD" \
  entityinside/net/minecraft/world/entity/InsideSnapOps.java \
  entityinside/net/minecraft/world/entity/InsideSnapRegistryOps.java

install_blob() { # fqcn(slash-form) — nested (include_bytes!) + flat (legacy) + equality gate
  local cls="$1" base nested
  base=$(basename "$cls")
  nested="entityinside/build/$cls.class"
  cp "$ALL_BUILD/$cls.class" "$nested"
  cp "$ALL_BUILD/$cls.class" "entityinside/build/$base.class"
  if ! cmp -s "$nested" "entityinside/build/$base.class"; then
    echo "FATAL: flat!=nested for $cls" >&2
    exit 1
  fi
  echo "blob: $nested ($(stat -c%s "$nested") bytes) + flat (flat==nested OK)"
}

install_blob net/minecraft/world/entity/InsideSnapOps
install_blob net/minecraft/world/entity/InsideSnapOps\$Snap
install_blob net/minecraft/world/entity/InsideSnapOps\$Lane
install_blob net/minecraft/world/entity/InsideSnapRegistryOps
