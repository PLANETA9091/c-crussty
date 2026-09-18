#!/usr/bin/env python3
"""s7151_census2.py — точный census через zipfile+UTF8-поиск + javap валидацию.
Выход: S7151_CENSUS.md — вызыватели updateFluidHeightAndDoFluidPushing и
LevelChunkSection.setBlockState (владельцы methodref'ов)."""
import zipfile, subprocess, os

KERNEL = "/home/z/c-crussty/research/inside-cache-2026-09-18/run-s7149b-cumulative/patched-kernel.jar"
WORK = "/tmp/s7151_census"
JAVAP = "/tmp/toolchain/jdk-21.0.12.1+1/bin/javap"
OUT = "/home/z/c-crussty/research/fluid-dirty-2026-09-18/S7151_CENSUS.md"

SCAN = b"updateFluidHeightAndDoFluidPushing"
SECW = b"setBlockState"

os.makedirs(WORK, exist_ok=True)
scan_callers, secw_owners = [], []
zf = zipfile.ZipFile(KERNEL)
for zi in zf.infolist():
    if not zi.filename.endswith(".class"):
        continue
    b = zf.read(zi.filename)
    cls = zi.filename[:-6]
    has_scan = SCAN in b
    has_secw = SECW in b
    if not (has_scan or has_secw):
        continue
    # методref владельца: класс после "net/minecraft/.../" до ".class"
    path = os.path.join(WORK, cls + ".class")
    d = os.path.dirname(path)
    os.makedirs(d, exist_ok=True)
    with open(path, "wb") as fh:
        fh.write(b)
    scan_callers.append((cls, has_scan))
    if has_secw:
        secw_owners.append(cls)

# классы, ДЕЙСТВИТЕЛЬНО вызывающие scan (invokevirtual с этим methodref)
res_scan = []
for cls, _ in scan_callers:
    p = os.path.join(WORK, cls + ".class")
    out = subprocess.run([JAVAP, "-c", "-p", "-cp", WORK, cls.replace("/", ".")],
                         capture_output=True, text=True).stdout
    if "updateFluidHeightAndDoFluidPushing" in out and ("invokevirtual" in out or "invokespecial" in out):
        # есть ли ИМЕННО вызов (а не объявление)?
        ncall = sum(1 for ln in out.splitlines()
                    if "invoke" in ln and "updateFluidHeightAndDoFluidPushing" in ln)
        if ncall:
            sites = [ln.strip() for ln in out.splitlines()
                     if "invoke" in ln and "updateFluidHeightAndDoFluidPushing" in ln]
            res_scan.append((cls, ncall, sites))

# LevelChunk.setBlockState: какой LevelChunkSection.setBlockState перегрузки вызываются
lc = "net.minecraft.level.chunk.LevelChunk".replace("level", "world/level")
lc = "net.minecraft.world.level.chunk.LevelChunk"
out_lc = subprocess.run([JAVAP, "-c", "-p", "-cp", WORK, lc], capture_output=True, text=True).stdout
setblock_sites = []
lines = out_lc.splitlines()
cur = None
for ln in lines:
    if ln and not ln.startswith(" ") and "(" in ln and ";" in ln and not ln.startswith("Compiled"):
        cur = ln.strip()
    if "invoke" in ln and "LevelChunkSection.setBlockState" in ln:
        setblock_sites.append((cur, ln.strip()))

with open(OUT, "w") as fh:
    fh.write("# S7-151 CENSUS v2 — вызыватели scan/section-write (kernel e2992d63)\n\n")
    fh.write("## Вызыватели Entity.updateFluidHeightAndDoFluidPushing (invoke): %d классов\n\n" % len(res_scan))
    for cls, n, sites in res_scan:
        fh.write("- `%s` — %d вызовов:\n" % (cls, n))
        for s in sites[:6]:
            fh.write("    %s\n" % s)
    fh.write("\n## LevelChunk: вызовы LevelChunkSection.setBlockState:\n\n")
    for cur, s in setblock_sites:
        fh.write("- in `%s`\n    %s\n" % (cur, s))
print(open(OUT).read()[:4000])
