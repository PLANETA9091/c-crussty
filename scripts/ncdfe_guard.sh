#!/usr/bin/env bash
# =============================================================================
# ncdfe_guard.sh — NCDFE-страж авто (TASK-466-C84; R2 fail-closed ROUND-471-S13)
# =============================================================================
# R2 (S13 ROUND-471) закрывает fail-open пути R1-класса (Л208 S16 x470):
#   FO-1 javap-нет/блоб-нет -> SKIP exit 2 = зелёный у наивных колл-сайтов
#        -> NCDFE_STRICT=1: SKIP считается FAIL (exit 1) в гейт-контексте;
#   FO-2 слепой парсер: пустой/усечённый javap-выход -> touch=0 -> [OK] exit 0
#        (демо: NCDFE_JAVAP=stub на MobPushOps дал [OK] touch=0, exit 0)
#        -> C0 parse-floor: 0 инструкций/0 заголовков методов = FAIL, не OK;
#   FO-3 selftest раньше покрывал только 4 хороших блоба + магию-негатив —
#        C2/C3/J1/J2-детекторы были непокрыты -> режим --vectors: 5 векторов,
#        все обязаны отработать как ожидалось, иначе вектор-FAIL.
# КАНОН (docs/LAB_LEDGER.md L6, закон v18.3/19.0/v20):
#   Перед вердиктом ЛЮБОГО носителя T1 NCDFE=0 ОБЯЗАТЕЛЬНО.
#   Гонка arm/define EntityGoalQueryOps @ MobPushOps.pushables:467:
#   JVM резолвит чужой Ops-класс через определяющий loader ПОСЛЕ arm'а lever'а
#   -> NoClassDefFoundError, HotSpot КЭШИРУЕТ NCDFE per-constant-pool-entry
#   (ошибка повторяется ×N весь ран даже после позднего define; cv3-1 ×3938).
#   Фикс-паттерн = EARLY-define (ensure_bridge_early, EARLY_ANCHORS
#   src/entity_query.rs) + probe-then-define (mobs_manager.rs:436-451).
#
# ПРОВЕРКИ .class (javap, без запуска JVM-кода):
#   C0 parse-floor (R2): javap-выход обязан содержать >=1 инструкцию/заголовок;
#   C1 CAFEBABE + major==65; C2 <clinit> без cross-Ops; C3 throwable-маркеры;
#   C4 INFO: uncovered/arm-gated touch'и (канон ×456: lever-ветка + HARD gate).
# ПРОВЕРКИ .java: J1 static-init cross-Ops; J2 catch(Throwable) при touch'ах.
# EXIT-КОДЫ: 0=OK, 1=FAIL (в т.ч. SKIP при NCDFE_STRICT=1), 2=SKIP/usage.
# USAGE:
#   scripts/ncdfe_guard.sh <файл.class|файл.java> [еще...]
#   scripts/ncdfe_guard.sh --selftest         # 4 блоба мастера, 0 FAIL
#   scripts/ncdfe_guard.sh --vectors          # R2: 5 fail-open векторов
#   scripts/ncdfe_guard.sh --negative-test    # битый класс обязан exit 1
# ОКРУЖЕНИЕ: NCDFE_JAVAP, NCDFE_JAVAC (для --vectors), NCDFE_STRICT=1.
# =============================================================================
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

JAVAP="${NCDFE_JAVAP:-}"
if [ -z "$JAVAP" ]; then
  for c in /tmp/jdk21/bin/javap /home/z/tools/jdk-21.0.12.1+1/bin/javap; do
    if [ -x "$c" ]; then JAVAP="$c"; break; fi
  done
fi
[ -n "$JAVAP" ] || JAVAP="$(command -v javap 2>/dev/null || true)"
JAVAC="${NCDFE_JAVAC:-}"
if [ -z "$JAVAC" ] && [ -n "$JAVAP" ]; then JAVAC="$(dirname "$JAVAP")/javac"; fi

STRICT="${NCDFE_STRICT:-0}"
OK=0; FAIL=0; SKIP=0
say()  { printf '%s\n' "$*"; }

check_class() { # $1 = path ; returns 0 OK / 1 FAIL / 2 SKIP
  local path="$1"
  if [ ! -f "$path" ]; then
    say "[SKIP] $path — файл отсутствует"; return 2
  fi
  if [ -z "$JAVAP" ] || [ ! -x "$JAVAP" ]; then
    say "[SKIP] $path — javap недоступен (подними стенд: bash scripts/ensure_javap.sh)"; return 2
  fi
  local magic major
  magic=$(od -An -tx1 -j0 -N4 "$path" | tr -d ' \n')
  if [ "$magic" != "cafebabe" ]; then
    say "[FAIL] $path — битый класс: magic $magic != cafebabe (truncate/не classfile)"; return 1
  fi
  major=$(od -An -tu1 -j6 -N2 "$path" | awk '{print $1*256+$2}')
  if [ "$major" != "65" ]; then
    say "[FAIL] $path — major $major != 65 (пересобери --release 21: kernel JVM, arm-слой отказывает major>живой JVM)"; return 1
  fi
  local jp
  jp="$("$JAVAP" -p -c "$path" 2>&1)"
  if [ $? -ne 0 ] || [[ "$jp" == Error:* || "$jp" == Exception:* ]]; then
    say "[FAIL] $path — javap не смог разобрать класс (битый/corrupt constant pool)"; return 1
  fi
  # C0 (R2): parse-floor — слепой парсер не имеет права давать [OK]
  local floor
  floor=$(printf '%s\n' "$jp" | grep -Ec '^[[:space:]]*[0-9]+: |^  [^ ].*[;{]$' || true)
  if [ "${floor:-0}" -eq 0 ]; then
    say "[FAIL] $path — parse-floor: javap-выход пуст/не распознан (инструкций=0; слепой парсер = fail-open R2 закрыт)"; return 1
  fi
  local own
  own=$(printf '%s\n' "$jp" | awk '/^ (public |final |abstract )*(class|interface) /{n=$(NF-1)} END{print n}')
  [ -n "$own" ] || own="$(basename "$path" .class)"
  own="${own%%\$*}"
  local analysis
  analysis=$(printf '%s\n' "$jp" | awk -v own="$own" '
    function opssimple(s, t) {
      if (match(s, /[A-Za-z_][A-Za-z0-9_]*Ops[.:;]/)) {
        t = substr(s, RSTART, RLENGTH); t = substr(t, 1, length(t)-1)
        if (t != own) return t
      }
      return ""
    }
    function flushm() {
      for (i in TPC) {
        cov = 0
        for (j = 1; j <= EN; j++)
          if (ETYPE[j] ~ /java\/lang\/Throwable/ || ETYPE[j] == "any")
            if (TPC[i] >= EF[j] && TPC[i] < ET[j]) { cov = 1; break }
        if (!cov) UNCOV++
        TOT++
      }
      delete TPC; delete EF; delete ETYPE; EN = 0
    }
    /^  [^ ].*[;{]$/ && !/^    / && $0 !~ /Exception table/ {
      if ($0 ~ /^  static \{\}/) { flushm(); M = "<clinit>" }
      else { flushm(); M = $0; sub(/^  /, "", M) }
      TABLE = 0; next
    }
    /^      Exception table:/ { TABLE = 1; next }
    TABLE && /^ +[0-9]+ +[0-9]+ +[0-9]+/ {
      EN++; EF[EN] = $1; ET[EN] = $2; ETYPE[EN] = $NF; next
    }
    TABLE && !/^ +[0-9]+/ { TABLE = 0 }
    /^[[:space:]]*[0-9]+: / {
      if (match($0, /\/\/ .*/)) {
        t = opssimple(substr($0, RSTART))
        if (t != "") {
          if (M == "<clinit>") CHIT = CHIT " " t
          pc = $1; sub(/:$/, "", pc); TPC[pc] = 1
          if (!(t in SEEN)) { SEEN[t] = 1; LIST = LIST " " t }
        }
      }
    }
    END { flushm(); print TOT+0, UNCOV+0, length(SEEN), (CHIT == "" ? "-" : CHIT), (LIST == "" ? "-" : LIST) }
  ')
  read -r TOT UNCOV NREF CHIT LIST <<< "$analysis"
  [ "$CHIT" = "-" ] && CHIT=""
  [ "$LIST" = "-" ] && LIST=""
  local HANDLERS
  HANDLERS=$(printf '%s\n' "$jp" | grep -Ec 'Class java/lang/Throwable|^[[:space:]]*[0-9]+ +[0-9]+ +[0-9]+ +any$' || true)
  if [ -n "${CHIT// /}" ]; then
    say "[FAIL] $path — define-порядок: <clinit> резолвит чужие Ops классы:${CHIT} (arm/define рейс: define через этот loader раньше EARLY-define моста; канон ×456)"; return 1
  fi
  if [ "${TOT:-0}" -gt 0 ] && [ "${HANDLERS:-0}" -eq 0 ]; then
    say "[FAIL] $path — throwable-маркеры отсутствуют: cross-Ops touch'ей ${TOT} (ссылки:${LIST:-}), но ни одного handler'а java/lang/Throwable|any (нет fail-closed фолбэка)"; return 1
  fi
  say "[OK]   $path — major=65 crossOps-классы:${LIST:- none} touch=${TOT:-0} (uncovered/arm-gated=${UNCOV:-0}, INFO: обязан быть за lever-веткой + ensure_bridge_early HARD gate) throwable-handlers=${HANDLERS:-0}"
  return 0
}

check_java() { # $1 = path
  local path="$1"
  if [ ! -f "$path" ]; then
    say "[SKIP] $path — файл отсутствует"; return 2
  fi
  local own; own="$(basename "$path" .java)"
  local clinit_hits
  clinit_hits=$(awk -v own="$own" '
    function opshits(line,   t, out, s) {
      out = ""; s = line
      while (match(s, /[A-Za-z_][A-Za-z0-9_]*Ops[.:]/)) {
        t = substr(s, RSTART, RLENGTH); t = substr(t, 1, length(t)-1)
        if (t != own) out = out " " t
        s = substr(s, RSTART+RLENGTH)
      }
      return out
    }
    { line = $0 }
    line ~ /(^|[^A-Za-z0-9_])static[ \t]*\{/ { IN = 1; DEPTH = 0 }
    IN {
      n = gsub(/\{/, "{", line); d2 = gsub(/\}/, "}", line)
      DEPTH += n - d2
      h = opshits(line); if (h != "") HIT = HIT h
      if (DEPTH <= 0 && line !~ /static[ \t]*\{[^\}]*\}/) { IN = 0 }
      if (n > d2) IN = 1
      next
    }
    /^[[:space:]]*static\b/ && /=/ {
      h = opshits(line); if (h != "") HIT = HIT h
    }
    END { print HIT }
  ' "$path")
  local touches handlers
  touches=$(grep -oE '[A-Za-z_][A-Za-z0-9_]*Ops[.:]' "$path" 2>/dev/null \
            | sed 's/[.:]$//' | grep -v "^$own\$" | sort -u | tr '\n' ' ')
  handlers=$(grep -Ec 'catch[[:space:]]*\([[:space:]]*(java\.lang\.)?Throwable' "$path" || true)
  if [ -n "${clinit_hits// /}" ]; then
    say "[FAIL] $path — define-порядок: static-init/static-field инициализаторы резолвят чужие Ops классы:${clinit_hits} (define-time resolution = arm/define рейс ×456)"; return 1
  fi
  if [ -n "${touches// /}" ] && [ "${handlers:-0}" -eq 0 ]; then
    say "[FAIL] $path — throwable-маркеры отсутствуют: cross-Ops touch'и (${touches}), а catch (Throwable ни одного — нет fail-closed фолбэка"; return 1
  fi
  say "[OK]   $path — crossOps:${touches:- none} catch(Throwable)=${handlers:-0} static-init чист"
  return 0
}

check_one() {
  local path="$1" rc=0
  case "$path" in
    *.class) check_class "$path"; rc=$? ;;
    *.java)  check_java  "$path"; rc=$? ;;
    *)
      if [ -f "$path" ]; then
        say "[SKIP] $path — тип не поддержан (нужен .class или .java)"; rc=2
      else
        say "[SKIP] $path — файл отсутствует"; rc=2
      fi ;;
  esac
  case "$rc" in 0) OK=$((OK+1)) ;; 1) FAIL=$((FAIL+1)) ;; *) SKIP=$((SKIP+1)) ;;
  esac
  return "$rc"
}

SELFTEST_BLOBS=(
  "mobpush/build/net/minecraft/world/entity/MobPushOps.class"
  "entitygoalquery/build/net/minecraft/world/entity/EntityGoalQueryOps.class"
  "queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class"
  "chunksched/build/net/minecraft/server/level/ChunkSchedOps.class"
)
run_selftest() {
  say "NCDFE-GUARD SELFTEST (T1 NCDFE=0 канон, LAB_LEDGER L6) — блобы мастера:"
  local b
  for b in "${SELFTEST_BLOBS[@]}"; do
    if [ -f "$ROOT/$b" ]; then check_one "$ROOT/$b"
    else say "[SKIP] $ROOT/$b — блоба нет в дереве"; SKIP=$((SKIP+1)); fi
  done
  say "T1-NCDFE-SELFTEST: ok=$OK fail=$FAIL skip=$SKIP"
  if [ "$FAIL" -eq 0 ] && [ "$OK" -ge 3 ]; then
    say "T1-NCDFE-SELFTEST PASS"; return 0
  fi
  say "T1-NCDFE-SELFTEST FAIL (требование канона: 0 FAIL на известных блобах)"; return 1
}

run_negative_test() {
  local tmp rc
  tmp="$(mktemp "${TMPDIR:-/tmp}/ncdfe_negative_XXXXXX.class")"
  printf 'NOT-CA-FE-BA-BE garbage %s' "$(head -c 200 /dev/urandom | base64 -w0 2>/dev/null)" > "$tmp"
  say "NCDFE-GUARD NEGATIVE-TEST: битый класс $tmp"
  check_one "$tmp"; rc=$?
  rm -f "$tmp"
  if [ "$rc" -eq 1 ]; then say "NEGATIVE-TEST PASS (битый класс -> exit 1)"; return 0; fi
  say "NEGATIVE-TEST FAIL (ожидался exit 1, получен $rc)"; return 1
}

# --- R2 vectors: 5 fail-open closures, each must behave as expected -----------
run_vectors() { # returns 0 iff all vectors PASS
  local vp=0 vf=0 tmpd rc out
  tmpd="$(mktemp -d "${TMPDIR:-/tmp}/ncdfe_vectors_XXXXXX")"
  say "NCDFE-GUARD VECTORS (R2 fail-closed, ROUND-471-S13):"
  # V1 blind-parser: пустой javap-выход обязан дать parse-floor FAIL (R1-аналог)
  if [ -f "$ROOT/${SELFTEST_BLOBS[0]}" ]; then
    printf '#!/bin/sh\nexit 0\n' > "$tmpd/blindjavap"; chmod +x "$tmpd/blindjavap"
    out="$(NCDFE_JAVAP="$tmpd/blindjavap" bash "$0" "$ROOT/${SELFTEST_BLOBS[0]}" 2>&1)"; rc=$?
    if [ "$rc" -eq 1 ] && printf '%s' "$out" | grep -q 'parse-floor'; then
      say "V1 blind-parser->parse-floor FAIL: PASS"; vp=$((vp+1))
    else say "V1 blind-parser->parse-floor FAIL: FAIL (rc=$rc)"; vf=$((vf+1)); fi
  else say "V1: FAIL (нет ${SELFTEST_BLOBS[0]})"; vf=$((vf+1)); fi
  # V2 clinit-touch vector обязан FAIL C2; V3 touch-no-handler обязан FAIL C3;
  # V4 touch+handler обязан OK (вектор-пары компилируются javac на месте)
  if [ -n "$JAVAC" ] && [ -x "$JAVAC" ]; then
    cat > "$tmpd/OtherOps.java" <<'EOF'
public class OtherOps { public static int idCount = 7; }
EOF
    cat > "$tmpd/ClinitTouchOps.java" <<'EOF'
public class ClinitTouchOps { static { int x = OtherOps.idCount; } static int X; public static int get() { return X; } }
EOF
    cat > "$tmpd/TouchNoHandlerOps.java" <<'EOF'
public class TouchNoHandlerOps { public int f(OtherOps o) { return o.idCount; } }
EOF
    cat > "$tmpd/TouchHandlerOps.java" <<'EOF'
public class TouchHandlerOps { public int f(OtherOps o) { try { return o.idCount; } catch (Throwable t) { return -1; } } }
EOF
    "$JAVAC" -d "$tmpd" "$tmpd"/OtherOps.java "$tmpd"/ClinitTouchOps.java "$tmpd"/TouchNoHandlerOps.java "$tmpd"/TouchHandlerOps.java >/dev/null 2>&1
    out="$(bash "$0" "$tmpd/ClinitTouchOps.class" 2>&1)"; rc=$?
    if [ "$rc" -eq 1 ] && printf '%s' "$out" | grep -q 'define-порядок'; then
      say "V2 clinit-touch->FAIL C2: PASS"; vp=$((vp+1))
    else say "V2 clinit-touch->FAIL C2: FAIL (rc=$rc)"; vf=$((vf+1)); fi
    out="$(bash "$0" "$tmpd/TouchNoHandlerOps.class" 2>&1)"; rc=$?
    if [ "$rc" -eq 1 ] && printf '%s' "$out" | grep -q 'throwable-маркеры отсутствуют'; then
      say "V3 touch-no-handler->FAIL C3: PASS"; vp=$((vp+1))
    else say "V3 touch-no-handler->FAIL C3: FAIL (rc=$rc)"; vf=$((vf+1)); fi
    out="$(bash "$0" "$tmpd/TouchHandlerOps.class" 2>&1)"; rc=$?
    if [ "$rc" -eq 0 ]; then
      say "V4 touch+handler->OK: PASS"; vp=$((vp+1))
    else say "V4 touch+handler->OK: FAIL (rc=$rc)"; vf=$((vf+1)); fi
  else
    say "V2-V4: FAIL (javac недоступен: задай NCDFE_JAVAC)"; vf=$((vf+3))
  fi
  # V5 strict-skip: NCDFE_STRICT=1 превращает SKIP в FAIL (fail-closed гейт)
  out="$(NCDFE_STRICT=1 bash "$0" "$tmpd/NoSuchGhostOps.class" 2>&1)"; rc=$?
  if [ "$rc" -eq 1 ]; then
    say "V5 strict-skip->exit1: PASS"; vp=$((vp+1))
  else say "V5 strict-skip->exit1: FAIL (rc=$rc)"; vf=$((vf+1)); fi
  rm -rf "$tmpd"
  say "NCDFE-VECTORS: pass=$vp fail=$vf (of 5)"
  if [ "$vf" -eq 0 ]; then say "NCDFE-VECTORS PASS"; return 0; fi
  say "NCDFE-VECTORS FAIL"; return 1
}

if [ $# -eq 0 ]; then
  say "usage: $0 <file.class|file.java>... | --selftest | --vectors | --negative-test"
  say "exit: 0=OK 1=FAIL (в т.ч. SKIP при NCDFE_STRICT=1) 2=SKIP/usage"
  exit 2
fi
if [ "$1" = "--selftest" ]; then run_selftest; exit $?; fi
if [ "$1" = "--vectors" ]; then run_vectors; exit $?; fi
if [ "$1" = "--negative-test" ]; then run_negative_test; exit $?; fi

for f in "$@"; do check_one "$f"; done
say "NCDFE-GUARD SUMMARY: ok=$OK fail=$FAIL skip=$SKIP (exit 0/1/2 = OK/FAIL/SKIP; канон LAB_LEDGER L6: T1 NCDFE=0 до вердикта)"
if [ "$FAIL" -gt 0 ]; then exit 1; fi
if [ "$SKIP" -gt 0 ]; then
  if [ "$STRICT" = "1" ]; then say "NCDFE_STRICT=1: SKIP->$FAIL (fail-closed R2)"; exit 1; fi
  exit 2
fi
exit 0
