#!/usr/bin/env python3
"""TASK-452-C step-1b: structural fixes after union resolution.

Patterns:
A) Java stacked gate chains: head line ends `));`, next line is an orphaned
   `|| ... ));` continuation (their side of the same expression) -> merge into
   one chain, dedupe cmp452_mega.
B) Rust single-statement chains: head line ends `;`, next line orphaned
   `|| flag == "...";` -> merge, dedupe mega.
C) Rust double-if (mobs_manager): consecutive `if f == ... {` / `} else if`
   lines from both sides -> merge conditions, dedupe mega.
"""
import re, glob

MEGA = "cmp452_mega"

def dedupe_mega(line):
    parts = line.split(f'"{MEGA}"')
    return ('"' + MEGA + '"').join(parts)

def fix_java(path):
    with open(path) as f:
        lines = f.readlines()
    out, merged = [], 0
    i = 0
    while i < len(lines):
        cur = lines[i].rstrip("\n")
        nxt = lines[i + 1].rstrip("\n") if i + 1 < len(lines) else None
        if (nxt is not None
                and re.search(r'\)\s*;\s*$', cur) and "||" in cur
                and re.match(r'\s*\|\|', nxt) and re.search(r'\)\s*;\s*$', nxt)):
            # merge: drop the closing `);` of cur, append nxt's continuation
            body_cur = re.sub(r'\)\s*;\s*$', '', cur)
            body_nxt = nxt.strip()
            new = dedupe_mega(body_cur + " " + body_nxt)
            out.append(new + "\n")
            merged += 1
            i += 2
            continue
        out.append(lines[i])
        i += 1
    if merged:
        with open(path, "w") as f:
            f.writelines(out)
    print(f"java merges ({merged}): {path}")
    return merged

def fix_rust_stmt(path):
    with open(path) as f:
        lines = f.readlines()
    out, merged = [], 0
    i = 0
    while i < len(lines):
        cur = lines[i].rstrip("\n")
        nxt = lines[i + 1].rstrip("\n") if i + 1 < len(lines) else None
        if (nxt is not None
                and cur.rstrip().endswith(";") and "||" in cur and "==" in cur
                and re.match(r'\s*\|\|', nxt) and nxt.rstrip().endswith(";")):
            body_cur = cur.rstrip()[:-1]  # drop ;
            new = dedupe_mega(body_cur + " " + nxt.strip())
            out.append(new + "\n")
            merged += 1
            i += 2
            continue
        out.append(lines[i])
        i += 1
    if merged:
        with open(path, "w") as f:
            f.writelines(out)
    print(f"rust stmt merges ({merged}): {path}")
    return merged

def fix_rust_double_if(path):
    with open(path) as f:
        lines = f.readlines()
    out, merged = [], 0
    i = 0
    while i < len(lines):
        cur = lines[i].rstrip("\n")
        nxt = lines[i + 1].rstrip("\n") if i + 1 < len(lines) else None
        if nxt is not None and re.match(r'\s*(\} else )?if f == .* \{', cur) \
                and re.match(r'\s*(\} else )?if f == .* \{', nxt) \
                and 'cmp45' in cur and 'cmp45' in nxt:
            indent = re.match(r"\s*", cur).group(0)
            prefix = "} else " if cur.strip().startswith("} else ") else ""
            cond_c = cur.strip()
            cond_n = nxt.strip()
            # strip prefix and trailing `{`+comment
            m_c = re.match(r'(?:\} else )?if (f == .*)\{(.*)$', cond_c)
            m_n = re.match(r'(?:\} else )?if (f == .*)\{(.*)$', cond_n)
            assert m_c and m_n, (path, cur, nxt)
            cond = m_c.group(1).rstrip()  # keep head comment out of condition
            head_comment = m_c.group(2).strip()
            tail = m_n.group(1).rstrip()
            if not tail.endswith(MEGA) and f'== "{MEGA}"' in tail:
                tail = tail.replace(f' || f == "{MEGA}"', "")
            new = (indent + prefix + "if " + cond + " " + tail + " {"
                   + ((" " + head_comment) if head_comment else ""))
            out.append(new + "\n")
            merged += 1
            i += 2
            continue
        out.append(lines[i])
        i += 1
    if merged:
        with open(path, "w") as f:
            f.writelines(out)
    print(f"rust if merges ({merged}): {path}")
    return merged

total = 0
for p in ["mobai/net/minecraft/world/entity/MobAiOps.java",
          "sscan/net/minecraft/world/entity/MobScanOps.java",
          "entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java",
          "goalops/net/minecraft/world/entity/ai/goal/GoalOps.java",
          "queryplane/net/minecraft/world/entity/QueryPlaneOps.java",
          "mobpush/net/minecraft/world/entity/MobPushOps.java",
          "entityinside/net/minecraft/world/entity/ItemEntityManager.java",
          "colpush/net/minecraft/world/entity/ColpushOps.java"]:
    total += fix_java(p)
for p in sorted(glob.glob("src/*.rs")):
    total += fix_rust_stmt(p)
total += fix_rust_double_if("src/mobs_manager.rs")
print(f"TOTAL structural merges: {total}")
