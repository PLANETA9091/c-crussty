#!/usr/bin/env bash
# =============================================================================
# ncdfe_guard.sh — NCDFE-страж авто (TASK-466-C84)
# =============================================================================
# КАНОН (docs/LAB_LEDGER.md L6, закон v18.3/19.0):
#   Перед вердиктом ЛЮБОГО носителя T1 NCDFE=0 ОБЯЗАТЕЛЬНО.
#   Гонка arm/define EntityGoalQueryOps @ MobPushOps.pushables:467:
#   JVM резолвит чужой Ops-класс через определяющий loader ПОСЛЕ arm'а lever'а
#   → NoClassDefFoundError, и HotSpot КЭШИРУЕТ NCDFE per-constant-pool-entry
#   (ошибка повторяется ×N весь ран даже после позднего define).
#   Фикс-паттерн = EARLY-define в раннем arm-хуке (ensure_bridge_early,
#   EARLY_ANCHORS в src/entity_query.rs) + mirror-drift иглы.
#   NCDFE>0 = DELIVERY-FAIL, пара аннулируется (отравленные +20.1/+41.3 ×456).
#
# FIX Л180h (S53, ветка round-468-s53-ncdfeguard) — false-FAIL на workerError-substring:
#   ошибка javap детектится ТОЛЬКО по exit-коду + строкам, НАЧИНАЮЩИМСЯ с
#   "Error:"/"Exception:" (line-anchored ^\s*(Error|Exception):). Прежний substring
#   *"Error:"* бил ЗДОРОВЫЕ блобы: "// Field workerError:Ljava/lang/Throwable;" в
#   RegionTickOps.class (мастер c1196321, 29 таких строк в дизасме) матрился как
#   "битый constant pool" → false-FAIL exit 1. workerError-строка в логе javap
#   ≠ ошибка стража. "Exception table:" якорю не соответствует (двоеточие не
#   сразу после слова) — C3-парсинг не затронут.
#
# ЧТО ПРОВЕРЯЕТ (javap, без запуска JVM-кода):
#   .class-блоб:
#     C1 CAFEBABE + class-file major == 65 (--release 21 = kernel JVM; src
#        отказывается армить при major > живой JVM — stale-блоб = мертвая плоскость);
#     C2 define-порядок: <clinit> НЕ должен ссылаться на чужие *Ops классы —
#        cross-Ops резолв в clinit = разрешение чужого моста в момент define
#        этого класса = тот самый arm/define рейс (EARLY-define нарушен);
#     C3 throwable-маркеры: если blob трогает чужие *Ops классы — в классе
#        должен быть хотя бы один Exception-table handler
#        `Class java/lang/Throwable` / `any` (fail-closed маркер дисциплины
#        ваниль-фолбэка); отсутствие = FAIL;
#     C4 INFO: cross-Ops touch'и, не покрытые Throwable-хендлером, помечаются
#        "arm-gated" — канон ×456: такой touch обязан стоять за лениво
#        резолвимой lever-веткой + HARD publish gate ensure_bridge_early на
#        arm-слое (Rust). Это INFO, не FAIL (мобильно-каноничная форма).
#   .java-исходник (зеркало на уровне исходников):
#     J1 cross-Ops ссылки в static{} блоке / static-field инициализаторах = FAIL;
#     J2 если есть cross-Ops touch'и и НЕТ ни одного `catch (Throwable` = FAIL.
#
# EXIT-КОДЫ: 0 = OK (все входы чисты), 1 = FAIL (NCDFE-риск/битый класс),
#            2 = SKIP/usage (файл отсутствует, тип не поддержан, javap недоступен).
# Любой FAIL >> 1; без FAIL, но с SKIP >> 2; все OK >> 0.
#
# ИНТЕГРАЦИЯ В MERGE-СКРИПТЫ (куда вставлять):
#   ... после rebuild блобов (scripts/build_*_ops.sh), ДО ledger-push/вердикта:
#     # --- GATE: NCDFE-страж (canon LAB_LEDGER L6: T1 NCDFE=0 до вердикта) ---
#     if ! bash scripts/ncdfe_guard.sh \
#            mobpush/build/net/minecraft/world/entity/MobPushOps.class \
#            entitygoalquery/build/net/minecraft/world/entity/EntityGoalQueryOps.class; then
#       echo "MERGE ABORT: NCDFE-GUARD FAIL = DELIVERY-FAIL (отравленные пары ×456)" >&2
#       exit 1
#     fi
#   ... пассивный дым-гейт в начале гейт-стадии: bash scripts/ncdfe_guard.sh --selftest
#   ... самодостаточный негатив-контроль стенда: bash scripts/ncdfe_guard.sh --negative-test
#
# USAGE:
#   scripts/ncdfe_guard.sh <файл.class|файл.java> [еще...]
#   scripts/ncdfe_guard.sh --selftest         # 4 известных блоба мастера, 0 FAIL
#   scripts/ncdfe_guard.sh --negative-test    # битый класс обязан дать exit 1
#
# ОКРУЖЕНИЕ: NCDFE_JAVAP — путь к javap; иначе /tmp/jdk21/bin/javap,
#            /home/z/tools/jdk-21.0.12.1+1/bin/javap, `command -v javap`
#            (поднять стенд: bash scripts/ensure_javap.sh).
# =============================================================================
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# --- javap resolution --------------------------------------------------------
JAVAP="${NCDFE_JAVAP:-}"
if [ -z "$JAVAP" ]; then
  for c in /tmp/jdk21/bin/javap /home/z/tools/jdk-21.0.12.1+1/bin/javap; do
    if [ -x "$c" ]; then JAVAP="$c"; break; fi
  done
fi
[ -n "$JAVAP" ] || JAVAP="$(command -v javap 2>/dev/null || true)"

OK=0; FAIL=0; SKIP=0
say()  { printf '%s\n' "$*"; }

# --- check one .class blob ---------------------------------------------------
check_class() { # $1 = path ; verdict lines; returns 0 OK / 1 FAIL / 2 SKIP
  local path="$1"
  if [ ! -f "$path" ]; then
    say "[SKIP] $path — файл отсутствует"; return 2
  fi
  if [ -z "$JAVAP" ] || [ ! -x "$JAVAP" ]; then
    say "[SKIP] $path — javap недоступен (подними стенд: bash scripts/ensure_javap.sh)"; return 2
  fi
  # C1: magic + major (bytes 0-3 magic, 6-7 major, big-endian)
  local magic major
  magic=$(od -An -tx1 -j0 -N4 "$path" | tr -d ' \n')
  if [ "$magic" != "cafebabe" ]; then
    say "[FAIL] $path — битый класс: magic $magic != cafebabe (truncate/не classfile)"; return 1
  fi
  major=$(od -An -tu1 -j6 -N2 "$path" | awk '{print $1*256+$2}')
  if [ "$major" != "65" ]; then
    say "[FAIL] $path — major $major != 65 (пересобери --release 21: kernel JVM, arm-слой отказывает major>живой JVM)"; return 1
  fi
  # C2/C3/C4: javap -p -c parse
  # FIX Л180h/S53: stderr в отдельный файл; реальная ошибка javap = ненулевой
  #   exit ИЛИ line-anchored "Error:"/"Exception:" (stderr/stdout). Substring
  #   *"Error:"* ловил комментарии дизасма здоровых блобов (workerError-field
  #   @ RegionTickOps.class = false-FAIL). mktemp-fail деградирует в
  #   fail-closed (jrc!=0 → FAIL); молчаливый SKIP невозможен.
  local jp jout jerr jrc jtmp
  jtmp="$(mktemp "${TMPDIR:-/tmp}/ncdfe_jerr.XXXXXX" 2>/dev/null)" || jtmp="/tmp/ncdfe_jerr.$$"
  jout="$("$JAVAP" -p -c "$path" 2>"$jtmp")"; jrc=$?
  jerr="$(cat "$jtmp" 2>/dev/null)"
  rm -f "$jtmp"
  jp="$jout"
  if [ "$jrc" -ne 0 ] \
     || printf '%s\n' "$jerr" | grep -Eq '^[[:space:]]*(Error|Exception):' \
     || printf '%s\n' "$jout" | grep -Eq '^[[:space:]]*(Error|Exception):'; then
    say "[FAIL] $path — javap не смог разобрать класс (битый/corrupt constant pool)"; return 1
  fi
  # own simple name: last class declaration header "public|final|abstract class|interface ... <Name> {"
  local own
  own=$(printf '%s\n' "$jp" | awk '/^ (public |final |abstract )*(class|interface) /{n=$(NF-1)} END{print n}')
  [ -n "$own" ] || own="$(basename "$path" .class)"
  own="${own%%\$*}"
  # Walk javap: per method collect cross-Ops touch PCs; per method Exception table.
  local analysis
  analysis=$(printf '%s\n' "$jp" | awk -v own="$own" '
    function opssimple(s, t) {           # extract "XxxOps" token from javap comment
      if (match(s, /[A-Za-z_][A-Za-z0-9_]*Ops[.:;]/)) {
        t = substr(s, RSTART, RLENGTH); t = substr(t, 1, length(t)-1)
        if (t != own) return t
      }
      return ""
    }
    function flushm() {                  # coverage of this method touches
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
    # new method header (2-space indent, ends with ";", not an instruction/comment/table row)
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
    /^[[:space:]]*[0-9]+: / {             # instruction line
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

# --- check one .java source --------------------------------------------------
check_java() { # $1 = path
  local path="$1"
  if [ ! -f "$path" ]; then
    say "[SKIP] $path — файл отсутствует"; return 2
  fi
  local own; own="$(basename "$path" .java)"
  # J1: cross-Ops refs в static{} блоках и static-field инициализаторах
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
  # cross-Ops touch sites + throwable markers
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

# --- dispatch one input ------------------------------------------------------
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
  case "$rc" in 0) OK=$((OK+1)) ;; 1) FAIL=$((FAIL+1)) ;; *) SKIP=$((SKIP+1)) ;; esac
  return "$rc"
}

# --- self-test on known master blobs -----------------------------------------
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

# --- negative test: corrupt class must FAIL with exit 1 ----------------------
run_negative_test() {
  local tmp rc
  tmp="$(mktemp /tmp/ncdfe_negative_XXXXXX.class)"
  printf 'NOT-CA-FE-BA-BE garbage %s' "$(head -c 200 /dev/urandom | base64 -w0 2>/dev/null)" > "$tmp"
  say "NCDFE-GUARD NEGATIVE-TEST: битый класс $tmp"
  check_one "$tmp"; rc=$?
  rm -f "$tmp"
  if [ "$rc" -eq 1 ]; then say "NEGATIVE-TEST PASS (битый класс -> exit 1)"; return 0; fi
  say "NEGATIVE-TEST FAIL (ожидался exit 1, получен $rc)"; return 1
}

# --- main --------------------------------------------------------------------
if [ $# -eq 0 ]; then
  say "usage: $0 <file.class|file.java>... | --selftest | --negative-test"
  say "exit: 0=OK 1=FAIL 2=SKIP/usage"
  exit 2
fi
if [ "$1" = "--selftest" ]; then run_selftest; exit $?; fi
if [ "$1" = "--negative-test" ]; then run_negative_test; exit $?; fi

for f in "$@"; do check_one "$f"; done
say "NCDFE-GUARD SUMMARY: ok=$OK fail=$FAIL skip=$SKIP (exit 0/1/2 = OK/FAIL/SKIP; канон LAB_LEDGER L6: T1 NCDFE=0 до вердикта)"
if [ "$FAIL" -gt 0 ]; then exit 1; fi
if [ "$SKIP" -gt 0 ]; then exit 2; fi
exit 0
