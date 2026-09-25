#!/usr/bin/env python3
"""wire_paldelta_p31.py — TASK-460-03 climb wiring (round-460-pdclimb-1).

Extends every STRICT-OR paldelta lever site (rust src/ + java Ops sources)
with the climb lever `cmp457_paldelta_p31` (family alias: paldelta carrier
+ INSIDE-BATCH P31). Mechanical, per-occurrence, operator-preserving:

  java : X.equals("cmp457_paldelta")            -> + " || X.equals("cmp457_paldelta_p31")"
  rust : v == "cmp457_paldelta"  /  t == ...    -> + " || V == "cmp457_paldelta_p31""
         | Some("cmp457_paldelta") / | Ok(...)  -> + " | Some/Ok("cmp457_paldelta_p31")"

Idempotent: skips files already containing the climb lever.
"""
import re, sys, pathlib

CLIMB = "cmp457_paldelta_p31"
BASE = "cmp457_paldelta"
ROOT = pathlib.Path(__file__).resolve().parent.parent

def wire_text(text):
    n = 0
    # java:  <recv>.equals("cmp457_paldelta")   (recv = f.trim() / t / lever ...)
    def java_repl(m):
        nonlocal n
        n += 1
        recv = m.group(1)
        return f'{recv}.equals("{BASE}") || {recv}.equals("{CLIMB}")'
    text = re.sub(r'(\b[A-Za-z_$][\w$]*(?:\(\))?(?:\.\w+\(\))*)\.equals\("cmp457_paldelta"\)', java_repl, text)

    # rust:  <var> == "cmp457_paldelta"   (var = v / t / lever / flag ...)
    def rust_eq_repl(m):
        nonlocal n
        n += 1
        var = m.group(1)
        return f'{var} == "{BASE}" || {var} == "{CLIMB}"'
    text = re.sub(r'\b([A-Za-z_][\w$]*)\s*==\s*"cmp457_paldelta"', rust_eq_repl, text)

    # rust match arms: | Some("cmp457_paldelta") / | Ok("cmp457_paldelta")
    def rust_arm_repl(m):
        nonlocal n
        n += 1
        return f'{m.group(1)}("{BASE}") | {m.group(1)}("{CLIMB}")'
    text = re.sub(r'(\|\s+(?:Some|Ok)\()"cmp457_paldelta"', rust_arm_repl, text)
    return text, n

def main():
    total_files, total_sites = 0, 0
    rust_files = sorted((ROOT / "src").glob("*.rs"))
    java_files = [p for p in ROOT.rglob("*.java") if "/build/" not in str(p) and "/quantum/build/" not in str(p)]
    for p in rust_files + java_files:
        if "wire_paldelta_p31" in p.name:
            continue
        text = p.read_text()
        if CLIMB in text:
            continue
        if BASE not in text:
            continue
        new, n = wire_text(text)
        if n:
            p.write_text(new)
            total_files += 1
            total_sites += n
            print(f"  {p.relative_to(ROOT)}: {n} site(s)")
    print(f"WIRED: {total_sites} sites across {total_files} files -> +'{CLIMB}'")

if __name__ == "__main__":
    main()
