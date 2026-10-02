#!/usr/bin/env python3
"""TASK-93 S7-35: multi-window boot JFR attribution + registry-island gate (R1).
Parses `jfr print --events jdk.ExecutionSample` text, splits the boot into
log-marker windows, and per window prints: thread share, top leaves, family
taxonomy (I/O vs DFU vs invoke-bootstrap vs registry/worldgen/construct),
plus the R1 gate: share of window samples with NO I/O frame in stack and
net.minecraft/mojang construction leaf = 'registry-island construction'.

Usage: analyze_boot_windows.py <jfr_print.txt> <start HH:MM:SS> <W0_end HH:MM:SS> <W1_end HH:MM:SS> <Done HH:MM:SS[.frac]>
"""
import re
import sys
from collections import Counter

FAMS = [
    ("io_zip", r"^(java\.io\.|java\.util\.zip\.|java\.nio\.|sun\.nio\.|jdk\.internal\.jimage\.|jdk\.internal\.module\.SystemModule)"),
    ("dfu", r"^com\.mojang\.datafixers"),
    ("invoke_boot", r"^java\.lang\.invoke\."),
    ("classload", r"(ClassLoader\.|\.defineClass|jdk\.internal\.loader\.|BuiltinClassLoader)"),
    ("mc_registry_res", r"^net\.minecraft\.(core\.|registry|resources|server\.pack|server\.databindings)"),
    ("mc_worldgen", r"^net\.minecraft\.(world\.level\.level|world\.level\.biome|util\.noise|world\.level\.dimension)"),
    ("mc_ctor", r"\.<(init|clinit)>\("),
    ("bukkit_paper", r"^(org\.bukkit|io\.papermc|com\.destroystokyo)"),
    ("jvm_misc", r"^(java\.util\.|java\.lang\.|jdk\.internal\.misc|java\.security|java\.lang\.reflect)"),
]

def tosec(ts):
    # fixed S7-35: H*3600 + M*60 + S (old loop multiplied minutes into the
    # running seconds sum — monotonic+injective so past window slices were
    # still valid via self-consistency, but absolute comparisons were wrong)
    p = ts.split(":")
    return float(p[0]) * 3600 + float(p[1]) * 60 + float(p[-1])

def main():
    path = sys.argv[1]
    w = [tosec(x) for x in sys.argv[2:6]]  # start, w0end, w1end, done
    names = ["W0_pre(JVM+plugin)", "W1_registry_window", "W2_env_to_Done"]
    bounds = [(w[0], w[1]), (w[1], w[2]), (w[2], w[3] + 0.5)]

    blocks = re.findall(r"jdk\.ExecutionSample \{(.*?)\n\}",
                        open(path, encoding="utf-8", errors="replace").read(), re.S)
    parsed = []
    for b in blocks:
        tsm = re.search(r"startTime = (\d+:\d+:[\d.]+)", b)
        if not tsm:
            continue
        t = tosec(tsm.group(1))
        thr = re.search(r'sampledThread = "([^"]+)"', b)
        frames = re.findall(r"^\s{4}(\S+)\(", b, re.M)
        if frames:
            parsed.append((t, thr.group(1) if thr else "?", frames))
    print(f"parsed samples: {len(parsed)}")

    for name, (lo, hi) in zip(names, bounds):
        win = [(t, th, fr) for t, th, fr in parsed if lo - 0.02 <= t < hi]
        n = len(win)
        if not n:
            print(f"\n== {name} == EMPTY ({lo:.1f}-{hi:.1f})")
            continue
        dur = hi - lo
        print(f"\n== {name} [{lo:.1f}-{hi:.1f}) {dur:.1f}s, {n} samples ==")
        thr_c = Counter(th for _, th, _ in win)
        print("-- threads:", ", ".join(f"{t} {c} ({c*100//n}%)" for t, c in thr_c.most_common(6)))

        # family taxonomy: any-frame match (first family wins by priority order)
        fam = Counter()
        noio = 0
        for _, _, fr in win:
            st = "\n".join(fr)
            hit = next((fn for fn, rx in FAMS if re.search(rx, st, re.M)), "other")
            fam[hit] += 1
            if not re.search(FAMS[0][1], st, re.M):
                noio += 1
        print("-- families (any-frame, first-hit):", ", ".join(
            f"{f} {c} ({c*100//n}%)" for f, c in fam.most_common()))

        leaf = Counter(fr[0].split("(")[0] for _, _, fr in win)
        print("-- top leaves:")
        for f, c in leaf.most_common(10):
            print(f"   {c:4d} {f}")

        path3 = Counter(" <- ".join(x.split("(")[0].rsplit(".", 1)[-1]
                                    for x in fr[:3]) for _, _, fr in win)
        print("-- top 3-frame paths:")
        for p, c in path3.most_common(12):
            print(f"   {c:4d} {p}")

        # R1 gate: construction = no I/O frame AND leaf in net.minecraft/mojang
        ctor_leaf = re.compile(r"^(net\.minecraft|com\.mojang)\.")
        isl = sum(1 for _, _, fr in win
                  if not re.search(FAMS[0][1], "\n".join(fr), re.M)
                  and ctor_leaf.match(fr[0].split("(")[0]))
        print(f"-- R1 GATE: no-IO {noio}/{n} = {noio*100//n}%; "
              f"registry-island-leaf(noIO+mc/mojang leaf) {isl}/{n} = {isl*100//n}%")

if __name__ == "__main__":
    main()
