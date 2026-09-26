#!/usr/bin/env bash
# build_chunksched_ops.sh (TASK-456-C) — compile the ChunkSchedOps scheduling
# bridge into chunksched/build/ (nested include_bytes! path + flat legacy
# copy, flat==nested byte-equality gate — x93 discipline).
#
# Classpath = the round-396-a patched kernel fixture jar (net.minecraft +
# moonrise classes; the round-j2b jar is BANNED, lesson x93) + fastutil
# (+paper-api/adventure when present). ConcurrentLong2ReferenceChainedHashTable
# resolves from chunksched/concurrentutil-stub (COMPILE-ONLY shape stub,
# chunksend netty-compile-stub precedent — the real class ships with the
# server's concurrentutil module at runtime; the stub is NEVER packaged).
#
# ZERO nested classes (kernel-loader define, colpush NCDFE lesson): the build
# dir must contain exactly the two copies of ChunkSchedOps.class.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$HERE/.." && pwd)"
JAVAC="${JDK21:-/home/z/tools/jdk-21.0.12.1+1}/bin/javac"
KERNEL_JAR="${KERNEL_JAR:-$REPO/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL_JAR" ] || KERNEL_JAR=/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar
[ -f "$KERNEL_JAR" ] || KERNEL_JAR=/home/z/tools/patched-kernel.jar

OUT="$REPO/chunksched/build"
rm -rf "$OUT"
mkdir -p "$OUT"

CP="$KERNEL_JAR:/home/z/tools/fastutil.jar"
for extra in /home/z/tools/paper-api-1.21.10.jar /home/z/tools/adventure-api-4.24.0.jar /home/z/tools/adventure-key-4.24.0.jar; do
  [ -f "$extra" ] && CP="$CP:$extra"
done

STUB="$(mktemp -d)"
trap 'rm -rf "$STUB"' EXIT
"$JAVAC" --release 21 -nowarn -d "$STUB" \
  "$REPO/chunksched/concurrentutil-stub/ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.java"
CP="$CP:$STUB"

"$JAVAC" --release 21 -nowarn -cp "$CP" -d "$OUT" \
  "$REPO/chunksched/net/minecraft/server/level/ChunkSchedOps.java"

# Sanity: the compiled blob must declare the two retarget statics with the
# EXACT receiver-prepended canonical descriptors (redirect stack-shape
# contracts; classfile.rs CHUNKSCHED_REDIRECT_TARGETS pins the same strings)
# + the native table + no nested classes.
"$(dirname "$JAVAC")/javap" -p -cp "$OUT" net.minecraft.server.level.ChunkSchedOps \
  > "$OUT/chunksched_ops.txt"
grep -q "getNow(net.minecraft.server.level.ServerChunkCache, int, int)" "$OUT/chunksched_ops.txt"
grep -q "onSetFullChunk(net.minecraft.server.level.ServerChunkCache, int, int, net.minecraft.world.level.chunk.LevelChunk)" "$OUT/chunksched_ops.txt"
grep -q "selfTest()" "$OUT/chunksched_ops.txt"
grep -q "arm()" "$OUT/chunksched_ops.txt"
grep -q "mirrorEvent(long, boolean)" "$OUT/chunksched_ops.txt"
grep -q "schedProbe()" "$OUT/chunksched_ops.txt"
if find "$OUT/net" "$OUT/ca" -name '*.class' 2>/dev/null | grep -q '\$'; then
  echo "FATAL: nested class detected in bridge build" >&2
  exit 1
fi

# x93 discipline: NESTED path first (include_bytes! contract), then flat
# legacy copy, then byte-equality gate.
NESTED="$OUT/net/minecraft/server/level/ChunkSchedOps.class"
FLAT="$OUT/ChunkSchedOps.class"
cp "$NESTED" "$FLAT"
cmp -s "$NESTED" "$FLAT" || { echo "GATE FAIL: flat != nested" >&2; exit 1; }
echo "chunksched: builds ok (flat==nested, ChunkSchedOps $(stat -c%s "$NESTED") bytes)"
ls -la "$OUT/net/minecraft/server/level/"
