#!/usr/bin/env bash
# build_inside_acquire.sh — сборка java-моста InsideAcquireOps (ID-P33,
# TASK-459-73 scaffold). Канон кернела: javac --release 21, major <= 65
# (kernel JVM = Java 21); hard-fail выше 65. Stub vanilla-free — голый javac.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$ROOT/entityinside/net/minecraft/world/entity/InsideAcquireOps.java"
OUT="$ROOT/entityinside/build"

# JAVAC: repo-канон (round-396-a kernel jar урок — javac-cp фиксируем тут).
JAVAC="${JAVAC:-}"
if [ -z "$JAVAC" ]; then
  for c in /home/z/tools/jdk-21.0.12.1+1/bin/javac /tmp/jdk21/bin/javac javac; do
    if command -v "$c" >/dev/null 2>&1; then JAVAC="$c"; break; fi
  done
fi

mkdir -p "$OUT"
rm -f "$OUT"/net/minecraft/world/entity/InsideAcquireOps*.class

# --release 21 => major 65; выше кернел не грузит (kernel JVM = Java 21).
# -d = пакетный корень (javac сам создаёт net/minecraft/world/entity).
"$JAVAC" --release 21 -d "$OUT" "$SRC"

# Встраиваемый блоб обязан быть <= major 65 (guard версии activate-шага 3).
CLASSES=("$OUT"/net/minecraft/world/entity/InsideAcquireOps*.class)
for c in "${CLASSES[@]}"; do
  major=$(od -An -j6 -N2 -tu2 --endian=big "$c")
  if [ "$major" -gt 65 ]; then
    echo "FATAL: $c major=$major > 65 (kernel JVM=21)" >&2
    exit 1
  fi
  echo "built: $(basename "$c") (major=$major)"
done
echo "inside_acquire: blob OK ($(du -b "${CLASSES[0]}" | cut -f1) bytes) — include_bytes! приземляется в wiring-фазе"
