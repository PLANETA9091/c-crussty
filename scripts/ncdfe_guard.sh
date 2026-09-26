#!/usr/bin/env bash
# =============================================================================
# ncdfe_guard.sh — NCDFE-страж авто (TASK-466-C84)
# =============================================================================
# КАНОН (docs/LAB_LEDGER.md L6, закон v18.3/19.0):
#   Перед вердиктом ЛЮБОГО носителя T1 NCDFE=0 ОБЯЗАТЕЛЬНО.
#   Гонка arm/define EntityGoalQueryOps @ MobPushOps.pushables — СИМВОЛ-ПИН
#   (имя метода + дескриптор; канон S43, ветка round-467-s43-pin):
#     MobPushOps.pushables(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/
#     entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;
#     -> invokestatic EntityGoalQueryOps.pushCandidates(...)Z @pc 96 (master 04d58e6c).
#   LINE-ПИНЫ (:NNN) ЗАПРЕЩЕНЫ: дрейф-хроника :463 (×452a, decl 442) -> :467
#   (×456, decl 446) -> decl :448 + javap-LNT touch :469@PC91 (master 04d58e6c;
#   c98 31a2b4a8 блоб byte-identical) — канон-пин :467 = +19 строк от факта,
#   grep line-numbers даёт ложный FAIL (M01 §3). Извлечение пина:
#     ncdfe_guard.sh --pin <blob.class>...   (exit 0 = symbol-pin резолвится)
#   JVM резолвит чужой Ops-класс через определяющий loader ПОСЛЕ arm'а lever'а
#   → NoClassDefFoundError, и HotSpot КЭШИРУЕТ NCDFE per-constant-pool-entry
#   (ошибка повторяется ×N весь ран даже после позднего define).
#   Фикс-паттерн = EARLY-define в раннем arm-хуке (ensure_bridge_early,
#   EARLY_ANCHORS в src/entity_query.rs) + mirror-drift иглы.
#   NCDFE>0 = DELIVERY-FAIL, пара аннулируется (отравленные +20.1/+41.3 ×456).
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
#   ... symbol-pin гейт (S43): после rebuild блобов — ncdfe_guard.sh --pin \
#         mobpush/build/net/minecraft/world/entity/MobPushOps.class
#       (exit 0 = пин pushables(Level,Entity,AABB)->List резолвится; 1 = дрейф
#        дескриптора — переканонизация по S43, НЕ по line-numbers)
#
# USAGE:
#   scripts/ncdfe_guard.sh <файл.class|файл.java> [еще...]
#   scripts/ncdfe_guard.sh --selftest         # 4 известных блоба мастера, 0 FAIL
#   scripts/ncdfe_guard.sh --negative-test    # битый класс обязан дать exit 1
#   scripts/ncdfe_guard.sh --pin <blob.class> [еще...]   # SYMBOL-PIN извлечение
#        # печатает канон-пин (метод+дескриптор -> cross-Ops touch @pc) для каждого
#        # блоба; exit 0 = пин резолвится (для MobPushOps: декларация pushables
#        # совпадает с канон-дескриптором (Level,Entity,AABB)->List), 1 = дрейф/нет,
#        # 2 = файл отсутствует. ТОЛЬКО symbol — line-numbers не канон (S43).
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
  local jp
  jp="$("$JAVAP" -p -c "$path" 2>&1)"
  if [ $? -ne 0 ] || [[ "$jp" == *"Error:"* || "$jp" == *"Exception:"* ]]; then
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

# --- symbol-pin extraction (S43 canon: method name + descriptor, NO line-numbers)
# Canon-pin form:
#   SYMBOL-PIN <blob> <Owner.method(paramTypes)->retType> -> <Target.method(...)> @pc <N>
# For a blob owning `pushables` (MobPushOps): the erased declaration MUST equal
# the canon descriptor (net.minecraft.world.level.Level, net.minecraft.world.entity.Entity,
# net.minecraft.world.phys.AABB) -> java.util.List — иначе SYMBOL-PIN-DRIFT = exit 1.
# javap LNT line-numbers печатает только как INFO — грепать их ЗАПРЕЩЕНО (M01 §3).
print_symbol_pins() { # $@ = blobs; exit 0 pin resolved / 1 drift-or-none / 2 missing
  # канон (S43): erased JVM-дескриптор формы javap, дженерики сняты:
  local canon_params="(net.minecraft.world.level.Level,net.minecraft.world.entity.Entity,net.minecraft.world.phys.AABB)"
  local canon_ret="java.util.List"
  local rc_all=0 found_any=0 b
  for b in "$@"; do
    if [ ! -f "$b" ]; then say "[PIN-SKIP] $b — файл отсутствует"; rc_all=2; continue; fi
    if [ -z "$JAVAP" ] || [ ! -x "$JAVAP" ]; then
      say "[PIN-SKIP] $b — javap недоступен"; rc_all=2; continue; fi
    local jp own pins
    jp="$("$JAVAP" -p -c -l "$b" 2>&1)"
    if [ $? -ne 0 ] || [[ "$jp" == *"Error:"* || "$jp" == *"Exception:"* ]]; then
      say "[PIN-FAIL] $b — javap не разобрал класс"; rc_all=1; continue; fi
    own=$(printf '%s\n' "$jp" | awk '/^ *(public |final |abstract )*(class|interface) /{n=$(NF-1)} END{print n}')
    [ -n "$own" ] || own="$(basename "$b" .class)"
    own="${own%%\$*}"; own="${own##*/}"; own="${own##*.}"   # simple name (FQCN/slashed)
    pins=$(printf '%s\n' "$jp" | awk -v own="$own" '
      function stripgen(s) { gsub(/<[^<>]*>/, "", s); return s }
      function sq(s) { gsub(/^ +| +$/, "", s); gsub(/, /, ",", s); return s }
      # method header: "  MODS RET NAME(PARAMS);" or "  static {};"
      /^  [^ ].*[;{]$/ && !/^    / && $0 !~ /Exception table/ {
        LNT = 0; LNTN = 0
        if ($0 ~ /^  static \{\}/) { M = "<clinit>"; SIG = "()" }
        else {
          line = $0; sub(/^  /, "", line); sub(/;[ \t]*$/, "", line)
          p = index(line, "(")
          head = substr(line, 1, p - 1)              # MODS RET NAME
          n = split(head, hw, " "); M = hw[n]        # NAME = last token before (
          ret = (n >= 2) ? hw[n - 1] : ""
          SIG = sq(stripgen(substr(line, p)))        # (params) — скобки включены
          SIG = SIG "->" sq(stripgen(ret))
        }
        next
      }
      /^ +LineNumberTable:/ { LNT = 1; next }
      LNT && /^ +line [0-9]+: [0-9]+/ { if (LNTN == 0) { LNTN = $2 + 0 }
        if (M == "pushables" && PUSH_LNTN == 0) PUSH_LNTN = LNTN; next }
      /^ +Exception table:/ { LNT = 0; next }
      LNT && /^  [a-zA-Z]/ { LNT = 0 }               # next attribute => LNT over
      /^[[:space:]]*[0-9]+: / {                      # instruction line
        if (match($0, /\/\/ Method [A-Za-z0-9_/$]*Ops\.[A-Za-z0-9_<>$]+:/)) {
          t = substr($0, RSTART + 10, RLENGTH - 11)  # Target.method, srez prefiksa // Method i hvostovogo dvoetochiya
          pc = $1; sub(/:$/, "", pc)
          print "SYMBOL-PIN " own "." M SIG " -> " t " @pc " pc
          if (M == "pushables") { PUSH_SEEN = 1; PUSH_SIG = SIG; PUSH_LNTN = LNTN }
        }
      }
      END { print "META " PUSH_SEEN+0 "|" PUSH_SIG "|" PUSH_LNTN+0 }
    ' 2>/dev/null)
    local meta push_seen sig lntn
    meta=$(printf '%s\n' "$pins" | awk '/^META /{print}' | tail -1)
    push_seen=$(printf '%s' "$meta" | cut -d'|' -f1 | awk '{gsub(/META /,""); print $1+0}')
    sig=$(printf '%s' "$meta" | cut -d'|' -f2)
    lntn=$(printf '%s' "$meta" | cut -d'|' -f3 | awk '{print $1+0}')
    printf '%s\n' "$pins" | grep '^SYMBOL-PIN ' || true
    if [ "$lntn" -gt 0 ]; then
      say "[PIN-INFO] $b — pushables javap-LNT first-line ${lntn} (НЕ канон; line-grep запрещён, M01 §3)"
    fi
    if [ "$own" = "MobPushOps" ]; then
      if [ "$push_seen" -eq 1 ] && [ "$sig" = "${canon_params}->${canon_ret}" ]; then
        say "[CANON-PIN-OK] $b — pushables${sig} == канон-дескриптор (S43: symbol-pin, line-free)"
        found_any=$((found_any+1))
      else
        say "[SYMBOL-PIN-DRIFT] $b — pushables декларация '${sig:-ABSENT}' != канон '${canon_params}->${canon_ret}' — переканонизируй --pin (S43)"; rc_all=1
      fi
    elif [ -n "$(printf '%s\n' "$pins" | grep '^SYMBOL-PIN ')" ]; then
      found_any=$((found_any+1))
    else
      say "[PIN-NONE] $b — cross-Ops touch'ей нет (пин не требуется)"; found_any=$((found_any+1))
    fi
  done
  [ "$found_any" -eq 0 ] && [ "$rc_all" -eq 0 ] && rc_all=1
  return "$rc_all"
}

# --- main --------------------------------------------------------------------
if [ $# -eq 0 ]; then
  say "usage: $0 <file.class|file.java>... | --selftest | --negative-test | --pin <blob.class>..."
  say "exit: 0=OK 1=FAIL 2=SKIP/usage"
  exit 2
fi
if [ "$1" = "--selftest" ]; then run_selftest; exit $?; fi
if [ "$1" = "--negative-test" ]; then run_negative_test; exit $?; fi
if [ "$1" = "--pin" ]; then shift; [ $# -gt 0 ] || { say "usage: $0 --pin <blob.class>..."; exit 2; }; print_symbol_pins "$@"; exit $?; fi

for f in "$@"; do check_one "$f"; done
say "NCDFE-GUARD SUMMARY: ok=$OK fail=$FAIL skip=$SKIP (exit 0/1/2 = OK/FAIL/SKIP; канон LAB_LEDGER L6: T1 NCDFE=0 до вердикта)"
if [ "$FAIL" -gt 0 ]; then exit 1; fi
if [ "$SKIP" -gt 0 ]; then exit 2; fi
exit 0
