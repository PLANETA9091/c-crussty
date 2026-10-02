#!/usr/bin/env bash
# build_chunkparse_ops.sh (TASK-419-C) — compile the ChunkParseOps bridge
# into chunkparse/build/. Classpath = the round-396-a patched kernel fixture
# jar (net.minecraft classes; the round-j2b jar is BANNED, lesson x93) +
# fastutil (+paper-api/adventure when present). com.mojang.serialization.Codec
# resolves from chunkparse/stubs (compile-time descriptor stub, never defined
# at runtime — see the stub header).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$HERE/.." && pwd)"
JAVAC="${JDK21:-/home/z/tools/jdk-21.0.12.1+1}/bin/javac"
KERNEL_JAR="${KERNEL_JAR:-$REPO/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL_JAR" ] || KERNEL_JAR=/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar

OUT="$REPO/chunkparse/build"
rm -rf "$OUT"
mkdir -p "$OUT"

CP="$KERNEL_JAR:/home/z/tools/fastutil.jar"
for extra in /home/z/tools/paper-api-1.21.10.jar /home/z/tools/adventure-api-4.24.0.jar /home/z/tools/adventure-key-4.24.0.jar /home/z/tools/concurrentutil-0.0.7.jar; do
  [ -f "$extra" ] && CP="$CP:$extra"
done
CP="$CP:$REPO/chunkparse/stubs"

"$JAVAC" --release 21 -nowarn -cp "$CP" -d "$OUT" \
  "$REPO/chunkparse/net/minecraft/world/level/chunk/storage/ChunkParseOps.java"

# Sanity: the compiled blob must declare parseSection with the EXACT vanilla
# lambda descriptor (redirect stack-shape contract) and no nested classes.
"$(dirname "$JAVAC")/javap" -p -cp "$OUT" net.minecraft.world.level.chunk.storage.ChunkParseOps \
  > "$OUT/chunkparse_ops.txt"
grep -q \
  "parseSection(com.mojang.serialization.Codec<?>, net.minecraft.world.level.ChunkPos, int, net.minecraft.nbt.CompoundTag)" \
  "$OUT/chunkparse_ops.txt"
if ls "$OUT"/net/minecraft/world/level/chunk/storage/ | grep -q '\$'; then
  echo "FATAL: nested class detected in bridge build" >&2
  exit 1
fi
echo "chunkparse: builds ok"
ls -la "$OUT/net/minecraft/world/level/chunk/storage/"
