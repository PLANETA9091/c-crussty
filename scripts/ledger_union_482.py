#!/usr/bin/env python3
"""ledger_union_482.py — трёхсторонний union LAB_LEDGER.md (защита от сметённых секций).

origin 4f024d2f (16 секций ×482) + worktree (32 новых) + восстановление 9 swept
(C01,C02,C03,C08,C11,C17,C18,C19,C20 из локальных коммитов f86b240c/fa421f86).
"""
import re, subprocess, os

REPO = "/home/z/c-crussty"
LED = f"{REPO}/docs/LAB_LEDGER.md"
SWEEP_IDS = ["C01", "C02", "C03", "C08", "C11", "C17", "C18", "C19", "C20"]

def sh(cmd):
    return subprocess.run(cmd, shell=True, capture_output=True, text=True, cwd=REPO).stdout

def sections(text):
    """Разбить на (заголовок, тело) — заголовки ## уровня."""
    parts = re.split(r"(?m)^(## .+)$", text)
    head, secs = parts[0], []
    for i in range(1, len(parts), 2):
        secs.append((parts[i].strip(), parts[i+1]))
    return head, secs

origin = sh("git show 4f024d2f:docs/LAB_LEDGER.md")
f86 = sh("git show f86b240c:docs/LAB_LEDGER.md")
wt = open(LED, errors="replace").read()

h_o, s_o = sections(origin)
h_f, s_f = sections(f86)
h_w, s_w = sections(wt)

def key(t):
    m = re.match(r"## (.+?)(?:\s*\(|$)", t)
    return re.sub(r"\s+", " ", m.group(1)).strip() if m else t

map_f = {key(t): (t, b) for t, b in s_f}
wt_keys = {key(t) for t, b in s_w}
o_keys = {key(t) for t, b in s_o}
f_keys = set(map_f)

restored = []
for sid in SWEEP_IDS:
    hits = [k for k in f_keys if f"ЛАБ-{sid}" in k or k.endswith(f"ЛАБ-{sid}")]
    if not hits:
        # альтернативный матч: секция содержит «ТИК-482» и «ЛАБ-C01»
        hits = [k for k in f_keys if "ТИК-482" in k and f"ЛАБ-{sid}" in k]
    if hits and hits[0] not in wt_keys and hits[0] not in o_keys:
        restored.append(map_f[hits[0]])
        print(f"restored {hits[0]}")

# worktree-only секции (не в origin)
wt_only = [(t, b) for t, b in s_w if key(t) not in o_keys]
print(f"worktree-only: {len(wt_only)}; origin total: {len(s_o)}; restored: {len(restored)}")

# Финальный файл: origin + worktree-only + restored (все в конце, порядок worktree)
out = origin.rstrip("\n") + "\n"
for t, b in wt_only + restored:
    out += f"\n{t}\n{b}"
open(LED, "w").write(out)
print("written; total lines:", out.count("\n"))
