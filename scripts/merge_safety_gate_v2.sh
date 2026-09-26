#!/usr/bin/env bash
# =============================================================================
# merge_safety_gate_v2.sh — S36 (x467): ЕДИНАЯ ТОЧКА merge-гейтов.
#
# Композиция (5 фаз, одна команда; глубже M09 x467: без 3-файл-патча, без
# хардкода списка блобов — авто-локация мостов + tier-severity major-сweep +
# маркер-ценз как фаза + per-фазные ms-тайминги + JSON-вердикт):
#   P1 CARGO     cargo check --quiet (0 err обязателен)           [анти-Л141-УРОК-3]
#   P2 BLOBS     scripts/check_blobs_sync.sh (source<->blob cp)   [анти-×93, Л141-УРОК-3]
#   P3 CASE_ARM  bash -n всех tracked *.sh + case_arm_scan.py     [анти-Л145 canary-405]
#   P4 NCDFE     javap-фаза: selftest 4 canon-блоба + 10 мостов
#                авто-локацией по basename + major-сweep плоскости
#                */build/net/minecraft/**Ops.class (мост=FAIL,
#                вне-мост stale=WARN; STRICT_PLANE=1 -> весь plane FAIL) [канон L6: T1=0]
#   P5 MARKER    маркер-ценз: конфликтные маркеры <<<<<<< / >>>>>>> (любой
#                tracked-файл; ^=======$ FAIL только при паре в том же файле) +
#                branch-glue origin/(round-N|master|HEAD) в *.rs/*.java
#                (Л141-УРОК-3: склейки маркеров при line-union)   [анти-Л141-УРОК-4]
#
# ЛЮБАЯ фаза FAIL = вердикт RED, exit 1, merge-команда не запускается.
# WARN не блокируют (печатаются + в JSON). Каждый запуск печатает per-фазные ms.
#
# ОДНА КОМАНДА ИЗ MERGE-СКРИПТОВ (канон-вставка: после rebuild блобов,
# до ledger-push/вердикта — Л-466-C84.1):
#   bash scripts/merge_safety_gate_v2.sh --check-only || { echo "MERGE ABORT: gate v2 RED" >&2; exit 1; }
# Обёртка merge-запуска (гейты -> merge-скрипт -> пост-гейты; пускач
# Л141-УРОК-1: setsid + exec>file, НЕ nohup+tee):
#   bash scripts/merge_safety_gate_v2.sh -- CMD...                 # detached
#   bash scripts/merge_safety_gate_v2.sh --foreground -- CMD...    # ждёт + пост-гейты
#
# Флаги: --check-only | --fast | --json FILE | --selftest-negative | -- CMD...
# Env:   GATE_SKIP_BLOBS=1 / GATE_SKIP_CARGO=1 / GATE_SKIP_NCDFE=1 (--fast = все),
#        GATE_LOG_DIR (def $REPO/.merge_gate_logs), GATE_CARGO_TARGET_DIR
#        (def $REPO/target; в worktree можно указать тёплый общий),
#        GATE_V2_STRICT_PLANE=1 (P4: вся плоскость FAIL-тир),
#        KERNEL_JAR канон ×466-урок-4, NCDFE_JAVAP (env > /tmp/jdk21 > tools > PATH).
# Exit:  0 = GREEN, 1 = RED (FAIL в фазе/команде/negative), 2 = usage.
# =============================================================================
set -uo pipefail

MODE="check-only"; FAST=0; JSON_OUT=""; NEGATIVE=0; ARGS=()
while [ $# -gt 0 ]; do
  case "$1" in
    --check-only) MODE="check-only"; shift ;;
    --fast)       FAST=1; shift ;;
    --json)       JSON_OUT="${2:-}"; [ -n "$JSON_OUT" ] || { echo "usage: --json FILE" >&2; exit 2; }; shift 2 ;;
    --selftest-negative) NEGATIVE=1; MODE="negative"; shift ;;
    --foreground) MODE="foreground"; shift ;;
    --)           shift; ARGS=("$@"); [ "$MODE" = "check-only" ] && MODE="run"; break ;;
    -h|--help)    sed -n '2,44p' "$0"; exit 0 ;;
    *)            echo "usage-FAIL: неизвестный аргумент '$1' (см. --help)" >&2; exit 2 ;;
  esac
done

REPO="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO" || exit 1
command -v python3 >/dev/null 2>&1 || { echo "GATEV2-FAIL: python3 нет" >&2; exit 1; }
if ! command -v cargo >/dev/null 2>&1 && [ -x "${HOME:-/home/z}/.cargo/bin/cargo" ]; then
  export PATH="${HOME:-/home/z}/.cargo/bin:$PATH"; fi
KERNEL_CANON="/home/z/tools/patched-kernel.jar"                     # ×466-урок-4
if [ -f "$KERNEL_CANON" ] && { [ -z "${KERNEL_JAR:-}" ] || [ ! -f "${KERNEL_JAR:-/nonexistent}" ]; }; then
  export KERNEL_JAR="$KERNEL_CANON"; fi
JAVAP="${NCDFE_JAVAP:-}"
[ -n "$JAVAP" ] || for c in /tmp/jdk21/bin/javap /home/z/tools/jdk-21.0.12.1+1/bin/javap; do
  [ -x "$c" ] && { JAVAP="$c"; break; }; done
[ -n "$JAVAP" ] || JAVAP="$(command -v javap 2>/dev/null || true)"
export NCDFE_JAVAP="$JAVAP"

LOG_DIR="${GATE_LOG_DIR:-$REPO/.merge_gate_logs}"; mkdir -p "$LOG_DIR" 2>/dev/null || LOG_DIR="/tmp/merge_gate_logs"
TS="$(date +%Y%m%d-%H%M%S)"; TS_JSON="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
FAILED=0; WARNED=0; G_T0=0
phase_fail() { printf 'GATEV2-FAIL: %s\n' "$1" >&2; FAILED=$((FAILED+1)); }
phase_warn() { WARNED=$((WARNED+1)); }
now_ms() { local t="${EPOCHREALTIME:-$(date +%s).000000}"; echo "${t/./}"; }  # микросекунды

# канонические мосты P4 (M09 x467 G6-канон, 10 блобов) — авто-локация по basename
BRIDGE_NAMES="MobPushOps EntityGoalQueryOps QueryPlaneOps ChunkSchedOps ColpushOps InsideBatchOps CollideBatchOps ChunkSendOps ChunkPacketEncodeOps ChunkParseOps"

declare -a JSON_PHASES=()
json_phase() { JSON_PHASES+=("{\"phase\":\"$1\",\"ms\":$2,\"status\":\"$3\",\"detail\":\"$4\"}"); }

echo "== merge_safety_gate_v2 @ $TS_JSON repo=$REPO =="
echo "KERNEL_JAR=${KERNEL_JAR:-unset} NCDFE_JAVAP=${NCDFE_JAVAP:-unset}"
G_T0=$(now_ms)

# ============================== P1: CARGO ====================================
T0=$(now_ms)
if [ "$FAST" = "1" ] || [ "${GATE_SKIP_CARGO:-0}" = "1" ]; then
  echo "P1 cargo: SKIP (fast/GATE_SKIP_CARGO)"; json_phase "P1-cargo" 0 "SKIP" "fast"
else
  export CARGO_TARGET_DIR="${GATE_CARGO_TARGET_DIR:-$REPO/target}"
  if P1_OUT="$(cargo check --quiet 2>&1)"; then
    P1_MS=$(( ($(now_ms) - T0) / 1000 ))
    echo "P1 cargo check: 0 err, ${P1_MS}ms"; json_phase "P1-cargo" "$P1_MS" "OK" "0 err"
  else
    printf '%s\n' "$P1_OUT" | tail -15; phase_fail "P1 cargo check: compile err (Л141-УРОК-3 ценз)"
    P1_MS=$(( ($(now_ms) - T0) / 1000 )); json_phase "P1-cargo" "$P1_MS" "FAIL" "compile err"
  fi
fi

# ============================== P2: BLOBS ====================================
T0=$(now_ms)
if [ "$FAST" = "1" ] || [ "${GATE_SKIP_BLOBS:-0}" = "1" ]; then
  echo "P2 blobs: SKIP (fast/GATE_SKIP_BLOBS)"; json_phase "P2-blobs" 0 "SKIP" "fast"
else
  if [ -f scripts/check_blobs_sync.sh ]; then
    if P2_OUT="$(bash scripts/check_blobs_sync.sh 2>&1)"; then
      P2_TAIL="$(printf '%s\n' "$P2_OUT" | tail -1)"
      P2_MS=$(( ($(now_ms) - T0) / 1000 ))
      echo "P2 blob-cp: $P2_TAIL, ${P2_MS}ms"; json_phase "P2-blobs" "$P2_MS" "OK" "$P2_TAIL"
    else
      printf '%s\n' "$P2_OUT" | grep -E "^FAIL" | head -10 || printf '%s\n' "$P2_OUT" | tail -10
      P2_MS=$(( ($(now_ms) - T0) / 1000 ))
      phase_fail "P2 blob-cp: check_blobs_sync FAIL (stale blob / cp-drift)"
      json_phase "P2-blobs" "$P2_MS" "FAIL" "out-of-sync"
    fi
  else
    phase_fail "P2 blob-cp: scripts/check_blobs_sync.sh отсутствует"
    json_phase "P2-blobs" 0 "FAIL" "script missing"
  fi
fi

# ============================== P3: CASE_ARM =================================
T0=$(now_ms); P3_SH=0; P3_BN_FAIL=0; P3_RC=0
SH_LIST="$(mktemp /tmp/gatev2_shlist.XXXXXX)"
git ls-files '*.sh' > "$SH_LIST"; P3_SH=$(wc -l < "$SH_LIST")
while IFS= read -r f; do
  [ -f "$f" ] || continue
  if ! bash -n "$f" 2>>"$LOG_DIR/p3_bashn_$TS.err"; then
    P3_BN_FAIL=$((P3_BN_FAIL+1)); phase_fail "P3 bash -n: $f (детали: $LOG_DIR/p3_bashn_$TS.err)"
  fi
done < "$SH_LIST"
[ "$P3_BN_FAIL" = "0" ] && rm -f "$LOG_DIR/p3_bashn_$TS.err"
# shellcheck disable=SC2046
P3_OUT="$(python3 scripts/case_arm_scan.py $(cat "$SH_LIST") 2>&1)"; P3_RC=$?
P3_LAST="$(printf '%s\n' "$P3_OUT" | tail -1)"
P3_CA_FAIL="$(printf '%s\n' "$P3_LAST" | sed -n 's/.*, \([0-9]*\) FAIL, .*/\1/p')"; P3_CA_FAIL="${P3_CA_FAIL:-0}"
P3_CA_WARN="$(printf '%s\n' "$P3_LAST" | sed -n 's/.*FAIL, \([0-9]*\) WARN.*/\1/p')"; P3_CA_WARN="${P3_CA_WARN:-0}"
printf '%s\n' "$P3_OUT" | grep -E "FAIL|WARN" | head -10 || true
rm -f "$SH_LIST"
if [ "$P3_RC" -ne 0 ]; then phase_fail "P3 case-arm: ${P3_CA_FAIL} FAIL (арм без ';;' — x466-Л145 класс; canary-405 landmine)"; fi
P3_MS=$(( ($(now_ms) - T0) / 1000 ))
echo "P3 case_arm: bash -n ${P3_SH} file(s) / ${P3_BN_FAIL} err; ';;'-scan ${P3_CA_FAIL} FAIL, ${P3_CA_WARN} WARN, ${P3_MS}ms"
json_phase "P3-case_arm" "$P3_MS" "$([ "$P3_RC" -ne 0 ] && echo FAIL || echo OK)" "sh=${P3_SH} bashn_err=${P3_BN_FAIL} ca_fail=${P3_CA_FAIL} ca_warn=${P3_CA_WARN}"

# ============================== P4: NCDFE (javap-фаза) =======================
T0=$(now_ms); P4_NOK=0; P4_NFAIL=0; P4_NSKIP=0; P4_PLANE=0; P4_STALE=0; P4_STALE_FAIL=0; P4_ST="n/a"
if [ "$FAST" = "1" ] || [ "${GATE_SKIP_NCDFE:-0}" = "1" ]; then
  echo "P4 ncdfe: SKIP (fast/GATE_SKIP_NCDFE)"; json_phase "P4-ncdfe" 0 "SKIP" "fast"
else
  if [ -z "$JAVAP" ] || [ ! -x "$JAVAP" ]; then
    phase_fail "P4 ncdfe: javap недоступен (NCDFE_JAVAP / bash scripts/ensure_javap.sh)"
    json_phase "P4-ncdfe" 0 "FAIL" "no javap"
  else
    # (a) selftest канона (4 блоба мастера)
    if ST_OUT="$(bash scripts/ncdfe_guard.sh --selftest 2>&1)"; then
      P4_ST="PASS $(printf '%s\n' "$ST_OUT" | grep -oE 'ok=[0-9]+ fail=[0-9]+ skip=[0-9]+' | tail -1)"
    else
      P4_ST="FAIL $(printf '%s\n' "$ST_OUT" | grep -oE 'ok=[0-9]+ fail=[0-9]+ skip=[0-9]+' | tail -1)"
      phase_fail "P4 ncdfe selftest: $P4_ST (канон L6: T1 NCDFE=0)"
    fi
    # (b) 10 мостов: авто-локация по basename в */build/net/minecraft/**
    BRIDGE_LIST="$(mktemp /tmp/gatev2_bridges.XXXXXX)"
    ALL_PLANE="$(mktemp /tmp/gatev2_plane.XXXXXX)"
    git ls-files '*Ops.class' | grep -E '/build/net/minecraft/' | grep -v '\$' | grep -v '/selftest/' > "$ALL_PLANE"
    for bn in $BRIDGE_NAMES; do
      HIT="$(grep -E "/${bn}\.class$" "$ALL_PLANE" | head -1)"
      if [ -n "$HIT" ]; then echo "$HIT" >> "$BRIDGE_LIST"; else
        phase_fail "P4 ncdfe: мост $bn не найден в плоскости */build/net/minecraft/** (мерж съел мост?)"; fi
    done
    if [ -s "$BRIDGE_LIST" ]; then
      NCD_OUT="$(bash scripts/ncdfe_guard.sh $(cat "$BRIDGE_LIST") 2>&1)"; NCD_RC=$?
      P4_NOK="$(printf '%s\n' "$NCD_OUT" | grep -c '^\[OK\]')"; P4_NOK="${P4_NOK:-0}"
      P4_NFAIL="$(printf '%s\n' "$NCD_OUT" | grep -c '^\[FAIL\]')"; P4_NFAIL="${P4_NFAIL:-0}"
      P4_NSKIP="$(printf '%s\n' "$NCD_OUT" | grep -c '^\[SKIP\]')"; P4_NSKIP="${P4_NSKIP:-0}"
      printf '%s\n' "$NCD_OUT" | grep '^\[FAIL\]' | head -10 || true
      [ "$NCD_RC" -eq 0 ] || phase_fail "P4 ncdfe мосты: ${P4_NFAIL} FAIL / ${P4_NOK} OK / ${P4_NSKIP} SKIP (arm/define-рейс; DELIVERY-FAIL ×456)"
    fi
    # (c) major-сweep всей плоскости: мост=FAIL-тир, вне-моста=WARN (STRICT_PLANE=1 -> всё FAIL)
    while IFS= read -r f; do
      [ -f "$f" ] || continue
      major=$(od -An -tu1 -j6 -N2 "$f" | awk '{print $1*256+$2}')
      [ "$major" = "65" ] && continue
      P4_STALE=$((P4_STALE+1)); IS_BRIDGE=0
      for bn in $BRIDGE_NAMES; do case "$f" in */"$bn".class) IS_BRIDGE=1; break;; esac; done
      if [ "$IS_BRIDGE" = "1" ] || [ "${GATE_V2_STRICT_PLANE:-0}" = "1" ]; then
        P4_STALE_FAIL=$((P4_STALE_FAIL+1))
        phase_fail "P4 major-сweep: $f major=$major != 65 (пересобери --release 21; kernel JVM)"
      else
        phase_warn; echo "P4 major-WARN (вне-моста): $f major=$major != 65 (legacy-плоскость; rebuild --release 21)"
      fi
    done < "$ALL_PLANE"
    P4_PLANE=$(wc -l < "$ALL_PLANE"); rm -f "$BRIDGE_LIST" "$ALL_PLANE"
    P4_MS=$(( ($(now_ms) - T0) / 1000 ))
    P4_STATUS="OK"
    [ "${P4_NFAIL:-0}" != "0" ] && P4_STATUS="FAIL"
    [ "$P4_STALE_FAIL" != "0" ] && P4_STATUS="FAIL"
    case "$P4_ST" in FAIL*) P4_STATUS="FAIL";; esac
    echo "P4 ncdfe: selftest ${P4_ST}; мосты ${P4_NOK} OK / ${P4_NFAIL} FAIL / ${P4_NSKIP} SKIP; plane ${P4_PLANE} blob(s), stale ${P4_STALE} (FAIL-тир ${P4_STALE_FAIL}), ${P4_MS}ms"
    json_phase "P4-ncdfe" "$P4_MS" "$P4_STATUS" "selftest=${P4_ST}; bridges_ok=${P4_NOK} bridges_fail=${P4_NFAIL}; plane=${P4_PLANE} stale=${P4_STALE} stale_fail=${P4_STALE_FAIL}"
  fi
fi

# ============================== P5: MARKER-ценз ==============================
T0=$(now_ms); P5_FILES=0; P5_TOTAL=0; P5_CM=0; P5_GLUE=0
marker_scan() { # $@ = файлы; FAIL при конфликт-маркерах / branch-glue (rs/java)
  local f found=0
  for f in "$@"; do
    [ -f "$f" ] || continue
    P5_FILES=$((P5_FILES+1))
    local hits_cm hits_eq hits_glue
    hits_cm=$(grep -cE '^(<{7}|>{7})' "$f" 2>/dev/null); hits_cm="${hits_cm:-0}"
    hits_eq=$(grep -cE '^={7}$' "$f" 2>/dev/null); hits_eq="${hits_eq:-0}"
    hits_glue=0
    case "$f" in
      *.rs|*.java)
        hits_glue=$(grep -cE 'origin/(round-[0-9]|master|HEAD)' "$f" 2>/dev/null); hits_glue="${hits_glue:-0}" ;;
    esac
    if [ "${hits_cm:-0}" -gt 0 ]; then
      P5_CM=$((P5_CM+hits_cm)); found=1
      phase_fail "P5 маркер-ценз: незакрытый конфликт в $f (<<<<<<< / >>>>>>> ×${hits_cm}) — line-union недомержен"
    fi
    if [ "${hits_eq:-0}" -gt 0 ] && [ "${hits_cm:-0}" -gt 0 ]; then
      phase_fail "P5 маркер-ценз: полный конфликт-регион (^=======) в $f"
    fi
    if [ "${hits_glue:-0}" -gt 0 ]; then
      P5_GLUE=$((P5_GLUE+hits_glue)); found=1
      phase_fail "P5 маркер-ценз: branch-glue origin/... в коде $f (×${hits_glue}; Л141-УРОК-3: склейка маркеров при line-union)"
    fi
    [ "$found" = "1" ] && return 1
    found=0
  done
  return 0
}
T0b=$(now_ms)
P5_TOTAL=$(git ls-files | wc -l)
# (i) tracked-файлы: git grep префильтр (конфликт-маркеры везде; glue в rs/java), потом точный scan
CAND="$(mktemp /tmp/gatev2_markers.XXXXXX)"
{ git grep -lI -E '^(<{7}|>{7})' -- . 2>/dev/null || true
  git grep -lI -E 'origin/(round-[0-9]|master|HEAD)' -- '*.rs' '*.java' 2>/dev/null || true; } | sort -u > "$CAND"
if [ -s "$CAND" ]; then marker_scan $(cat "$CAND") || true; fi
rm -f "$CAND"
P5_MS=$(( ($(now_ms) - T0b) / 1000 ))
P5_STATUS="OK"; [ "$P5_CM" != "0" ] && P5_STATUS="FAIL"; [ "$P5_GLUE" != "0" ] && P5_STATUS="FAIL"
echo "P5 маркер-ценз: tracked ${P5_TOTAL} file(s) (префильтр-кандидаты ${P5_FILES}), конфликт-маркеры ${P5_CM}, branch-glue ${P5_GLUE}, ${P5_MS}ms"
json_phase "P5-marker" "$P5_MS" "$P5_STATUS" "tracked=${P5_TOTAL} candidates=${P5_FILES} conflict=${P5_CM} glue=${P5_GLUE}"

# ============================== ВЕРДИКТ ======================================
TOTAL_MS=$(( ($(now_ms) - G_T0) / 1000 ))
echo "== гейт-вердикт v2: $([ "$FAILED" = "0" ] && echo GREEN || echo RED) (FAIL=$FAILED WARN=$WARNED, total ${TOTAL_MS}ms) =="
if [ -n "$JSON_OUT" ]; then
  {
    printf '{"tool":"merge_safety_gate_v2","ts":"%s","repo":"%s","verdict":"%s","fail":%d,"warn":%d,"total_ms":%d,"phases":[\n' \
      "$TS_JSON" "$REPO" "$([ "$FAILED" = "0" ] && echo GREEN || echo RED)" "$FAILED" "$WARNED" "$TOTAL_MS"
    for i in "${!JSON_PHASES[@]}"; do
      [ "$i" -gt 0 ] && printf ',\n'
      printf '%s' "${JSON_PHASES[$i]}"
    done
    printf '\n]}\n'
  } > "$JSON_OUT"
  echo "json: $JSON_OUT"
fi
if [ "$FAILED" != "0" ]; then
  echo "merge_safety_gate_v2: RED — merge-команда НЕ запускается" >&2
  exit 1
fi
[ "$MODE" = "check-only" ] && exit 0

# ============================== ОБЁРТКА CMD ==================================
if [ "$MODE" = "run" ] || [ "$MODE" = "foreground" ]; then
  CMD_LOG="$LOG_DIR/mergev2_$TS.log"
  printf '%s\n' "${ARGS[@]}" > "$CMD_LOG"; echo "cmd: ${ARGS[*]}"; echo "log: $CMD_LOG"
  if [ "$MODE" = "foreground" ]; then
    bash -c 'cd "$1" || exit 127; exec > "$2" 2>&1; shift 2; exec "$@"' _ "$REPO" "$CMD_LOG" "${ARGS[@]}"
    CMD_RC=$?; echo "cmd exit=$CMD_RC (лог: $CMD_LOG)"
    [ "$CMD_RC" -ne 0 ] && { echo "merge_safety_gate_v2: команда упала — пост-гейты до ремонта не имеют смысла" >&2; exit 1; }
    echo "== пост-гейты (re-run после merge-команды) =="
    exec "$0" $([ "$FAST" = "1" ] && echo --fast) --check-only $([ -n "$JSON_OUT" ] && echo --json "$JSON_OUT")
  else
    setsid bash -c 'cd "$1" || exit 127; exec >> "$2" 2>&1; shift 2; echo "== started $(date -u +%FT%TZ) pid=$$ =="; exec "$@"' _ "$REPO" "$CMD_LOG" "${ARGS[@]}" &
    SPAWN_PID=$!; disown "$SPAWN_PID" 2>/dev/null || true
    echo "detached pid=$SPAWN_PID (setsid, лог: $CMD_LOG)"; echo "merge_safety_gate_v2: GREEN, команда отстреляна в фон"
    exit 0
  fi
fi

# ============================== NEGATIVE-SELFTEST ============================
# 3 исторических дефект-класса ОБЯЗАНЫ ловиться (иначе детектор мёртв):
if [ "$NEGATIVE" = "1" ]; then
  echo "== NEGATIVE-SELFTEST (3 дефект-класса) =="
  RC=0
  # N1: конфликт-маркеры + branch-glue (маркер-ценз P5)
  NT="$(mktemp /tmp/gatev2_neg_XXXXXX.java)"
  printf 'class X { // origin/round-99-fake glue\n<<<<<<< HEAD\nint a;\n=======\nint b;\n>>>>>>> origin/round-99-fake\n}\n' > "$NT"
  N1_FAILED=0; marker_scan "$NT" || N1_FAILED=1
  rm -f "$NT"
  if [ "$N1_FAILED" = "1" ]; then echo "N1 маркер-ценз: PASS (конфликт+glue пойманы)"; else echo "N1 маркер-ценз: FAIL (пропустил дефект)"; RC=1; fi
  # N2: исторический case-arm landmine (canary-405: bee2b585 run_world3.sh, арм @532)
  NL="$(mktemp /tmp/gatev2_neg_landmine.XXXXXX.sh)"
  git show bee2b585:bench/world3/run_world3.sh > "$NL" 2>/dev/null
  if [ -s "$NL" ]; then
    python3 scripts/case_arm_scan.py "$NL" >/dev/null 2>&1; N2_RC=$?
    if [ "$N2_RC" -ne 0 ]; then echo "N2 case-arm landmine @bee2b585: PASS (exit $N2_RC != 0)"; else echo "N2 case-arm landmine: FAIL (scan дал exit 0 — детектор мёртв)"; RC=1; fi
  else echo "N2 case-arm landmine: SKIP (bee2b585 недоступен)"; fi
  rm -f "$NL"
  # N3: битый .class обязан FAIL (ncdfe_guard --negative-test)
  if bash scripts/ncdfe_guard.sh --negative-test >/dev/null 2>&1; then echo "N3 ncdfe corrupt-class: PASS"; else echo "N3 ncdfe corrupt-class: FAIL"; RC=1; fi
  echo "== NEGATIVE-SELFTEST вердикт: $([ "$RC" = "0" ] && echo "3/3 PASS" || echo FAIL) =="
  exit "$RC"
fi
exit 0
