#!/usr/bin/env python3
"""TASK-449-C (alpha): retag composition levers -> cmp449_mega4 across rust gates.
Protects the explanatory 'retag cmp443_mega/...' doc comments from being double-retagged."""
import os, re

OLD = ["cmp443_mega", "cmp445_collide", "cmp446_items"]
PROTECT = "retag cmp443_mega/cmp445_collide/cmp446_items -> cmp449_mega4"
PH = "\x00PROTECTED\x00"

src = "src"
changed = {}
for fn in os.listdir(src):
    if not fn.endswith(".rs"):
        continue
    p = os.path.join(src, fn)
    s = open(p).read()
    orig = s
    s = s.replace(PROTECT, PH)
    for a in OLD:
        s = s.replace(a, "cmp449_mega4")
    s = s.replace(PH, PROTECT)
    if s != orig:
        open(p, "w").write(s)
        n = sum(orig.count(a) for a in OLD)
        changed[fn] = n
for k, v in sorted(changed.items()):
    print(f"{k}: {v} retagged")
print("TOTAL files:", len(changed))
