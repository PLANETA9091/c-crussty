#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
world_diff_parity_v2.py — world-diff bit-parity АРБИТР v2 (ROUND-468 / S50).
Развитие S74_world_diff_parity.py (ROUND-467, C2ME-методика tests/world-diff).

ЧТО НОВОГО В v2 (суперсет v1, 0 новых зависимостей — только stdlib):
  1. ЧЕК-СУММЫ ЧАНКОВ (seed-идентичные):
     • world-режим: per-chunk sha256 канонического серила (x,z,status,sectionY,palette,data)
       -> счётчики chunks_checksum_equal (seed-идентичные чанки) / checksum_diff top-N;
     • log-режим: seed-идентичность ландшафта = равенство серий spawnable-chunk polls
       (mobcaps header) + forceload chunks/cmds обеих сторон (гейт CHUNK-CHECKSUMS).
  2. PER-TYPE POPULATION ДИФФ:
     • log-режим: per-type из «top entity types (max seen)» md-среза + raw mobcaps
       per-type строки — таблица дрейфов, гейт PER-TYPE-POPULATION
       band |a-b| <= max(floor, tol*max) (канон parity_validator: tol=0.05, floor=64);
     • world-режим (опция --entities): per-type из entities/-region (NBT «id» каждого
       entity) — те же банды; block-entities не числа (не population).
  3. TICK-BEHIND:
     • log-режим: гейт TICK-BEHIND — канон 0 на обеих сторонах; считаются строки
       «Can't keep up!» / «Running Nms or M ticks behind» (raw, суммарно M тиков)
       и «tick-behind warnings in log: N» (report, tier ниже raw);
     • world-режим: info-прокси Δticks по level.dat Time обеих сторон
       (region/ ../level.dat); --strict-time превращает Δ!=0 в FAIL.
  4. ИНТЕГРАЦИЯ В MERGE-ГЕЙТ: НОВАЯ ФАЗА P6-PARITY gate_v2 (scripts/merge_safety_gate_v2.sh,
     ветка round-467-s36-gatev2 @fc5df459, фазы P1 cargo / P2 blobs / P3 case_arm /
     P4 ncdfe / P5 marker) — вставка МЕЖДУ P5 и ВЕРДИКТ, SKIP-able (артефакты пары
     есть только при вердикте; в fast-режиме SKIP). Фаза зовёт:
       PAIR_VAN="$VAN_LOGS" PAIR_LEG="$LEG_LOGS" \
         python3 scripts/world_diff_parity_v2.py --gate2 --env-pair \
           --json "$LOG_DIR/p6_parity.json"
     exit 0 = OK/SKIP, 1 = FAIL (phase_fail), 3 = SKIP.
     Также совместимо с parity_validator.py (round-466-c85-parity): log-режим —
     надмножество его 8 гейтов (W1..W9), JSON-схема унаследована.

РЕЖИМЫ:
  world : world_diff_parity_v2.py regionA regionB [--out MD] [--json J] [--entities]
          [--gate-chunks F] [--gate-blocks F] [--top N] [--no-status]
          [--strict-time] [--dump-checksums FILE]
  logs  : world_diff_parity_v2.py van.log... -- leg.log... [--entity-tolerance 0.05]
          [--entity-abs-floor 64] [--strict-entities] [--ignore-world] [--quiet]
          [--json J] [--gate2] [--out MD]
  авто: оба аргумента-каталога = world; иначе logs. Самотест: --self-test (герметичный,
  12/12: синтетический Anvil + синтетическая пара логов + негатив-мутации всех гейтов).

C2ME-канон строки: "%d/%d chunks, %d block differences (%.4f%%)".
Гейт world: на ваниль-парах blocks-diff == 0.0000% (bit-mismatch на leg-паре = блок
мерджа, закон 20d тяжёлый стенд). Гейт logs: WORLD-PARITY-OK при 0 FAIL.

Пример (пара тика, S50):
  python3 world_diff_parity_v2.py pair/van/0_world-bench.txt -- pair/leg/0_world-bench.txt
"""

from __future__ import annotations

import gzip
import hashlib
import json
import os
import re
import statistics
import struct
import sys
import tempfile
import time
import zlib
from collections import Counter
from typing import Dict, List, Optional, Tuple

AIR = "minecraft:air"
TOP_DEFAULT = 10
U64 = 0xFFFFFFFFFFFFFFFF

# паритет-банды per-type (канон parity_validator.py round-466-c85-parity)
ENTITY_TOLERANCE = 0.05
ENTITY_ABS_FLOOR = 64
CPU_BAND = (6_000_000, 9_500_000)  # канон band 6.0-9.5M


class RegionError(Exception):
    pass


# ----------------------------------------------------------------------------
# минимальный NBT-ридер (Java Edition, payload уже декомпрессирован) — v1
# ----------------------------------------------------------------------------

def _read_name(buf: bytes, off: int) -> Tuple[str, int]:
    (n,) = struct.unpack_from(">H", buf, off)
    off += 2
    return buf[off:off + n].decode("utf-8", "replace"), off + n


def _read_payload(tag: int, buf: bytes, off: int):
    if tag == 1:
        return struct.unpack_from(">b", buf, off)[0], off + 1
    if tag == 2:
        return struct.unpack_from(">h", buf, off)[0], off + 2
    if tag == 3:
        return struct.unpack_from(">i", buf, off)[0], off + 4
    if tag == 4:
        return struct.unpack_from(">q", buf, off)[0], off + 8
    if tag == 5:
        return struct.unpack_from(">f", buf, off)[0], off + 4
    if tag == 6:
        return struct.unpack_from(">d", buf, off)[0], off + 8
    if tag == 7:
        (n,) = struct.unpack_from(">i", buf, off)
        off += 4
        return bytes(buf[off:off + n]), off + n
    if tag == 8:
        return _read_name(buf, off)
    if tag == 9:
        et = buf[off]
        (n,) = struct.unpack_from(">i", buf, off + 1)
        off += 5
        out = []
        for _ in range(max(0, n)):
            v, off = _read_payload(et, buf, off)
            out.append(v)
        return out, off
    if tag == 10:
        out: dict = {}
        while True:
            et = buf[off]
            if et == 0:
                return out, off + 1
            name, off = _read_name(buf, off + 1)
            out[name], off = _read_payload(et, buf, off)
    if tag == 11:
        (n,) = struct.unpack_from(">i", buf, off)
        n = max(0, n)
        return list(struct.unpack_from(f">{n}i", buf, off + 4)), off + 4 + 4 * n
    if tag == 12:
        (n,) = struct.unpack_from(">i", buf, off)
        n = max(0, n)
        return list(struct.unpack_from(f">{n}q", buf, off + 4)), off + 4 + 8 * n
    raise RegionError(f"unsupported NBT tag {tag}")


def read_nbt(buf: bytes, off: int = 0) -> dict:
    """Root TAG_Compound -> dict."""
    if off >= len(buf) or buf[off] != 10:
        raise RegionError(f"root tag {buf[off] if off < len(buf) else 'EOF'} != TAG_Compound")
    _, off = _read_name(buf, off + 1)
    val, _ = _read_payload(10, buf, off)
    return val


# ----------------------------------------------------------------------------
# Anvil region reader — v1
# ----------------------------------------------------------------------------

COMP_GZIP, COMP_ZLIB, COMP_RAW = 1, 2, 3


def iter_region_chunks(region_path: str):
    """Yield (cx, cz, root) для всех живых чанков одного r.X.Z.mca."""
    with open(region_path, "rb") as f:
        raw = f.read()
    if len(raw) < 8192:
        return
    m = os.path.basename(region_path)
    parts = m.split(".")
    try:
        rx, rz = int(parts[1]), int(parts[2])
    except (IndexError, ValueError):
        rx = rz = 0
    for i in range(1024):
        o = i * 4
        off_sectors = (raw[o] << 16) | (raw[o + 1] << 8) | raw[o + 2]
        cnt = raw[o + 3]
        if off_sectors == 0 or cnt == 0:
            continue
        start = off_sectors * 4096
        if start + 5 > len(raw):
            raise RegionError(f"{m}: chunk idx {i} вне файла")
        (plen,) = struct.unpack_from(">I", raw, start)
        comp = raw[start + 4]
        payload = raw[start + 5:start + 4 + plen]
        if comp == COMP_GZIP:
            payload = gzip.decompress(payload)
        elif comp == COMP_ZLIB:
            payload = zlib.decompress(payload)
        elif comp != COMP_RAW:
            raise RegionError(f"{m}: compression={comp} (LZ4/Zstd 1.21.5+) не поддержан stdlib — пересохрани zlib")
        root = read_nbt(payload, 0)
        cx = int(root.get("xPos", rx * 32 + (i % 32)))
        cz = int(root.get("zPos", rz * 32 + (i // 32)))
        yield cx, cz, root


def scan_region_dir(dir_path: str) -> Dict[Tuple[int, int], dict]:
    """Каталог region/ -> {(cx,cz): root}. Каталог без *.mca = RegionError."""
    if not os.path.isdir(dir_path):
        raise RegionError(f"нет каталога region: {dir_path}")
    out: Dict[Tuple[int, int], dict] = {}
    files = sorted(fn for fn in os.listdir(dir_path) if fn.endswith(".mca"))
    if not files:
        raise RegionError(f"0 *.mca в {dir_path}")
    for fn in files:
        for cx, cz, root in iter_region_chunks(os.path.join(dir_path, fn)):
            out[(cx, cz)] = root
    return out


# ----------------------------------------------------------------------------
# block_states -> (serialised palette, packed data) — v1
# ----------------------------------------------------------------------------

def palette_str(entry) -> str:
    if isinstance(entry, str):
        return entry
    if not isinstance(entry, dict):
        return str(entry)
    name = entry.get("Name", "?")
    props = entry.get("Properties")
    if isinstance(props, dict) and props:
        kv = ",".join(f"{k}={props[k]}" for k in sorted(props))
        return f"{name}[{kv}]"
    return name


def section_blocks(sec: dict) -> Tuple[List[str], Optional[List[int]], int]:
    """Секция -> (palette strings, data longs | None, Y). Нет block_states = весь air."""
    y = int(sec.get("Y", 0))
    bs = sec.get("block_states")
    if not isinstance(bs, dict) or "palette" not in bs:
        return [AIR], None, y
    pal = [palette_str(e) for e in bs.get("palette") or []]
    data = bs.get("data")
    if isinstance(data, (bytes, bytearray)):
        data = list(struct.unpack(f">{len(data)}q", data))
    return pal, data, y


def unpack_indices(pal_len: int, data: Optional[List[int]], n: int = 4096) -> List[int]:
    """PalettedContainer unpack (блоки: min 4 bit, вход не пересекает long)."""
    if pal_len <= 1 or data is None:
        return [0] * n
    bits = max(4, (pal_len - 1).bit_length())
    epl = 64 // bits
    mask = (1 << bits) - 1
    out = [0] * n
    dl = len(data)
    for i in range(n):
        li, sh = divmod(i, epl)
        if li >= dl:
            break
        out[i] = ((int(data[li]) & U64) >> (sh * bits)) & mask
    return out


def chunk_sections(root: dict) -> Dict[int, Tuple[List[str], Optional[List[int]]]]:
    secs: Dict[int, Tuple[List[str], Optional[List[int]]]] = {}
    for sec in root.get("sections") or []:
        if isinstance(sec, dict):
            pal, data, y = section_blocks(sec)
            secs[y] = (pal, data)
    return secs


def chunk_status(root: dict) -> str:
    return str(root.get("Status", root.get("status", "?")))


def entity_type(root: dict) -> str:
    """entities/-NBT -> строка типа (id / identifier)."""
    eid = root.get("id") or root.get("identifier") or "?"
    return str(eid)


# ----------------------------------------------------------------------------
# v2: пер-тип population из entities/-региона + level.dat Time (tick-behind прокси)
# ----------------------------------------------------------------------------

def scan_entities_dir(dir_path: str) -> Counter:
    """Каталог entities/ -> Counter{type: count}. Нет каталога = пустой Counter."""
    out: Counter = Counter()
    if not os.path.isdir(dir_path):
        return out
    files = sorted(fn for fn in os.listdir(dir_path) if fn.endswith(".mca"))
    for fn in files:
        for _cx, _cz, root in iter_region_chunks(os.path.join(dir_path, fn)):
            ents = root.get("Entities") or []
            for e in ents if isinstance(ents, list) else []:
                if isinstance(e, dict):
                    out[entity_type(e)] += 1
    return out


def read_level_time(world_dir: str) -> Optional[int]:
    """level.dat -> Time (TAG_Long). None = нет/битый."""
    p = os.path.join(world_dir, "level.dat")
    if not os.path.isfile(p):
        return None
    try:
        with open(p, "rb") as f:
            payload = gzip.decompress(f.read())
        root = read_nbt(payload, 0)
        t = root.get("Time")
        return int(t) if t is not None else None
    except Exception:  # noqa: BLE001 — level.dat не обязателен
        return None


# ----------------------------------------------------------------------------
# diff (world-режим) — v1 + чек-суммы чанков + entities/Time
# ----------------------------------------------------------------------------

def _canon_digest(pal: List[str], data: Optional[List[int]]) -> str:
    h = hashlib.sha256()
    h.update(("|".join(pal)).encode())
    for l in (data or []):
        h.update(struct.pack(">q", l))
    return h.hexdigest()


def _side_digest(S: Dict[Tuple[int, int], dict]) -> str:
    h = hashlib.sha256()
    for key in sorted(S):
        h.update(f"{key[0]},{key[1]},{chunk_status(S[key])};".encode())
        secs = chunk_sections(S[key])
        for y in sorted(secs):
            h.update(f"{y}:{_canon_digest(*secs[y])};".encode())
    return h.hexdigest()


def diff_worlds(a_dir: str, b_dir: str, compare_status: bool = True,
                with_entities: bool = False) -> dict:
    A = scan_region_dir(a_dir)
    B = scan_region_dir(b_dir)
    keys = sorted(set(A) | set(B))
    chunks_total = len(keys)
    chunks_mismatched = 0
    blocks_compared = 0
    blocks_diff = 0
    pair_counter: Counter = Counter()      # (idA -> idB) -> блоки
    per_id_total: Counter = Counter()      # нетто-дельта счётчика ID в зонах расхождения
    per_chunk_digest: Dict[Tuple[int, int], str] = {}

    # v2: чек-суммы чанков (seed-идентичные)
    checks_a: Dict[Tuple[int, int], str] = {}
    checks_b: Dict[Tuple[int, int], str] = {}

    for key in keys:
        ra, rb = A.get(key), B.get(key)
        if ra is None or rb is None:
            chunks_mismatched += 1
            side = "missing_in_B" if rb is None else "missing_in_A"
            pair_counter[(f"__chunk_{side}__", f"{key[0]},{key[1]}")] += 1
            continue
        sa, sb = chunk_sections(ra), chunk_sections(rb)
        if compare_status and chunk_status(ra) != chunk_status(rb):
            chunks_mismatched += 1
            pair_counter[(f"__status:{chunk_status(ra)}__", f"__status:{chunk_status(rb)}__")] += 1
            continue
        chunk_bad = False
        sec_lines = []
        for y in sorted(set(sa) | set(sb)):
            if y not in sa or y not in sb:
                chunk_bad = True
                pair_counter[("__section_missing__" if y not in sa else "__section_extra__", f"y={y}")] += 1
                continue
            pal_a, dat_a = sa[y]
            pal_b, dat_b = sb[y]
            sec_lines.append(f"{y}:{_canon_digest(pal_a, dat_a)}:{_canon_digest(pal_b, dat_b)}")
            if pal_a == pal_b and dat_a == dat_b:
                blocks_compared += 4096      # fast path: бит-в-байт
                continue
            idx_a = unpack_indices(len(pal_a), dat_a)
            idx_b = unpack_indices(len(pal_b), dat_b)
            for j in range(4096):
                blocks_compared += 1
                ida = pal_a[idx_a[j]] if idx_a[j] < len(pal_a) else AIR
                idb = pal_b[idx_b[j]] if idx_b[j] < len(pal_b) else AIR
                if ida != idb:
                    blocks_diff += 1
                    pair_counter[(ida, idb)] += 1
                    per_id_total[ida] -= 1
                    per_id_total[idb] += 1
                    chunk_bad = True
        if chunk_bad:
            chunks_mismatched += 1
        per_chunk_digest[key] = hashlib.sha256("".join(sec_lines).encode()).hexdigest()
        checks_a[key] = hashlib.sha256("".join(
            f"{y}:{_canon_digest(*sa[y])}" for y in sorted(sa)).encode()).hexdigest()
        checks_b[key] = hashlib.sha256("".join(
            f"{y}:{_canon_digest(*sb[y])}" for y in sorted(sb)).encode()).hexdigest()

    shared = set(checks_a) & set(checks_b)
    checksum_equal = sum(1 for k in shared if checks_a[k] == checks_b[k])
    checksum_diff = [{"chunk": f"{k[0]},{k[1]}", "sha256_a": checks_a[k][:12],
                      "sha256_b": checks_b[k][:12]}
                     for k in sorted(shared) if checks_a[k] != checks_b[k]]

    top = [{"from": k[0], "to": k[1], "count": v} for k, v in pair_counter.most_common(TOP_DEFAULT)]
    per_id = [{"id": k, "delta": v} for k, v in per_id_total.most_common(TOP_DEFAULT)]
    pct = (blocks_diff / blocks_compared * 100.0) if blocks_compared else 0.0
    cpct = (chunks_mismatched / chunks_total * 100.0) if chunks_total else 0.0

    # v2: entities per-type (опция) + level.dat Time (tick-behind прокси)
    ents = None
    if with_entities:
        ea = scan_entities_dir(os.path.join(os.path.dirname(a_dir.rstrip("/")) or ".", "entities"))
        eb = scan_entities_dir(os.path.join(os.path.dirname(b_dir.rstrip("/")) or ".", "entities"))
        ents = entity_band_report(ea, eb, ENTITY_TOLERANCE, ENTITY_ABS_FLOOR)
    wa = os.path.dirname(a_dir.rstrip("/")) or "."
    wb = os.path.dirname(b_dir.rstrip("/")) or "."
    time_a, time_b = read_level_time(wa), read_level_time(wb)

    return {
        "world_digest_a": _side_digest(A), "world_digest_b": _side_digest(B),
        "world_digest_equal": _side_digest(A) == _side_digest(B),
        "chunks_total": chunks_total, "chunks_mismatched": chunks_mismatched,
        "chunks_diff_pct": round(cpct, 6),
        "blocks_compared": blocks_compared, "blocks_diff": blocks_diff,
        "blocks_diff_pct": round(pct, 6),
        # v2 чек-суммы чанков (seed-идентичные)
        "chunks_checksum_shared": len(shared),
        "chunks_checksum_equal": checksum_equal,
        "chunks_checksum_diff": checksum_diff[:TOP_DEFAULT],
        "chunk_checksums_a": checks_a, "chunk_checksums_b": checks_b,
        "top_id_divergences": top, "per_id_total_delta": per_id,
        "entities_band": ents,
        "level_time_a": time_a, "level_time_b": time_b,
        "tick_delta": (abs(time_a - time_b) if (time_a is not None and time_b is not None) else None),
        "pair_digest": hashlib.sha256(
            "".join(per_chunk_digest[k] for k in sorted(per_chunk_digest)).encode()).hexdigest(),
    }


# ----------------------------------------------------------------------------
# v2: log-режим — факты стороны (регексы parity_validator + run-env-эхо CI-лога)
# ----------------------------------------------------------------------------

RE_LOG_TS = re.compile(r"^\[\d{2}:\d{2}:\d{2}\s+(INFO|WARN|ERROR)\]", re.M)
RE_INJECT_START = re.compile(
    r"POPULATION INJECT START target=(?P<target>\d+) seed=(?P<seed>\d+)"
    r"(?: loadedChunks=(?P<loaded>\d+))?"
    r"(?: plan\(items=(?P<items>\d+), hostiles=(?P<hostiles>\d+), passives=(?P<passives>\d+)\))?"
)
RE_INJECT_DONE = re.compile(
    r"POPULATION INJECT DONE target=(?P<target>\d+) injected=(?P<injected>\d+)"
    r" items=(?P<items>\d+) hostiles=(?P<hostiles>\d+) passives=(?P<passives>\d+)"
)
RE_FIXTURE = re.compile(r"FIXTURE-VALIDITY\s*[:=]\s*\**\s*(?P<v>VALID|INVALID)")
RE_FORCELOAD_MARK = re.compile(
    r"Marked (?P<chunks>\d+) chunks in (?P<dim>[\w:.\-]+) from \[[^\]]*\] to \[[^\]]*\] to be force loaded"
)
RE_TICK_BEHIND = re.compile(r"Can't keep up!|Running \d+ms or \d+ ticks behind|running behind")
RE_TICKS_BEHIND_N = re.compile(r"Running \d+ms or (?P<ticks>\d+) ticks behind")
RE_TYPE_LINE = re.compile(
    r"^(?:\[[\d:]+\s+\w+\]:\s*)?\s*(?P<cnt>\d+) \(\d+\) : (?P<type>[\w:.\-]+)\s*$", re.M)
RE_FL_MD = re.compile(
    r"forceload commands issued:\s*(?P<cmds>\d+)\s*\((?P<chunks>\d+) chunks force-loaded\)")
RE_TOTALS_MD = re.compile(r"entity totals seen:\s*\[(?P<vals>[^\]]*)\]")
RE_TYPES_MD = re.compile(r"top entity types \(max seen\):\s*(?P<body>[^\n]+)")
RE_TYPES_MD_PAIR = re.compile(r"(?P<type>[\w:.\-]+)×(?P<cnt>\d+)")
RE_TB_MD = re.compile(r"tick-behind warnings in log:\s*(?P<n>\d+)")
RE_GATE_MD = re.compile(r"gate (?P<id>1[abc]) [^:]*: (?P<res>PASS|FAIL)")
RE_SPAWN_POLLS_MD = re.compile(r"spawnable-chunk polls[^:]*:\s*\[(?P<vals>[^\]]*)\]")
RE_HEARTBEAT_MD = re.compile(r"alive-check heartbeat series:\s*\[(?P<vals>[^\]]*)\]")
RE_BOOT_MD = re.compile(r"boot reached Done:\s*\**(?P<done>\d+)\**")
RE_CHURN_MD = re.compile(r"gate 1b [^:]*: (?:PASS|FAIL) \(summons=\d+, polls=\d+, delta=(?P<delta>\d+)\)")
# опциональный CI-таймстамп-префикс строки ("2026-09-26T13:09:11.9460189Z ")
_TS = r"(?:[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9:.]+Z\s+)?"
RE_WORLD_SHA_MD = re.compile(r"^" + _TS + r"(?:[-*]\s*)?world_sha256:\s*(?P<v>[0-9a-fA-F]{8,64})", re.M)
RE_CPU_MD = re.compile(r"^" + _TS + r"(?:[-*]\s*)?runner_cpu_index:\s*(?P<v>\d+)", re.M)
RE_ENV_KV = re.compile(r"^" + _TS + r"(?:[-*]\s*)?(?P<key>[a-z_0-9]+):\s*(?P<val>[^\n(]*?)(?:\s*\(|\s*$)", re.M)
# run-env-эхо шага 9 CI-лога (одной строкой)
RE_RUNENV_ECHO = re.compile(
    r"run-env: world_sha256=(?P<sha>[0-9a-fA-F]{8,64})\s+"
    r"runner_cpu_index=(?P<cpu>\d+)\s+fake_players=(?P<fp>\d+)")
RE_TPS = re.compile(r"TPS polls captured: (\d+), first-of-window values: \[([^\]]*)\]")
RE_STW = re.compile(r"(\d+) pauses / total ([\d.]+) ms STW")

TIER = {"raw": 0, "report": 1, "env": 2}


class LogFacts:
    """Модель фактов одной стороны log-режима (аналог RunFacts parity_validator)."""

    def __init__(self):
        self.source = ""
        self.fixture_validity: Optional[str] = None
        self.fixture_gates: Dict[str, str] = {}
        self.fake_players: Optional[int] = None
        self.alive_players: List[int] = []
        self.spawnable_chunks: List[int] = []
        self.pop_seed: Optional[int] = None
        self.pop_target: Optional[int] = None
        self.pop_injected: Optional[int] = None
        self.pop_items: Optional[int] = None
        self.pop_hostiles: Optional[int] = None
        self.pop_passives: Optional[int] = None
        self.pop_plan: Optional[Tuple[int, int, int]] = None
        self.forceload_cmds: Optional[int] = None
        self.forceload_chunks: Optional[int] = None
        self.entity_totals: List[int] = []
        self.entity_types: Dict[str, int] = {}
        self.tick_behind: Optional[int] = None
        self.ticks_behind_sum: Optional[int] = None
        self.tb_report: Optional[int] = None   # md-срез «tick-behind warnings in log: N»
        self.world_sha256: Optional[str] = None
        self.runner_cpu_index: Optional[str] = None
        self.boot_done: Optional[int] = None
        self.churn_delta: Optional[int] = None
        self.tps_median: Optional[float] = None
        self.tps_n: Optional[int] = None
        self.stw_total: Optional[float] = None
        self.provenance: Dict[str, str] = {}

    def set(self, name, value, tier: str, fname: str) -> None:
        cur = self.provenance.get(name)
        cur_tier = TIER[cur.split(":", 1)[0]] if cur else 99
        if TIER[tier] < cur_tier:
            setattr(self, name, value)
            self.provenance[name] = f"{tier}:{os.path.basename(fname)}"


def _ints(csv: str) -> List[int]:
    return [int(x) for x in re.findall(r"-?\d+", csv)]


def parse_log_text(text: str, f: LogFacts, fname: str) -> None:
    m = RE_RUNENV_ECHO.search(text)
    if m:
        f.set("world_sha256", m.group("sha").lower(), "raw", fname)
        f.set("runner_cpu_index", m.group("cpu"), "raw", fname)
        f.set("fake_players", int(m.group("fp")), "raw", fname)
    m = RE_INJECT_START.search(text)
    if m:
        f.set("pop_seed", int(m.group("seed")), "raw", fname)
        f.set("pop_target", int(m.group("target")), "raw", fname)
        if m.group("items"):
            f.set("pop_plan", (int(m.group("items")), int(m.group("hostiles")),
                               int(m.group("passives"))), "raw", fname)
    m = RE_INJECT_DONE.search(text)
    if m:
        f.set("pop_target", int(m.group("target")), "raw", fname)
        f.set("pop_injected", int(m.group("injected")), "raw", fname)
        f.set("pop_items", int(m.group("items")), "raw", fname)
        f.set("pop_hostiles", int(m.group("hostiles")), "raw", fname)
        f.set("pop_passives", int(m.group("passives")), "raw", fname)
    m = RE_FIXTURE.search(text)
    if m:
        f.set("fixture_validity", m.group("v"), "raw", fname)
    for m in RE_GATE_MD.finditer(text):
        g, res = m.group("id"), m.group("res")
        if g not in f.fixture_gates or res == "FAIL":
            f.fixture_gates[g] = res
    for m in RE_CHURN_MD.finditer(text):
        f.churn_delta = int(m.group("delta"))
    fl_cmds = fl_chunks = 0
    for m in RE_FORCELOAD_MARK.finditer(text):
        fl_cmds += 1
        fl_chunks += int(m.group("chunks"))
    if fl_cmds:
        f.set("forceload_cmds", fl_cmds, "raw", fname)
        f.set("forceload_chunks", fl_chunks, "raw", fname)
    # tick-behind: строки-маркеры (raw) — два маркера в одной строке vanilla = 1 строка
    tb = sum(1 for ln in text.splitlines() if RE_TICK_BEHIND.search(ln))
    f.set("tick_behind", tb, "raw", fname)
    ticks = sum(int(m.group("ticks")) for m in RE_TICKS_BEHIND_N.finditer(text))
    if ticks:
        f.set("ticks_behind_sum", ticks, "raw", fname)
    # per-type из md-среза / mobcaps per-type raw
    m = RE_TYPES_MD.search(text)
    if m:
        md_types = {p.group("type"): int(p.group("cnt"))
                    for p in RE_TYPES_MD_PAIR.finditer(m.group("body"))}
        if md_types:
            merged = dict(md_types)
            for t, c in f.entity_types.items():
                merged[t] = max(c, md_types.get(t, 0))
            f.entity_types = merged
            f.provenance.setdefault("entity_types", f"raw:{os.path.basename(fname)}")
    m = RE_TOTALS_MD.search(text)
    if m:
        vals = _ints(m.group("vals"))
        if vals:
            f.set("entity_totals", vals, "report", fname)
    m = RE_TB_MD.search(text)
    if m:
        f.tb_report = int(m.group("n"))   # кросс-чек raw/md (гейт решает)
        f.set("tick_behind", int(m.group("n")), "report", fname)
    m = RE_SPAWN_POLLS_MD.search(text)
    if m and _ints(m.group("vals")):
        f.set("spawnable_chunks", _ints(m.group("vals")), "report", fname)
    m = RE_HEARTBEAT_MD.search(text)
    if m:
        for tok in re.findall(r"'?(\d+)/\d+'?", m.group("vals")):
            f.alive_players.append(int(tok))
    m = RE_FL_MD.search(text)
    if m:
        f.set("forceload_cmds", int(m.group("cmds")), "report", fname)
        f.set("forceload_chunks", int(m.group("chunks")), "report", fname)
    m = RE_BOOT_MD.search(text)
    if m:
        f.set("boot_done", int(m.group("done")), "report", fname)
    m = RE_WORLD_SHA_MD.search(text)
    if m:
        f.set("world_sha256", m.group("v").lower(), "report", fname)
    m = RE_CPU_MD.search(text)
    if m:
        f.set("runner_cpu_index", m.group("v"), "report", fname)
    for m in RE_ENV_KV.finditer(text):
        key, val = m.group("key"), m.group("val").strip()
        try:
            iv = int(val)
        except ValueError:
            continue
        if key == "population_target":
            f.set("pop_target", iv, "env", fname)
        elif key == "population_seed":
            f.set("pop_seed", iv, "env", fname)
        elif key == "fake_players":
            f.set("fake_players", iv, "env", fname)
    m = RE_TPS.search(text)
    if m:
        f.tps_n = int(m.group(1))
        vals = [float(x) for x in m.group(2).split(",")]
        tail = vals[1:] if len(vals) > 1 else vals
        if tail:
            f.tps_median = statistics.median(tail)
    m = RE_STW.search(text)
    if m:
        f.stw_total = float(m.group(2))


def load_log_side(paths: List[str]) -> LogFacts:
    f = LogFacts()
    f.source = ",".join(os.path.basename(p) for p in paths)
    for p in paths:
        with open(p, "r", encoding="utf-8", errors="replace") as fh:
            parse_log_text(fh.read(), f, p)
    return f


# ----------------------------------------------------------------------------
# v2: per-type band + отчёт
# ----------------------------------------------------------------------------

def _band(a: int, b: int, tol: float, floor: int) -> Tuple[bool, float]:
    drift = abs(a - b)
    limit = max(floor, tol * max(a, b))
    return drift <= limit, (drift / max(a, b) * 100.0 if max(a, b) else 0.0)


def entity_band_report(A: Dict[str, int], B: Dict[str, int], tol: float,
                       floor: int, strict: bool = False) -> dict:
    shared = sorted(set(A) & set(B))
    one_side = sorted(set(A) ^ set(B))
    rows, bad, worst = [], [], (0.0, "—")
    for t in shared:
        a, b = A[t], B[t]
        in_band, pct = _band(a, b, 0.0 if strict else tol, 0 if strict else floor)
        rows.append({"type": t, "a": a, "b": b, "drift_pct": round(pct, 4),
                     "in_band": in_band})
        if pct > worst[0]:
            worst = (pct, t)
        if not in_band:
            bad.append(f"{t}: {a} vs {b} (Δ{pct:.2f}%)")
    rows.sort(key=lambda r: -r["drift_pct"])
    return {"shared": len(shared), "one_side": one_side, "rows": rows,
            "violations": bad, "worst_drift_pct": round(worst[0], 4),
            "worst_type": worst[1],
            "tol": tol, "floor": floor, "strict": strict}


OK, FAIL, SKIP, INFO = "OK", "FAIL", "SKIP", "INFO"


class GateResult:
    def __init__(self, gate: str, status: str, detail: str):
        self.gate, self.status, self.detail = gate, status, detail

    def as_dict(self):
        return {"gate": self.gate, "status": self.status, "detail": self.detail}


def compare_logs(van: LogFacts, leg: LogFacts, tol: float, floor: int,
                 strict_entities: bool = False, ignore_world: bool = False) -> List[GateResult]:
    out: List[GateResult] = []

    def both(getter):
        a, b = getter(van), getter(leg)
        return (None, a, b) if a is None or b is None else ((a, b), a, b)

    # W1 FIXTURE-VALIDITY (+гейты 1a/1b/1c)
    pair, a, b = both(lambda f: f.fixture_validity)
    if pair is None:
        out.append(GateResult("FIXTURE-VALIDITY", FAIL,
                              f"нет данных (vanilla={a}, leg={b}) — паритет не верифицируем"))
    elif a == b == "VALID":
        bad, sub = [], []
        for g in sorted(set(van.fixture_gates) | set(leg.fixture_gates)):
            ga, gb = van.fixture_gates.get(g), leg.fixture_gates.get(g)
            if ga and gb:
                sub.append(f"{g}:{ga}=={gb}")
                if ga != gb or ga != "PASS":
                    bad.append(f"{g}: {ga} vs {gb}")
            elif (ga or gb) and (ga or gb) != "PASS":
                bad.append(f"{g}: {ga or '—'} vs {gb or '—'}")
        if bad:
            out.append(GateResult("FIXTURE-VALIDITY", FAIL, f"VALID==VALID, но гейты: {'; '.join(bad)}"))
        else:
            d = " ".join(sub) if sub else "гейты 1a/1b/1c не публиковались"
            out.append(GateResult("FIXTURE-VALIDITY", OK, f"VALID==VALID; {d}"))
    else:
        out.append(GateResult("FIXTURE-VALIDITY", FAIL, f"vanilla={a} leg={b} (ожидается VALID==VALID)"))

    # W2 WORLD (парная дисциплина S7-96d)
    if ignore_world:
        out.append(GateResult("WORLD", SKIP, "--ignore-world"))
    else:
        pair, a, b = both(lambda f: f.world_sha256)
        if pair is None:
            out.append(GateResult("WORLD", SKIP, f"world_sha256 не найден (vanilla={a}, leg={b})"))
        elif a == b:
            out.append(GateResult("WORLD", OK, f"sha256={a[:12]}…"))
        else:
            out.append(GateResult("WORLD", FAIL,
                                  f"разные стенды: {a[:12]}… vs {b[:12]}… (парить по world_sha256, S7-96d)"))

    # W3 SEED
    pair, a, b = both(lambda f: f.pop_seed)
    if pair is None:
        out.append(GateResult("SEED", FAIL, f"seed не найден (vanilla={a}, leg={b})"))
    else:
        out.append(GateResult("SEED", OK if a == b else FAIL,
                              f"seed={a}" if a == b else f"{a} != {b}"))

    # W4 CHUNK-CHECKSUMS (seed-идентичные чанки: ландшафт spawnable/forceload)
    sp_pair, spa, spb = both(lambda f: f.spawnable_chunks)
    fl_pair, fla, flb = both(lambda f: f.forceload_chunks)
    if sp_pair is None and fl_pair is None:
        out.append(GateResult("CHUNK-CHECKSUMS", FAIL,
                              "spawnable-chunk polls и forceload не найдены с обеих сторон — "
                              "seed-идентичность ландшафта чанков не верифицируем"))
    else:
        bad = []
        if sp_pair is not None and spa != spb:
            bad.append(f"spawnable polls {spa} != {spb}")
        if fl_pair is not None:
            if fla != flb:
                bad.append(f"forceload chunks {fla} != {flb}")
            else:
                _, ca, cb = both(lambda f: f.forceload_cmds)
                if ca is not None and cb is not None and ca != cb:
                    bad.append(f"forceload cmds {ca} != {cb}")
        if bad:
            out.append(GateResult("CHUNK-CHECKSUMS", FAIL, "; ".join(bad)))
        else:
            det = []
            if sp_pair is not None:
                det.append(f"spawnable polls {spa[0]}×{len(spa)} == {spb[0]}×{len(spb)}")
            if fl_pair is not None:
                det.append(f"forceload chunks={fla}")
            out.append(GateResult("CHUNK-CHECKSUMS", OK, "; ".join(det) + " (seed-идентичный ландшафт)"))

    # W5 POP-TOTALS
    fields = [("target", "pop_target"), ("injected", "pop_injected"),
              ("items", "pop_items"), ("hostiles", "pop_hostiles"), ("passives", "pop_passives")]
    common, one_side, mism = [], [], []
    for label, attr in fields:
        a, b = getattr(van, attr), getattr(leg, attr)
        if a is not None and b is not None:
            common.append((label, a, b))
            if a != b:
                mism.append(f"{label}: {a} != {b}")
        elif a is not None or b is not None:
            one_side.append(f"{label}(van={a},leg={b})")
    _, pa, pb = both(lambda f: f.pop_plan)
    if pa is not None and pb is not None and pa != pb:
        mism.append(f"plan(INJECT START): {pa} != {pb}")
    if mism:
        out.append(GateResult("POP-TOTALS", FAIL, "; ".join(mism)))
    elif common:
        vals = " ".join(f"{l}={b}" for l, _a, b in common)
        note = f" [полусторона: {', '.join(one_side)}]" if one_side else ""
        out.append(GateResult("POP-TOTALS", OK, vals + note))
    else:
        out.append(GateResult("POP-TOTALS", FAIL,
                              "POPULATION INJECT DONE не найден ни на одной стороне — pop-totals не верифицируем"))

    # W6 PER-TYPE-POPULATION
    if not van.entity_types or not leg.entity_types:
        out.append(GateResult("PER-TYPE-POPULATION", FAIL,
                              f"per-type данные не найдены (vanilla={len(van.entity_types)} типов,"
                              f" leg={len(leg.entity_types)} типов)"))
    else:
        rep = entity_band_report(van.entity_types, leg.entity_types, tol, floor, strict_entities)
        if rep["violations"]:
            out.append(GateResult("PER-TYPE-POPULATION", FAIL,
                                  f"{len(rep['violations'])} нарушений из {rep['shared']} общих типов: "
                                  + "; ".join(rep["violations"][:8])))
        else:
            note = (f"; односторонние {len(rep['one_side'])} типов" if rep["one_side"] else "")
            out.append(GateResult("PER-TYPE-POPULATION", OK,
                                  f"{rep['shared']} общих типов в банде (tol={tol:.2%}, floor={floor}),"
                                  f" макс-дрейф {rep['worst_drift_pct']:.2f}% ({rep['worst_type']}){note}"))

    # W7 TICK-BEHIND (канон 0) + кросс-чек raw-скан vs md-срез на каждой стороне
    pair, a, b = both(lambda f: f.tick_behind)
    if pair is None:
        out.append(GateResult("TICK-BEHIND", FAIL,
                              f"нет данных (vanilla={a}, leg={b}) — канон требует tick-behind=0"))
    elif a == b == 0:
        extra = ""
        ta, tb2 = van.ticks_behind_sum, leg.ticks_behind_sum
        if ta or tb2:
            extra = f"; ticks-behind van={ta} leg={tb2}"
        mism = [s for s, f_ in (("van", van), ("leg", leg))
                if f_.tb_report is not None and f_.tick_behind is not None
                and f_.tb_report != f_.tick_behind]
        if mism:
            out.append(GateResult("TICK-BEHIND", FAIL,
                                  f"raw/md рассинхрон ({', '.join(mism)}) — raw-скан и md-срез противоречат"))
        else:
            out.append(GateResult("TICK-BEHIND", OK, "0 == 0" + extra))
    else:
        d = [f"vanilla={a}", f"leg={b}"]
        ta, tb2 = van.ticks_behind_sum, leg.ticks_behind_sum
        if ta or tb2:
            d.append(f"ticks-behind van={ta} leg={tb2}")
        out.append(GateResult("TICK-BEHIND", FAIL, ", ".join(d) + " (канон: 0)"))

    # W8 FAKE-PLAYERS
    pair, a, b = both(lambda f: f.fake_players)
    if pair is not None:
        out.append(GateResult("FAKE-PLAYERS", OK if a == b else FAIL,
                              f"{a} == {b}" if a == b else f"{a} != {b}"))
    else:
        out.append(GateResult("FAKE-PLAYERS", SKIP, "fake_players не найден с обеих сторон"))

    # W9 CPU-BAND (инфо: парность по runner_cpu_index; вне-банда = pairing-проблема)
    pair, a, b = both(lambda f: f.runner_cpu_index)
    if pair is None:
        out.append(GateResult("CPU-BAND", SKIP, "runner_cpu_index не найден"))
    else:
        ia, ib = int(a), int(b)
        inb = CPU_BAND[0] <= ia <= CPU_BAND[1] and CPU_BAND[0] <= ib <= CPU_BAND[1]
        delta = abs(ia - ib)
        out.append(GateResult("CPU-BAND", INFO if inb else FAIL,
                              f"van={ia} leg={ib} Δcpu={delta} "
                              f"({'in-band 6.0-9.5M' if inb else 'OUT-OF-BAND 6.0-9.5M'})"))

    return out


def evaluate_logs(van_paths: List[str], leg_paths: List[str], tol: float, floor: int,
                  strict_entities: bool = False, ignore_world: bool = False):
    van = load_log_side(van_paths)
    leg = load_log_side(leg_paths)
    gates = compare_logs(van, leg, tol, floor, strict_entities, ignore_world)
    failed = [g for g in gates if g.status == FAIL]
    evaluated = [g for g in gates if g.status != SKIP]
    if not evaluated:
        v, reasons = "WORLD-PARITY-FAIL", ["ни один гейт не вычислен — вход пуст/неопознан"]
    elif failed:
        v, reasons = "WORLD-PARITY-FAIL", [f"{g.gate}: {g.detail}" for g in failed]
    else:
        v, reasons = "WORLD-PARITY-OK", []
    return v, reasons, gates, van, leg


# ----------------------------------------------------------------------------
# отчёты (world + logs) и гейты
# ----------------------------------------------------------------------------

def render_world_md(r: dict, a_dir: str, b_dir: str) -> str:
    L = ["# REGION_DIFF (world-diff bit-parity v2, C2ME-методика S74→S50)", "",
         f"- side A: `{a_dir}` (world_digest `{r['world_digest_a'][:16]}…`)",
         f"- side B: `{b_dir}` (world_digest `{r['world_digest_b'][:16]}…`)",
         f"- world digest equal: **{r['world_digest_equal']}**", "",
         f"**{r['chunks_mismatched']}/{r['chunks_total']} chunks, "
         f"{r['blocks_diff']} block differences ({r['blocks_diff_pct']:.4f}%)**", "",
         f"- чек-суммы чанков (seed-идентичные): "
         f"{r['chunks_checksum_equal']}/{r['chunks_checksum_shared']} shared-чанков равны; "
         f"checksum-diff: {len(r['chunks_checksum_diff'])} в top",
         f"- level.dat Time: A={r['level_time_a']} B={r['level_time_b']} "
         f"(Δticks={r['tick_delta']} — tick-behind прокси)"]
    if r.get("entities_band"):
        eb = r["entities_band"]
        L.append(f"- entities per-type: {eb['shared']} общих типов, "
                 f"worst {eb['worst_drift_pct']:.2f}% ({eb['worst_type']}), "
                 f"нарушений банды {len(eb['violations'])}")
    L.append("")
    if r["top_id_divergences"]:
        L += ["## top block-ID divergences", "", "| from | to | count |", "|---|---|---|"]
        L += [f"| {d['from']} | {d['to']} | {d['count']} |" for d in r["top_id_divergences"]]
        L.append("")
    if r["per_id_total_delta"]:
        L += ["## per-ID totals delta (зоны расхождения)", "", "| block-ID | Δcount |", "|---|---|"]
        L += [f"| {d['id']} | {d['delta']:+d} |" for d in r["per_id_total_delta"]]
        L.append("")
    return "\n".join(L)


def render_logs_md(v, reasons, gates, van: LogFacts, leg: LogFacts,
                   van_paths, leg_paths, rep=None) -> str:
    L = ["# WORLD-PARITY (log-режим арбитра v2, закон 4 + 20d)", "",
         f"- vanilla: {', '.join(os.path.basename(p) for p in van_paths)}"
         f" (runner={van.runner_cpu_index}, seed={van.pop_seed}, tps_med={van.tps_median}, stw={van.stw_total})",
         f"- leg    : {', '.join(os.path.basename(p) for p in leg_paths)}"
         f" (runner={leg.runner_cpu_index}, seed={leg.pop_seed}, tps_med={leg.tps_median}, stw={leg.stw_total})",
         "", "| статус | гейт | деталь |", "|---|---|---|"]
    L += [f"| {g.status} | {g.gate} | {g.detail} |" for g in gates]
    tail = f" [{'; '.join(reasons)}]" if reasons and v.endswith("FAIL") else ""
    L += ["", f"**VERDICT: {v}{tail}**"]
    if rep:
        L += ["", "## per-type population diff (top по дрейфу)", "",
              "| тип | vanilla | leg | дрейф % | в банде |", "|---|---|---|---|---|"]
        L += [f"| {r['type']} | {r['a']} | {r['b']} | {r['drift_pct']:.2f} | "
              f"{'да' if r['in_band'] else 'НЕТ'} |" for r in rep["rows"]]
    return "\n".join(L) + "\n"


def gate_world(r: dict, gate_chunks: float, gate_blocks: float,
               strict_time: bool = False) -> Tuple[bool, str]:
    ok = (r["chunks_diff_pct"] <= gate_chunks + 1e-9) and (r["blocks_diff_pct"] <= gate_blocks + 1e-9)
    if strict_time and r["tick_delta"] not in (None, 0):
        ok = False
    msg = (f"WORLD-DIFF {'PASS' if ok else 'FAIL'}: "
           f"{r['chunks_mismatched']}/{r['chunks_total']} chunks ({r['chunks_diff_pct']:.4f}%), "
           f"{r['blocks_diff']} block differences ({r['blocks_diff_pct']:.4f}%), "
           f"checksums-equal {r['chunks_checksum_equal']}/{r['chunks_checksum_shared']}, "
           f"world_digest {'==' if r['world_digest_equal'] else '!='}, "
           f"Δticks={r['tick_delta']}, "
           f"top={[(d['from'], d['to'], d['count']) for d in r['top_id_divergences'][:3]]}")
    return ok, msg


# ----------------------------------------------------------------------------
# gate_v2 P6-PARITY интеграция
# ----------------------------------------------------------------------------

def gate2_line(verdict: str, gates: List[GateResult], ms: int) -> str:
    failed = [g for g in gates if g.status == FAIL]
    status = "OK" if verdict.endswith("OK") else "FAIL"
    det = "; ".join(f"{g.gate}:{g.status}" for g in gates)
    return f"P6-parity: {status} ({verdict}, FAIL={len(failed)}, {ms}ms) {det}"


def gate2_json_phase(verdict: str, gates: List[GateResult], ms: int) -> dict:
    failed = [g for g in gates if g.status == FAIL]
    status = "OK" if verdict.endswith("OK") else "FAIL"
    det = f"{verdict} FAIL={len(failed)} " + "; ".join(
        f"{g.gate}:{g.status}" for g in gates if g.status != INFO)
    return {"phase": "P6-parity", "ms": ms, "status": status, "detail": det[:400]}


# ----------------------------------------------------------------------------
# самотест v2: герметичный (синтетика) — 12 чеков
# ----------------------------------------------------------------------------

def _w_payload(tag: int, val) -> bytes:
    if isinstance(val, (bytes, bytearray)):
        return bytes(val)
    if tag == 3:
        return struct.pack(">i", val)
    if tag == 8:
        b = val.encode()
        return struct.pack(">H", len(b)) + b
    if tag == 9:
        et, items = val
        return bytes([et]) + struct.pack(">i", len(items)) + b"".join(_w_payload(et, x) for x in items)
    if tag == 10:
        out = b""
        for k, v in val.items():
            kb = k.encode()
            if isinstance(v, (bytes, bytearray)):
                out += bytes([10]) + struct.pack(">H", len(kb)) + kb + bytes(v)
            else:
                t, v = v
                out += bytes([t]) + struct.pack(">H", len(kb)) + kb + _w_payload(t, v)
        return out + b"\x00"
    if tag == 12:
        return struct.pack(">i", len(val)) + b"".join(struct.pack(">q", x) for x in val)
    raise ValueError(f"encoder tag {tag}")


def _named_root(val: dict) -> bytes:
    return bytes([10]) + struct.pack(">H", 0) + _w_payload(10, val)


def make_chunk_bytes(cx: int, cz: int, palette: List[str], data: List[int], status: str = "full") -> bytes:
    pal_compounds = [_w_payload(10, {"Name": (8, p)}) for p in palette]
    bs = _w_payload(10, {"palette": (9, (10, pal_compounds)), "data": (12, data)})
    sec = _w_payload(10, {"Y": (3, 0), "block_states": (10, bs)})
    root = {"DataVersion": (3, 4189), "xPos": (3, cx), "zPos": (3, cz),
            "Status": (8, status), "sections": (9, (10, [sec]))}
    return _named_root(root)


def write_region(path: str, chunks: Dict[Tuple[int, int], bytes]) -> None:
    buf = bytearray(8192)
    sector = 2
    for (cx, cz), payload in chunks.items():
        idx = (cx & 31) + (cz & 31) * 32
        comp = zlib.compress(payload)
        blob = struct.pack(">I", len(comp) + 1) + bytes([COMP_ZLIB]) + comp
        n = (len(blob) + 4095) // 4096
        buf[idx * 4:idx * 4 + 3] = sector.to_bytes(3, "big")
        buf[idx * 4 + 3] = n
        buf.extend(blob.ljust(n * 4096, b"\x00"))
        sector += n
    with open(path, "wb") as f:
        f.write(bytes(buf))


def synth_world(dirpath: str, mutate: bool = False, drop_chunk: bool = False) -> None:
    os.makedirs(dirpath, exist_ok=True)
    pal = ["minecraft:stone", "minecraft:dirt", "minecraft:oak_planks"]
    base = [i % 3 for i in range(4096)]
    longs: List[int] = []
    for i in range(0, 4096, 16):
        v = 0
        for j, e in enumerate(base[i:i + 16]):
            v |= e << (j * 4)
        longs.append(v - (1 << 64) if v >= (1 << 63) else v)
    chunks = {}
    for cx, cz in ((0, 0), (1, 0), (-1, 0)):
        if drop_chunk and (cx, cz) == (1, 0):
            continue
        pal2 = list(pal)
        if mutate and (cx, cz) == (0, 0):
            pal2[0] = "minecraft:dirt"
        chunks[(cx, cz)] = make_chunk_bytes(cx, cz, pal2, longs)
    write_region(os.path.join(dirpath, "r.0.0.mca"), chunks)


SYN_LOG = """[12:00:00 INFO]: [BenchPopulation] POPULATION INJECT DONE target=150000 injected=150000 items=105000 hostiles=30000 passives=15000
[12:05:00 INFO]: [BenchPopulation] TOPUP-SCAN tick=600 aliveReal(items=103000,hostiles=29500,passives=14800) deficit(items=0,hostiles=0,passives=0) aliveEst(items-model)=103000 topupSpawnedTotal=0
- world_sha256: aaaa0000000000000000000000000000000000000000000000000000000000
- runner_cpu_index: 6800000
- fake_players: 4 (BENCH-4 fixture)
- population_target: 150000 (BENCH-X150K)
- population_seed: 42 (deterministic)
- entity totals seen: [1000, 1010]
- top entity types (max seen): minecraft:husk×5178, minecraft:spider×4792, minecraft:cow×3392
- tick-behind warnings in log: 0
- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3348)
- gate 1c alive-check steady at N=4: PASS
- **FIXTURE-VALIDITY: VALID**
"""


def synth_log(dirpath: str, name: str, mutations: Optional[List[Tuple[str, str]]] = None) -> str:
    os.makedirs(dirpath, exist_ok=True)
    t = SYN_LOG
    for a, b in (mutations or []):
        assert a in t, f"selftest: образец не найден {a!r}"
        t = t.replace(a, b, 1)
    p = os.path.join(dirpath, name)
    with open(p, "w", encoding="utf-8") as f:
        f.write(t)
    return p


def self_test() -> int:
    checks: List[Tuple[str, bool, str]] = []
    tol, floor = ENTITY_TOLERANCE, ENTITY_ABS_FLOOR
    with tempfile.TemporaryDirectory(prefix="wdp2-selftest-") as td:
        wa = os.path.join(td, "A", "region")
        wb = os.path.join(td, "B", "region")
        wc = os.path.join(td, "C", "region")
        synth_world(wa)
        synth_world(wb)                                   # identity B == A
        synth_world(wc, mutate=True, drop_chunk=True)     # мутация + missing

        # T1 identity A/A
        r1 = diff_worlds(wa, wb)
        checks.append(("T1 identity: 0/0 chunks, 0 blocks", (
            r1["chunks_mismatched"] == 0 and r1["blocks_diff"] == 0), ""))
        checks.append(("T1 world_digest_equal True", r1["world_digest_equal"] is True, ""))
        # T2 checksums: seed-идентичные 3/3
        checks.append(("T2 checksums identity: 3/3 equal", (
            r1["chunks_checksum_shared"] == 3 and r1["chunks_checksum_equal"] == 3), ""))
        ok1, _ = gate_world(r1, 0.0, 0.0)
        checks.append(("T2 gate identity -> PASS", ok1 is True, ""))

        # T3 мутация
        r2 = diff_worlds(wa, wc)
        checks.append(("T3 mutated: 1366 block diffs, 2 chunk mismatch", (
            r2["blocks_diff"] == 1366 and r2["chunks_mismatched"] == 2), ""))
        checks.append(("T3 checksums: equal 1/2 shared (мутант + missing вне shared)", (
            r2["chunks_checksum_shared"] == 2 and r2["chunks_checksum_equal"] == 1
            and len(r2["chunks_checksum_diff"]) == 1), ""))
        ok2, msg2 = gate_world(r2, 0.0, 0.0)
        checks.append(("T3 gate мутация -> FAIL", ok2 is False and "FAIL" in msg2, msg2[:80]))

        # T4 unpack round-trip
        base4 = [i % 3 for i in range(4096)]
        longs4 = []
        for i4 in range(0, 4096, 16):
            v4 = 0
            for j4, e4 in enumerate(base4[i4:i4 + 16]):
                v4 |= e4 << (j4 * 4)
            longs4.append(v4 - (1 << 64) if v4 >= (1 << 63) else v4)
        checks.append(("T4 unpack round-trip 4-bit", unpack_indices(3, longs4) == base4, ""))

        # T5..T11 log-режим
        lv = os.path.join(td, "logs_van")
        ll = os.path.join(td, "logs_leg")
        pv = synth_log(lv, "van.log")
        pl = synth_log(ll, "leg.log")
        v, reasons, gates, van, leg = evaluate_logs([pv], [pl], tol, floor)
        names = {g.gate: g for g in gates}
        checks.append(("T5 log identity pair -> WORLD-PARITY-OK", v == "WORLD-PARITY-OK",
                       "; ".join(reasons)[:160]))
        checks.append(("T5 per-type 3 общих типа, worst=0.00%", (
            names["PER-TYPE-POPULATION"].status == OK and "макс-дрейф 0.00%" in names["PER-TYPE-POPULATION"].detail), ""))
        checks.append(("T6 TICK-BEHIND OK 0==0", names["TICK-BEHIND"].status == OK, ""))
        checks.append(("T7 CHUNK-CHECKSUMS OK 289×5 seed-идентичный", (
            names["CHUNK-CHECKSUMS"].status == OK and "289×5" in names["CHUNK-CHECKSUMS"].detail), ""))

        # негативы: per-type мутация husk +30%
        pl2 = synth_log(os.path.join(td, "mut1"), "leg.log",
                        [("minecraft:husk×5178", "minecraft:husk×6732")])
        v2, reasons2, gates2, _v, _l = evaluate_logs([pv], [pl2], tol, floor)
        n2 = {g.gate: g for g in gates2}
        checks.append(("T8 husk+30% -> FAIL(PER-TYPE-POPULATION)", (
            v2 == "WORLD-PARITY-FAIL" and n2["PER-TYPE-POPULATION"].status == FAIL
            and "husk" in n2["PER-TYPE-POPULATION"].detail), "; ".join(reasons2)[:120]))

        # негатив: tick-behind 0 -> 1 (канон 0 нарушен)
        pl3 = synth_log(os.path.join(td, "mut2"), "leg.log",
                        [("tick-behind warnings in log: 0", "tick-behind warnings in log: 1")])
        v3, reasons3, gates3, _v, _l = evaluate_logs([pv], [pl3], tol, floor)
        n3 = {g.gate: g for g in gates3}
        checks.append(("T9 tick-behind 0 vs 1 -> FAIL(TICK-BEHIND)", (
            v3 == "WORLD-PARITY-FAIL" and n3["TICK-BEHIND"].status == FAIL), "; ".join(reasons3)[:120]))

        # негатив: seed 42 -> 43
        pl4 = synth_log(os.path.join(td, "mut3"), "leg.log",
                        [("population_seed: 42", "population_seed: 43")])
        v4, reasons4, gates4, _v, _l = evaluate_logs([pv], [pl4], tol, floor)
        n4 = {g.gate: g for g in gates4}
        checks.append(("T10 seed 42 vs 43 -> FAIL(SEED)", (
            v4 == "WORLD-PARITY-FAIL" and n4["SEED"].status == FAIL), "; ".join(reasons4)[:120]))

        # негатив: spawnable polls 289 -> 144 (ландшафт чанков)
        pl5 = synth_log(os.path.join(td, "mut4"), "leg.log",
                        [("[289, 289, 289, 289, 289]", "[144, 144, 144, 144, 144]")])
        v5, reasons5, gates5, _v, _l = evaluate_logs([pv], [pl5], tol, floor)
        n5 = {g.gate: g for g in gates5}
        checks.append(("T11 spawnable 289 vs 144 -> FAIL(CHUNK-CHECKSUMS)", (
            v5 == "WORLD-PARITY-FAIL" and n5["CHUNK-CHECKSUMS"].status == FAIL), "; ".join(reasons5)[:120]))

        # T12 gate2-машина: P6-parity строка + json-фаза
        line = gate2_line("WORLD-PARITY-OK", gates, 12)
        j = gate2_json_phase("WORLD-PARITY-OK", gates, 12)
        checks.append(("T12 gate2 P6-parity: OK-строка + json_phase", (
            line.startswith("P6-parity: OK") and j["phase"] == "P6-parity"
            and j["status"] == "OK"), line[:80]))

    for name, ok, extra in checks:
        print(f"  {'PASS' if ok else 'FAIL'}  {name}" + (f"   [{extra}]" if (extra and not ok) else ""))
    bad = [n for n, ok, _ in checks if not ok]
    print(f"SELFTEST {len(checks) - len(bad)}/{len(checks)}" + (f"  FAILED: {bad}" if bad else ""))
    return 0 if not bad else 1


# ----------------------------------------------------------------------------
# CLI
# ----------------------------------------------------------------------------

USAGE = ("world-режим: world_diff_parity_v2.py regionA regionB [--out MD] [--json J] [--entities] "
         "[--gate-chunks F] [--gate-blocks F] [--top N] [--no-status] [--strict-time] [--dump-checksums FILE]\n"
         "log-режим : world_diff_parity_v2.py van.log... -- leg.log... [--entity-tolerance 0.05] "
         "[--entity-abs-floor 64] [--strict-entities] [--ignore-world] [--quiet] [--json J] [--gate2] [--out MD]\n"
         "P6-фаза   : PAIR_VAN=\"...\" PAIR_LEG=\"...\" world_diff_parity_v2.py --gate2 --env-pair --json J\n"
         "самотест  : world_diff_parity_v2.py --self-test")


def _parse_cli(argv: List[str]) -> dict:
    o = {"mode": "auto", "out": None, "json": None, "gate2": False, "quiet": False,
         "entities": False, "no_status": False, "strict_time": False, "dump": None,
         "gate_chunks": 0.0, "gate_blocks": 0.0, "top": TOP_DEFAULT,
         "tol": ENTITY_TOLERANCE, "floor": ENTITY_ABS_FLOOR, "strict_entities": False,
         "ignore_world": False, "self_test": False, "env_pair": False, "van": [], "leg": []}
    side = "van"
    i = 0
    while i < len(argv):
        t = argv[i]
        if t in ("-h", "--help"):
            print(__doc__)
            raise SystemExit(0)
        if t == "--self-test":
            o["self_test"] = True
            i += 1
            continue
        if t == "--":
            side = "leg"
            i += 1
            continue
        if t == "--mode":
            o["mode"] = argv[i + 1]; i += 2
            continue
        if t == "--out":
            o["out"] = argv[i + 1]; i += 2
            continue
        if t == "--json":
            o["json"] = argv[i + 1]; i += 2
            continue
        if t == "--dump-checksums":
            o["dump"] = argv[i + 1]; i += 2
            continue
        if t == "--gate-chunks":
            o["gate_chunks"] = float(argv[i + 1]); i += 2
            continue
        if t == "--gate-blocks":
            o["gate_blocks"] = float(argv[i + 1]); i += 2
            continue
        if t == "--top":
            o["top"] = int(argv[i + 1]); i += 2
            continue
        if t == "--entity-tolerance":
            o["tol"] = float(argv[i + 1]); i += 2
            continue
        if t == "--entity-abs-floor":
            o["floor"] = int(argv[i + 1]); i += 2
            continue
        flags = {"--gate2": "gate2", "--quiet": "quiet", "--entities": "entities",
                 "--no-status": "no_status", "--strict-time": "strict_time",
                 "--strict-entities": "strict_entities", "--ignore-world": "ignore_world",
                 "--env-pair": "env_pair"}
        if t in flags:
            o[flags[t]] = True
            i += 1
            continue
        if t.startswith("-") and len(t) > 1:
            raise ValueError(f"неизвестная опция: {t}")
        if side == "van":
            o["van"].append(t)
        else:
            o["leg"].append(t)
        i += 1
    return o


def main(argv: Optional[List[str]] = None) -> int:
    global TOP_DEFAULT
    argv = list(sys.argv[1:] if argv is None else argv)
    try:
        args = _parse_cli(argv)
    except ValueError as e:
        print(f"ОШИБКА CLI: {e}\n{USAGE}", file=sys.stderr)
        return 2
    except SystemExit:
        return 0
    if args["self_test"]:
        return self_test()
    if args["env_pair"]:
        # gate_v2 P6-фаза: стороны через env (двойной '--' в CLI неоднозначен)
        args["van"] = (os.environ.get("PAIR_VAN") or "").split()
        args["leg"] = (os.environ.get("PAIR_LEG") or "").split()
    if not args["van"] or not args["leg"]:
        print("ОШИБКА CLI: нужны обе стороны\n" + USAGE, file=sys.stderr)
        return 2
    a, b = args["van"][0], args["leg"][0]
    mode = args["mode"]
    if mode == "auto":
        mode = "world" if (os.path.isdir(a) and os.path.isdir(b)) else "logs"

    if mode == "world":
        TOP_DEFAULT = args["top"]
        try:
            r = diff_worlds(a, b, compare_status=not args["no_status"],
                            with_entities=args["entities"])
        except RegionError as e:
            print(f"WORLD-DIFF ERROR: {e}", file=sys.stderr)
            return 2
        if args["dump"]:
            with open(args["dump"], "w", encoding="utf-8") as f:
                json.dump({"a": r["chunk_checksums_a"], "b": r["chunk_checksums_b"]}, f, indent=1)
        if args["out"]:
            with open(args["out"], "w", encoding="utf-8") as f:
                f.write(render_world_md(r, a, b))
        if args["json"]:
            dump = {k: v for k, v in r.items() if not k.startswith("chunk_checksums_")}
            with open(args["json"], "w", encoding="utf-8") as f:
                json.dump(dump, f, indent=1, ensure_ascii=False)
        ok, msg = gate_world(r, args["gate_chunks"], args["gate_blocks"], args["strict_time"])
        print(msg)
        if args["out"]:
            print(f"report: {args['out']}" + (f"; json: {args['json']}" if args["json"] else ""))
        return 0 if ok else 1

    # ---- log-режим ----
    try:
        t0 = time.time()
        v, reasons, gates, van, leg = evaluate_logs(
            args["van"], args["leg"], args["tol"], args["floor"],
            args["strict_entities"], args["ignore_world"])
        ms = int((time.time() - t0) * 1000)
        rep = entity_band_report(van.entity_types, leg.entity_types,
                                 0.0 if args["strict_entities"] else args["tol"],
                                 0 if args["strict_entities"] else args["floor"])
    except OSError as e:
        print(f"ОШИБКА входа: {e}", file=sys.stderr)
        return 2
    if args["json"]:
        with open(args["json"], "w", encoding="utf-8") as f:
            json.dump({"verdict": v, "reasons": reasons,
                       "gates": [g.as_dict() for g in gates],
                       "per_type": rep,
                       "vanilla": {"runner_cpu_index": van.runner_cpu_index,
                                   "world_sha256": van.world_sha256,
                                   "pop_seed": van.pop_seed,
                                   "tick_behind": van.tick_behind,
                                   "tps_median": van.tps_median,
                                   "stw_total": van.stw_total,
                                   "types": van.entity_types},
                       "leg": {"runner_cpu_index": leg.runner_cpu_index,
                               "world_sha256": leg.world_sha256,
                               "pop_seed": leg.pop_seed,
                               "tick_behind": leg.tick_behind,
                               "tps_median": leg.tps_median,
                               "stw_total": leg.stw_total,
                               "types": leg.entity_types},
                       "params": {"entity_tolerance": args["tol"],
                                  "entity_abs_floor": args["floor"],
                                  "strict_entities": args["strict_entities"],
                                  "ignore_world": args["ignore_world"]}},
                      f, ensure_ascii=False, indent=2, default=str)
    if args["out"]:
        with open(args["out"], "w", encoding="utf-8") as f:
            f.write(render_logs_md(v, reasons, gates, van, leg, args["van"], args["leg"], rep))
    if args["gate2"]:
        print(gate2_line(v, gates, ms))
        return 0 if v.endswith("OK") else 1
    lines = []
    if not args["quiet"]:
        lines.append("== WORLD-DIFF PARITY ARBITER v2 (закон 4 + 20d, ROUND-468/S50) ==")
        lines.append(f"vanilla: {', '.join(args['van'])}")
        lines.append(f"leg    : {', '.join(args['leg'])}")
        for g in gates:
            lines.append(f"  {g.status:<4} | {g.gate:<20} | {g.detail}")
        lines.append(f"  info | TPS/STW            | van med={van.tps_median} stw={van.stw_total} | "
                     f"leg med={leg.tps_median} stw={leg.stw_total} "
                     f"(churn Δ van={van.churn_delta} leg={leg.churn_delta}; "
                     f"alive-polls van={len(van.alive_players)} leg={len(leg.alive_players)})")
        lines.append("  info | PER-TYPE top-3     | "
                     + "; ".join(f"{r['type']} {r['a']}→{r['b']} ({r['drift_pct']:+.2f}%)"
                                 for r in rep["rows"][:3]))
    tail = f" [{'; '.join(reasons)}]" if reasons and v.endswith("FAIL") else ""
    lines.append(f"VERDICT: {v}{tail}")
    print("\n".join(lines))
    return 0 if v.endswith("OK") else 1


if __name__ == "__main__":
    sys.exit(main())
