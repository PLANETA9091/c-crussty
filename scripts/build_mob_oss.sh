#!/usr/bin/env bash
# Build the mob_ossport bridge class (TASK-401-F, Lithium unpushable_cramming
# push-lane port). Same discipline as scripts/build_items_oss.sh: pure-Java
# bridge compiled against the real runtime kernel jar, --release 21 pins the
# class-file major to 65 = the kernel JVM (Java 21); mob_oss.rs refuses to arm
# if the embedded major exceeds the live JVM's.
#
# Usage: scripts/build_mob_oss.sh [javac] [kernel-jar]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if command -v javac > /dev/null 2>&1; then JAVAC=javac
  elif [ -x /home/z/tools/jdk-21.0.12.1+1/bin/javac ]; then JAVAC=/home/z/tools/jdk-21.0.12.1+1/bin/javac
  else echo "no javac found (pass one as arg 1 or install a JDK)" >&2; exit 1; fi
fi

KERNEL_JAR="${2:-${SERVER_JAR:-/tmp/pdec/matsrv/versions/1.21.10/purpur-1.21.10.jar}}"
if [ ! -f "$KERNEL_JAR" ]; then
  KERNEL_JAR=$(ls /home/z/c-crussty/research/*/run-*/patched-kernel.jar 2>/dev/null | head -1 || true)
  [ -n "$KERNEL_JAR" ] && [ -f "$KERNEL_JAR" ] || { echo "kernel jar not found" >&2; exit 1; }
fi

OUT_DIR=mob/build
mkdir -p "$OUT_DIR"

"$JAVAC" --release 21 -nowarn -proc:none \
  -classpath "$KERNEL_JAR:/home/z/tools/fastutil.jar:/home/z/tools/guava.jar" \
  -d "$OUT_DIR" \
  mob/net/minecraft/world/entity/MobOssOps.java

echo "built:"
ls -la "$OUT_DIR"/net/minecraft/world/entity/
