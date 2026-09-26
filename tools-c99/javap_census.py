#!/usr/bin/env python3
"""javap-census (x466-C99): map method -> getenv/equals callsites for bridge classes.
Separates <clinit> (one-shot gate bake) from per-call methods (hot-path gates)."""
import subprocess, sys, re

JAVAP = "/home/z/tools/jdk-21.0.12.1+1/bin/javap"

def census(path):
    out = subprocess.run([JAVAP, "-p", "-c", path], capture_output=True, text=True).stdout
    methods = collections = {}
    cur = None
    results = {}
    for line in out.splitlines():
        if re.match(r"^  \S.*\(.*\);?\s*$", line) and not line.startswith("    "):
            cur = line.strip().rstrip(";")
            results[cur] = {"getenv": 0, "equals": 0, "flagstr": 0}
            continue
        if cur is None:
            continue
        if "getenv" in line and "Method" in line:
            results[cur]["getenv"] += 1
        if re.search(r"equals:\(", line):
            results[cur]["equals"] += 1
        if "CRUSSTY" in line:
            results[cur]["flagstr"] += 1
    return results

for path in sys.argv[1:]:
    res = census(path)
    clinit = res.get("static {}", {"getenv": 0, "equals": 0, "flagstr": 0})
    print(f"### {path}")
    print(f"  <clinit>: getenv={clinit['getenv']} equals={clinit['equals']} flagstr={clinit['flagstr']}")
    for k, v in res.items():
        if k == "static {}" or not (v["getenv"] or v["equals"] or v["flagstr"]):
            continue
        print(f"  PER-CALL {k}  getenv={v['getenv']} equals={v['equals']} flagstr={v['flagstr']}")
