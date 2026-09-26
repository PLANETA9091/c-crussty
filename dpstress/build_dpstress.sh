#!/bin/bash
# Build C100 DATAPACK-STRESS CENSUS helper (DpStressOps) — dpstress/ module.
#
# Requirements: a JDK (java on PATH or JAVA_HOME) + ECJ jar (reused from
# redstone/, same offline-compiler precedent) + the materialized kernel jar
# (mojang-mapped Purpur 1.21.10) as the compile classpath. Produces
# dpstress/build/net/minecraft/server/DpStressOps.class for include_bytes!
# by the byte hook + a FLAT copy (check_blobs_sync flat==nested canon, x93).
#
# NOTE: no server boot here — javac-equivalent offline compile (INJECTS-ONLY).
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
KERNEL_JAR="${KERNEL_JAR:-/home/z/tools/patched-kernel.jar}"
ECJ="$HERE/ecj.jar"

if [ ! -f "$KERNEL_JAR" ]; then
  echo "KERNEL_JAR not found: $KERNEL_JAR (set KERNEL_JAR=<materialized purpur jar>)" >&2
  exit 1
fi
if [ ! -f "$ECJ" ]; then
  cp "$HERE/../redstone/ecj.jar" "$ECJ" 2>/dev/null || cp "$HERE/../randomtick/ecj.jar" "$ECJ" 2>/dev/null || {
    echo "downloading ECJ (offline compiler, runs on plain JRE)" >&2
    curl -sL -o "$ECJ" "https://repo1.maven.org/maven2/org/eclipse/jdt/ecj/3.36.0/ecj-3.36.0.jar"
  }
fi
if [ -z "${JAVA_HOME:-}" ] && ! command -v java >/dev/null 2>&1; then
  echo "no java on PATH and JAVA_HOME unset" >&2
  exit 1
fi
JAVACMD="${JAVA_HOME:+$JAVA_HOME/bin/java}"; JAVACMD="${JAVACMD:-java}"

mkdir -p "$HERE/build"
"$JAVACMD" -jar "$ECJ" -source 21 -target 21 -cp "$KERNEL_JAR" -d "$HERE/build" \
  "$HERE/src/net/minecraft/server/DpStressOps.java"

# flat==nested canon (x93: include_bytes! embeds the NESTED path)
NESTED="$HERE/build/net/minecraft/server/DpStressOps.class"
FLAT="$HERE/build/DpStressOps.class"
cp -f "$NESTED" "$FLAT"
cmp -s "$NESTED" "$FLAT" || { echo "FAIL: flat != nested" >&2; exit 1; }
echo "flat==nested: byte-identical ($(wc -c < "$NESTED") bytes)"

JAVAP_BIN="${JAVA_HOME:+$JAVA_HOME/bin/javap}"
if [ -z "$JAVAP_BIN" ]; then JAVAP_BIN=$(command -v javap || echo /home/z/tools/jdk-21.0.12.1+1/bin/javap); fi
echo "== javap gate: $("$JAVAP_BIN" -p -c "$NESTED" | grep -cE "execTag|selfTest|armState") census symbols =="
echo "built: $(find "$HERE/build" -name '*.class')"
