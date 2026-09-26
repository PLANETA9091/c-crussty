#!/usr/bin/env bash
# merge_safety_gate.sh — x466 14f/C77 + x466-C84/M09-467: единый 6-ценз-гейт + обёртка merge-скриптов.
#
# ЗАЧЕМ (уроки x466, Л141-Л146):
#   ×466-урок-3 / Л145: MERGE #9 line-union съел ';;' арма cmp456_chunkmono в
#     bench/world3/run_world3.sh → bash -n FAIL line 532 → canary-405
#     (36226176808) failure @07:21:11Z → ВСЕ диспатчи роя горели 100%.
#     Канон Л145: javap+javac+cargo+**bash -n** — 4-й ценз обязателен.
#   ×466-урок-4 / Л141-УРОК-2: build_456b_blobs_all.sh find-ил БИТЫЙ jar из
#     старых round-* (invalid END header) → канон KERNEL_JAR env всегда.
#   Л141-УРОК-1: nohup+tee/process-substitution хрупки (merge-скрипт «умирал»
#     на say) → канон setsid + exec > file.
#
# ГЕЙТЫ (последовательно, любой FAIL = стоп, merge-команда НЕ запускается):
#   G1 bash -n  — синтаксис КАЖДОГО tracked *.sh (git ls-files '*.sh')
#   G2 ';;'-скан — case-arm терминаторы (scripts/case_arm_scan.py): FAIL на
#      арм-без-';;'-перед-следующим-армом (дубль bash -n с точным армом),
#      WARN на арм-без-';;'-перед-esac (легальный bash, line-union-хрупкий)
#   G3 blob-cp  — scripts/check_blobs_sync.sh (javap/CP/flat==nested ценз)
#   G4 cargo    — cargo check (0 err обязателен)
#   G5 ncdfe-selftest — scripts/ncdfe_guard.sh --selftest (пассивный дым-гейт,
#      канон Л-466-C84.1: T1 NCDFE=0 до вердикта; javap arm/define-гонки)
#   G6 ncdfe-bridge  — ncdfe_guard на 10 bridge-блобов (NCDFE_JAVAP env):
#      FAIL (exit 1) = вердикт RED, merge НЕ запускается (отравленные ×456)
#
# Usage:
#   scripts/merge_safety_gate.sh --check-only              # только 6 гейтов
#   scripts/merge_safety_gate.sh [--fast] -- CMD...        # гейты → setsid-отстрел CMD (Л141-УРОК-1), лог печатается
#   scripts/merge_safety_gate.sh [--fast] --foreground -- CMD...  # гейты → CMD в фоне лога → пост-гейты
#
# Env: GATE_SKIP_BLOBS=1 (пропустить G3), GATE_SKIP_CARGO=1 (пропустить G4),
#      GATE_SKIP_NCDFE=1 (пропустить G6), --fast = то же, что все три SKIP,
#      GATE_LOG_DIR=... (по умолчанию
#      $REPO/.merge_gate_logs), GATE_CARGO_TARGET_DIR=... (по умолчанию
#      $REPO/target; при disk-полноте можно указать общий тёплый target),
#      KERNEL_JAR (см. канон ниже), NCDFE_JAVAP (см. канон ниже).
#
# Exit: 0 = гейты зелёные (и команда отстреляна/прошла), 1 = гейт/команда FAIL, 2 = usage.

set -uo pipefail

MODE="check-only"
FAST=0
FOREGROUND=0
ARGS=()
while [ $# -gt 0 ]; do
  case "$1" in
    --check-only)  MODE="check-only"; shift ;;
    --fast)        FAST=1; shift ;;
    --foreground)  FOREGROUND=1; shift ;;
    --)            shift; ARGS=("$@"); MODE="run"; break ;;
    -h|--help)     sed -n '2,40p' "$0"; exit 0 ;;
    *)             echo "usage-FAIL: неизвестный аргумент '$1' (см. --help)" >&2; exit 2 ;;
  esac
done

REPO="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO" || exit 1
# не-env-хрупкость (канон ×466-урок-4 обобщён): cargo может отсутствовать в
# не-login shell — добавляем канонический путь, если есть
if ! command -v cargo >/dev/null 2>&1 && [ -x "${HOME:-/home/z}/.cargo/bin/cargo" ]; then
  export PATH="${HOME:-/home/z}/.cargo/bin:$PATH"
fi
LOG_DIR="${GATE_LOG_DIR:-$REPO/.merge_gate_logs}"
mkdir -p "$LOG_DIR" 2>/dev/null || LOG_DIR="/tmp/merge_gate_logs"
TS="$(date +%Y%m%d-%H%M%S)"

# ×466-урок-4 канон: KERNEL_JAR на живой jar (find ловит битые round-* артефакты)
KERNEL_CANON="/home/z/tools/patched-kernel.jar"
if [ -f "$KERNEL_CANON" ]; then
  if [ -z "${KERNEL_JAR:-}" ] || [ ! -f "${KERNEL_JAR:-/nonexistent}" ]; then
    export KERNEL_JAR="$KERNEL_CANON"
  fi
fi

# ×466-C84 канон: NCDFE_JAVAP — javap для ncdfe_guard (не-env-хрупкость;
# priority: env > /tmp/jdk21 > tools/jdk-21.0.12.1+1 > PATH)
if [ -z "${NCDFE_JAVAP:-}" ]; then
  for _NCD in /tmp/jdk21/bin/javap /home/z/tools/jdk-21.0.12.1+1/bin/javap; do
    if [ -x "$_NCD" ]; then export NCDFE_JAVAP="$_NCD"; break; fi
  done
  if [ -z "${NCDFE_JAVAP:-}" ] && command -v javap >/dev/null 2>&1; then
    export NCDFE_JAVAP="$(command -v javap)"
  fi
fi

FAILED=0
gate_fail() { printf 'GATE-FAIL: %s\n' "$1" >&2; FAILED=1; }

echo "== merge_safety_gate @ $(date -u +%Y-%m-%dT%H:%M:%SZ) repo=$REPO =="
echo "KERNEL_JAR=${KERNEL_JAR:-unset} (×466-урок-4 канон)"
echo "NCDFE_JAVAP=${NCDFE_JAVAP:-unset} (×466-C84 канон)"

# ---- G1: bash -n всех tracked *.sh ----
G1_T0=$(date +%s)
SH_LIST="$(mktemp /tmp/gate_shlist.XXXXXX)"
git ls-files '*.sh' > "$SH_LIST"
N_SH=$(wc -l < "$SH_LIST")
N_BASHN_FAIL=0
while IFS= read -r f; do
  [ -f "$f" ] || continue
  if ! bash -n "$f" 2>>"$LOG_DIR/g1_bashn_$TS.err"; then
    N_BASHN_FAIL=$((N_BASHN_FAIL + 1))
    gate_fail "G1 bash -n: $f (детали: $LOG_DIR/g1_bashn_$TS.err)"
  fi
done < "$SH_LIST"
rm -f "$SH_LIST"
[ "$N_BASHN_FAIL" = "0" ] && rm -f "$LOG_DIR/g1_bashn_$TS.err"
G1_T1=$(date +%s)
echo "G1 bash -n: $N_SH file(s), $N_BASHN_FAIL err, $((G1_T1-G1_T0))s"

# ---- G2: ';;' case-arm сканер ----
G2_T0=$(date +%s)
SH_LIST2="$(mktemp /tmp/gate_shlist2.XXXXXX)"
git ls-files '*.sh' > "$SH_LIST2"
# shellcheck disable=SC2046
G2_OUT="$(python3 scripts/case_arm_scan.py $(cat "$SH_LIST2") 2>&1)"; G2_RC=$?
rm -f "$SH_LIST2"
G2_NFAIL="$(printf '%s\n' "$G2_OUT" | tail -1 | sed -n 's/.*, \([0-9]*\) FAIL, .*/\1/p')"
G2_NWARN="$(printf '%s\n' "$G2_OUT" | tail -1 | sed -n 's/.*FAIL, \([0-9]*\) WARN/\1/p')"
G2_NFAIL="${G2_NFAIL:-?}"; G2_NWARN="${G2_NWARN:-?}"
printf '%s\n' "$G2_OUT" | grep -E "FAIL|WARN" | head -20 || true
if [ "$G2_RC" -ne 0 ]; then
  gate_fail "G2 case-arm сканер: $G2_NFAIL FAIL (арм без ';;' — x466-Л145 класс; полные детали выше)"
fi
G2_T1=$(date +%s)
echo "G2 ';;'-scan: $G2_NFAIL FAIL, $G2_NWARN WARN, $((G2_T1-G2_T0))s"

# ---- G3: blob-cp (check_blobs_sync) ----
if [ "$FAST" = "1" ] || [ "${GATE_SKIP_BLOBS:-0}" = "1" ]; then
  echo "G3 blob-cp: SKIP (fast/GATE_SKIP_BLOBS)"
else
  G3_T0=$(date +%s)
  if [ -f scripts/check_blobs_sync.sh ]; then
    if G3_OUT="$(bash scripts/check_blobs_sync.sh 2>&1)"; then
      G3_TAIL="$(printf '%s\n' "$G3_OUT" | tail -1)"
      echo "G3 blob-cp: $G3_TAIL"
    else
      printf '%s\n' "$G3_OUT" | grep -E "^FAIL" | head -10 || printf '%s\n' "$G3_OUT" | tail -10
      gate_fail "G3 blob-cp: check_blobs_sync FAIL (stale blob / cp-drift — анти-×93, анти-Л141-УРОК-3)"
    fi
  else
    gate_fail "G3 blob-cp: scripts/check_blobs_sync.sh отсутствует"
  fi
  G3_T1=$(date +%s)
  echo "G3 blob-cp: $((G3_T1-G3_T0))s"
fi

# ---- G4: cargo check ----
if [ "$FAST" = "1" ] || [ "${GATE_SKIP_CARGO:-0}" = "1" ]; then
  echo "G4 cargo: SKIP (fast/GATE_SKIP_CARGO)"
elif [ -f Cargo.toml ]; then
  G4_T0=$(date +%s)
  export CARGO_TARGET_DIR="${GATE_CARGO_TARGET_DIR:-$REPO/target}"
  if G4_OUT="$(cargo check --quiet 2>&1)"; then
    G4_T1=$(date +%s)
    echo "G4 cargo check: 0 err, $((G4_T1-G4_T0))s"
  else
    printf '%s\n' "$G4_OUT" | tail -15
    gate_fail "G4 cargo check: compile err (Л141-УРОК-3 ценз)"
  fi
else
  echo "G4 cargo: SKIP (нет Cargo.toml)"
fi

# ---- G5: NCDFE-страж selftest (пассивный дым-гейт, канон Л-466-C84.1) ----
# Вставка ПОСЛЕ G4, ДО вердикта: канон Л-466-C84.1 «после rebuild блобов,
# до ledger-push/вердикта» — G3 уже проверил blob-sync, вердикт ниже ждёт G5/G6.
if [ ! -f scripts/ncdfe_guard.sh ]; then
  echo "G5 ncdfe-selftest: SKIP (scripts/ncdfe_guard.sh отсутствует)"
else
  G5_T0=$(date +%s)
  if G5_OUT="$(bash scripts/ncdfe_guard.sh --selftest 2>&1)"; then
    G5_T1=$(date +%s)
    echo "G5 ncdfe-selftest: $(printf '%s\n' "$G5_OUT" | tail -1), $((G5_T1-G5_T0))s"
  else
    G5_RC=$?
    printf '%s\n' "$G5_OUT" | tail -5
    gate_fail "G5 ncdfe-selftest: rc=$G5_RC (T1 NCDFE!=0 на известных блобах — LAB_LEDGER L6)"
  fi
fi

# ---- G6: NCDFE-страж на 10 bridge-блобов (arm/define EntityGoalQueryOps-класс) ----
NCDFE_BRIDGE_BLOBS=(
  "mobpush/build/net/minecraft/world/entity/MobPushOps.class"
  "entitygoalquery/build/net/minecraft/world/entity/EntityGoalQueryOps.class"
  "queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class"
  "chunksched/build/net/minecraft/server/level/ChunkSchedOps.class"
  "colpush/build/net/minecraft/world/entity/ColpushOps.class"
  "entityinside/build/net/minecraft/world/entity/InsideBatchOps.class"
  "entityinside/build/net/minecraft/world/entity/CollideBatchOps.class"
  "chunksend/build/net/minecraft/server/network/ChunkSendOps.class"
  "chunksend/build/net/minecraft/server/network/ChunkPacketEncodeOps.class"
  "chunkparse/build/net/minecraft/world/level/chunk/storage/ChunkParseOps.class"
)
if [ "$FAST" = "1" ] || [ "${GATE_SKIP_NCDFE:-0}" = "1" ]; then
  echo "G6 ncdfe-bridge: SKIP (fast/GATE_SKIP_NCDFE)"
elif [ ! -f scripts/ncdfe_guard.sh ]; then
  echo "G6 ncdfe-bridge: SKIP (scripts/ncdfe_guard.sh отсутствует)"
else
  G6_T0=$(date +%s)
  G6_EXIST=()
  for _b in "${NCDFE_BRIDGE_BLOBS[@]}"; do [ -f "$_b" ] && G6_EXIST+=("$_b"); done
  if [ "${#G6_EXIST[@]}" = "0" ]; then
    echo "G6 ncdfe-bridge: SKIP (0/10 bridge-блобов в дереве)"
  else
    if G6_OUT="$(bash scripts/ncdfe_guard.sh "${G6_EXIST[@]}" 2>&1)"; then
      G6_T1=$(date +%s)
      echo "G6 ncdfe-bridge: ${#G6_EXIST[@]}/10 blobs $(printf '%s\n' "$G6_OUT" | tail -1), $((G6_T1-G6_T0))s"
    else
      G6_RC=$?
      printf '%s\n' "$G6_OUT" | tail -10
      if [ "$G6_RC" = "1" ]; then
        gate_fail "G6 ncdfe-bridge: NCDFE FAIL на bridge-блобе (arm/define-гонка — отравленные ×456, DELIVERY-FAIL)"
      else
        echo "G6 ncdfe-bridge: WARN rc=$G6_RC (SKIP — javap недоступен/блоб частично отсутствует)"
      fi
    fi
  fi
fi

echo "== гейт-вердикт: $([ "$FAILED" = "0" ] && echo GREEN || echo RED) =="

if [ "$FAILED" != "0" ]; then
  echo "merge_safety_gate: RED — merge-команда НЕ запускается" >&2
  exit 1
fi

if [ "$MODE" = "check-only" ] || [ "${#ARGS[@]}" = "0" ]; then
  exit 0
fi

# ---- Обёртка: запуск merge-скрипта ----
CMD_LOG="$LOG_DIR/merge_$TS.log"
printf '%s\n' "${ARGS[@]}" > "$CMD_LOG"
echo "cmd: ${ARGS[*]}"
echo "log: $CMD_LOG"

if [ "$FOREGROUND" = "1" ]; then
  # Л141-УРОК-1: exec > file стабильно; живой хвост не нужен гейту
  bash -c 'cd "$1" || exit 127; exec > "$2" 2>&1; shift 2; exec "$@"' _ "$REPO" "$CMD_LOG" "${ARGS[@]}"
  CMD_RC=$?
  echo "cmd exit=$CMD_RC (лог: $CMD_LOG)"
  if [ "$CMD_RC" -ne 0 ]; then
    echo "merge_safety_gate: команда упала — пост-гейты НЕ имеют смысла до ремонта" >&2
    exit 1
  fi
  # пост-гейты: merge-скрипт мог поменять .sh/java/blob/rust — re-run ЦЕНЗОВ
  echo "== пост-гейты (re-run после merge-команды) =="
  exec "$0" $([ "$FAST" = "1" ] && echo --fast) --check-only
else
  # detached: setsid + exec > file (канон Л141-УРОК-1; НЕ nohup+tee)
  setsid bash -c 'cd "$1" || exit 127; exec >> "$2" 2>&1; shift 2; echo "== started $(date -u +%FT%TZ) pid=$$ =="; exec "$@"' _ "$REPO" "$CMD_LOG" "${ARGS[@]}" &
  SPAWN_PID=$!
  disown "$SPAWN_PID" 2>/dev/null || true
  echo "detached pid=$SPAWN_PID (setsid, лог дописывается в $CMD_LOG)"
  echo "merge_safety_gate: GREEN, команда отстреляна в фон"
  exit 0
fi
