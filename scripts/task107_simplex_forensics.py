#!/usr/bin/env python3
# TASK-107 SimplexNoise.dot candidate forensics (agent-7625532f)
# Full-depth JFR stack re-extraction + call-context census over RAW_TASK105 (warm) and
# RAW_TASK106 (cold) profiles. Pre-registered gates in CLAIMS.md TASK-107:
#   (A) dot share in DORMANT warm runs >=3% of execution samples
#   (B) caller context must be a dense amortisable loop (>=500 calls/fill phase)
#   (C) effect ceiling: dot_share*(1-1/2) >=3% of burst CPU
import subprocess, os, re, sys, collections

JFR = "/home/z/jdk21/bin/jfr"
BASE = "/home/z/c-crussty/bench/graal_ab"
RUNS = [
    ("warm-dormant-1", f"{BASE}/RAW_TASK105/run_JFRAD1"),
    ("warm-armed-1",   f"{BASE}/RAW_TASK105/run_JFRA2"),
    ("warm-armed-2",   f"{BASE}/RAW_TASK105/run_JFRA3"),
    ("warm-dormant-2", f"{BASE}/RAW_TASK105/run_JFRAD4"),
    ("cold-dormant-1", f"{BASE}/RAW_TASK106/run_CD1"),
    ("cold-armed-1",   f"{BASE}/RAW_TASK106/run_CA2"),
    ("cold-armed-2",   f"{BASE}/RAW_TASK106/run_CA3"),
    ("cold-dormant-2", f"{BASE}/RAW_TASK106/run_CD4"),
]

SIMPLEX = re.compile(r'SimplexNoise')
DOT = re.compile(r'SimplexNoise\.dot\(')
GETVAL = re.compile(r'SimplexNoise\.getValue')

def extract(run_dir):
    jfr_file = os.path.join(run_dir, "burst.jfr")
    out_file = os.path.join(run_dir, "exec_full.txt")
    if not os.path.exists(out_file) or os.path.getsize(out_file) == 0:
        with open(out_file, "w") as fh:
            subprocess.run([JFR, "print", "--events", "jdk.ExecutionSample",
                            "--stack-depth", "96", jfr_file],
                           stdout=fh, stderr=subprocess.DEVNULL, check=True)
    return out_file

def parse(path):
    """Yield list-of-frames per sample."""
    with open(path) as fh:
        cur = None
        for line in fh:
            if line.startswith("jdk.ExecutionSample {"):
                if cur: yield cur
                cur = []
            elif cur is not None:
                m = re.match(r'^    ([a-zA-Z][^\s(]*\([^)]*\))?(\s*line: \d+)?\s*$', line.rstrip())
                if line.startswith("    ") and ("line: " in line or "(" in line):
                    f = line.strip().rsplit(" line:", 1)[0].strip()
                    if f and not f.startswith("..."):
                        cur.append(f)
                elif cur and line.startswith("  }"):
                    yield cur
                    cur = None
        if cur: yield cur

summary = []
for label, run_dir in RUNS:
    p = extract(run_dir)
    total = 0
    dot_leaf = 0
    simplex_any = 0
    gv_leaf = 0
    caller_ctx = collections.Counter()
    leaf_top = collections.Counter()
    for frames in parse(p):
        if not frames: continue
        total += 1
        leaf_top[frames[0].split('(')[0]] += 1
        if DOT.search(frames[0]):
            dot_leaf += 1
            ctx = " <- ".join(f.split('(')[0].split('.')[-1] for f in frames[1:5])
            caller_ctx[ctx] += 1
        if GETVAL.search(frames[0]):
            gv_leaf += 1
        if any(SIMPLEX.search(f) for f in frames):
            simplex_any += 1
    pct = lambda n: (100.0*n/total) if total else 0.0
    summary.append((label, total, pct(dot_leaf), pct(simplex_any), pct(gv_leaf)))
    print(f"== {label}: total={total} dot_leaf={pct(dot_leaf):.2f}% simplex_any={pct(simplex_any):.2f}% getValue_leaf={pct(gv_leaf):.2f}%")
    if caller_ctx:
        print("   top dot-callers (frames below leaf):")
        for k, v in caller_ctx.most_common(4):
            print(f"     {100.0*v/total if total else 0:5.2f}%  {k}")
    print("   top-6 leaves: " + " | ".join(f"{k}:{v}" for k, v in leaf_top.most_common(6)))

print("\n== JVM identity check ==")
for label, run_dir in RUNS:
    boot = os.path.join(run_dir, "boot.log")
    if os.path.exists(boot):
        vm = ""
        with open(boot) as fh:
            for line in fh:
                if "VM" in line and ("OpenJDK" in line or "Graal" in line or "Temurin" in line or "version" in line.lower()):
                    vm = line.strip()[:120]; break
        print(f"{label}: {vm[:120]}")
