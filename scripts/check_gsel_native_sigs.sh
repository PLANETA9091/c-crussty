#!/usr/bin/env bash
# check_gsel_native_sigs.sh — TASK-415-B iter-2 institutional gate (урок
# pfb1: RegisterNatives sig-мисматч = hook dormant = нога мерила ваниль+шум).
#
# ГЕЙТ: javap -s дескрипторы native-методов в gsel/build/.../GoalBatchOps.class
# обязаны В ТОЧНОСТИ встречаться как CString::new("...") строки в
# src/goal_batch.rs (RegisterNatives сигнатуры). Любой мисматч = fail (exit 1)
# ДО диспатча — не на живой ноге.
#
# Usage: scripts/check_gsel_native_sigs.sh [javap]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAP="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javap}"
CLS=gsel/build/net/minecraft/world/entity/ai/goal/GoalBatchOps.class
RS=src/goal_batch.rs

[ -f "$CLS" ] || { echo "FAIL: $CLS missing (запусти scripts/build_gsel_blobs_all.sh)" >&2; exit 1; }
[ -f "$RS" ]  || { echo "FAIL: $RS missing" >&2; exit 1; }

# (name, descriptor) пары native-методов из .class (ground truth javap)
mapfile -t PAIRS < <("$JAVAP" -p -s "$CLS" | awk '
  /static native/ { name=$4; sub(/\(.*/,"",name); innative=1; next }
  innative && /descriptor:/ { gsub(/^ *descriptor: /,""); print name" "$1; innative=0 }
')

[ "${#PAIRS[@]}" -eq 3 ] || { echo "FAIL: ожидалось 3 native (probe/register/epoch), javap дал ${#PAIRS[@]}" >&2; exit 1; }

rc=0
for p in "${PAIRS[@]}"; do
  name="${p%% *}"; sig="${p##* }"
  if grep -qF "CString::new(\"$sig\")" "$RS"; then
    echo "OK  $name $sig — найден в $RS"
  else
    echo "FAIL $name $sig — RegisterNatives-строка отсутствует/отличается в $RS (pfb1-класс бага)" >&2
    rc=1
  fi
done

# Rust fns vs JVM major gate: класс обязан быть major 65 (release 21)
major=$("$JAVAP" -v "$CLS" | awk '/major version/ {print $3; exit}')
[ "$major" = "65" ] || { echo "FAIL: class major=$major, ожидался 65" >&2; rc=1; }

[ "$rc" = "0" ] && echo "== gsel native-sig gate PASS ==" || echo "== gsel native-sig gate FAIL ==" >&2
exit "$rc"
