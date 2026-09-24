#!/usr/bin/env bash
# build_438c_chunksend.sh (TASK-438-C) — compile the ChunkSendOps bridge into
# chunksend/build/ (nested include_bytes! path + flat legacy copy, flat==nested
# byte-equality gate — x93 discipline). Classpath = the round-396-a patched
# kernel fixture jar (net.minecraft classes; the round-j2b jar is BANNED,
# lesson x93) + fastutil (+paper-api/adventure when present). No stubs needed:
# the bridge compiles against the kernel surface directly (PlayerChunkSender
# shape contract).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$HERE/.." && pwd)"
JAVAC="${JDK21:-/home/z/tools/jdk-21.0.12.1+1}/bin/javac"
KERNEL_JAR="${KERNEL_JAR:-$REPO/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL_JAR" ] || KERNEL_JAR=/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar

OUT="$REPO/chunksend/build"
rm -rf "$OUT"
mkdir -p "$OUT"

CP="$KERNEL_JAR:/home/z/tools/fastutil.jar"
for extra in /home/z/tools/paper-api-1.21.10.jar /home/z/tools/adventure-api-4.24.0.jar /home/z/tools/adventure-key-4.24.0.jar /home/z/tools/concurrentutil-0.0.7.jar; do
  [ -f "$extra" ] && CP="$CP:$extra"
done

# TASK-444-B: ChunkPacketEncodeOps needs javac to resolve the supertype chain
# RegistryFriendlyByteBuf -> FriendlyByteBuf -> io.netty.buffer.ByteBuf. The
# kernel jar does not expose netty, so a COMPILE-TIME-ONLY shape stub is built
# here (chunksend/netty-compile-stub/). It NEVER ships: the emitted bridge
# bytecode references FriendlyByteBuf/minecraft declaring classes only, and
# every netty-typed operation goes through the reflective surface resolved at
# runtime inside the kernel JVM (real netty). Descriptor drift is impossible
# by construction.
STUB="$(mktemp -d)"
trap 'rm -rf "$STUB"' EXIT
"$JAVAC" --release 21 -nowarn -d "$STUB" \
  "$REPO/chunksend/netty-compile-stub/io/netty/buffer/ByteBuf.java" \
  "$REPO/chunksend/netty-compile-stub/io/netty/buffer/Unpooled.java"
CP="$CP:$STUB"

"$JAVAC" --release 21 -nowarn -cp "$CP" -d "$OUT" \
  "$REPO/chunksend/net/minecraft/server/network/ChunkSendOps.java" \
  "$REPO/chunksend/net/minecraft/server/network/ChunkPacketEncodeOps.java"

# Sanity: the compiled blobs must declare the redirect targets with the EXACT
# canonical descriptors (redirect stack-shape contracts) and no nested classes.
"$(dirname "$JAVAC")/javap" -p -cp "$OUT" net.minecraft.server.network.ChunkSendOps \
  > "$OUT/chunksend_ops.txt"
grep -q \
  "sendChunk(net.minecraft.server.network.ServerGamePacketListenerImpl, net.minecraft.server.level.ServerLevel, net.minecraft.world.level.chunk.LevelChunk)" \
  "$OUT/chunksend_ops.txt"
grep -q "selfTest()" "$OUT/chunksend_ops.txt"
"$(dirname "$JAVAC")/javap" -p -cp "$OUT" net.minecraft.server.network.ChunkPacketEncodeOps \
  > "$OUT/chunkencode_ops.txt"
grep -q \
  "write(net.minecraft.network.protocol.game.ClientboundLevelChunkWithLightPacket, net.minecraft.network.RegistryFriendlyByteBuf)" \
  "$OUT/chunkencode_ops.txt"
grep -q "selfTest()" "$OUT/chunkencode_ops.txt"
if ls "$OUT"/net/minecraft/server/network/ | grep -q '\$'; then
  echo "FATAL: nested class detected in bridge build" >&2
  exit 1
fi

# x93 discipline: NESTED path first (include_bytes! contract), then flat
# legacy copy, then byte-equality gate (BOTH bridges).
NESTED="$OUT/net/minecraft/server/network/ChunkSendOps.class"
FLAT="$OUT/ChunkSendOps.class"
cp "$NESTED" "$FLAT"
cmp -s "$NESTED" "$FLAT" || { echo "GATE FAIL: flat != nested" >&2; exit 1; }
NESTED5="$OUT/net/minecraft/server/network/ChunkPacketEncodeOps.class"
FLAT5="$OUT/ChunkPacketEncodeOps.class"
cp "$NESTED5" "$FLAT5"
cmp -s "$NESTED5" "$FLAT5" || { echo "GATE FAIL: flat != nested (chunk5)" >&2; exit 1; }
echo "chunksend: builds ok (flat==nested, ChunkSendOps $(stat -c%s "$NESTED") bytes, ChunkPacketEncodeOps $(stat -c%s "$NESTED5") bytes)"
ls -la "$OUT/net/minecraft/server/network/"
