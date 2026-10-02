#!/usr/bin/env bash
# build_queryplane_ops.sh — TASK-412-B/413-B wave (cmp412_b2p1).
#
# Rebuilds the .class blobs whose SOURCES this round touched (lesson ×93:
# a stale blob = sleeping gate; ALWAYS rebuild from sources before dispatch,
# and NEVER rm -rf shared build dirs — entityinside/build holds blobs built
# by other waves' scripts, restore-compatible incremental javac only):
#   1. mobpush/build      <- mobpush/MobPushOps.java        (oversized de-globalization)
#   2. sscan/build        <- mobpush/MobPushOps.java
#                            + sscan/MobScanOps.java         (keep sscan copy in sync)
#   3. entityinside/build <- entityinside/ItemEntityManager.java (gate STRICT OR)
#   4. mobai/build        <- mobai/MobAiOps.java
#                            + mobpush/MobPushOps.java       (gate STRICT OR)
#   5. queryplane/build   <- queryplane/QueryPlaneOps.java  (NEW: entity-query
#                            snapshot plane — Level.getEntitiesOfClass /
#                            moonrise$getHardCollidingEntities whole-body
#                            redirects + ChunkEntitySlices.addEntity probe)
#
# --release 21 pins class major 65 = kernel JVM; src/queryplane.rs refuses to
# arm if the embedded major exceeds the live JVM's.
#
# Usage: scripts/build_queryplane_ops.sh [javac] [kernel.jar]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL_JAR="${2:-/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL_JAR" ] || KERNEL_JAR="$(find research -name 'patched-kernel.jar' -size +10M 2>/dev/null | head -1)"
[ -n "$KERNEL_JAR" ] && [ -f "$KERNEL_JAR" ] || { echo "kernel jar not found" >&2; exit 1; }

# Full compile classpath: kernel + fastutil + adventure-api + adventure-key
# + paper-api (round-406-E mandate; /tmp copies are NOT acceptable sources).
# NOTE: sources pulling org.joml through kernel signatures (InsideDietVisitor
# etc.) need a joml jar — NOT this wave (untouched sources keep their
# committed blobs; see git restore recipe in the banner).
FASTUTIL="${FASTUTIL_JAR:-/home/z/tools/fastutil.jar}"
ADVENTURE="${ADVENTURE_API_JAR:-/home/z/tools/adventure-api-4.24.0.jar}"
ADVENTURE_KEY="${ADVENTURE_KEY_JAR:-/home/z/tools/adventure-key-4.24.0.jar}"
PAPER_API="${PAPER_API_JAR:-/home/z/tools/paper-api-1.21.10.jar}"

CP="$KERNEL_JAR"
for j in "$FASTUTIL" "$ADVENTURE" "$ADVENTURE_KEY" "$PAPER_API"; do
    [ -f "$j" ] && CP="$CP:$j"
done

build() { # build <out_dir> <src...>
    local out="$1"; shift
    mkdir -p "$out"
    "$JAVAC" --release 21 -nowarn -cp "$CP" -d "$out" "$@"
}

rm -rf mobpush/build sscan/build mobai/build queryplane/build
build mobpush/build  mobpush/net/minecraft/world/entity/MobPushOps.java
build sscan/build    mobpush/net/minecraft/world/entity/MobPushOps.java \
                     sscan/net/minecraft/world/entity/MobScanOps.java
build entityinside/build entityinside/net/minecraft/world/entity/ItemEntityManager.java
build mobai/build    mobai/net/minecraft/world/entity/MobAiOps.java \
                     mobpush/net/minecraft/world/entity/MobPushOps.java
build queryplane/build queryplane/net/minecraft/world/entity/QueryPlaneOps.java

echo "build output (javac sha below is grep-able):"
for d in mobpush/build sscan/build entityinside/build mobai/build queryplane/build; do
    find "$d" -name '*.class' -printf '%p %s bytes\n' | sort
done
sha1sum queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class \
        mobpush/build/net/minecraft/world/entity/MobPushOps.class
