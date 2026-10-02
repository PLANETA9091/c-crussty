#!/usr/bin/env python3
"""TASK-456-C: STRICT-OR needle cmp456_chunkmono into EVERY master-carrier
gate (rust eq/match styles + java equals style) — mirror-drift lesson x452:
prod gates and test helpers move synchronously. Idempotent (skips lines that
already carry cmp456_chunkmono)."""
import re, sys

NEW = "cmp456_chunkmono"
ANCHOR = "cmp450_chunk"  # every master-carrier gate already lists it (diet-fix canon)

def needle_line(line: str):
    if NEW in line:
        return line
    out = line
    # java: X.trim().equals("cmp450_chunk")
    out = re.sub(r'(\w+)\.trim\(\)\.equals\("cmp450_chunk"\)',
                 lambda m: f'{m.group(1)}.trim().equals("cmp450_chunk") || {m.group(1)}.trim().equals("{NEW}")',
                 out)
    # java: X.equals("cmp450_chunk") (non-trim)
    out = re.sub(r'(?<![\w.])((?:\w+\.)?\w+)\.equals\("cmp450_chunk"\)',
                 lambda m: f'{m.group(1)}.equals("cmp450_chunk") || {m.group(1)}.equals("{NEW}")'
                 if ".trim().equals" not in out else out,
                 out, count=0) if ".trim().equals(\"cmp450_chunk\")" not in line else out
    # rust match arm: | Ok("cmp450_chunk")
    out = out.replace('| Ok("cmp450_chunk")', f'| Ok("cmp450_chunk") | Ok("{NEW}")')
    # rust eq: v == "cmp450_chunk"  (any ident)
    out = re.sub(r'\b(\w+)\s*==\s*"cmp450_chunk"',
                 lambda m: f'{m.group(1)} == "cmp450_chunk" || {m.group(1)} == "{NEW}"',
                 out)
    return out

def gate_line(line: str) -> bool:
    if "cmp450_chunk" not in line:
        return False
    if ('.equals("' in line) or ('Ok("' in line) or ('== "' in line):
        return True
    return False

changed = {}
import glob
paths = []
for root in ("src",):
    for p in glob.glob(f"{root}/**/*.rs", recursive=True):
        paths.append(p)
for p in glob.glob("*/net/minecraft/world/entity/*.java") + glob.glob("*/net/minecraft/server/level/*.java") + glob.glob("*/net/minecraft/server/network/*.java") + glob.glob("*/net/minecraft/world/level/chunk/storage/*.java"):
    paths.append(p)
paths = sorted(set(paths))
for p in paths:
    if p.endswith("chunk_sched.rs"):
        continue
    try:
        with open(p) as f:
            lines = f.readlines()
    except Exception:
        continue
    out = []
    n = 0
    for ln in lines:
        if gate_line(ln) and NEW not in ln:
            out.append(needle_line(ln))
            n += 1
        else:
            out.append(ln)
    if n:
        with open(p, "w") as f:
            f.writelines(out)
        changed[p] = n
for p, n in sorted(changed.items()):
    print(f"{n:3d} needles: {p}")
print(f"TOTAL: {sum(changed.values())} needles in {len(changed)} files")
