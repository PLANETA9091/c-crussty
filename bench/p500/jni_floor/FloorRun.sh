#!/usr/bin/env bash
# FloorRun.sh — JNI-floor micro-bench runner (task B11, bench/p500/jni_floor/)
#
# Compiles FloorBench + FloorGroup (+ the six P500 stub sources they call) and
# runs the floor lanes on a real JVM. Mirrors run_p500.sh mechanics:
#   - same natives trio System.load'ed via -Dfloor.libs (absolute paths),
#   - same JVM sizing (-Xms256m -Xmx1g -XX:+UseG1GC),
#   - TSV on stdout (floor_report.tsv), human log on stderr (floor_run.log).
#
# JDK resolution order (task spec):
#   1. FLOOR_JAVA_HOME env override (for sandboxes where the spec dir is absent)
#   2. /home/z/jdk21 when marker /tmp/jdk21.ready exists (project convention)
#   3. /home/z/jdk21 if present even without the marker
#   4. $JAVA_HOME, then system javac
#   5. none -> COMPILE-CHECK NOTE + exact remediation, exit 2
#
# Outputs (this dir):
#   floor_report.tsv   RESULT/SUMMARY lanes (grep-able)
#   floor_run.log      stderr incl. per-lane progress
#   floor_flags.txt    -XX:+PrintFlagsFinal grep of JNI/JIT-relevant flags
set -u
cd "$(dirname "$0")"
REPO_ROOT="$(cd ../../.. && pwd)"

# ---------------- 1) JDK ----------------
if [ -n "${FLOOR_JAVA_HOME:-}" ] && [ -x "$FLOOR_JAVA_HOME/bin/javac" ]; then
  JAVAC="$FLOOR_JAVA_HOME/bin/javac"; JAVA="$FLOOR_JAVA_HOME/bin/java"
  echo "jdk: FLOOR_JAVA_HOME override -> $FLOOR_JAVA_HOME"
elif [ -f /tmp/jdk21.ready ] && [ -x /home/z/jdk21/bin/javac ]; then
  JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
  echo "jdk: /home/z/jdk21 (marker /tmp/jdk21.ready present)"
elif [ -x /home/z/jdk21/bin/javac ]; then
  JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
  echo "jdk: /home/z/jdk21 (marker /tmp/jdk21.ready ABSENT)"
elif [ -x "${JAVA_HOME:-/nonexistent}/bin/javac" ]; then
  JAVAC="$JAVA_HOME/bin/javac"; JAVA="$JAVA_HOME/bin/java"
  echo "jdk: JAVA_HOME -> $JAVA_HOME"
elif command -v javac >/dev/null 2>&1; then
  JAVAC="$(command -v javac)"; JAVA="$(command -v java)"
  echo "jdk: system"
else
  cat <<'EOF'
COMPILE-CHECK NOTE (FloorRun.sh): no javac found — cannot compile FloorBench.
  Expected JDK 21 at /home/z/jdk21 (marker /tmp/jdk21.ready) — not present.
  System java is a JRE only (javac missing).
  Remediation (worklog SESSION 002 recipe):
    curl -L -o /tmp/jdk21.tar.gz \
      "https://api.adoptium.net/v3/binary/latest/21/ga/linux/x64/jdk/hotspot/normal/eclipse"
    mkdir -p /home/z/jdk21 && tar -xzf /tmp/jdk21.tar.gz -C /home/z/jdk21 --strip-components=1
    touch /tmp/jdk21.ready
  Or rerun immediately with an existing JDK:
    FLOOR_JAVA_HOME=/path/to/jdk21 ./FloorRun.sh
EOF
  exit 2
fi
echo "using: $JAVAC ($("$JAVAC" -version 2>&1))"

# ---------------- 2) natives (same trio as bench/p500/run_p500.sh) --------
MAIN_SO="$REPO_ROOT/native/libpaper_native_jni.so"
CHUNK_SO="$REPO_ROOT/native/libpaper_chunk_encode_jni.so"
CHUNK_SO2="$REPO_ROOT/native/libpaper_native_chunk_encode_jni.so"
LIBS=""
[ -f "$MAIN_SO" ]  && LIBS="$MAIN_SO"
[ -f "$CHUNK_SO" ] && LIBS="${LIBS:+$LIBS:}$CHUNK_SO"
[ -f "$CHUNK_SO2" ] && LIBS="${LIBS:+$LIBS:}$CHUNK_SO2"
if [ -z "$LIBS" ]; then
  echo "WARN: no native libs found under $REPO_ROOT/native/ — native lanes will report NO-NATIVE"
else
  echo "natives: $LIBS"
fi

# ---------------- 3) compile (self-contained ./classes) -------------------
STUBS=""
for s in PaperNativePluginNameLog PaperNativeRangeChoice \
         PaperNativeSpigotLoadOrderDependency PaperNativeStaticCacheGet \
         PaperNativePluginStartupRollup PaperNativeClimateParameterDistance; do
  [ -f "../java/$s.java" ] || { echo "FATAL: ../java/$s.java missing (run bench/p500/gen_p500_bench.py first)"; exit 1; }
  STUBS="$STUBS ../java/$s.java"
done
[ -f "../java/p500/Group.java" ] || { echo "FATAL: ../java/p500/Group.java missing"; exit 1; }

mkdir -p classes
"$JAVAC" -d classes $STUBS ../java/p500/Group.java FloorBench.java FloorGroup.java || exit 1
echo "compile OK -> classes/"

# ---------------- 4) JVM flags snapshot (JNI/JIT-relevant) ----------------
"$JAVA" -XX:+PrintFlagsFinal -version 2>&1 \
  | grep -Ei 'jni|inlin|tiered|compilethreshold|backgroundcomp|usecompiler|loopunroll|frequency|UseG1GC|UseCompressedOops|StackGuardPages|UseNotificationThread|EnableJVMCI|UseDynamicNumberOfGCThreads|DontCompileHugeMethods' \
  > floor_flags.txt || true
echo "flags snapshot: floor_flags.txt ($(wc -l < floor_flags.txt) lines)"
"$JAVA" -version 2>&1 | sed 's/^/vm> /'

# ---------------- 5) run ---------------------------------------------------
COUNT="${FLOOR_COUNT:-10000000}"
echo "run: FloorBench (floor.count=$COUNT) -> floor_report.tsv"
"$JAVA" -Xms256m -Xmx1g -XX:+UseG1GC \
  -Dfloor.libs="$LIBS" \
  -Dfloor.count="$COUNT" \
  -cp classes FloorBench > floor_report.tsv 2> floor_run.log
rc=$?
echo "exit=$rc"

echo "---- floor_report.tsv ----"
cat floor_report.tsv
echo "---- stderr tail (floor_run.log) ----"
tail -20 floor_run.log | sed 's/^/  /'

# ---------------- 6) if natives did not bind: deployment guidance ---------
if grep -q 'NO-NATIVE' floor_report.tsv; then
  cat <<'EOF'

DEPLOYMENT NOTE: the Crussty CE natives did not bind in this JVM
(see NO-NATIVE rows above). Two proven run paths:
  1) STANDALONE (this is how bench/p500/run_p500.sh drives the same .so files —
     results/p500_raw.tsv was produced this way, 129 kernels, 0 crashes):
       ./FloorRun.sh            # this script
  2) LIVE SERVER JVM (bootstrap-loader classes; natives registered by the
     c-crussty injector): inject FloorBench/FloorGroup classes the same way
     bench classes would be delivered and invoke FloorBench.main() /
     FloorGroup.main() inside the server JVM, OR drive FloorGroup through
     the p500.Group contract (it is a drop-in p500.Group; p500.Bench
     hardcodes Class.forName("G"+gid) so it needs a tiny driver loop —
     FloorGroup.main IS that driver, same methodology).
     Exact command once the server is up:
       cd /home/z/ccrussty/c-crussty/bench/p500 && ./run_p500.sh   # harness env sanity
       cd jni_floor && ./FloorRun.sh                               # floor lanes
EOF
fi
echo "done: floor_report.tsv, floor_run.log, floor_flags.txt"
exit $rc
