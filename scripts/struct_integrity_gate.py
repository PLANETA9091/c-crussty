#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
struct_integrity_gate.py — S12 (ROUND-470): STRUCT-INTEGRITY GATE v1 (стаб-вайринг).
Preregistered анти-коррупция-гейт для БУДУЩИХ parallel-structgen рычагов
(Moonrise #192/#191 класс: параллельная генерация структур рвёт placement через
границы чанков) + анти-слепота world_sha256 (Paper #14125: чанк-ген НЕ чистая
функция сида → при region_threads>0 world/raw sha256-гейты слепы — канон 0/169).

КАНОН ПРЕ-РЕГИСТРАЦИИ (менять после первого лега запрещено — preregister):
  G-S1 SEED-IDENTITY   level.dat seed (Data.WorldGenSettings.seed | Data.RandomSeed)
                       обязан совпадать; не совпал = FAIL-NOTCOMPARABLE (не FAIL мира).
  G-S2 DETERMINISM     контроль-канал: повторный серийный прогон той же стороны обязан
                       давать 169/169; иначе env недетерминирован → REFUTED-CENS abort
                       (lever не судим, вердикт запрещён). repeat=2 на сторону.
  G-S3 CHUNK-PARITY    семантические per-чанк дайджесты probe-окна 13×13 = 169 чанков;
                       требование СТРОГО 169/169 (структурная коррупция = коррупция,
                       бандов нет). Отчёт top-K diff (cx,cz) — локализация обязательна.
  G-S4 STRUCT-MARKERS  дайджест structure-starts (root["structures"]) равен обеих сторон;
                       односторонний старт = FAIL. Ловит потерянный старт при целом
                       ландшафте — content-гейт этот класс НЕ видит (Moonrise #192).
  G-S5 BLIND-RULE      world_sha256 / raw-NBT дигесты НЕ являются доказательством pass
                       (слепота 0/169 + 100% ложных-брейков raw-NBT на live-риге,
                       volатильное поле @92 — Л199). Сырой дигест = теневая метрика
                       blind_raw, никогда — вердикт.

ЧТО ХЕШИРУЕМ (canon дайджеста, порядок фикс; документ docs/S12_STRUCTGATE.md):
  semantic(c) = sha256("S12v1|" + "{cx},{cz},{status}" + | sorted sections
                       "{y}:" + sha256(block-id sequence))
  block-id sequence = "|".join(palette[unpacked_idx[i]] for i in 0..4095)
  → инвариантен к reorder палитры/упаковке (бенигн-недетерминизм Paper #14125),
    чувствителен к любому изменению декодированного блока.
  raw(c) = world_diff_parity_v2._canon_digest (палитра-строка + data longs) — теневой.

КОГДА (фазы съёма):
  T0 gen-from-scratch, Status==full (ПОСЛЕ placement структур), ДО сейва и ДО тиков
  (тик-плоскость мутирует блоки и отравит дайджест — тики покрыты parity-v2 P6).
  В CI: две ноги gen-from-scratch (region_threads=0 серийная vs lever), один сид,
  артефакты = оба мира + report. Интеграция: фаза P7-STRUCT после P6-PARITY
  (SKIP-able как P6), либо standalone до мерджа structgen-рычага.

CLI:
  --selftest                        офлайн-фикстуры 13×13, вердикты с числами
  --world-a DIR --world-b DIR       реальные два мира (region/), [--anchor X Z],
                                    [--level-a F --level-b F] для G-S1, [--json F]
Exit: 0 GREEN / 1 RED / 2 usage. Зависимости: только stdlib + world_diff_parity_v2.py.
"""
import argparse
import gzip
import hashlib
import importlib.util
import json
import os
import struct
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location(
    "wdp2", os.path.join(HERE, "world_diff_parity_v2.py"))
wdp2 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(wdp2)

PROBE_R = 6                                  # preregister: окно 13×13
PROBE_TOTAL = (2 * PROBE_R + 1) ** 2         # 169 — канон Paper #14125 0/169
SEM_PREFIX = "S12v1|"
OK, FAIL = "OK", "FAIL"


# ----------------------------------------------------------------------------
# canon дайджестов
# ----------------------------------------------------------------------------

def probe_window(anchor=(0, 0)):
    ax, az = anchor
    return [(ax + dx, az + dz)
            for dx in range(-PROBE_R, PROBE_R + 1)
            for dz in range(-PROBE_R, PROBE_R + 1)]


def semantic_chunk_digest(cx, cz, root):
    """Семантический per-чанк sha256: декодированная последовательность block-id."""
    h = hashlib.sha256()
    h.update(f"{SEM_PREFIX}{cx},{cz},{wdp2.chunk_status(root)}".encode())
    secs = wdp2.chunk_sections(root)
    n_pal = 0
    for y in sorted(secs):
        pal, data = secs[y]
        n_pal = max(n_pal, len(pal))
        idx = wdp2.unpack_indices(len(pal), data, 4096)
        hi = hashlib.sha256()
        last = len(pal) - 1
        hi.update("|".join(pal[min(i, last)] for i in idx).encode())
        h.update(f"{y}:{hi.hexdigest()};".encode())
    return h.hexdigest(), n_pal


def raw_chunk_digest(root):
    """Теневой raw-дигест (canon parity-v2 _canon_digest) — НЕ pass-доказательство."""
    secs = wdp2.chunk_sections(root)
    h = hashlib.sha256()
    for y in sorted(secs):
        h.update(f"{y}:{wdp2._canon_digest(*secs[y])};".encode())
    return h.hexdigest()


def structure_markers(root):
    """Дайджест structure-starts. Пусто = нет тега structures."""
    st = root.get("structures")
    if not isinstance(st, dict):
        return ""
    parts = []
    starts = st.get("starts")
    if isinstance(starts, dict):
        for k in sorted(starts):
            parts.append(str(k))
            v = starts[k]
            if isinstance(v, dict):
                for pk in ("id", "ChunkX", "ChunkZ"):
                    if v.get(pk) is not None:
                        parts.append(f"{pk}={v[pk]}")
                ch = v.get("Children")
                parts.append(f"pieces={len(ch) if isinstance(ch, list) else '?'}")
                bb = v.get("bbox")
                if bb is not None:
                    parts.append(f"bbox={sorted(bb.items()) if isinstance(bb, dict) else bb}")
    refs = st.get("References", st.get("references"))
    if isinstance(refs, dict):
        for k in sorted(refs):
            parts.append(f"ref:{k}")
    h = hashlib.sha256()
    h.update("|".join(parts).encode())
    return h.hexdigest()


def read_seed(world_dir):
    """level.dat -> seed (TAG_Long|TAG_Int толерантно). None = нет/битый."""
    p = os.path.join(world_dir, "level.dat")
    if not os.path.isfile(p):
        return None
    try:
        with open(p, "rb") as f:
            root = wdp2.read_nbt(gzip.decompress(f.read()), 0)
        data = root.get("Data")
        if not isinstance(data, dict):
            return None
        wgs = data.get("WorldGenSettings")
        if isinstance(wgs, dict) and wgs.get("seed") is not None:
            return int(wgs["seed"])
        for k in ("RandomSeed", "seed"):
            if data.get(k) is not None:
                return int(data[k])
    except Exception:  # noqa: BLE001 — level.dat не обязателен (фикстуры)
        return None
    return None


# ----------------------------------------------------------------------------
# gate
# ----------------------------------------------------------------------------

def run_gate(a_region, b_region, anchor=(0, 0), seed_a=None, seed_b=None):
    """Пarts: region-каталоги. Возвращает verdict-словарь (canon G-S1..G-S5)."""
    t0 = time.perf_counter()
    A = wdp2.scan_region_dir(a_region)
    B = wdp2.scan_region_dir(b_region)
    coords = probe_window(anchor)
    matched = 0
    diffs, missing = [], []
    blind_raw_mismatch = 0
    markers_a, markers_b = {}, {}
    for c in coords:
        ra, rb = A.get(c), B.get(c)
        if ra is None or rb is None:
            missing.append(c)
            continue
        da, _ = semantic_chunk_digest(c[0], c[1], ra)
        db, _ = semantic_chunk_digest(c[0], c[1], rb)
        if da == db:
            matched += 1
        else:
            diffs.append(c)
        if raw_chunk_digest(ra) != raw_chunk_digest(rb):
            blind_raw_mismatch += 1
        markers_a[c] = structure_markers(ra)
        markers_b[c] = structure_markers(rb)
    markers_equal = (missing == [] and markers_a == markers_b)
    marker_diffs = sorted(c for c in coords
                          if c not in missing and markers_a[c] != markers_b[c])
    ms = int((time.perf_counter() - t0) * 1000)
    seed_equal = (seed_a is None and seed_b is None) or (seed_a == seed_b)
    verdict = "STRUCT-INTEGRITY-OK" if (
        not missing and matched == PROBE_TOTAL and markers_equal and seed_equal
    ) else "STRUCT-INTEGRITY-FAIL"
    if not seed_equal:
        verdict = "STRUCT-NOTCOMPARABLE"      # G-S1: не судим мир — судим сцены
    return {
        "verdict": verdict, "probe_total": PROBE_TOTAL, "matched": matched,
        "diffs": diffs[:8], "diff_count": len(diffs), "missing": missing[:8],
        "missing_count": len(missing), "markers_equal": markers_equal,
        "marker_diffs": marker_diffs[:8], "blind_raw_mismatch": blind_raw_mismatch,
        "blind_raw_total": PROBE_TOTAL - len(missing),
        "seed_a": seed_a, "seed_b": seed_b, "seed_equal": seed_equal, "ms": ms,
    }


def render(r):
    return (f"{r['verdict']} matched={r['matched']}/{r['probe_total']} "
            f"markers={'EQUAL' if r['markers_equal'] else 'DIFF'} "
            f"seed={'OK' if r['seed_equal'] else 'DIFF'} "
            f"blind_raw={r['blind_raw_mismatch']}/{r['blind_raw_total']} "
            f"ms={r['ms']}")


# ----------------------------------------------------------------------------
# fixture-хелперы (selftest) — 13×13 окно в r.0.0.mca
# ----------------------------------------------------------------------------

def _pack_indices(pal_len, idx):
    """Обратное unpack_indices (та же раскладка: биты >= 4, вход не пересекает long)."""
    bits = max(4, (pal_len - 1).bit_length())
    epl = 64 // bits
    acc = [0] * ((4096 + epl - 1) // epl)
    for i, v in enumerate(idx):
        li, sh = divmod(i, epl)
        acc[li] |= v << (sh * bits)
    return [v - (1 << 64) if v >= (1 << 63) else v for v in acc]


BASE_PAL = ["minecraft:stone", "minecraft:dirt", "minecraft:oak_planks"]


def _base_indices(mutation=None):
    idx = [i % 3 for i in range(4096)]
    if mutation == "dirt":                    # T2: блок-мутация в (0,0)
        idx = [1 if v == 0 else v for v in idx]
    return idx


def _chunk_bytes(cx, cz, pal, data_long, structures=None):
    pal_compounds = [wdp2._w_payload(10, {"Name": (8, p)}) for p in pal]
    bs = wdp2._w_payload(10, {"palette": (9, (10, pal_compounds)),
                              "data": (12, data_long)})
    sec = wdp2._w_payload(10, {"Y": (3, 0), "block_states": (10, bs)})
    root = {"DataVersion": (3, 4189), "xPos": (3, cx), "zPos": (3, cz),
            "Status": (8, "full"), "sections": (9, (10, [sec]))}
    if structures is not None:
        root["structures"] = (10, structures)
    return wdp2._named_root(root)


def synth_window(dirpath, reorder=False, mutate_chunk=None, struct_chunk=None):
    """13×13 = 169 чанков, один region. reorder = бенигн-перестановка палитры
    (декодированная последовательность блоков ИДЕНТИЧНА — Paper #14125 класс)."""
    os.makedirs(dirpath, exist_ok=True)
    pal = BASE_PAL
    if reorder:  # та же декодированная послед-сть: stone<-0, dirt<-1, planks<-2
        pal = [BASE_PAL[2], BASE_PAL[0], BASE_PAL[1]]  # new_idx: 0=planks 1=stone 2=dirt
        remap = {0: 1, 1: 2, 2: 0}
    chunks = {}
    for cx, cz in probe_window((0, 0)):
        idx = _base_indices()
        if reorder:
            idx = [remap[v] for v in idx]
        if mutate_chunk is not None and (cx, cz) == mutate_chunk:
            idx = _base_indices("dirt")
            if reorder:
                idx = [remap[v] for v in idx]
        structures = None
        if struct_chunk is not None and (cx, cz) == struct_chunk:
            structures = {"starts": (10, {
                "minecraft:village": (10, {"id": (8, "minecraft:village"),
                                           "ChunkX": (3, cx), "ChunkZ": (3, cz),
                                           "pieces": (3, 1)})})}
        chunks[(cx, cz)] = _chunk_bytes(cx, cz, pal, _pack_indices(len(pal), idx),
                                        structures)
    wdp2.write_region(os.path.join(dirpath, "r.0.0.mca"), chunks)


def write_level_dat(world_dir, seed):
    os.makedirs(world_dir, exist_ok=True)
    root = {"Data": (10, {"RandomSeed": (3, int(seed))})}
    with open(os.path.join(world_dir, "level.dat"), "wb") as f:
        f.write(gzip.compress(wdp2._named_root(root)))


# ----------------------------------------------------------------------------
# selftest — preregistered проверки с числами
# ----------------------------------------------------------------------------

def self_test() -> int:
    import tempfile
    checks = []
    with tempfile.TemporaryDirectory(prefix="s12-structgate-") as td:
        # T0 pack/unpack roundtrip (инвариант канона)
        idx = _base_indices()
        rt = wdp2.unpack_indices(3, _pack_indices(3, idx), 4096)
        checks.append(("T0 pack/unpack roundtrip 4096 idx equal", rt == idx, ""))

        wa = os.path.join(td, "A", "region")
        wb = os.path.join(td, "B", "region")
        wc = os.path.join(td, "C", "region")
        wd = os.path.join(td, "D", "region")
        we = os.path.join(td, "E", "region")
        wf = os.path.join(td, "F", "region")
        synth_window(wa)                                   # серийная база
        synth_window(wb)                                   # повтор (детерминизм-канал)
        synth_window(wc, mutate_chunk=(0, 0))              # коррупция блока (0,0)
        synth_window(wd, struct_chunk=(1, 1))              # односторонний structure-start
        synth_window(we, reorder=True)                     # бенигн reorder палитры
        synth_window(wf, reorder=True, mutate_chunk=(3, -2))  # reorder+реальная коррупция

        sz = os.path.getsize(os.path.join(wa, "r.0.0.mca"))
        checks.append(("T1 identity: matched 169/169, blind_raw 0/169, OK", None, ""))
        r1 = run_gate(wa, wb)
        checks[-1] = ("T1 identity: matched 169/169, blind_raw 0/169, OK",
                      (r1["matched"] == 169 and r1["verdict"] == "STRUCT-INTEGRITY-OK"
                       and r1["blind_raw_mismatch"] == 0), render(r1))

        r2 = run_gate(wa, wc)
        cross = wdp2.diff_worlds(wa, wc)
        checks.append((
            "T2 corruption: 168/169 RED, локализация (0,0), blocks_diff 1366",
            (r2["matched"] == 168 and r2["verdict"] == "STRUCT-INTEGRITY-FAIL"
             and r2["diff_count"] == 1 and r2["diffs"][:1] == [(0, 0)]
             and cross["blocks_diff"] == 1366),
            render(r2) + f" | wdp2 blocks_diff={cross['blocks_diff']}"))

        r3 = run_gate(wa, wd)
        checks.append((
            "T3 markers: content 169/169 НО односторонний старт → FAIL",
            (r3["matched"] == 169 and r3["verdict"] == "STRUCT-INTEGRITY-FAIL"
             and r3["markers_equal"] is False and r3["marker_diffs"][:1] == [(1, 1)]),
            render(r3)))

        r4 = run_gate(wa, we)
        checks.append((
            "T4 benign reorder: semantic 169/169 OK, raw-слепота 169/169",
            (r4["matched"] == 169 and r4["verdict"] == "STRUCT-INTEGRITY-OK"
             and r4["blind_raw_mismatch"] == 169),
            render(r4)))

        r5 = run_gate(wa, wf)
        checks.append((
            "T5 reorder+коррупция: semantic локализует (3,-2), raw слеп 168/169+",
            (r5["matched"] == 168 and r5["diffs"][:1] == [(3, -2)]
             and r5["blind_raw_mismatch"] >= 168),
            render(r5)))

        write_level_dat(os.path.join(td, "GA"), 470)
        write_level_dat(os.path.join(td, "GB"), 471)
        s_a = read_seed(os.path.join(td, "GA"))
        s_b = read_seed(os.path.join(td, "GB"))
        r6 = run_gate(wa, wb, seed_a=s_a, seed_b=s_b)
        checks.append((
            "T6 seed-identity: 470≠471 → STRUCT-NOTCOMPARABLE (G-S1)",
            (s_a == 470 and s_b == 471 and r6["verdict"] == "STRUCT-NOTCOMPARABLE"),
            render(r6)))

        checks.append((
            f"T7 fixture: 169 чанков = 1 region ({sz} B), тайминг gate ≤ 30s",
            (len(wdp2.scan_region_dir(wa)) == 169 and sz > 8192 and r4["ms"] < 30000),
            f"region={sz}B ms={r4['ms']}"))

    failed = 0
    for name, ok, detail in checks:
        status = "?" if ok is None else ("PASS" if ok else "FAIL")
        if ok is False:
            failed += 1
        print(f"[{status}] {name}" + (f" :: {detail}" if detail else ""))
    print(f"STRUCT-GATE selftest: {len(checks) - failed}/{len(checks)} GREEN")
    return 1 if failed else 0


# ----------------------------------------------------------------------------
# CLI
# ----------------------------------------------------------------------------

def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description="S12 STRUCT-INTEGRITY GATE v1")
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--world-a")
    ap.add_argument("--world-b")
    ap.add_argument("--level-a")
    ap.add_argument("--level-b")
    ap.add_argument("--anchor", nargs=2, type=int, default=[0, 0], metavar=("X", "Z"))
    ap.add_argument("--json")
    args = ap.parse_args(argv)

    if args.selftest:
        return self_test()
    if not args.world_a or not args.world_b:
        ap.print_help()
        return 2
    sa = read_seed(args.level_a or args.world_a)
    sb = read_seed(args.level_b or args.world_b)
    r = run_gate(args.world_a, args.world_b, anchor=tuple(args.anchor),
                 seed_a=sa, seed_b=sb)
    print(render(r))
    if r["diffs"]:
        print("top-diffs (semantic): " + ", ".join(f"({x},{z})" for x, z in r["diffs"]))
    if r["marker_diffs"]:
        print("marker-diffs: " + ", ".join(f"({x},{z})" for x, z in r["marker_diffs"]))
    if args.json:
        with open(args.json, "w") as f:
            json.dump(r, f, indent=1)
    return 0 if r["verdict"] == "STRUCT-INTEGRITY-OK" else 1


if __name__ == "__main__":
    sys.exit(main())
