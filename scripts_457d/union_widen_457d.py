#!/usr/bin/env python3
"""union_widen_457d.py — TASK-457-D (C1 SIMD-noise carrier, law 8 axis).

STRICT-OR union widen: add the round carrier id "cmp457_noisesimd" to EVERY
gate that carries the certified union id "cmp450_chunk" (the x455 chunk-comp
cert carrier). Mechanic per fe7e7120/c25782b4 (spawn rebaze) — union ids are
appended, historical ids untouched, empty/foreign flag = vanilla bit-in-byte.

Rules:
  R1 rust gate:    `<v> == "cmp450_chunk"`       -> `... || <v> == "cmp457_noisesimd"`
  R2 rust marker:  `Ok("cmp450_chunk") => BODY`  -> insert `Ok("cmp457_noisesimd") => BODY'`
                   (BODY' = BODY with the id string swapped; single-line arms only)
  R3 java gate:    `X.equals("cmp450_chunk")`    -> `... || X.equals("cmp457_noisesimd")`

Idempotent per occurrence (skips lines already carrying cmp457_noisesimd).
"""
import re, pathlib

ROOT = pathlib.Path("/home/z/rounds/ROUND-457/agent-d")
OLD = "cmp450_chunk"
NEW = "cmp457_noisesimd"

r1 = re.compile(r'(\w+) == "' + re.escape(OLD) + '"')
r2 = re.compile(r'Ok\("' + re.escape(OLD) + r'"\) => ([^\n]+)$')
r3 = re.compile(r'([\w.()]+)\.equals\("' + re.escape(OLD) + r'"\)')

report = []

def widen_file(p, kind):
    txt = p.read_text(encoding="utf-8")
    orig = txt
    n1 = n2 = n3 = 0
    if kind == "rust":
        def sub1(m):
            nonlocal n1
            n1 += 1
            return f'{m.group(1)} == "{OLD}" || {m.group(1)} == "{NEW}"'
        txt = r1.sub(sub1, txt)
        out_lines = []
        for line in txt.split("\n"):
            m = r2.search(line)
            if m and NEW not in line:
                body = m.group(1).replace(OLD, NEW)
                out_lines.append(line)
                indent = line[: len(line) - len(line.lstrip())]
                out_lines.append(f'{indent}Ok("{NEW}") => {body}')
                n2 += 1
            else:
                out_lines.append(line)
        txt = "\n".join(out_lines)
    else:
        def sub3(m):
            nonlocal n3
            n3 += 1
            return f'{m.group(1)}.equals("{OLD}") || {m.group(1)}.equals("{NEW}")'
        txt = r3.sub(sub3, txt)
    if txt != orig:
        p.write_text(txt, encoding="utf-8")
        report.append(f"{p.relative_to(ROOT)}: R1={n1} R2={n2} R3={n3}")
        return True
    return False

def main():
    rust = sorted((ROOT / "src").glob("*.rs"))
    java_files = [p for p in ROOT.rglob("*.java")
                  if "research" not in p.parts and "/build/" not in str(p)]
    changed = []
    for p in rust:
        if OLD in p.read_text(encoding="utf-8"):
            if widen_file(p, "rust"):
                changed.append(str(p.relative_to(ROOT)))
    for p in java_files:
        if OLD in p.read_text(encoding="utf-8"):
            if widen_file(p, "java"):
                changed.append(str(p.relative_to(ROOT)))
    print(f"CHANGED {len(changed)} files:")
    for c in changed:
        print("  ", c)
    for r in report:
        print(" ", r)

if __name__ == "__main__":
    main()
