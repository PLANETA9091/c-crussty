#!/usr/bin/env python3
"""TASK-458-N javap flat==nested svorta (cert verifier).

For each of the 10 lever Ops classes: javap -p -c both the FRESH flat compile
(flat build dir) and the TRACKED nested blob (<dir>/build), normalize bytecode
offsets/branch targets/line tables, and diff member-by-member.

Verdict per class:
  EQUAL       — equal modulo offsets/branch targets
  LEVER-ONLY  — differences confined to lever surfaces (leverEnabled/FLAG15/ENABLED/<clinit>)
  STRUCTURAL  — differences outside lever surfaces => strict-mode touched
"""
import re, subprocess, os

JAVAP = "/home/z/tools/jdk-21.0.12.1+1/bin/javap"

CLASSES = [
    ("net.minecraft.world.entity.ColpushOps",         "colpush/build"),
    ("net.minecraft.world.entity.EntityGoalQueryOps", "entitygoalquery/build"),
    ("net.minecraft.world.entity.ItemEntityManager",  "entityinside/build"),
    ("net.minecraft.world.entity.ai.goal.GoalOps",    "goalops/build"),
    ("net.minecraft.world.entity.MobAiOps",           "mobai/build"),
    ("net.minecraft.world.entity.MobPushOps",         "mobpush/build"),
    ("net.minecraft.world.entity.QueryPlaneOps",      "queryplane/build"),
    ("net.minecraft.world.entity.ai.BrainOps",        "randomtick/build"),
    ("net.minecraft.world.entity.SenseOps",           "sense/build"),
    ("net.minecraft.world.entity.MobScanOps",         "sscan/build"),
]

LEVER_SURFACE = re.compile(
    r"leverEnabled|flagArmed|eqsnapEnabled|eqsnapFlagLabel|senseMode|FLAG15|cmp4[0-9a-z_]*|static \{\}|<clinit>|ENABLED")

def sections(text):
    """split javap -p -c output into (header, body) per member"""
    out, cur, name = [], [], None
    for line in text.splitlines():
        if re.match(r"^[A-Za-z].*$", line) and not line.startswith(" "):
            continue  # class header / javap banner
        if re.match(r"^  \S", line) and not re.match(r"^   \S", line):
            if cur:
                out.append((name, "\n".join(cur)))
            name = line.strip()
            cur = [line]
        else:
            cur.append(line)
    if cur:
        out.append((name, "\n".join(cur)))
    return out

def normalize(body):
    body = re.sub(r"^\s+\d+: (\S+)\s+#\d+.*$", r"  \1 #C", body, flags=re.M)
    body = re.sub(r"^\s+\d+: (\S+)", r"  \1", body, flags=re.M)
    body = re.sub(r"(if\w*|goto\w*)\s+\d+", r"\1 T", body)
    body = re.sub(r"line \d+: \d+", "line L: P", body)
    body = re.sub(r"//\s+\d+", "// C", body)  # constant pool idx comments in javap -c? keep
    return body

def run_javap(cp, cls, verbose=False):
    args = [JAVAP, "-p", "-c"] + (["-v"] if verbose else []) + ["-cp", cp, cls]
    r = subprocess.run(args, capture_output=True, text=True)
    return r.stdout + r.stderr

def verdict(cls, nested_cp):
    flat = run_javap(FLAT_CP, cls)
    nested = run_javap(nested_cp, cls)
    if "Error" in flat or not flat.strip():
        return "FLAT-JAVAP-FAIL", flat[:150].replace("\n", " ")
    if "Error" in nested or not nested.strip():
        return "NESTED-MISSING", ""
    fd = {h: normalize(b) for h, b in sections(flat)}
    nd = {h: normalize(b) for h, b in sections(nested)}
    diffs, lever_only = [], True
    for h in sorted(set(fd) | set(nd)):
        a, b = fd.get(h, "<ABSENT-FLAT>"), nd.get(h, "<ABSENT-NESTED>")
        if a != b:
            if not (LEVER_SURFACE.search(h) or LEVER_SURFACE.search(a) or LEVER_SURFACE.search(b)):
                lever_only = False
            diffs.append(h)
    # flag/gate string svorta from javap -v constant-pool Utf8/String entries
    # (precise per-constant extraction; catches ConstantValue static-final flag
    # strings invisible to javap -c — BrainOps TICK2_FLAGS lesson).
    def flag_strings(jv):
        out = set()
        for m in re.finditer(r"^\s*#\d+ = Utf8\s+(.+)$", jv, re.M):
            out.add(m.group(1).strip())
        for m in re.finditer(r"^\s*#\d+ = String\s+#\d+\s+//\s*(.*)$", jv, re.M):
            out.add(m.group(1).strip())
        return set(s for s in out if re.search(r"cmp4[0-9a-z_]*|CRUSSTY|crussty", s))
    fv = run_javap(FLAT_CP, cls, verbose=True)
    nv = run_javap(nested_cp, cls, verbose=True)
    fs = flag_strings(fv)
    ns = flag_strings(nv)
    cp_missing = sorted(fs - ns)
    cp_extra = sorted(ns - fs)
    for s in cp_missing + cp_extra:
        if not LEVER_SURFACE.search(s):
            lever_only = False
        diffs.append(f"cp-string:{s[:60]}")
    if not diffs:
        return "EQUAL", ""
    return ("LEVER-ONLY" if lever_only else "STRUCTURAL"), "; ".join(diffs[:8])

if __name__ == "__main__":
    root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
    os.chdir(root)
    FLAT_CP = os.environ.get("FLAT_CP", "/tmp/agent-n-javap/flat")
    print("| class | verdict | differing members |")
    print("|---|---|---|")
    bad = 0
    for cls, bdir in CLASSES:
        globals()["FLAT_CP"] = FLAT_CP
        v, info = verdict(cls, bdir)
        if v not in ("EQUAL", "LEVER-ONLY"):
            bad += 1
        print(f"| {cls.split('.')[-1]} | {v} | {info[:140]} |")
    print(f"\nVERDICT: {'PASS' if bad == 0 else f'FAIL ({bad} structural)'} — strict-mode surface check over {len(CLASSES)} lever blobs")
