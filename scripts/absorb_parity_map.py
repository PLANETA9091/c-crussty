#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
absorb_parity_map.py — EXPOSED-маппинг parity-вердиктов для absorb-канона
(TASK T-472-S45, клейм ABSORB-PARSER-OK-EXPOSED, ROUND-472, тик 08:08+08 2026-09-27).

ПРОБЛЕМА (следующий ботлнек S34): absorb-канон
(/home/z/scripts/absorb_472.py тик-парсер + scripts/absorb_464.py STW-ценз M1)
не знает вердикта WORLD-PARITY-OK-EXPOSED (P6-спека v2, арбитр
scripts/world_diff_parity_v2.py, ветка round-472-s34-parityv2@17f7eafd).
Легаси-триаж `v.endswith("OK")` / точное сравнение == "WORLD-PARITY-OK"
классифицирует OK-EXPOSED как не-OK → пары зоны B Δcpu∈(184154, 2290829]
ложноразбраковываются в REFUTED в runs.jsonl (ложный parity-FAIL).

ФИКС: единый канон-маппинг verdict → runs.jsonl-тег + PASS-класс:
  WORLD-PARITY-OK            → PARITY-OK        pass=True  exit 0  (зона A)
  WORLD-PARITY-OK-EXPOSED    → OK-EXPOSED       pass=True  exit 0  (зона B, тег обязателен)
  WORLD-PARITY-FAIL          → REFUTED-PARITY   pass=False exit 1  (легитимный лов lever-дрейфа)
  WORLD-PARITY-FAIL-UNPAIRED → REFUTED-UNPAIRED pass=False exit 1  (зона C)
  WORLD-PARITY-SKIP / нет данных → PARITY-SKIP / PARITY-UNKNOWN pass=None
  (анти-ложный-REFUTED: skip/незнание НЕ равен провалу; REFUTED ставится ТОЛЬКО
   по FAIL/FAIL-UNPAIRED — правило anti_false_refuted ниже).

ЗОНЫ-ТАБЛИЦА P6-v2 (docs/P6_PARITY_GATE_V2_SPEC.md §3–§4, S34):
  A: Δcpu ≤ 184154            → plain OK (tol 5%/floor 64)
  B: 184154 < Δcpu ≤ 2290829  → dilate husk 9% / spider 8% → OK-EXPOSED (plain OK запрещён)
  C: Δcpu > 2290829           → FAIL-UNPAIRED

CLI:
  python3 scripts/absorb_parity_map.py --map <parity.json> [...]      # маппинг пар
  python3 scripts/absorb_parity_map.py --verify-s34 [DIR]             # 5 пар S34, 0 ложных REFUTED
  python3 scripts/absorb_parity_map.py --selftest                     # юнит-инварианты маппинга
  python3 scripts/absorb_parity_map.py --zones-table                  # печать зоны-таблицы P6-v2
"""
from __future__ import annotations

import json
import os
import sys
from typing import Dict, List, Optional, Tuple

# ---- P6-спека v2: константы-близнецы world_diff_parity_v2.py (S34@17f7eafd) ----
CPU_ZONE_A_CEILING = 184_154
CPU_ZONE_B_CEILING = 2_290_829
ZONE_B_DILATE = {"minecraft:husk": 0.09, "minecraft:spider": 0.08}
PER_TYPE_BAND_B_PCT = {k: v * 100 for k, v in ZONE_B_DILATE.items()}  # для зон-таблицы §4

# ---- канон-маппинг вердикт → (тег runs.jsonl, PASS-класс, exit) ----
VERDICT_MAP: Dict[str, Tuple[str, Optional[bool], int]] = {
    "WORLD-PARITY-OK":            ("PARITY-OK",        True,  0),
    "WORLD-PARITY-OK-EXPOSED":    ("OK-EXPOSED",       True,  0),
    "WORLD-PARITY-FAIL":          ("REFUTED-PARITY",   False, 1),
    "WORLD-PARITY-FAIL-UNPAIRED": ("REFUTED-UNPAIRED", False, 1),
    "WORLD-PARITY-SKIP":          ("PARITY-SKIP",      None,  3),
}
UNKNOWN_TAG = ("PARITY-UNKNOWN", None, None)  # НЕ REFUTED (анти-ложный-REFUTED)


def map_verdict(verdict: str) -> Dict:
    """Вердикт арбитра v2 → {tag, pass, exit} для absorb/runs.jsonl-триажа."""
    tag, ok, code = VERDICT_MAP.get(verdict, UNKNOWN_TAG)
    return {"verdict": verdict, "tag": tag, "pass": ok, "exit": code}


def anti_false_refuted(verdict: str) -> bool:
    """True = вердикт-тег обязан быть REFUTED*-формы. Всё PASS/UNKNOWN — False."""
    m = map_verdict(verdict)
    return m["tag"].startswith("REFUTED")


def cpu_zone(delta_cpu: Optional[int]) -> Tuple[str, Optional[int]]:
    """Δcpu → (зона 'A'|'B'|'C'|'?', Δcpu). Нет данных → ('?', None) — не FAILED."""
    if delta_cpu is None:
        return "?", None
    if delta_cpu <= CPU_ZONE_A_CEILING:
        return "A", delta_cpu
    if delta_cpu <= CPU_ZONE_B_CEILING:
        return "B", delta_cpu
    return "C", delta_cpu


ZONES_TABLE_P6V2 = [
    # зона, диапазон Δcpu, правило, вердикт, W9-статус (спека §3)
    ("A", "Δcpu ≤ 184,154", "канон tol=5%/floor=64", "WORLD-PARITY-OK", "INFO"),
    ("B", "184,154 < Δcpu ≤ 2,290,829",
     "dilate husk=9%/spider=8%, прочие 5% (floor 64); plain OK ЗАПРЕЩЁН",
     "WORLD-PARITY-OK-EXPOSED", "WARN"),
    ("C", "Δcpu > 2,290,829", "за потолком калибровки — пара не верифицируема",
     "WORLD-PARITY-FAIL-UNPAIRED", "FAIL"),
]
# Δcpu-потолок по типам (спека §4): дрейф @2.107M OK-EXPOSED / @2.259M / @2.291M FAIL
PER_TYPE_CALIBRATION = [
    ("minecraft:husk",       "9% dilate",  "+7.03% OK-EXPOSED", "+9.03% FAIL", "+10.02% FAIL", "churn-носитель"),
    ("minecraft:spider",     "8% dilate",  "+6.70% OK-EXPOSED", "+12.74% FAIL", "+14.05% FAIL", "churn-носитель"),
    ("minecraft:item",       "5%",         "+4.06% OK",         "+6.68% FAIL",  "+7.80% FAIL",  "band-защита 5171"),
    ("minecraft:item_frame", "5%+floor",   "0.00%",             "0.00%",        "0.00%",        "статика, иммунна"),
]


def zones_table_text() -> str:
    L = ["## Зоны Δcpu P6-v2 (спека §3)", "", "| зона | диапазон | правило | вердикт | W9 |",
         "|---|---|---|---|---|"]
    L += ["| %s | %s | %s | %s | %s |" % z for z in ZONES_TABLE_P6V2]
    L += ["", "## Δcpu-потолок по типам (спека §4)", "",
          "| тип | band B | @2.107M | @2.259M | @2.291M | роль |", "|---|---|---|---|---|---|"]
    L += ["| %s | %s | %s | %s | %s | %s |" % r for r in PER_TYPE_CALIBRATION]
    return "\n".join(L)


def map_pair_json(js: dict) -> Dict:
    """parity-JSON арбитра v2 → строка маппинга с кросс-чеком зоны.

    Кросс-чек: plain OK обязан быть зоной A; OK-EXPOSED — зоной B; FAIL-UNPAIRED — C.
    Расхождение = флаг inconsistent (триаж обязан остановиться, не домыслить).
    """
    verdict = js.get("verdict", "")
    m = map_verdict(verdict)
    van = (js.get("vanilla") or {}).get("runner_cpu_index")
    leg = (js.get("leg") or {}).get("runner_cpu_index")
    delta = abs(int(van) - int(leg)) if (van is not None and leg is not None) else None
    zone, delta = cpu_zone(delta)
    expected = {"A": "WORLD-PARITY-OK", "B": "WORLD-PARITY-OK-EXPOSED",
                "C": "WORLD-PARITY-FAIL-UNPAIRED", "?": None}[zone]
    if expected is not None and m["pass"] is not False:
        m["inconsistent"] = verdict != expected
    elif zone == "C" and verdict == "WORLD-PARITY-FAIL":
        m["inconsistent"] = False          # FAIL в зоне C допускается до гейта W9
    m["zone"] = zone
    m["delta_cpu"] = delta
    m["dilate"] = ZONE_B_DILATE if zone == "B" else None
    return m


def triage_pair(parity_json_path: str) -> Dict:
    """Файл parity-JSON → вердикт-тег runs.jsonl. Легаси-сравнение приложено как
    regression-зонд: legacy_endswith_OK == False на OK-EXPOSED = баг S34-ботлнека."""
    js = json.load(open(parity_json_path))
    m = map_pair_json(js)
    m["legacy_endswith_OK_false_REFUTED"] = (not str(m["verdict"]).endswith("OK")
                                             and m["verdict"] == "WORLD-PARITY-OK-EXPOSED")
    return m


# ----------------------------------------------------------------------------
# verify-s34: 5 пар S34 → 0 ложных REFUTED (главный приёмочный гейт S45)
# ----------------------------------------------------------------------------
EXPECTED_S34 = {
    "p6v2_S88":   ("WORLD-PARITY-OK",         "PARITY-OK",        "A", True),
    "p6v2_AA":    ("WORLD-PARITY-OK",         "PARITY-OK",        "A", True),
    "p6v2_EXPO":  ("WORLD-PARITY-OK-EXPOSED", "OK-EXPOSED",       "B", True),
    "p6v2_ADAPT": ("WORLD-PARITY-FAIL",       "REFUTED-PARITY",   "B", False),
    "p6v2_N32B":  ("WORLD-PARITY-FAIL",       "REFUTED-PARITY",   "B", False),
}


def verify_s34(fix_dir: str) -> Tuple[int, List[str]]:
    lines, fails = [], 0
    false_refuted = 0
    for name, (vd, tag, zone, ok) in EXPECTED_S34.items():
        p = os.path.join(fix_dir, name + ".json")
        if not os.path.exists(p):
            print(f"MISS {name}: нет {p}"); fails += 1; continue
        m = triage_pair(p)
        checks = {
            "verdict": m["verdict"] == vd,
            "tag": m["tag"] == tag,
            "zone": m["zone"] == zone,
            "pass_class": m["pass"] is ok,
            "consistent": not m.get("inconsistent", False),
        }
        if ok and m["tag"].startswith("REFUTED"):
            false_refuted += 1
        bad = [k for k, v in checks.items() if not v]
        status = "OK " if not bad else "FAIL"
        if bad: fails += 1
        lines.append(f"{status} {name}: Δcpu={m['delta_cpu']} зона {m['zone']} → "
                     f"{m['verdict']} → тег [{m['tag']}] pass={m['pass']}"
                     + (f"  !! {bad}" if bad else ""))
    lines.append(f"false_REFUTED (PASS-класс помечен REFUTED): {false_refuted}")
    ok_all = fails == 0 and false_refuted == 0
    lines.append("VERIFY-S34: " + ("PASS 5/5, 0 ложных REFUTED" if ok_all else f"FAIL ({fails})"))
    return (0 if ok_all else 1), lines


def selftest() -> int:
    # 1. канон-маппинг
    assert map_verdict("WORLD-PARITY-OK")["tag"] == "PARITY-OK"
    assert map_verdict("WORLD-PARITY-OK-EXPOSED")["tag"] == "OK-EXPOSED"
    assert map_verdict("WORLD-PARITY-OK-EXPOSED")["pass"] is True      # ключ S45
    assert map_verdict("WORLD-PARITY-OK-EXPOSED")["exit"] == 0
    assert map_verdict("WORLD-PARITY-FAIL")["tag"] == "REFUTED-PARITY"
    assert map_verdict("WORLD-PARITY-FAIL-UNPAIRED")["tag"] == "REFUTED-UNPAIRED"
    assert map_verdict("что-то-новое")["tag"] == "PARITY-UNKNOWN"      # не REFUTED
    assert not anti_false_refuted("WORLD-PARITY-OK-EXPOSED")
    assert anti_false_refuted("WORLD-PARITY-FAIL")
    # 2. легаси-баг воспроизводится (регрессия S34-ботлнека): endswith("OK") на EXPOSED = False
    assert not "WORLD-PARITY-OK-EXPOSED".endswith("OK")
    # 3. зоны-границы точно по калибровке S34
    assert cpu_zone(184_154)[0] == "A" and cpu_zone(184_155)[0] == "B"
    assert cpu_zone(2_290_829)[0] == "B" and cpu_zone(2_290_830)[0] == "C"
    assert cpu_zone(2_107_395)[0] == "B" and cpu_zone(None)[0] == "?"
    # 4. кросс-чек зоны против вердикта
    js = {"verdict": "WORLD-PARITY-OK-EXPOSED",
          "vanilla": {"runner_cpu_index": "6573789"}, "leg": {"runner_cpu_index": "8681184"}}
    assert map_pair_json(js)["zone"] == "B" and map_pair_json(js)["delta_cpu"] == 2_107_395
    js["verdict"] = "WORLD-PARITY-OK"
    assert map_pair_json(js).get("inconsistent") is True   # plain OK в зоне B = инконсистентно
    print("SELFTEST: PASS (маппинг 6/6, зоны-границы, легаси-баг воспроизведён)")
    return 0


def main(argv: List[str]) -> int:
    if "--selftest" in argv:
        return selftest()
    if "--zones-table" in argv:
        print(zones_table_text()); return 0
    if "--verify-s34" in argv:
        i = argv.index("--verify-s34")
        fix = argv[i + 1] if len(argv) > i + 1 else os.path.join(
            os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
            "tests", "fixtures", "s45_p6v2")
        code, lines = verify_s34(fix)
        print("\n".join(lines)); return code
    paths = [a for a in argv[1:] if not a.startswith("-")]
    if not paths:
        print(__doc__); return 2
    for p in paths:
        m = triage_pair(p)
        print(json.dumps(m, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
