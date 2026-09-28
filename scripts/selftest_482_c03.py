#!/usr/bin/env python3
"""selftest_482_c03.py — [482-C03] replay selftest патчей normtool_478 на распарсенных
данных ROUND-482 (absorb_482.json + r364*/gc.log), закон B3: selftest перед вердиктом.

Патчи:
  [482-C03.1] m1 fail-closed TRI-STATE (UNKNOWN: gc.log нет / 0 completion-строк /
              young_n==0 при stw_total>0 → m1_clean=False, не PASS);
  [482-C03.2] biomes-exempt (биом-AIOOBE probe-класс exempt, прецедент Л-474-C88.2).

Отвечает: сколько ранов окна ×482 меняют класс после патчей; не перецензуриваются ли
HOST-CENS ×21 / CORRIDOR-BREACH ×26 (не снимает ли exempt чужие цензы).
"""
import glob, json, os, sys, zipfile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from normtool_478 import gc_canon, RE_BIOME_AIOOBE, STW_MAX_S, YOUNG_MAX_MS

OUT = "/home/z/rounds/ROUND-482/absorb"
# Проверенный артефакт единственного AIOOBE-рана окна (re-download 10966816511):
# строка "java.lang.ArrayIndexOutOfBoundsException: Index 6 out of bounds for
# length 6" @ CollectingNeighborUpdater (redstone/СТЗ-класс) — НЕ биом-probe.
AIOOBE_ZIP = "/tmp/a36412051936.zip"


def m1_state_new(gc):
    """[482-C03.1] канон tri-state — зеркало parse_bundle."""
    if not gc or gc.get("pauses", 0) == 0:
        return "UNKNOWN"
    if gc["total_ms"] > STW_MAX_S * 1000.0:
        return "CENS"
    if gc["young"] == 0:
        return "UNKNOWN"
    if gc["young_sum_ms"] / gc["young"] > YOUNG_MAX_MS:
        return "CENS"
    return "CLEAN"


def gc_from_json(v):
    """gc-канон из полей absorb_482.json (эквивалент gc_canon по young/full)."""
    yn, fu = v.get("young_n"), v.get("fulls", 0)
    if yn is None:
        return None
    return {"pauses": yn + fu, "young": yn, "full": fu,
            "total_ms": v.get("stw_total_s", 0.0) * 1000.0,
            "young_sum_ms": v.get("young_avg_ms", 0.0) * yn,
            "max_ms": v.get("max_ms", 0.0)}


def main():
    d = json.load(open(f"{OUT}/absorb_482.json"))
    gc_dir = {}
    for g in glob.glob(f"{OUT}/r364*/gc.log"):
        gc_dir[os.path.basename(os.path.dirname(g))] = gc_canon(
            open(g, errors="replace").read())

    # --- биом-классификация AIOOBE-ранов окна [482-C03.2] ---
    aioobe_sig = {}
    if os.path.exists(AIOOBE_ZIP):
        with zipfile.ZipFile(AIOOBE_ZIP) as z:
            log = next((z.read(n).decode(errors="replace")
                        for n in z.namelist() if n.endswith("server-stdout.log")), "")
        aioobe_sig["r36412051936"] = ("biome-probe" if any(
            RE_BIOME_AIOOBE.search(l) for l in log.splitlines()
            if "ArrayIndexOutOfBoundsException" in l) else "other(real)")

    flips_m1, reclass, stats = [], [], {}
    for k, v in sorted(d.items()):
        if "class" not in v:
            continue
        gc = gc_dir.get(k) or gc_from_json(v)
        st = m1_state_new(gc)
        old_cls = v["class"]
        # absorb-таксономия с патчем: m1-PASS требует state==CLEAN
        m1_pass = (st == "CLEAN")
        corridor = v.get("corridor")
        vv = v.get("vanilla_valid")
        # [482-C03.2] vanilla_valid-нога: биом-probe exempt НЕ меняет здесь gate,
        # т.к. в окне ×482 0 биом-probe ранов (проверено ниже по aioobe_sig)
        aioobe_n = v.get("aioobe", 0)
        exempt_biome = aioobe_sig.get(k) == "biome-probe"
        aioobe_eff = 0 if (exempt_biome and aioobe_n <= 2) else aioobe_n
        if not v.get("band"):
            new_cls = "BAND-DEAD(free)"
        elif not m1_pass:
            new_cls = "M1-UNKNOWN(не-PASS)" if st == "UNKNOWN" else "HOST-CENS(M1)"
        elif vv and aioobe_eff == 0 and corridor:
            new_cls = ("STRICT-IN-POINT(банк-фид)" if v.get("strict")
                       else "VANILLA-VALID-§3(банк-фид)")
        elif not corridor:
            new_cls = "CORRIDOR-BREACH(инфра-ценз)"
        elif v.get("armed"):
            new_cls = "ARMED(leg/безумие)"
        else:
            new_cls = "UNCLASS"
        stats[old_cls] = stats.get(old_cls, 0) + 1
        if old_cls != new_cls:
            reclass.append((k, old_cls, new_cls, st))
        if v.get("m1") and st == "UNKNOWN":
            flips_m1.append((k, old_cls, gc.get("young") if gc else None))

    young_min_by_cls = {}
    for k, v in d.items():
        if "class" not in v:
            continue
        c = v["class"]
        yn = v.get("young_n")
        if yn is not None:
            young_min_by_cls[c] = min(young_min_by_cls.get(c, 10**9), yn)

    print(json.dumps({
        "classified_runs": sum(stats.values()),
        "class_dist_before": stats,
        "m1_fail_closed_flips_PASS_to_UNKNOWN": len(flips_m1),
        "m1_flips_detail": flips_m1[:10],
        "biomes_exempt_probe_runs_in_window": sum(
            1 for s in aioobe_sig.values() if s == "biome-probe"),
        "aioobe_runs_window": {k: s for k, s in aioobe_sig.items()},
        "reclassified_after_patch": len(reclass),
        "reclass_detail": reclass[:10],
        "young_n_min_by_class": young_min_by_cls,
        "verdict": "SELFTEST-482-C03: "
                   + ("OK — 0 переклассификаций, цензы ×21/×26 стоят честно"
                      if not reclass and not flips_m1 else "СМОТРИ reclass_detail"),
    }, ensure_ascii=False, indent=1))


if __name__ == "__main__":
    main()
