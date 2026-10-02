#!/usr/bin/env bash
# S7-148: CI-эквивалент локальной проверки компиляции BenchPopulationPlugin
# (тот же FIX_CP, что в run_world3.sh: kernel + все libraries).
set -euo pipefail
cd /home/z/c-crussty
JAVAC=/tmp/toolchain/jdk-21.0.12.1+1/bin/javac
KERNEL=/tmp/kernelout/patched-kernel.jar
LIBS=/tmp/s7147mat/server/libraries
FIX_CP="$KERNEL"
while IFS= read -r j; do FIX_CP="$FIX_CP:$j"; done < <(find "$LIBS" -name '*.jar' 2>/dev/null)
OUT=/tmp/s7148popclasses
rm -rf "$OUT" && mkdir -p "$OUT"
"$JAVAC" --release 21 -proc:none -cp "$FIX_CP" -d "$OUT" \
  bench/world3/population/BenchPopulationPlugin.java
echo "Compile-OK ($(find "$OUT" -name '*.class' | wc -l) classes)"
