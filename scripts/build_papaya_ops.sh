#!/usr/bin/env bash
# build_papaya_ops.sh — build the PapayaShardReadOps sidecar class
# (TASK-460-02, round-460-chkswing-1; swing layer H05 on the
# cmp456_chunkmono carrier d73758a3; transferred from round-459-h05).
#
# Sidecar is JDK-ONLY (<clinit> touches j.u.c atomics + arrays, never a
# net.minecraft.* class — NCDFE-канон T1): compiles with --release 8 and NO
# kernel jar on the classpath.
#
# --release 8 pins the class-file major to 52 (h05 scaffold canon; kernel
# JVM = Java 21 hard-fails only above major 65) — src/papaya_arm.rs refuses
# to arm if the embedded major exceeds the live JVM's.
#
# GATES (lesson-408: a stale/absent blob = placebo lever — always rebuild):
#   1. javac exit 0;
#   2. javap -p flat==nested: the flat classfile declares ZERO nested
#      classes (no PapayaShardReadOps$*) and the PAPAYA_SHARD_MARK needle
#      is visible;
#   3. exact single-class output dir (PapayaShardReadOps.class, nothing else).
#
# Usage: scripts/build_papaya_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if [ -x /home/z/tools/jdk-21.0.12.1+1/bin/javac ]; then JAVAC=/home/z/tools/jdk-21.0.12.1+1/bin/javac
  elif command -v javac > /dev/null 2>&1; then JAVAC=javac
  else echo "no javac found (pass one as arg 1 or install a JDK)" >&2; exit 1; fi
fi
JAVAP="${JAVAC%javac}javap"

OUT_DIR=papaya/build
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

SRC=papaya/net/minecraft/server/level/PapayaShardReadOps.java
CLSID=papaya/net/minecraft/server/level/PapayaShardReadOps

"$JAVAC" --release 8 -nowarn -d "$OUT_DIR" "$SRC"

# Gate 3: exactly one classfile out (nested-class leak would show here too).
FILES="$(find "$OUT_DIR" -name '*.class' | sort)"
EXPECTED="$OUT_DIR/net/minecraft/server/level/PapayaShardReadOps.class"
if [ "$FILES" != "$EXPECTED" ]; then
  echo "JAVAP flat==nested FAIL: unexpected classfiles:" >&2
  echo "$FILES" >&2
  exit 1
fi

# Gate 2: javap flat==nested + needle.
JP="$("$JAVAP" -p -cp "$OUT_DIR" net.minecraft.server.level.PapayaShardReadOps)"
if echo "$JP" | grep -q '\$'; then
  echo "JAVAP flat==nested FAIL: nested class in javap output:" >&2
  echo "$JP" | grep '\$' >&2
  exit 1
fi
if ! echo "$JP" | grep -q 'PAPAYA_SHARD_MARK'; then
  echo "JAVAP needle FAIL: PAPAYA_SHARD_MARK not visible" >&2
  exit 1
fi

MAJOR=$(python3 -c "
import struct
b=open('$EXPECTED','rb').read()
assert b[:4]==b'\xca\xfe\xba\xbe'
print(struct.unpack('>H', b[6:8])[0])")
if [ "$MAJOR" -gt 65 ]; then
  echo "class major $MAJOR > 65 — repo canon hard-fail" >&2
  exit 1
fi

echo "JAVAP flat==nested PASS (0 nested classes; PAPAYA_SHARD_MARK needle visible; major $MAJOR)"
echo "built:"
ls -la "$OUT_DIR"/net/minecraft/server/level/
