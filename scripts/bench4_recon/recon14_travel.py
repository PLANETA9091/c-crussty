#!/usr/bin/env python3
"""RECON-14a: декомпозиция entity-travel под-лейна — где именно создаются Vec3/AABB."""
from collections import Counter

D = "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7168-parse-diag"

def recon(name, leaf_pat, title):
    tot = 0
    # группировка по (последнему minecraft-фрейму перед leaf) + сам leaf-класс
    sites = Counter()
    with open(f"{D}/{name}", "r", errors="replace") as f:
        for line in f:
            try:
                stack, cnt = line.rstrip("\n").rsplit(" ", 1)
                c = int(cnt)
            except ValueError:
                continue
            tot += c
            frames = stack.split(";")
            leaf = frames[-1]
            import re
            if not re.search(leaf_pat, leaf):
                continue
            # ближайший фрейм net/minecraft или cru.sty ниже leaf
            anchor = "?"
            for fr in reversed(frames[:-1]):
                if "net/minecraft" in fr or "cru/sty" in fr or "ca/spottedleaf" in fr:
                    anchor = fr.split("(")[0]
                    break
            sites[(anchor, leaf.split("(")[0][:80])] += c
    print(f"== {title} (total {name}={tot}) ==")
    for (anchor, leaf), v in sites.most_common(14):
        print(f"  {v:6d} ({100.0*v/tot:5.2f}%) ANCHOR={anchor[:110]} LEAF={leaf}")
    print()

recon("alloc-collapsed.txt", r"Vec3_\[i\]", "ALLOC leaf=Vec3.<init>")
recon("alloc-collapsed.txt", r"AABB_\[i\]", "ALLOC leaf=AABB.<init>")
