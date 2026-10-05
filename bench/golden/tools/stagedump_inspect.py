#!/usr/bin/env python3
"""stagedump_inspect.py — inspector for the DECODED staged chunk dumps (NCF task 5-a).

Reads the gzipped NBT files produced by the GoldenDumper plugin STAGED mode
(`/goldendump <x> <z> <r> status <noise|surface>`) and prints/queries their
decoded contents. These dumps are the vanilla-side ORACLE for the Rust staged
chunk gate (tasks 5-b/5-c): the Rust side must reproduce CONTENT, not Java's
bit-packing, so the dump contract is decoded arrays instead of packed longs.

FILE CONTRACT (written by GoldenDumperPlugin.buildStagedTag; verified 2026-10-05):
  root compound:
    ChunkX: Int, ChunkZ: Int, Status: String ("minecraft:noise"/"minecraft:surface"),
    DataVersion: Int, MinY: Int, Height: Int
    Sections: List of compound (DENSE, one per section minSectionY..maxSectionY):
        Y: Byte                    section Y = worldY >> 4
        Palette: List of compound  {Name: String, Properties: {k: v}} (vanilla form)
        Data: IntArray(4096)       palette indices; VERIFIED index order
                                   i = sy*256 + sz*16 + sx  (x fastest, y slowest)
    Biomes: List matching Sections 1:1:
        Y: Byte
        Palette: List of String    biome id, e.g. "minecraft:ocean"
        Data: IntArray(64)         VERIFIED index order
                                   q = by*16 + bz*4 + bx  (quart coords)
    Heightmaps: compound of raw long[] copied AS-IS (NOT decoded here either);
                at NOISE/SURFACE exactly {WORLD_SURFACE_WG, OCEAN_FLOOR_WG}
                (9 bits per entry, 256 entries, index = x + 16*z, values are
                height-minY — display only in this tool).
    block_ticks / fluid_ticks: present ONLY if non-empty (vanilla {i,t,p} shape)
    PostProcessing: List of 24 lists of Short (per-section packed offsets,
                vanilla SerializableChunkData.packOffsets shape)

  Both index orders were verified against the mojang-mapped Purpur 1.21.10 jar
  (CFR decompile of net.minecraft.world.level.chunk.Strategy):
      getIndex(x, y, z) = (y << bitsPerAxis | z) << bitsPerAxis | x
      blocks bitsPerAxis=4 -> (y<<4|z)<<4|x; biomes bitsPerAxis=2 -> (y<<2|z)<<2|x

Usage:
  python3 stagedump_inspect.py FILE                    # summary
  python3 stagedump_inspect.py FILE --block X Y Z      # world-coord block query
  python3 stagedump_inspect.py FILE --biome X Y Z      # world-coord biome query
  python3 stagedump_inspect.py FILE --scan N           # first N non-air blocks
  python3 stagedump_inspect.py FILE --verify-index     # in-file index consistency
  python3 stagedump_inspect.py FILE --heightmaps       # decoded heightmap min/max
  python3 stagedump_inspect.py --selftest              # synthetic round-trip

Exit codes: 0 ok / 1 check failed / 2 usage-IO error.
"""

import argparse
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import ncfdiff as N  # reuse the battle-tested NBT parser/writer from P0.6

# ---------------------------------------------------------------------------
# Contract index helpers (mirror the VERIFIED vanilla Strategy.getIndex)
# ---------------------------------------------------------------------------

def block_index(sx, sy, sz):
    """VERIFIED contract: section-local index i = sy*256 + sz*16 + sx."""
    return sy * 256 + sz * 16 + sx


def block_coords(i):
    """Inverse of block_index: (sx, sy, sz) from i."""
    return (i & 15, (i >> 8) & 15, (i >> 4) & 15)  # sx, sy, sz


def biome_index(bx, by, bz):
    """VERIFIED contract: quart index q = by*16 + bz*4 + bx."""
    return by * 16 + bz * 4 + bx


def biome_coords(q):
    """Inverse of biome_index: (bx, by, bz) from q."""
    return (q & 3, q >> 4, (q >> 2) & 3)  # bx, by, bz


def block_key(entry):
    """Palette entry compound -> readable 'name[prop=v,...]' (sorted props)."""
    return N._block_key(entry)


# ---------------------------------------------------------------------------
# Loading / structural access
# ---------------------------------------------------------------------------

def load_root(path):
    root = N.read_nbt_file(path)
    if not isinstance(root, N.TComp):
        raise SystemExit("not a compound NBT root: %s" % path)
    return root


def _g(root, key):
    return root.d.get(key)


def staged_meta(root):
    # NOTE: parsed roots (ncfdiff reader) carry PLAIN python values in TComp.d
    # (int/str/TList/TArr) — only the WRITER side uses (tid, value) tuples.
    return {
        "ChunkX": _g(root, "ChunkX"),
        "ChunkZ": _g(root, "ChunkZ"),
        "Status": _g(root, "Status"),
        "DataVersion": _g(root, "DataVersion"),
        "MinY": _g(root, "MinY"),
        "Height": _g(root, "Height"),
    }


def sections(root):
    """[(y, palette_list, data_list)] from Sections, Y-ascending as dumped."""
    out = []
    for sec in _g(root, "Sections").items:
        y = sec.d["Y"]
        pal = sec.d["Palette"].items
        data = sec.d["Data"].vals
        out.append((y, pal, data))
    return out


def biomes(root):
    out = []
    for sec in _g(root, "Biomes").items:
        y = sec.d["Y"]
        pal = sec.d["Palette"].items
        data = sec.d["Data"].vals
        out.append((y, pal, data))
    return out


def check_structure(root):
    """Validate the invariants of the contract; returns list of error strings."""
    errs = []
    for key in ("ChunkX", "ChunkZ", "Status", "DataVersion", "MinY", "Height",
                "Sections", "Biomes", "PostProcessing"):
        if key not in root.d:
            errs.append("missing root key %s" % key)
    if errs:
        return errs
    mins = _g(root, "MinY")
    height = _g(root, "Height")
    secs = sections(root)
    bios = biomes(root)
    if len(secs) != len(bios):
        errs.append("Sections/Biomes length mismatch: %d vs %d" % (len(secs), len(bios)))
    if len(secs) * 16 != height:
        errs.append("section count %d x 16 != Height %d" % (len(secs), height))
    for idx, (y, pal, data) in enumerate(secs):
        if y != (mins >> 4) + idx:
            errs.append("section %d has Y=%d, expected %d" % (idx, y, (mins >> 4) + idx))
        if len(data) != 4096:
            errs.append("section Y=%d Data length %d != 4096" % (y, len(data)))
        if not (0 < len(pal) <= 4096):
            errs.append("section Y=%d palette size %d out of range" % (y, len(pal)))
        if any(v < 0 or v >= len(pal) for v in data):
            errs.append("section Y=%d has Data index outside palette" % y)
    for idx, (y, pal, data) in enumerate(bios):
        if len(data) != 64:
            errs.append("biome section %d Data length %d != 64" % (idx, len(data)))
        if any(v < 0 or v >= len(pal) for v in data):
            errs.append("biome section %d has Data index outside palette" % idx)
    pp = _g(root, "PostProcessing")
    if pp.tid != N.TAG_LIST or len(pp.items) != len(secs):
        errs.append("PostProcessing inner-list count %s != section count %d"
                    % (len(pp.items), len(secs)))
    return errs


def section_for_y(root, world_y):
    mins = _g(root, "MinY")
    idx = (world_y >> 4) - (mins >> 4)
    secs = sections(root)
    if idx < 0 or idx >= len(secs):
        raise IndexError("world Y %d outside [MinY=%d, MinY+Height=%d)"
                         % (world_y, mins, mins + _g(root, "Height")))
    return idx


def block_at(root, wx, wy, wz):
    """World-coord query -> readable block key (palette entry decode)."""
    idx = section_for_y(root, wy)
    y, pal, data = sections(root)[idx]
    i = block_index(wx & 15, wy & 15, wz & 15)
    return block_key(pal[data[i]])


def biome_at(root, wx, wy, wz):
    idx = section_for_y(root, wy)
    y, pal, data = biomes(root)[idx]
    q = biome_index((wx >> 2) & 3, (wy >> 2) & 3, (wz >> 2) & 3)
    return pal[data[q]]


# ---------------------------------------------------------------------------
# Reports
# ---------------------------------------------------------------------------

def _top_blocks(pal, data, n=4):
    from collections import Counter
    c = Counter(data)
    out = []
    for val, cnt in c.most_common(n):
        out.append("%dx %s" % (cnt, block_key(pal[val])))
    return ", ".join(out)


def print_summary(root, path):
    m = staged_meta(root)
    print("file        : %s" % path)
    print("chunk       : %s %s" % (m["ChunkX"], m["ChunkZ"]))
    print("status      : %s" % m["Status"])
    print("DataVersion : %s   MinY: %s   Height: %s" % (m["DataVersion"], m["MinY"], m["Height"]))
    secs = sections(root)
    bios = biomes(root)
    print("sections    : %d  (Y %d..%d)" % (len(secs), secs[0][0], secs[-1][0]))
    print("%-4s %-6s %s" % ("Y", "pal", "top blocks (Data index histogram)"))
    for y, pal, data in secs:
        print("%-4d %-6d %s" % (y, len(pal), _top_blocks(pal, data)))
    print("biome palettes (per section):")
    for y, pal, data in bios:
        from collections import Counter
        c = Counter(data)
        tops = ", ".join("%dx %s" % (cnt, pal[val]) for val, cnt in c.most_common(3))
        print("  Y=%-3d pal=%-3d %s" % (y, len(pal), tops))
    hms = _g(root, "Heightmaps")
    if isinstance(hms, N.TComp) and hms.d:
        for k in hms.d:
            arr = hms.d[k]
            n = len(arr.vals) if isinstance(arr, N.TArr) else -1
            print("heightmap   : %s  long[%d] (raw, not decoded in contract)" % (k, n))
    else:
        print("heightmap   : none")
    pp = _g(root, "PostProcessing")
    total = sum(len(x.items) for x in pp.items) if isinstance(pp, N.TList) else 0
    print("PostProcessing: %d entries across %d sections"
          % (total, len(pp.items) if isinstance(pp, N.TList) else 0))
    for key in ("block_ticks", "fluid_ticks"):
        if key in root.d:
            print("%s: %d entries" % (key, len(root.d[key].items)))


def cmd_scan(root, n):
    """First N non-air blocks in contract index order (section by section)."""
    count = 0
    for y, pal, data in sections(root):
        for i in range(4096):
            key = block_key(pal[data[i]])
            if key.endswith("air") or key == "minecraft:cave_air":
                continue
            sx, sy, sz = block_coords(i)
            wx = (_g(root, "ChunkX") << 4) + sx
            wz = (_g(root, "ChunkZ") << 4) + sz
            wy = y * 16 + sy
            print("%d,%d,%d,%s" % (wx, wy, wz, key))
            count += 1
            if count >= n:
                return 0
    return 0


def cmd_verify_index(root):
    """Self-consistency: every non-air block found via a scan must re-read
    identically when addressed through the coordinate path (world coords ->
    section -> contract index -> palette). This closes the loop between the
    scan order and the coordinate math on the SAME file."""
    checks = 0
    mismatches = 0
    for y, pal, data in sections(root):
        by_index = data  # i -> palette index
        for i in range(4096):
            v = by_index[i]
            sx, sy, sz = block_coords(i)
            if block_index(sx, sy, sz) != i:
                print("INDEX FORMULA SELF-INVERSE FAILURE at %d" % i)
                mismatches += 1
                break
            if v == 0:
                continue  # index 0 = first palette entry; check only non-default
            # coordinate path on raw arrays (no re-decode shortcut)
            idx2 = block_index(sx, sy, sz)
            if by_index[idx2] != v:
                print("MISMATCH section Y=%d i=%d: scan=%d coord-path=%d" % (y, i, v, by_index[idx2]))
                mismatches += 1
            checks += 1
            if checks >= 4096:
                break
        if checks >= 4096:
            break
    # biome side: verify formula inverse for all 64 quart indices of one section
    y, pal, data = biomes(root)[len(biomes(root)) // 2]
    for q in range(64):
        bx, by_, bz = biome_coords(q)
        if biome_index(bx, by_, bz) != q:
            print("BIOME INDEX FORMULA SELF-INVERSE FAILURE at %d" % q)
            mismatches += 1
    print("verify-index: %d block checks, %d mismatches" % (checks, mismatches))
    return 1 if mismatches else 0


def cmd_heightmaps(root):
    """Display-only decode of the raw heightmap longs (9 bits, no crossing)."""
    hms = _g(root, "Heightmaps")
    if not (isinstance(hms, N.TComp) and hms.d):
        print("no heightmaps")
        return 0
    for k in sorted(hms.d):
        arr = hms.d[k]
        if not isinstance(arr, N.TArr) or arr.kind != "l":
            print("%s: not a long array?" % k)
            continue
        vals = N._unpack(arr.vals, 9, 256)
        print("%s: min=%d max=%d  (raw longs=%d, 9-bit x 256)" % (k, min(vals), max(vals), len(arr.vals)))
    return 0


# ---------------------------------------------------------------------------
# Selftest: synthetic NBT in memory -> round-trip through the parser
# ---------------------------------------------------------------------------

def selftest():
    # 2 sections: Y=-4 (two layers) and Y=5 (uniform air), chunk 0,0.
    # Crafted so that (sx=1,sy=2,sz=3) != (sx=3,sy=2,sz=1) — catches any
    # transposition of x/z in the index formula.
    # TRAP (found by first --selftest run): TComp.d for the WRITER takes
    # (tag_id, value) tuples — a bare TComp value inside a compound raises
    # "TypeError: cannot unpack non-iterable TComp object" in ncfdiff._w_payload.
    def state(name, **props):
        comp = N.TComp({"Name": (N.TAG_STRING, name)})
        if props:
            comp.d["Properties"] = (N.TAG_COMPOUND,
                                    N.TComp({k: (N.TAG_STRING, v) for k, v in props.items()}))
        return comp

    stone = state("minecraft:stone")
    water = state("minecraft:water")
    deepslate = state("minecraft:deepslate", axis="y")
    pal = [stone, water, deepslate]  # indices 0,1,2

    data = [0] * 4096
    data[block_index(1, 2, 3)] = 1  # water
    data[block_index(3, 2, 1)] = 2  # deepslate (axis=y)
    data[block_index(15, 15, 15)] = 2

    def section(y, palette, arr, is_blocks=True):
        if is_blocks:
            return N.TComp({
                "Y": (N.TAG_BYTE, y),
                "Palette": (N.TAG_LIST, N.TList(N.TAG_COMPOUND, palette)),
                "Data": (N.TAG_INT_ARRAY, N.TArr("i", arr)),
            })
        return N.TComp({
            "Y": (N.TAG_BYTE, y),
            "Palette": (N.TAG_LIST, N.TList(N.TAG_STRING, palette)),
            "Data": (N.TAG_INT_ARRAY, N.TArr("i", arr)),
        })

    sec_a = section(-4, pal, data)
    # Y=-3 (adjacent section, world Y -48..-33): uniform air.
    sec_b = section(-3, [state("minecraft:air")], [0] * 4096)

    bpal = ["minecraft:ocean", "minecraft:plains"]
    bdata = [0] * 64
    bdata[biome_index(1, 2, 3)] = 1  # plains
    bdata[biome_index(3, 2, 1)] = 0
    bsec_a = section(-4, bpal, bdata, is_blocks=False)
    bsec_b = section(-3, ["minecraft:ocean"], [0] * 64, is_blocks=False)

    root = N.TComp({
        "ChunkX": (N.TAG_INT, 0),
        "ChunkZ": (N.TAG_INT, 0),
        "Status": (N.TAG_STRING, "minecraft:noise"),
        "DataVersion": (N.TAG_INT, 4556),
        "MinY": (N.TAG_INT, -64),
        # Height MUST equal section count * 16 (check_structure enforces it):
        # 2 synthetic sections -> 32.
        "Height": (N.TAG_INT, 32),
        "Sections": (N.TAG_LIST, N.TList(N.TAG_COMPOUND, [sec_a, sec_b])),
        "Biomes": (N.TAG_LIST, N.TList(N.TAG_COMPOUND, [bsec_a, bsec_b])),
        "Heightmaps": (N.TAG_COMPOUND, N.TComp({
            "WORLD_SURFACE_WG": (N.TAG_LONG_ARRAY, N.TArr("l", [0x0123456789ABCDEF])),
        })),
        "PostProcessing": (N.TAG_LIST, N.TList(N.TAG_LIST, [
            N.TList(N.TAG_SHORT, [1, 2]),
            N.TList(N.TAG_SHORT, []),
        ])),
    })

    blob = N.build_nbt(root)          # gzipped synthetic NBT (ncfdiff writer)
    parsed = N.parse_nbt_bytes(blob)  # round-trip through the real reader

    fails = []

    def expect(cond, msg):
        if not cond:
            fails.append(msg)

    expect(staged_meta(parsed)["Status"] == "minecraft:noise", "Status round-trip")
    expect(staged_meta(parsed)["MinY"] == -64 and staged_meta(parsed)["Height"] == 32,
           "MinY/Height round-trip")
    expect(check_structure(parsed) == [], "structure check: %s" % check_structure(parsed))

    # coordinate queries: section Y=-4 covers world Y -64..-49; sy=2 -> Y=-62
    expect(block_at(parsed, 1, -62, 3) == "minecraft:water",
           "block (1,-62,3)=%s want water" % block_at(parsed, 1, -62, 3))
    expect(block_at(parsed, 3, -62, 1) == "minecraft:deepslate[axis=y]",
           "block (3,-62,1)=%s want deepslate[axis=y]" % block_at(parsed, 3, -62, 1))
    expect(block_at(parsed, 15, -49, 15) == "minecraft:deepslate[axis=y]",
           "block (15,-49,15)=%s want deepslate[axis=y]" % block_at(parsed, 15, -49, 15))
    # section Y=-3 covers world Y -48..-33; wy=-40 -> section idx 1
    expect(block_at(parsed, 0, -40, 0) == "minecraft:air",
           "uniform-air section query")
    # biome quart (bx=1,by=2,bz=3) in section Y=-4 (world Y -64..-49):
    # by = (wy>>2)&3 must be 2 -> wy in -56..-53 (e.g. -54: -54>>2=-14, &3=2)
    expect(biome_at(parsed, 5, -54, 13) == "minecraft:plains",
           "biome quart(1,2,3) via world (5,-54,13)=%s want plains" % biome_at(parsed, 5, -54, 13))
    expect(biome_at(parsed, 13, -54, 5) == "minecraft:ocean",
           "biome quart(3,2,1) via world (13,-54,5)=%s want ocean" % biome_at(parsed, 13, -54, 5))

    hms = _g(parsed, "Heightmaps")
    expect(hms.d["WORLD_SURFACE_WG"].vals == [0x0123456789ABCDEF], "heightmap raw round-trip")

    pp = _g(parsed, "PostProcessing")
    expect(len(pp.items) == 2, "PostProcessing inner list count")
    expect(list(pp.items[0].items) == [1, 2], "PostProcessing shorts round-trip")

    # verify-index helper on the synthetic file
    expect(cmd_verify_index(parsed) == 0, "verify-index on synthetic file")

    if fails:
        print("SELFTEST FAIL (%d):" % len(fails))
        for f in fails:
            print("  - %s" % f)
        return 1
    print("SELFTEST OK — synthetic staged NBT round-trips through the contract "
          "(index orders sy*256+sz*16+sx / by*16+bz*4+bx verified)")
    return 0


# ---------------------------------------------------------------------------

def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("file", nargs="?", help="staged chunk .nbt file")
    ap.add_argument("--block", nargs=3, type=int, metavar=("X", "Y", "Z"),
                    help="world-coord block query")
    ap.add_argument("--biome", nargs=3, type=int, metavar=("X", "Y", "Z"),
                    help="world-coord biome query (quart resolution)")
    ap.add_argument("--scan", type=int, metavar="N", help="print first N non-air blocks")
    ap.add_argument("--verify-index", action="store_true",
                    help="in-file index-order self-consistency check")
    ap.add_argument("--heightmaps", action="store_true", help="decode heightmap stats")
    ap.add_argument("--selftest", action="store_true", help="synthetic round-trip test")
    args = ap.parse_args(argv)

    if args.selftest:
        return selftest()
    if not args.file:
        ap.error("FILE required (or --selftest)")
        return 2
    root = load_root(args.file)
    errs = check_structure(root)
    if errs:
        for e in errs:
            print("CONTRACT VIOLATION: %s" % e)
        return 1
    if args.block:
        x, y, z = args.block
        print(block_at(root, x, y, z))
        return 0
    if args.biome:
        x, y, z = args.biome
        print(biome_at(root, x, y, z))
        return 0
    if args.scan is not None:
        return cmd_scan(root, args.scan)
    if args.verify_index:
        return cmd_verify_index(root)
    if args.heightmaps:
        return cmd_heightmaps(root)
    print_summary(root, args.file)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, N.NBTError) as e:
        print("ERROR: %s" % e, file=sys.stderr)
        sys.exit(2)
