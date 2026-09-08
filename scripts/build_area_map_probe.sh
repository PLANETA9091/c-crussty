#!/usr/bin/env bash
# TASK-68: compile the call-level probe driver (major 52) against the REAL
# kernel fixture class, embed-ready for src/area_map.rs include_bytes!.
#
# Unlike build_area_map.sh (which compiles against RuntimeStubs.java and
# deletes the stub classes), the probe must LINK against the real kernel
# shape (public update(III)Z / add(III)Z / getters / protected callbacks),
# so the classpath is the real major-65 fixture class. javac on JDK 21 reads
# major-65 classpath inputs at any --release; only the EMITTED bytecode is
# pinned (52). Output: area-map/build-probe/dev/crussty/areamapprobe/*.class
# (committed to git, like area-map/build and build-budget).
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${JAVAC:-}"
[ -z "$JAVAC" ] && { if command -v javac >/dev/null 2>&1; then JAVAC=javac; elif [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; fi; }
RELEASE="${AREA_MAP_PROBE_RELEASE:-8}"
OUT="area-map/build-probe"
SRC="area-map/probe-ca/dev/crussty/areamapprobe/AreaMapProbe.java"
FIXTURE=tests/fixtures/SingleUserAreaMap.class
MISC_PKG=ca/spottedleaf/moonrise/common/misc

command -v "$JAVAC" >/dev/null || { echo "javac not found (set JAVAC=)" >&2; exit 2; }
[ -f "$FIXTURE" ] || { echo "kernel fixture missing: $FIXTURE" >&2; exit 2; }

rm -rf "$OUT"
mkdir -p "$OUT"

# The fixture class must sit at its PACKAGE path on the classpath root
# (same trick as tests/area_map_smoke/run.sh fixture_ca) so the probe
# type-checks against the REAL kernel shape (update(III)Z / add(III)Z /
# getters / protected abstract callbacks).
FIXTURE_CP="$OUT/fixture_cp"
mkdir -p "$FIXTURE_CP/$MISC_PKG"
cp "$FIXTURE" "$FIXTURE_CP/$MISC_PKG/"

"$JAVAC" --release "$RELEASE" -nowarn -d "$OUT" -cp "$FIXTURE_CP" "$SRC"
rm -rf "$FIXTURE_CP"

# Guards: emitted major must be <= 65 (release 8 => 52) and both classfiles
# must exist (nested RecMap compiles to its own file).
"$JAVAC" --version
python3 - "$OUT/dev/crussty/areamapprobe/AreaMapProbe.class" "$OUT/dev/crussty/areamapprobe/AreaMapProbe\$RecMap.class" <<'PY'
import sys
for p in sys.argv[1:]:
    b = open(p, 'rb').read()
    major = int.from_bytes(b[6:8], 'big')
    assert major <= 65, f"{p}: major {major} > 65"
    print(f"{p}: {len(b)} B, major {major}")
PY

echo "probe classes built OK (release $RELEASE)"
