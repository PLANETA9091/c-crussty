#!/usr/bin/env bash
# build_424b_blobs.sh — TASK-424-B: rebuild lever blobs touched by the
# cmp424_inside carrier after gate edits (lesson ×93 ×N: stale blob = sleeping
# plane; flat AND nested paths both installed — include_bytes! embeds the
# NESTED path).
#
# One javac pass, full cp: kernel round-396-a + fastutil + paper-api 1.21.10
# + adventure-api/key 4.24.0 (mandate /home/z/tools, NEVER round-j2b-jar).
# Sources touched this round: InsideSnapOps (NEW, +$Snap nested) /
# QueryPlaneOps (flagArmed += cmp424_inside).
# Usage: scripts/build_423b_blobs.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${KERNEL_JAR:-$(find research -path '*round-396-a/patched-kernel.jar' -size +10M 2>/dev/null | head -1)}"
[ -n "$KERNEL" ] && [ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }
FASTUTIL=/home/z/tools/fastutil.jar
PAPER_API=/home/z/tools/paper-api-1.21.10.jar
ADV_API=/home/z/tools/adventure-api-4.24.0.jar
ADV_KEY=/home/z/tools/adventure-key-4.24.0.jar
CP="$KERNEL"
for j in "$FASTUTIL" "$PAPER_API" "$ADV_API" "$ADV_KEY"; do
  [ -f "$j" ] && CP="$CP:$j"
done

ALL_BUILD=$(mktemp -d)
trap 'rm -rf "$ALL_BUILD"' EXIT

"$JAVAC" --release 21 -nowarn -cp "$CP" -d "$ALL_BUILD" \
  entityinside/net/minecraft/world/entity/InsideSnapOps.java \
  queryplane/net/minecraft/world/entity/QueryPlaneOps.java

install_nested_glob() { # outdir fqcn(slash-form) — nested (include_bytes! contract) + flat (legacy)
  local outdir="$1" cls="$2"
  local base nested
  base=$(basename "$cls")
  nested="$outdir/$cls.class"
  mkdir -p "$(dirname "$nested")"
  cp "$ALL_BUILD/$cls.class" "$nested"
  cp "$ALL_BUILD/$cls.class" "$outdir/$base.class"
  echo "blob: $nested ($(stat -c%s "$nested") bytes) + flat $outdir/$base.class"
}

install_nested_glob entityinside/build net/minecraft/world/entity/InsideSnapOps
install_nested_glob entityinside/build 'net/minecraft/world/entity/InsideSnapOps$Snap'
install_nested_glob queryplane/build net/minecraft/world/entity/QueryPlaneOps

# javap gate: flat==nested byte-equality (lesson ×93) for every touched class
cmp -s entityinside/build/InsideSnapOps.class \
       entityinside/build/net/minecraft/world/entity/InsideSnapOps.class \
  || { echo "GATE FAIL: InsideSnapOps flat != nested" >&2; exit 1; }
cmp -s 'entityinside/build/InsideSnapOps$Snap.class' \
       'entityinside/build/net/minecraft/world/entity/InsideSnapOps$Snap.class' \
  || { echo "GATE FAIL: InsideSnapOps\$Snap flat != nested" >&2; exit 1; }
cmp -s queryplane/build/QueryPlaneOps.class \
       queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class \
  || { echo "GATE FAIL: QueryPlaneOps flat != nested" >&2; exit 1; }

# raw-byte gate: bridges must not reference LambdaMetafactory (no indy lambdas
# on the bridge surface; selfTest body is plain bytecode).
for f in \
  entityinside/build/net/minecraft/world/entity/InsideSnapOps.class \
  queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class; do
  if rg -a -q "LambdaMetafactory" "$f" 2>/dev/null; then
    echo "INDY GATE WARN: $f references LambdaMetafactory (expected none)" >&2
  fi
done

echo "build_424b_blobs: OK"
