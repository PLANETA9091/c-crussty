#!/usr/bin/env bash
# profile_server.sh — one-shot JFR profiling harness for the live
# Purpur+CRUSSTY server (or any same-user JVM whose cmdline matches
# launcher.jar|purpur|paper).
#
# Pipeline:
#   1. find the server JVM PID (pgrep -f 'launcher.jar|purpur|paper',
#      filtered to real java executables; override with --pid / $SERVER_PID)
#   2. start a JFR recording via `jcmd JFR.start settings=profile`
#        --mode jfr  : duration-bounded recording, JVM stops+dumps it itself
#        --mode jcmd : recording without duration, we stop it explicitly
#   3. optionally run --workload CMD in parallel (chunk-load burst etc.),
#      auto-TERM'd shortly after the recording window ends
#   4. stop/collect the .jfr into --output DIR
#   5. quick triage: `jfr summary`, top hot methods (self samples) and top
#      allocation sites; emits flame_diff.py-friendly CSVs next to the .jfr
#
# Usage:
#   scripts/profile_server.sh [--duration SECONDS] [--output DIR]
#                             [--mode jfr|jcmd] [--jdk PATH] [--workload CMD]
#                             [--pid PID] [--keep-raw] [--no-triage]
#
# Defaults: duration=120, output=profiles/, mode=jfr, jdk=$--jdk ->
#           /home/z/jdk21 -> $JAVA_HOME -> PATH.
#
# Exit codes: 0 ok | 1 usage | 2 no jcmd/jfr | 3 server PID not found |
#             4 jcmd control failed | 5 recording file missing
#
# No sudo required: JFR attach works for same-user JVMs.

set -euo pipefail

DURATION=120
OUTDIR="profiles"
MODE="jfr"
JDK_ARG="${PROFILE_JDK:-}"
WORKLOAD=""
PID_ARG="${SERVER_PID:-}"
KEEP_RAW=0
NO_TRIAGE=0

usage() {
  cat >&2 <<'EOF'
Usage: scripts/profile_server.sh [options]

Options:
  --duration SECONDS   recording length (default: 120)
  --output DIR         artifact directory (default: profiles/)
  --mode jfr|jcmd      jfr = duration-bounded, JVM auto-stops+dumps (default)
                       jcmd = open-ended, stopped explicitly by this script
  --jdk PATH           JDK home with bin/jcmd+bin/jfr
                       (default: $PROFILE_JDK, /home/z/jdk21, $JAVA_HOME, PATH)
  --workload CMD       shell command run DURING the recording window
                       (chunk-load burst, bot traffic, ...); auto-TERM'd after it
  --pid PID            target JVM PID (default: auto-detect 'launcher.jar|purpur|paper';
                       env override: SERVER_PID)
  --keep-raw           also save raw `jfr print --events jdk.ExecutionSample` dump
                       (input for flame_diff.py text mode; can be large)
  --no-triage          skip jfr summary / hot-method / allocation triage
  -h, --help           this help

Exit codes: 0 ok | 1 usage | 2 no jcmd/jfr | 3 server PID not found |
            4 jcmd control failed | 5 recording file missing
EOF
}

die() { # die CODE MSG...
  local code="$1"; shift
  echo "ERROR: $*" >&2
  exit "$code"
}

while [ $# -gt 0 ]; do
  case "$1" in
    --duration)  [ $# -ge 2 ] || die 1 "--duration needs SECONDS";  DURATION="$2";  shift 2 ;;
    --output)    [ $# -ge 2 ] || die 1 "--output needs DIR";        OUTDIR="$2";    shift 2 ;;
    --mode)      [ $# -ge 2 ] || die 1 "--mode needs jfr|jcmd";     MODE="$2";      shift 2 ;;
    --jdk)       [ $# -ge 2 ] || die 1 "--jdk needs PATH";          JDK_ARG="$2";   shift 2 ;;
    --workload)  [ $# -ge 2 ] || die 1 "--workload needs CMD";      WORKLOAD="$2";  shift 2 ;;
    --pid)       [ $# -ge 2 ] || die 1 "--pid needs PID";           PID_ARG="$2";   shift 2 ;;
    --keep-raw)  KEEP_RAW=1; shift ;;
    --no-triage) NO_TRIAGE=1; shift ;;
    -h|--help)   usage; exit 0 ;;
    *)           usage; die 1 "unknown argument: $1" ;;
  esac
done

case "$DURATION" in
  ''|*[!0-9]*) die 1 "--duration must be a positive integer (seconds), got '$DURATION'" ;;
esac
[ "$DURATION" -ge 5 ] || die 1 "--duration must be >= 5 s to collect a usable number of samples"
case "$MODE" in jfr|jcmd) ;; *) die 1 "--mode must be 'jfr' or 'jcmd', got '$MODE'" ;; esac

# ---------------------------------------------------------------- tools ----
# Resolve a JDK home containing bin/jcmd. Order: --jdk/-PROFILE_JDK,
# /home/z/jdk21, $JAVA_HOME, PATH.
resolve_jdk() {
  local c p d
  for c in "$JDK_ARG" "/home/z/jdk21" "${JAVA_HOME:-}"; do
    [ -n "$c" ] && [ -x "$c/bin/jcmd" ] && { printf '%s\n' "$c"; return 0; }
  done
  if p="$(command -v jcmd 2>/dev/null)"; then
    d="$(cd -- "$(dirname -- "$p")/.." && pwd)"
    [ -x "$d/bin/jcmd" ] && { printf '%s\n' "$d"; return 0; }
    printf '%s\n' "$(dirname -- "$p")/.."   # odd layout: jcmd on PATH only
    return 0
  fi
  return 1
}

if JDK_HOME="$(resolve_jdk)"; then
  JCMD="$JDK_HOME/bin/jcmd"
  [ -x "$JCMD" ] || JCMD="$(command -v jcmd)"
else
  echo "ERROR: jcmd not found — profiling needs a full JDK (a JRE has no jcmd/jfr)." >&2
  echo "  Fix (no sudo):   mkdir -p /home/z/jdk21 && curl -sSL \\" >&2
  echo "    https://api.adoptium.net/v3/binary/latest/21/ga/linux/x64/jdk/hotspot/normal/eclipse \\" >&2
  echo "    | tar xz --strip-components=1 -C /home/z/jdk21" >&2
  echo "  or point at an existing JDK:  $0 --jdk /path/to/jdk21" >&2
  exit 2
fi
JFR_BIN=""
if [ -x "$JDK_HOME/bin/jfr" ]; then
  JFR_BIN="$JDK_HOME/bin/jfr"
elif p="$(command -v jfr 2>/dev/null)"; then
  JFR_BIN="$p"
elif [ "$NO_TRIAGE" -eq 0 ]; then
  echo "WARN: 'jfr' CLI not found in $JDK_HOME — recording will still be saved, but triage/CSVs are skipped." >&2
fi

# ------------------------------------------------------------------- pid ----
if [ -n "$PID_ARG" ]; then
  [ -d "/proc/$PID_ARG" ] || die 3 "PID $PID_ARG (from --pid/\$SERVER_PID) does not exist"
  SERVER_PID="$PID_ARG"
else
  # 'paper'/'purpur' also match random processes (editors, tail -f logs...),
  # so keep only PIDs whose executable is really java.
  matched="$(pgrep -f 'launcher\.jar|purpur|paper' 2>/dev/null || true)"
  SERVER_PID=""
  n_java=0
  cands=""
  for p in $matched; do
    [ -r "/proc/$p/exe" ] || continue
    exe="$(readlink "/proc/$p/exe" 2>/dev/null || true)"
    case "$exe" in
      */java|*/java\ \(deleted\))
        n_java=$((n_java + 1))
        cands="$cands  PID $p: $(tr '\0' ' ' < "/proc/$p/cmdline" | cut -c1-120)
"
        SERVER_PID="$p"
        ;;
    esac
  done
  if [ "$n_java" -eq 0 ]; then
    echo "ERROR: no running server JVM found (pattern 'launcher.jar|purpur|paper')." >&2
    [ -n "$matched" ] && echo "Non-java processes matched the pattern: $(echo $matched | tr '\n' ' ')" >&2
    echo "  Boot the server first, e.g.:  cd /home/z/server && setsid /home/z/jdk21/bin/java -Xms1G -Xmx2G -jar launcher.jar --nogui &" >&2
    echo "  Or target a specific JVM:     $0 --pid <PID>" >&2
    exit 3
  fi
  if [ "$n_java" -gt 1 ]; then
    echo "ERROR: multiple java processes match — pick one explicitly:" >&2
    printf '%s' "$cands" >&2
    echo "  $0 --pid <PID>" >&2
    exit 3
  fi
fi

"$JCMD" "$SERVER_PID" VM.uptime >/dev/null 2>&1 || \
  die 4 "jcmd cannot attach to PID $SERVER_PID (other user? JVM already exiting? pid-namespace mismatch?)"

# ---------------------------------------------------------------- output ----
mkdir -p -- "$OUTDIR"
OUTDIR="$(cd -- "$OUTDIR" && pwd)"
BASE="profile_$(date +%Y%m%d_%H%M%S)"
ABS_JFR="$OUTDIR/$BASE.jfr"
REC="crussty_profile_$$"
JCMD_LOG="$OUTDIR/$BASE.jcmd.log"
: > "$JCMD_LOG"
CMDLINE="$(tr '\0' ' ' < "/proc/$SERVER_PID/cmdline" 2>/dev/null || true)"

{
  echo "started:      $(date -Is)"
  echo "server pid:   $SERVER_PID"
  echo "server cmd:   $CMDLINE"
  echo "mode:         $MODE"
  echo "duration s:   $DURATION"
  echo "workload:     ${WORKLOAD:-<none>}"
  echo "jdk home:     $JDK_HOME"
} > "$OUTDIR/$BASE.meta.txt"

echo "== Profiling PID $SERVER_PID for ${DURATION}s (mode=$MODE) =="
echo "   cmdline: $CMDLINE"
echo "   jdk:     $JDK_HOME"

# ---------------------------------------------------- workload (optional) ----
WPID=""
WLOG="$OUTDIR/$BASE.workload.log"
if [ -n "$WORKLOAD" ]; then
  : > "$WLOG"
  timeout --signal=TERM --kill-after=5 "$((DURATION + 10))" bash -c "$WORKLOAD" > "$WLOG" 2>&1 &
  WPID=$!
  echo "   workload: $WORKLOAD (pid $WPID, hard timeout $((DURATION + 10))s, log $WLOG)"
fi

# ---------------------------------------------------------------- record ----
finish_workload() {
  [ -n "$WPID" ] || return 0
  kill "$WPID" 2>/dev/null || true          # TERM the timeout wrapper
  pkill -P "$WPID" 2>/dev/null || true      # and its bash -c child
  wait "$WPID" 2>/dev/null || true
}

cleanup() {
  local rc=$?
  finish_workload
  "$JCMD" "$SERVER_PID" JFR.stop name="$REC" >/dev/null 2>&1 || true
  exit $rc
}
trap cleanup EXIT INT TERM

if [ "$MODE" = "jfr" ]; then
  # Duration-bounded: the JVM stops the recording itself and writes the file
  # (jcmd resolves 'filename' against the TARGET's CWD, so always absolute).
  if ! "$JCMD" "$SERVER_PID" JFR.start name="$REC" settings=profile \
        duration="${DURATION}s" filename="$ABS_JFR" >> "$JCMD_LOG" 2>&1; then
    cat "$JCMD_LOG" >&2
    die 4 "JFR.start failed on PID $SERVER_PID (see $JCMD_LOG)"
  fi
  echo "   recording '$REC' started (auto-stops after ${DURATION}s)"
  sleep "$DURATION"
  # Recording stop+dump may take a couple of seconds; poll briefly.
  n=0
  while [ ! -s "$ABS_JFR" ] && [ "$n" -lt 15 ]; do sleep 1; n=$((n + 1)); done
  if [ ! -s "$ABS_JFR" ]; then
    "$JCMD" "$SERVER_PID" JFR.dump name="$REC" filename="$ABS_JFR" >> "$JCMD_LOG" 2>&1 || true
    sleep 1
  fi
else
  # jcmd mode: open-ended recording, we control start/stop explicitly.
  if ! "$JCMD" "$SERVER_PID" JFR.start name="$REC" settings=profile >> "$JCMD_LOG" 2>&1; then
    cat "$JCMD_LOG" >&2
    die 4 "JFR.start failed on PID $SERVER_PID (see $JCMD_LOG)"
  fi
  echo "   recording '$REC' started (will be stopped explicitly)"
  sleep "$DURATION"
  if ! "$JCMD" "$SERVER_PID" JFR.stop name="$REC" filename="$ABS_JFR" >> "$JCMD_LOG" 2>&1; then
    cat "$JCMD_LOG" >&2
    die 4 "JFR.stop failed on PID $SERVER_PID (see $JCMD_LOG)"
  fi
fi
finish_workload
"$JCMD" "$SERVER_PID" JFR.stop name="$REC" >/dev/null 2>&1 || true   # no-op if already stopped
[ -s "$ABS_JFR" ] || die 5 "recording file was not produced: $ABS_JFR (see $JCMD_LOG)"

echo "   saved: $ABS_JFR ($(du -h "$ABS_JFR" | cut -f1))"

# ---------------------------------------------------------------- triage ----
# Raw dumps for flame_diff.py's text mode (can get big — opt-in).
if [ "$KEEP_RAW" -eq 1 ] && [ -n "$JFR_BIN" ]; then
  "$JFR_BIN" print --events jdk.ExecutionSample "$ABS_JFR" \
    > "$OUTDIR/$BASE.execution_samples.txt" 2>/dev/null || \
    echo "WARN: jfr print (raw dump) failed" >&2
fi

if [ "$NO_TRIAGE" -eq 0 ] && [ -n "$JFR_BIN" ]; then
  echo
  echo "== jfr summary =="
  if "$JFR_BIN" summary "$ABS_JFR" > "$OUTDIR/$BASE.summary.txt" 2>&1; then
    cat "$OUTDIR/$BASE.summary.txt"
  else
    echo "WARN: jfr summary failed" >&2
  fi

  # Hot methods: first (leaf) frame of every ExecutionSample = self time.
  # awk emits rows only (header would sort to the bottom of -k2 numeric),
  # then we prepend it above the sorted rows.
  HOT_CSV="$OUTDIR/$BASE.hot_methods.csv"
  "$JFR_BIN" print --events jdk.ExecutionSample "$ABS_JFR" 2>/dev/null | awk '
    /^jdk\.[A-Za-z0-9_]+ *\{/ { ev = $1; instack = 0; next }
    ev == "jdk.ExecutionSample" && /stackTrace *= *\[/ { instack = 1; taken = 0; next }
    instack && /^[[:space:]]*\]/ { instack = 0; next }
    instack && !taken {
      m = $0
      sub(/^[[:space:]]+/, "", m)
      sub(/[[:space:]]+line:.*$/, "", m)     # " line: N [bci: M]"
      sub(/[[:space:]]*\(.*$/, "", m)        # signature + "(Native Method)"
      sub(/\/0x[0-9a-fA-F]+/, "", m)         # hidden-class address suffix
      sub(/\$\$Lambda\+[0-9]+/, "$$Lambda", m)
      if (m != "") { cnt[m]++; total++ }
      taken = 1
    }
    END {
      for (k in cnt) printf "%s,%d\n", k, cnt[k]
      if (total == 0) print "#NO_SAMPLES" | "cat >&2"
    }
  ' | sort -t, -k2,2 -rn > "$HOT_CSV.rows" || echo "WARN: hot-method extraction failed" >&2
  { printf 'method,samples\n'; cat "$HOT_CSV.rows"; } > "$HOT_CSV" && rm -f "$HOT_CSV.rows"

  if grep -q '^method,' "$HOT_CSV" 2>/dev/null && [ "$(wc -l < "$HOT_CSV")" -gt 1 ]; then
    echo
    echo "== TOP HOT METHODS (self samples; full list: $HOT_CSV) =="
    echo "   samples  method"
    sed -n '2,16p' "$HOT_CSV" | awk -F, '{ printf "   %7s  %s\n", $2, $1 }'
  else
    echo
    echo "   (0 execution samples — server idle? raise --duration or pass --workload)"
  fi

  # Top allocation sites by sampled weight (jdk.ObjectAllocationSample).
  ALLOC_CSV="$OUTDIR/$BASE.alloc.csv"
  "$JFR_BIN" print --events jdk.ObjectAllocationSample "$ABS_JFR" 2>/dev/null | awk '
    /^[[:space:]]*objectClass *=/ { cls = $3; have_cls = 1 }
    /^[[:space:]]*weight *=/ {
      v = $(NF - 1) + 0; u = $NF; mult = 1
      if (u == "KB") mult = 1024; else if (u == "MB") mult = 1048576
      else if (u == "GB") mult = 1073741824
      if (have_cls) bytes[cls] += v * mult
    }
    END { for (k in bytes) printf "%s,%d\n", k, bytes[k] }
  ' | sort -t, -k2,2 -rn > "$ALLOC_CSV" || echo "WARN: allocation extraction failed" >&2

  if [ -s "$ALLOC_CSV" ]; then
    echo
    echo "== TOP ALLOCATION SITES (sampled weight; full list: $ALLOC_CSV) =="
    echo "     ~bytes  class"
    sed -n '1,10p' "$ALLOC_CSV" | awk -F, '{ printf "   %9s  %s\n", $2, $1 }'
  fi

  echo
  echo "== NEXT =="
  echo "   baseline diff:  python3 $(dirname "$0")/flame_diff.py <BASELINE> $ABS_JFR"
  echo "                   (accepts .jfr, *.hot_methods.csv or raw jfr-print dumps)"
fi

if [ -n "$WORKLOAD" ] && [ -s "$WLOG" ]; then
  echo
  echo "== workload output (tail) =="
  tail -n 10 "$WLOG"
fi

trap - EXIT INT TERM
echo
echo "done: $ABS_JFR"
