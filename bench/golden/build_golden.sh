#!/usr/bin/env bash
# bench/golden/build_golden.sh — compile + package the GoldenDumper plugin
# (NCF P0.3 golden harness). Plain JDK toolchain only; no external deps.
#
# Classpath needs TWO jars (verified 2026-10-05 against /home/z/server):
#   - the MOJANG-MAPPED server jar  (net.minecraft.*, org.bukkit.craftbukkit.*)
#     NOTE: $SERVER_DIR/versions/purpur-1.21.10.jar is the paperclip launcher
#     (no classes inside); the real mojang-mapped image lands at
#     $SERVER_DIR/versions/<mcver>/purpur-<mcver>.jar after the first boot.
#   - the Bukkit/Purpur API jar     (org.bukkit.* incl. JavaPlugin — the server
#     jar does NOT carry org.bukkit.plugin.java.JavaPlugin)
#
# Compiler resolution order (this sandbox ships a JRE-only java-21):
#   1. javac on PATH, $JAVA_HOME/bin/javac, /home/z/jdk21/bin/javac
#      -> javac --release 21
#   2. tools/ecj.jar (Eclipse compiler, repo precedent: bridges/paletted,
#      scripts/build_noise.sh) -> java -jar tools/ecj.jar -source 21 -target 21
#   Jar packaging: jar(1) if a JDK provides it, else python3 zipfile (stdlib).
#
# NEVER writes to $SERVER_DIR (read-only introspection); boots nothing.
# Usage: run from its own dir:  ./build_golden.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"     # bench/golden
REPO="$(cd "$ROOT/../.." && pwd)"         # c-crussty
SERVER_DIR="${SERVER_DIR:-/home/z/server}"
BUILD="$ROOT/build"
OUT_JAR="$ROOT/GoldenDumper.jar"

log() { printf '[build_golden] %s\n' "$*" >&2; }
die() { log "FATAL: $*"; exit 1; }

cd "$ROOT"

# ---------------------------------------------------------------------------
# 1. Locate the mojang-mapped server jar (read-only zip introspection)
# ---------------------------------------------------------------------------
find_mojang_jar() { # $@ = candidate dirs; prints first jar containing CraftWorld
    python3 - "$@" <<'PY'
import sys, zipfile, os
for d in sys.argv[1:]:
    if not os.path.isdir(d):
        continue
    cands = []
    for base, _dirs, files in os.walk(d):
        if base.count(os.sep) - d.count(os.sep) > 2:   # keep it shallow
            continue
        for f in files:
            if f.endswith('.jar'):
                cands.append(os.path.join(base, f))
    for p in sorted(cands):
        try:
            if 'org/bukkit/craftbukkit/CraftWorld.class' in zipfile.ZipFile(p).namelist():
                print(p); sys.exit(0)
        except Exception:
            continue
sys.exit(3)
PY
}

SERVER_JAR=""
if python3 -c "import zipfile,sys; sys.exit(0 if 'org/bukkit/craftbukkit/CraftWorld.class' in zipfile.ZipFile(sys.argv[1]).namelist() else 1)" \
        "$SERVER_DIR/versions/purpur-1.21.10.jar" 2>/dev/null; then
    SERVER_JAR="$SERVER_DIR/versions/purpur-1.21.10.jar"
else
    SERVER_JAR="$(find_mojang_jar "$SERVER_DIR/versions" || true)"
    if [ -z "$SERVER_JAR" ]; then
        SERVER_JAR="$(find_mojang_jar "$SERVER_DIR/cache" || true)"
    fi
fi
if [ -z "$SERVER_JAR" ]; then
    die "no mojang-mapped server jar with org.bukkit.craftbukkit classes found under
  $SERVER_DIR/versions or $SERVER_DIR/cache
Boot the server once first (paperclip materializes the mapped jar into
versions/<mcver>/ on first run), or point SERVER_DIR at the right root."
fi
log "server jar : $SERVER_JAR"

# ---------------------------------------------------------------------------
# 2. Locate the Bukkit/Purpur API jar (org.bukkit.* / JavaPlugin)
# ---------------------------------------------------------------------------
API_JAR="$(ls "$SERVER_DIR"/libraries/org/purpurmc/purpur/purpur-api/*/purpur-api-*.jar 2>/dev/null | sort | tail -1 || true)"
[ -n "$API_JAR" ] || API_JAR="$(ls "$SERVER_DIR"/libraries/io/papermc/paper/paper-api/*/paper-api-*.jar 2>/dev/null | sort | tail -1 || true)"
if [ -z "$API_JAR" ]; then
    die "no purpur-api/paper-api jar under $SERVER_DIR/libraries — boot the server once first so the launcher materializes it"
fi
log "api jar    : $API_JAR"

# ---------------------------------------------------------------------------
# 2b. Transitive API deps on the compile classpath.
# The purpur-api jar's classes carry transitive supertypes/interfaces from
# libraries (e.g. org.bukkit.command.CommandSender extends
# net.kyori.adventure.audience.Audience) — ECJ/javac must resolve them.
# Include-list of library families (kept explicit, no huge classpath).
# ---------------------------------------------------------------------------
API_DEPS="$(python3 - "$SERVER_DIR" <<'PY'
import glob, os, sys
root = sys.argv[1]
groups = [
    'net/kyori/**/*.jar',                       # adventure + examination + key
    'com/google/code/gson/**/*.jar',            # gson
    'com/google/guava/guava/**/*.jar',          # guava (failureaccess separate)
    'com/google/guava/failureaccess/**/*.jar',
    'com/google/code/findbugs/**/*.jar',        # jsr305
    'org/yaml/snakeyaml/**/*.jar',              # bukkit config
    'org/slf4j/slf4j-api/**/*.jar',
    'org/joml/**/*.jar',
    'it/unimi/dsi/fastutil/**/*.jar',
    'org/checkerframework/**/*.jar',            # checker-qual
    'com/google/errorprone/**/*.jar',
    'org/jspecify/**/*.jar',
    'net/md-5/**/*.jar',                        # bungeecord-chat (sendMessage overloads)
    'com/mojang/datafixerupper/**/*.jar',       # session-4: Pair/Either (climate_points reflection path)
]
jars = []
for g in groups:
    jars.extend(sorted(glob.glob(os.path.join(root, 'libraries', g), recursive=True)))
print(':'.join(jars))
PY
)"
CP="$SERVER_JAR:$API_JAR${API_DEPS:+:$API_DEPS}"

# ---------------------------------------------------------------------------
# 3. Compiler
# ---------------------------------------------------------------------------
JAVAC=""
if command -v javac >/dev/null 2>&1; then
    JAVAC="$(command -v javac)"
elif [ -n "${JAVA_HOME:-}" ] && [ -x "$JAVA_HOME/bin/javac" ]; then
    JAVAC="$JAVA_HOME/bin/javac"
elif [ -x /home/z/jdk21/bin/javac ]; then
    JAVAC=/home/z/jdk21/bin/javac
fi

rm -rf "$BUILD"
mkdir -p "$BUILD/classes"

SOURCES="$(find "$ROOT/src" -name '*.java')"
if [ -n "$JAVAC" ]; then
    log "javac      : $JAVAC (--release 21)"
    "$JAVAC" --release 21 -encoding UTF-8 -cp "$CP" -d "$BUILD/classes" $SOURCES
elif [ -f "$REPO/tools/ecj.jar" ]; then
    log "javac      : not found — falling back to ECJ ($REPO/tools/ecj.jar)"
    java -jar "$REPO/tools/ecj.jar" -source 21 -target 21 -encoding UTF-8 \
        -cp "$CP" -d "$BUILD/classes" $SOURCES
else
    die "no compiler: install a JDK (javac) or provide $REPO/tools/ecj.jar (repo precedent)"
fi

cp "$ROOT/src/plugin.yml" "$BUILD/classes/plugin.yml"

# ---------------------------------------------------------------------------
# 4. Package (jar tool if present, else python3 zipfile)
# ---------------------------------------------------------------------------
rm -f "$OUT_JAR"
if command -v jar >/dev/null 2>&1; then
    # paperweight-mappings-namespace: mojang — tells Paper's plugin remapper
    # this jar is compiled against MOJANG-mapped internals; without it Paper
    # assumes spigot mappings and rewrites NMS refs (broke getWorldData on
    # the first corpus run, NoSuchMethodError at runtime).
    printf 'paperweight-mappings-namespace: mojang\n' > "$BUILD/MANIFEST.MF"
    jar cfm "$OUT_JAR" "$BUILD/MANIFEST.MF" -C "$BUILD/classes" .
elif [ -n "${JAVA_HOME:-}" ] && [ -x "$JAVA_HOME/bin/jar" ]; then
    "$JAVA_HOME/bin/jar" cf "$OUT_JAR" -C "$BUILD/classes" .
else
    python3 - "$BUILD/classes" "$OUT_JAR" <<'PY'
import os, sys, zipfile
src, dst = sys.argv[1], sys.argv[2]
with zipfile.ZipFile(dst, 'w', zipfile.ZIP_DEFLATED) as z:
    if os.path.exists(os.path.join(src, 'plugin.yml')):
        z.write(os.path.join(src, 'plugin.yml'), 'plugin.yml')
    for base, _dirs, files in os.walk(src):
        for f in sorted(files):
            p = os.path.join(base, f)
            arc = os.path.relpath(p, src)
            if arc == 'plugin.yml':
                continue
            z.write(p, arc)
print('wrote', dst)
PY
fi

log "plugin jar : $OUT_JAR ($(du -h "$OUT_JAR" | cut -f1))"
log "listing head:"
python3 -m zipfile -l "$OUT_JAR" | head -12 || true
log "OK — copy to a corpus server's plugins/ dir (purity law: vanilla boot only, NO CRUSSTY agent)"
