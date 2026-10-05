#!/usr/bin/env python3
"""ncfdiff.py — semantic NBT diff for golden chunk dumps (NCF P0.6).

Compares two chunk NBT files (as produced by the GoldenDumper plugin,
NbtIo.writeCompressed -> gzip) and classifies:

  EQUAL           byte-identical files
  SEMANTIC_EQUAL  bytes differ, decoded semantics identical (e.g. InhabitedTime)
  DIVERGED        decoded semantics differ; report lists per-field diffs and,
                  for chunk sections, the FIRST divergent block / biome /
                  heightmap value with its coordinates

Exit codes: 0 = equal (either kind), 1 = diverged, 2 = usage / IO error.

HONEST LIMITS (P0.6 MVP — palette order + long packing subtleties must be
verified against the first REAL corpus dumps; findings go to the NCF worklog
LOG section, not silently patched here):
  * Heightmap bit width: HEIGHTMAP_BITS=9 (named constant) for 1.21 world
    height 384. VERIFY-1.21.10 via corpus DataVersion: confirm every real dump
    decodes to exactly 256 in-range values; if a DataVersion needs a different
    width, gate the constant on DataVersion here.
  * Heightmap index -> (x,z) mapping assumed x = i & 15, z = i >> 4 (i.e.
    index = x + 16*z). VERIFY against vanilla Heightmap when triaging the
    first heightmap divergence (affects the reported coordinates only).
  * Block-state palette decode: bits = max(4, ceil(log2(n))) for n>=2, entries
    packed WITHOUT crossing long boundaries (1.16+ format), index layout
    y*256 + z*16 + x ("yzx", x fastest) -> block coords x = i & 15,
    z = (i >> 4) & 15, y = (i >> 8) & 15 (+ section Y * 16). VERIFY via corpus.
  * Biome palette decode: same packing with min bits 1, 64 entries per section,
    index layout y*16 + z*4 + x -> quart coords x = i & 3, z = (i >> 2) & 3,
    y = i >> 4. VERIFY via corpus (quart-to-block = 4x).
  * "Direct/global" palettes (no palette tag, data only) are compared as raw
    index arrays; no registry mapping in MVP.
  * Lists (block_entities, entities, block_ticks, fluid_ticks, PostProcessing,
    structure starts/references) are compared ORDER-SENSITIVE — vanilla order
    should be deterministic for the same generation order; the P0.5 order test
    must confirm this.
  * BYTE_ONLY_FIELDS (byte diff with no semantic effect): InhabitedTime,
    LastUpdate, lastUpdateTime (world save time moves between runs). VERIFY
    via corpus: check which key the 1.21.10 record actually writes and whether
    anything else (e.g. Paper's ChunkBukkitValues) needs listing here.
  * The claim "SerializableChunkData.write() output == .mca region payload
    minus container framing" is VERIFY-1.21.10: check when the first corpus
    lands (compare a dump vs the bytes stored in the region file).

Usage:
  python3 ncfdiff.py a.nbt b.nbt [--report FILE]
  python3 ncfdiff.py --manifest dirA dirB [--report FILE]
  python3 ncfdiff.py --selftest
"""

import argparse
import gzip
import io
import math
import os
import struct
import sys
import zlib

# ---------------------------------------------------------------------------
# NBT reader (tag types 0..12; gzip, zlib and raw containers; UTF-8 names)
# ---------------------------------------------------------------------------

TAG_END = 0
TAG_BYTE = 1
TAG_SHORT = 2
TAG_INT = 3
TAG_LONG = 4
TAG_FLOAT = 5
TAG_DOUBLE = 6
TAG_BYTE_ARRAY = 7
TAG_STRING = 8
TAG_LIST = 9
TAG_COMPOUND = 10
TAG_INT_ARRAY = 11
TAG_LONG_ARRAY = 12

TID_NAME = {
    TAG_END: "END", TAG_BYTE: "byte", TAG_SHORT: "short", TAG_INT: "int",
    TAG_LONG: "long", TAG_FLOAT: "float", TAG_DOUBLE: "double",
    TAG_BYTE_ARRAY: "byte[]", TAG_STRING: "string", TAG_LIST: "list",
    TAG_COMPOUND: "compound", TAG_INT_ARRAY: "int[]", TAG_LONG_ARRAY: "long[]",
}


class TComp:
    __slots__ = ("d",)

    def __init__(self, d):
        self.d = d


class TList:
    __slots__ = ("tid", "items")

    def __init__(self, tid, items):
        self.tid = tid
        self.items = items


class TArr:
    __slots__ = ("kind", "vals")  # kind: 'b' | 'i' | 'l'

    def __init__(self, kind, vals):
        self.kind = kind
        self.vals = vals


class NBTError(Exception):
    pass


class _Cursor:
    def __init__(self, data):
        self.data = data
        self.off = 0

    def u1(self):
        v = self.data[self.off]
        self.off += 1
        return v

    def take(self, n):
        if self.off + n > len(self.data):
            raise NBTError("truncated NBT (need %d bytes at %d, have %d)"
                           % (n, self.off, len(self.data)))
        v = self.data[self.off:self.off + n]
        self.off += n
        return v

    def i1(self):
        return struct.unpack(">b", self.take(1))[0]

    def u2(self):
        return struct.unpack(">H", self.take(2))[0]

    def i2(self):
        return struct.unpack(">h", self.take(2))[0]

    def i4(self):
        return struct.unpack(">i", self.take(4))[0]

    def i8(self):
        return struct.unpack(">q", self.take(8))[0]

    def f4(self):
        return struct.unpack(">f", self.take(4))[0]

    def f8(self):
        return struct.unpack(">d", self.take(8))[0]

    def string(self):
        n = self.u2()
        raw = self.take(n)
        # Java "modified UTF-8" in practice is UTF-8 for all chunk NBT names;
        # decode leniently, names are ASCII in every observed dump.
        return raw.decode("utf-8", "replace")


def _read_payload(c, tid):
    if tid == TAG_BYTE:
        return c.i1()
    if tid == TAG_SHORT:
        return c.i2()
    if tid == TAG_INT:
        return c.i4()
    if tid == TAG_LONG:
        return c.i8()
    if tid == TAG_FLOAT:
        return c.f4()
    if tid == TAG_DOUBLE:
        return c.f8()
    if tid == TAG_BYTE_ARRAY:
        return TArr("b", list(c.take(c.i4())))
    if tid == TAG_STRING:
        return c.string()
    if tid == TAG_LIST:
        etid = c.u1()
        n = c.i4()
        if n < 0:
            raise NBTError("negative list length %d" % n)
        return TList(etid, [_read_payload(c, etid) for _ in range(n)])
    if tid == TAG_COMPOUND:
        d = {}
        while True:
            stid = c.u1()
            if stid == TAG_END:
                return TComp(d)
            name = c.string()
            d[name] = _read_payload(c, stid)
    if tid == TAG_INT_ARRAY:
        n = c.i4()
        return TArr("i", [struct.unpack(">i", c.take(4))[0] for _ in range(n)])
    if tid == TAG_LONG_ARRAY:
        n = c.i4()
        return TArr("l", [struct.unpack(">q", c.take(8))[0] for _ in range(n)])
    raise NBTError("unsupported tag id %d" % tid)


def parse_nbt_bytes(data):
    """Accept gzip (0x1f8b), zlib (low nibble of byte0 == 8) or raw NBT."""
    if data[:2] == b"\x1f\x8b":
        data = gzip.decompress(data)
    elif (data[0] & 0x0F) == 8:
        data = zlib.decompress(data)
    c = _Cursor(data)
    tid = c.u1()
    if tid != TAG_COMPOUND:
        raise NBTError("root is not a compound (tid=%d)" % tid)
    c.string()  # root name (usually "")
    return _read_payload(c, TAG_COMPOUND)


def read_nbt_file(path):
    with open(path, "rb") as f:
        return parse_nbt_bytes(f.read())


# ---------------------------------------------------------------------------
# Minimal NBT writer (selftest only — builds synthetic chunk blobs in memory)
# ---------------------------------------------------------------------------

def _w_payload(buf, tid, val):
    if tid in (TAG_BYTE_ARRAY, TAG_INT_ARRAY, TAG_LONG_ARRAY):
        if isinstance(val, TArr):
            val = val.vals
        if isinstance(val, (bytes, bytearray)):
            val = list(val)
    if tid == TAG_BYTE:
        buf += struct.pack(">b", val)
    elif tid == TAG_SHORT:
        buf += struct.pack(">h", val)
    elif tid == TAG_INT:
        buf += struct.pack(">i", val)
    elif tid == TAG_LONG:
        buf += struct.pack(">q", val)
    elif tid == TAG_FLOAT:
        buf += struct.pack(">f", val)
    elif tid == TAG_DOUBLE:
        buf += struct.pack(">d", val)
    elif tid == TAG_BYTE_ARRAY:
        buf += struct.pack(">i", len(val))
        buf += bytes(val)
    elif tid == TAG_STRING:
        raw = val.encode("utf-8")
        buf += struct.pack(">H", len(raw))
        buf += raw
    elif tid == TAG_LIST:
        buf += struct.pack(">B", val.tid)
        buf += struct.pack(">i", len(val.items))
        for item in val.items:
            _w_payload(buf, val.tid, item)
    elif tid == TAG_COMPOUND:
        for name, (vtid, vval) in val.d.items():
            buf += struct.pack(">B", vtid)
            _w_payload(buf, TAG_STRING, name)
            _w_payload(buf, vtid, vval)
        buf += b"\x00"
    elif tid == TAG_INT_ARRAY:
        buf += struct.pack(">i", len(val))
        for v in val:
            buf += struct.pack(">i", v)
    elif tid == TAG_LONG_ARRAY:
        buf += struct.pack(">i", len(val))
        for v in val:
            buf += struct.pack(">q", v)
    else:
        raise NBTError("cannot write tid %d" % tid)


def build_nbt(comp):
    """Serialize a TComp root to gzipped NBT bytes (deterministic, mtime=0)."""
    buf = bytearray()
    buf += b"\x0a"
    _w_payload(buf, TAG_STRING, "")
    _w_payload(buf, TAG_COMPOUND, comp)
    return gzip.compress(bytes(buf), mtime=0)


# ---------------------------------------------------------------------------
# Palette / heightmap decoders (semantic layer)
# ---------------------------------------------------------------------------

# VERIFY-1.21.10 via corpus: 9 bits per entry for 1.21 world height 384.
HEIGHTMAP_BITS = 9
HEIGHTMAP_ENTRIES = 256

# Byte-only (no semantic effect) top-level fields. VERIFY-1.21.10 via corpus.
BYTE_ONLY_FIELDS = {"InhabitedTime", "LastUpdate", "lastUpdateTime"}


def _unpack(values, bits, count):
    """Decode `count` entries of `bits` width from signed longs.

    1.16+ packing: entries never cross a long boundary.
    """
    if bits <= 0:
        return [0] * count
    per_long = 64 // bits
    mask = (1 << bits) - 1
    out = []
    for i in range(count):
        v = values[i // per_long] if (i // per_long) < len(values) else 0
        shift = (i % per_long) * bits
        out.append((v >> shift) & mask)
    return out


def _bits_for(palette_len, min_bits):
    if palette_len <= 1:
        return 0
    return max(min_bits, (palette_len - 1).bit_length())


def _block_key(pal_entry):
    if not isinstance(pal_entry, TComp):
        return repr(pal_entry)
    name = pal_entry.d.get("Name", "?")
    props = pal_entry.d.get("Properties")
    if isinstance(props, TComp) and props.d:
        ps = ";".join("%s=%s" % (k, props.d[k]) for k in sorted(props.d))
        return "%s[%s]" % (name, ps)
    return str(name)


def decode_states(section, key):
    """Decode section[key] (block_states or biomes) to per-index strings.

    Returns (values, mode) where mode is one of:
      'named'   -> list of palette-name strings (len 4096 blocks / 64 biomes)
      'raw'     -> raw palette indices (no palette tag in the dump)
      'empty'   -> no palette and no data on this side
      'invalid' -> cannot decode (payload shape unexpected; treated as divergence)
    """
    payload = section.d.get(key)
    if not isinstance(payload, TComp):
        return (None, "empty")
    palette = payload.d.get("palette")
    data = payload.d.get("data")
    is_biomes = (key == "biomes")
    n_entries = 64 if is_biomes else 4096
    min_bits = 1 if is_biomes else 4

    if isinstance(palette, TList) and isinstance(data, TArr):
        if data.kind != "l":
            return (None, "invalid")
        bits = _bits_for(len(palette.items), min_bits)
        raw = _unpack(data.vals, bits, n_entries)
        if bits == 0:
            return ([_block_key(palette.items[0])] * n_entries, "named")
        keys = [_block_key(p) for p in palette.items]
        if any(i >= len(keys) for i in raw):
            return (None, "invalid")
        return ([keys[i] for i in raw], "named")
    if isinstance(palette, TList) and data is None:
        # single-value container (uniform); palette must hold exactly one entry
        if len(palette.items) == 1:
            return ([_block_key(palette.items[0])] * n_entries, "named")
        return (None, "invalid")
    if palette is None and isinstance(data, TArr) and data.kind == "l":
        # global/direct palette: no registry mapping in MVP — raw indices.
        # Direct mode bit width: vanilla stores ceil(log2(registry size));
        # recover it from the data length (entries must fit).
        bits = None
        n_longs = len(data.vals)
        for cand_bits in range(min_bits, 33):
            per_long = 64 // cand_bits
            if per_long * n_longs >= n_entries:
                bits = cand_bits
                break
        if bits is None:
            return (None, "invalid")
        return (_unpack(data.vals, bits, n_entries), "raw")
    return (None, "invalid")


def decode_heightmap(longs):
    """Decode one heightmap long array to 256 values (9 bits, no crossing)."""
    return _unpack(longs, HEIGHTMAP_BITS, HEIGHTMAP_ENTRIES)


# ---------------------------------------------------------------------------
# Comparison engine
# ---------------------------------------------------------------------------

VERDICT_EQUAL = "EQUAL"
VERDICT_SEMANTIC = "SEMANTIC_EQUAL (bytes differ)"
VERDICT_DIVERGED = "DIVERGED"


class Report:
    def __init__(self, path_a, path_b):
        self.path_a = path_a
        self.path_b = path_b
        self.byte_only = []       # fields with byte diff, no semantic effect
        self.diverged_keys = []   # top-level keys that semantically differ
        self.details = []         # human-readable first-divergence lines

    def note_byte_only(self, field, va, vb):
        self.byte_only.append("%s: %s != %s" % (field, va, vb))

    def note_divergence(self, top_key, detail):
        if top_key not in self.diverged_keys:
            self.diverged_keys.append(top_key)
        line = "%s: %s" % (top_key, detail)
        if line not in self.details:
            self.details.append(line)

    def render(self):
        out = []
        out.append("A: %s" % self.path_a)
        out.append("B: %s" % self.path_b)
        for line in self.details:
            out.append("  " + line)
        if self.byte_only:
            for line in self.byte_only:
                out.append("  [byte-only] " + line)
        return "\n".join(out)


def _scalar_eq(a, b):
    if isinstance(a, bool) or isinstance(b, bool):
        return a == b
    if isinstance(a, (int, float)) and isinstance(b, (int, float)):
        if isinstance(a, float) and isinstance(b, float) and math.isnan(a) and math.isnan(b):
            return True
        return a == b
    return False


def deep_compare(a, b, rep, top_key, path):
    """Return True if semantically equal; records divergences in rep."""
    if isinstance(a, TComp) and isinstance(b, TComp):
        ok = True
        for k in sorted(set(a.d) | set(b.d)):
            if k not in a.d or k not in b.d:
                ok = False
                rep.note_divergence(top_key, "key %s%s missing in %s"
                                    % (path, k, "A" if k not in a.d else "B"))
                continue
            sub = "%s%s/" % (path, k)
            if not deep_compare(a.d[k], b.d[k], rep, top_key, sub):
                ok = False
        return ok
    if isinstance(a, TList) and isinstance(b, TList):
        if a.tid != b.tid and not (len(a.items) == 0 and len(b.items) == 0):
            rep.note_divergence(top_key, "%selem type %s != %s"
                                % (path, TID_NAME.get(a.tid, a.tid), TID_NAME.get(b.tid, b.tid)))
            return False
        if len(a.items) != len(b.items):
            rep.note_divergence(top_key, "%slen %d != %d (order-sensitive list)"
                                % (path, len(a.items), len(b.items)))
            return False
        ok = True
        for i, (ia, ib) in enumerate(zip(a.items, b.items)):
            if not deep_compare(ia, ib, rep, top_key, "%s[%d]/" % (path, i)):
                ok = False
                break  # first divergence is enough for lists (P0.6 MVP)
        return ok
    if isinstance(a, TArr) and isinstance(b, TArr):
        if a.kind != b.kind:
            rep.note_divergence(top_key, "%sarray kind %s != %s" % (path, a.kind, b.kind))
            return False
        if a.vals != b.vals:
            i = next((j for j, (x, y) in enumerate(zip(a.vals, b.vals)) if x != y), None)
            rep.note_divergence(top_key, "%sarray %s differs (first long idx %s)"
                                % (path, a.kind, i))
            return False
        return True
    if isinstance(a, (TComp, TList, TArr)) or isinstance(b, (TComp, TList, TArr)):
        rep.note_divergence(top_key, "%stype mismatch %s != %s" % (path, type(a).__name__, type(b).__name__))
        return False
    # scalars
    if a == b or _scalar_eq(a, b):
        return True
    if isinstance(a, str) and isinstance(b, str):
        rep.note_divergence(top_key, "%s%r != %r" % (path, a, b))
        return False
    rep.note_divergence(top_key, "%s%s != %s" % (path, a, b))
    return False


def _first_index(a, b):
    for i in range(min(len(a), len(b))):
        if a[i] != b[i]:
            return i
    if len(a) != len(b):
        return min(len(a), len(b))
    return None


def _state_coords(key, i):
    """Human-readable location of palette index i inside one section.

    blocks: index = y*256 + z*16 + x  (yzx, x fastest) -> x=i&15 z=(i>>4)&15 y=(i>>8)&15
    biomes: index = y*16 + z*4  + x   (quart, 4x4x4)  -> x=i&3  z=(i>>2)&3  y=i>>4
    """
    if key == "biomes":
        return "i=%d x=%d z=%d y=%d (quart)" % (i, i & 3, (i >> 2) & 3, i >> 4)
    return "i=%d x=%d z=%d y=%d" % (i, i & 15, (i >> 4) & 15, (i >> 8) & 15)


def compare_states(sec_a, sec_b, key, y, rep, n_entries):
    """Compare block_states/biomes of one section pair; report first divergence."""
    va, ma = decode_states(sec_a, key)
    vb, mb = decode_states(sec_b, key)
    label = "sections[Y=%d].%s" % (y, key)
    if ma == "empty" and mb == "empty":
        return True
    if ma == "invalid" or mb == "invalid" or va is None or vb is None:
        rep.note_divergence("sections", "%s undecodable payload (mode A=%s B=%s)" % (label, ma, mb))
        return False
    if ma == "raw" or mb == "raw":
        if ma != mb:
            rep.note_divergence("sections", "%s palette mode A=%s B=%s" % (label, ma, mb))
            return False
        i = _first_index(va, vb)
        if i is None and len(va) == len(vb):
            return True
        rep.note_divergence("sections", "%s raw index differs first at %d" % (label, i))
        return False
    i = _first_index(va, vb)
    if i is None and len(va) == len(vb):
        return True
    loc = _state_coords(key, i)
    sa = va[i] if i < len(va) else "<eof>"
    sb = vb[i] if i < len(vb) else "<eof>"
    rep.note_divergence("sections", "%s first divergence at %s — %s != %s" % (label, loc, sa, sb))
    return False


def compare_sections(a, b, rep):
    """Sections compared keyed by Y; first divergent block/biome value reported."""
    def by_y(sections):
        out = {}
        for idx, sec in enumerate(sections):
            if isinstance(sec, TComp) and "Y" in sec.d:
                out[sec.d["Y"]] = sec
            else:
                out["#%d" % idx] = sec
        return out

    ya, yb = by_y(a.items), by_y(b.items)
    ok = True
    for y in sorted(set(ya) - set(yb), key=str):
        rep.note_divergence("sections", "Y=%s present only in %s" % (y, "A"))
        ok = False
    for y in sorted(set(yb) - set(ya), key=str):
        rep.note_divergence("sections", "Y=%s present only in %s" % (y, "B"))
        ok = False
    for y in sorted(set(ya) & set(yb), key=str):
        sa, sb = ya[y], yb[y]
        # block states first, then biomes, then everything else structurally
        if not compare_states(sa, sb, "block_states", y, rep, 4096):
            ok = False
        if not compare_states(sa, sb, "biomes", y, rep, 64):
            ok = False
        for k in sorted(set(sa.d) | set(sb.d)):
            if k in ("block_states", "biomes", "Y"):
                continue
            if k not in sa.d or k not in sb.d:
                rep.note_divergence("sections", "Y=%s key %s missing in %s"
                                    % (y, k, "A" if k not in sa.d else "B"))
                ok = False
                continue
            if not deep_compare(sa.d[k], sb.d[k], rep, "sections", "Y=%s/%s/" % (y, k)):
                ok = False
    return ok


def compare_heightmaps(a, b, rep):
    """Heightmaps: compound of long arrays -> decode 9-bit values, compare."""
    ok = True
    for k in sorted(set(a.d) | set(b.d)):
        if k not in a.d or k not in b.d:
            rep.note_divergence("heightmaps", "%s present only in %s" % (k, "A" if k not in a.d else "B"))
            ok = False
            continue
        va, vb = a.d[k], b.d[k]
        if not (isinstance(va, TArr) and isinstance(vb, TArr) and va.kind == "l" and vb.kind == "l"):
            if not deep_compare(va, vb, rep, "heightmaps", "%s/" % k):
                ok = False
            continue
        da, db = decode_heightmap(va.vals), decode_heightmap(vb.vals)
        i = _first_index(da, db)
        if i is None and len(da) == len(db):
            continue
        loc = "i=%d x=%d z=%d" % (i, i & 15, i >> 4)
        rep.note_divergence("heightmaps", "%s first divergence at %s — %s != %s"
                            % (k, loc, da[i] if i is not None and i < len(da) else "<eof>",
                               db[i] if i is not None and i < len(db) else "<eof>"))
        ok = False
    return ok


def compare_chunks(a, b):
    """Full semantic comparison of two parsed chunk roots. Returns Report."""
    rep = Report("", "")
    if not (isinstance(a, TComp) and isinstance(b, TComp)):
        rep.note_divergence("<root>", "root is not a compound")
        return rep
    for k in sorted(set(a.d) | set(b.d)):
        if k not in a.d or k not in b.d:
            rep.note_divergence(k, "present only in %s" % ("A" if k not in a.d else "B"))
            continue
        if k == "sections" and isinstance(a.d[k], TList) and isinstance(b.d[k], TList):
            if not compare_sections(a.d[k], b.d[k], rep):
                pass  # divergences already recorded
            continue
        if k == "heightmaps" and isinstance(a.d[k], TComp) and isinstance(b.d[k], TComp):
            if not compare_heightmaps(a.d[k], b.d[k], rep):
                pass
            continue
        if k in BYTE_ONLY_FIELDS and isinstance(a.d[k], (int, float)) \
                and isinstance(b.d[k], (int, float)):
            # byte diff with no semantic effect (e.g. InhabitedTime save-to-save)
            if not _scalar_eq(a.d[k], b.d[k]):
                rep.note_byte_only(k, a.d[k], b.d[k])
            continue
        deep_compare(a.d[k], b.d[k], rep, k, "")
    return rep


def classify(path_a, path_b):
    """Compare two files -> (verdict, report). Raises OSError/NBTError on IO. """
    with open(path_a, "rb") as f:
        raw_a = f.read()
    with open(path_b, "rb") as f:
        raw_b = f.read()
    if raw_a == raw_b:
        return (VERDICT_EQUAL, Report(path_a, path_b))
    ta, tb = parse_nbt_bytes(raw_a), parse_nbt_bytes(raw_b)
    rep = compare_chunks(ta, tb)
    rep.path_a, rep.path_b = path_a, path_b
    if rep.diverged_keys:
        return (VERDICT_DIVERGED, rep)
    return (VERDICT_SEMANTIC, rep)


# ---------------------------------------------------------------------------
# CLI modes
# ---------------------------------------------------------------------------

def run_pair(args):
    try:
        verdict, rep = classify(args.a, args.b)
    except (OSError, NBTError, zlib.error, struct.error) as e:
        print("ncfdiff: %s" % e, file=sys.stderr)
        return 2
    print(verdict)
    print(rep.render())
    if args.report:
        with open(args.report, "w", encoding="utf-8") as f:
            f.write("ncfdiff %s %s\nverdict: %s\n\n%s\n"
                    % (args.a, args.b, verdict, rep.render()))
    return 0 if verdict != VERDICT_DIVERGED else 1


def find_dump_files(root):
    out = {}
    for base, _dirs, files in os.walk(root):
        for f in files:
            if f.startswith("c_") and f.endswith(".nbt"):
                rel = os.path.relpath(os.path.join(base, f), root)
                out[rel.replace(os.sep, "/")] = os.path.join(base, f)
    return out


def run_manifest(args):
    try:
        da, db = find_dump_files(args.a), find_dump_files(args.b)
    except OSError as e:
        print("ncfdiff: %s" % e, file=sys.stderr)
        return 2
    lines = []
    counts = {"EQUAL": 0, VERDICT_SEMANTIC: 0, VERDICT_DIVERGED: 0, "MISSING": 0}
    for rel in sorted(set(da) | set(db)):
        if rel not in da:
            lines.append("%s MISSING_IN_A" % rel)
            counts["MISSING"] += 1
            continue
        if rel not in db:
            lines.append("%s MISSING_IN_B" % rel)
            counts["MISSING"] += 1
            continue
        try:
            verdict, rep = classify(da[rel], db[rel])
        except (OSError, NBTError, zlib.error, struct.error) as e:
            lines.append("%s ERROR %s" % (rel, e))
            counts[VERDICT_DIVERGED] += 1
            continue
        counts[verdict] += 1
        lines.append("%s %s" % (rel, verdict))
        if verdict == VERDICT_DIVERGED:
            for d in rep.details[:8]:
                lines.append("    " + d)
    summary = "pairs=%d equal=%d semantic_equal=%d diverged=%d missing=%d" % (
        counts["EQUAL"] + counts[VERDICT_SEMANTIC] + counts[VERDICT_DIVERGED] + counts["MISSING"],
        counts["EQUAL"], counts[VERDICT_SEMANTIC], counts[VERDICT_DIVERGED], counts["MISSING"])
    print(summary)
    for line in lines:
        print(line)
    if args.report:
        with open(args.report, "w", encoding="utf-8") as f:
            f.write("ncfdiff --manifest %s %s\n%s\n\n" % (args.a, args.b, summary))
            f.write("\n".join(lines) + "\n")
    return 0 if (counts[VERDICT_DIVERGED] == 0 and counts["MISSING"] == 0) else 1


# ---------------------------------------------------------------------------
# Selftest: synthetic blobs, three verdicts, no server needed
# ---------------------------------------------------------------------------

def _pack(values, bits):
    """Inverse of _unpack: pack values into signed longs (no crossing)."""
    per_long = 64 // bits
    mask = (1 << bits) - 1
    n_longs = (len(values) + per_long - 1) // per_long
    longs = []
    for li in range(n_longs):
        acc = 0
        for j in range(per_long):
            idx = li * per_long + j
            v = values[idx] if idx < len(values) else 0
            acc |= (v & mask) << (j * bits)
        if acc >= 1 << 63:
            acc -= 1 << 64
        longs.append(acc)
    return longs


def _synth_chunk(inhabited, blocks_4096, hm=None, biome_idx=None):
    """Minimal FULL chunk NBT with one section (Y=4). blocks_4096: palette indices.

    biome_idx: None -> single-entry biome palette (no data array); otherwise a
    64-entry quart index list decoded against a two-entry biome palette.
    """
    def s(v):
        return (TAG_STRING, v)

    def c(d):
        return (TAG_COMPOUND, TComp(d))

    if biome_idx is None:
        biomes = (TAG_COMPOUND, TComp({
            "palette": (TAG_LIST, TList(TAG_STRING, ["minecraft:plains"])),
        }))
    else:
        biomes = (TAG_COMPOUND, TComp({
            "palette": (TAG_LIST, TList(TAG_STRING,
                                        ["minecraft:plains", "minecraft:desert"])),
            "data": (TAG_LONG_ARRAY, TArr("l", _pack(biome_idx, 1))),
        }))
    sec = TComp({
        "Y": (TAG_BYTE, 4),
        "block_states": (TAG_COMPOUND, TComp({
            "palette": (TAG_LIST, TList(TAG_COMPOUND, [
                TComp({"Name": (TAG_STRING, "minecraft:air")}),
                TComp({"Name": (TAG_STRING, "minecraft:stone"),
                       "Properties": (TAG_COMPOUND, TComp({}))}),
                TComp({"Name": (TAG_STRING, "minecraft:dirt")}),
            ])),
            "data": (TAG_LONG_ARRAY, TArr("l", _pack(blocks_4096, 4))),
        })),
        "biomes": biomes,
        "SkyLight": (TAG_BYTE_ARRAY, TArr("b", [15] * 8)),
        "BlockLight": (TAG_BYTE_ARRAY, TArr("b", [0] * 8)),
    })
    heightmaps = TComp({
        "MOTION_BLOCKING": (TAG_LONG_ARRAY,
                            TArr("l", _pack(hm if hm is not None else [70] * 256, HEIGHTMAP_BITS))),
    })
    root = TComp({
        "DataVersion": (TAG_INT, 4556),
        "Status": (TAG_STRING, "full"),
        "xPos": (TAG_INT, -3),
        "zPos": (TAG_INT, 12),
        "yPos": (TAG_INT, -4),
        "InhabitedTime": (TAG_LONG, inhabited),
        "LastUpdate": (TAG_LONG, 1000),
        "isLightOn": (TAG_BYTE, 1),
        "heightmaps": (TAG_COMPOUND, heightmaps),
        "sections": (TAG_LIST, TList(TAG_COMPOUND, [sec])),
        "block_entities": (TAG_LIST, TList(TAG_COMPOUND, [])),
        "block_ticks": (TAG_LIST, TList(TAG_COMPOUND, [])),
        "fluid_ticks": (TAG_LIST, TList(TAG_COMPOUND, [])),
        "PostProcessing": (TAG_LIST, TList(TAG_LIST, [])),
        "structure": (TAG_COMPOUND, TComp({
            "starts": (TAG_COMPOUND, TComp({})),
            "References": (TAG_COMPOUND, TComp({})),
        })),
    })
    return build_nbt(root)


def run_selftest():
    blocks = [0] * 4096
    for i in (0, 1, 2, 1000, 4095):
        blocks[i] = 1  # minecraft:stone
    base = _synth_chunk(inhabited=0, blocks_4096=blocks)

    # 1) byte-identical pair -> EQUAL
    v1, _ = classify_bytes(base, base)
    assert v1 == VERDICT_EQUAL, "selftest: expected EQUAL, got %s" % v1

    # 2) InhabitedTime differs -> SEMANTIC_EQUAL (bytes differ)
    other_time = _synth_chunk(inhabited=777, blocks_4096=blocks)
    v2, rep2 = classify_bytes(base, other_time)
    assert v2 == VERDICT_SEMANTIC, "selftest: expected SEMANTIC_EQUAL, got %s (%s)" % (v2, rep2.render())

    # 3) one block value differs -> DIVERGED with first-divergence coordinates
    blocks_b = list(blocks)
    blocks_b[1000] = 2  # minecraft:dirt
    diverged = _synth_chunk(inhabited=0, blocks_4096=blocks_b)
    v3, rep3 = classify_bytes(base, diverged)
    assert v3 == VERDICT_DIVERGED, "selftest: expected DIVERGED, got %s" % v3
    assert "sections" in rep3.diverged_keys, rep3.render()
    # index 1000 with yzx layout (i = y*256 + z*16 + x):
    # x = 1000 & 15 = 8, z = (1000 >> 4) & 15 = 14, y = 1000 >> 8 = 3
    assert "x=8 z=14 y=3" in rep3.render(), rep3.render()
    assert "minecraft:stone" in rep3.render() and "minecraft:dirt" in rep3.render(), rep3.render()

    # 4) heightmap divergence -> DIVERGED with (x,z) location
    hm = [70] * 256
    hm[37] = 71  # i=37 -> x=5, z=2 (index = x + 16*z per VERIFY note)
    hm_chunk = _synth_chunk(inhabited=0, blocks_4096=blocks, hm=hm)
    v4, rep4 = classify_bytes(base, hm_chunk)
    assert v4 == VERDICT_DIVERGED, "selftest: expected DIVERGED, got %s" % v4
    assert "MOTION_BLOCKING" in rep4.render() and "x=5 z=2" in rep4.render(), rep4.render()

    # 5) heightmap decoder sanity: 256 entries, 9 bits -> 37 longs, values round-trip
    vals = decode_heightmap(_pack(hm, HEIGHTMAP_BITS))
    assert len(vals) == 256 and vals[37] == 71 and vals[0] == 70, vals[:40]

    # 6) biome divergence -> DIVERGED with quart coordinates (x=i&3, z=(i>>2)&3, y=i>>4)
    biomes_a = [0] * 64
    biomes_b = [0] * 64
    biomes_b[5] = 1  # i=5 -> quart x=1 z=1 y=0 -> minecraft:desert
    base_b = _synth_chunk(inhabited=0, blocks_4096=blocks, biome_idx=biomes_a)
    diverged_b = _synth_chunk(inhabited=0, blocks_4096=blocks, biome_idx=biomes_b)
    v6, rep6 = classify_bytes(base_b, diverged_b)
    assert v6 == VERDICT_DIVERGED, "selftest: expected DIVERGED, got %s" % v6
    assert "biomes" in rep6.render() and "x=1 z=1 y=0 (quart)" in rep6.render(), rep6.render()

    print("selftest OK (EQUAL, SEMANTIC_EQUAL, DIVERGED, block/biome/heightmap decode)")
    return 0


def classify_bytes(raw_a, raw_b):
    if raw_a == raw_b:
        return (VERDICT_EQUAL, Report("<mem A>", "<mem B>"))
    ta, tb = parse_nbt_bytes(raw_a), parse_nbt_bytes(raw_b)
    rep = compare_chunks(ta, tb)
    rep.path_a, rep.path_b = "<mem A>", "<mem B>"
    if rep.diverged_keys:
        return (VERDICT_DIVERGED, rep)
    return (VERDICT_SEMANTIC, rep)


def main(argv):
    ap = argparse.ArgumentParser(
        prog="ncfdiff", description="semantic NBT diff for golden chunk dumps (NCF P0.6)")
    ap.add_argument("a", nargs="?", help="file A (or dirA with --manifest)")
    ap.add_argument("b", nargs="?", help="file B (or dirB with --manifest)")
    ap.add_argument("--manifest", action="store_true",
                    help="manifest mode: compare all c_*.nbt pairs under dirA and dirB")
    ap.add_argument("--report", help="write the full diff report to FILE")
    ap.add_argument("--selftest", action="store_true",
                    help="build synthetic NBT pairs in memory and assert the three verdicts")
    args = ap.parse_args(argv)

    if args.selftest:
        return run_selftest()
    if args.manifest:
        if not args.a or not args.b:
            ap.error("--manifest requires dirA and dirB")
        return run_manifest(args)
    if not args.a or not args.b:
        ap.error("usage: ncfdiff a.nbt b.nbt [--report FILE] | --manifest dirA dirB | --selftest")
    return run_pair(args)


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv[1:]))
    except BrokenPipeError:
        sys.exit(2)
    except (OSError, NBTError) as e:
        print("ncfdiff: %s" % e, file=sys.stderr)
        sys.exit(2)
