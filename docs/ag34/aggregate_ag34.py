#!/usr/bin/env python3
"""aggregate_ag34.py — сборка GATE-3 банка волны-517 (AG-34).
Читает norm_batch_*.jsonl (normtool_478, --biomes-exempt) + известные нормированные леги
GATE-3 из леджера → топ-леги, min-of-3 топ-3 vs leg_req 22.74, вердикт №24.
"""
import json, glob, statistics

LEG_REQ = 22.74  # 20 + max(anchor ax52 +2.74), LEDGER ×514, запас +0.16

# Нормированные GATE-3 леги (lever cmp456_chunkmono_p31snap @3fefb39; из clm/AG-19 ×516 и леджера)
KNOWN = [
    {"run_id": 36756762341, "seed": 1833, "src": "AG-80 leg-B", "norm_v5": 27.85, "cpu": 6_910_000},
    {"run_id": 36756762340, "seed": 1832, "src": "AG-80 leg-A", "norm_v5": -1.49, "cpu": 8_460_000},
    {"run_id": 0, "seed": 1848, "src": "AG-81", "norm_v5": 10.69, "cpu": 7_220_000},
    {"run_id": 0, "seed": 1849, "src": "AG-81", "norm_v5": 17.27, "cpu": 6_540_000},
    {"run_id": 0, "seed": 1670, "src": "AG-50-компо", "norm_v5": 11.85, "cpu": 6_630_000},
    # леджер ×512-515 (чистые NORM-леги того же носителя/класса №24)
    {"run_id": 0, "seed": 0, "src": "P31-IB leg-1 rep1", "norm_v5": 25.51, "cpu": 9_011_778},
    {"run_id": 0, "seed": 0, "src": "P31-IB leg-1 rep2", "norm_v5": 20.93, "cpu": 6_656_157},
    {"run_id": 0, "seed": 0, "src": "P31-IB leg-1 rep3", "norm_v5": 15.52, "cpu": 6_745_145},
    {"run_id": 0, "seed": 0, "src": "ib4", "norm_v5": 19.55, "cpu": 6_817_717},
    {"run_id": 0, "seed": 0, "src": "ib5", "norm_v5": 13.61, "cpu": 0},
    {"run_id": 0, "seed": 0, "src": "ib6", "norm_v5": 21.65, "cpu": 0},
    {"run_id": 0, "seed": 0, "src": "ib1", "norm_v5": 22.77, "cpu": 0},
    {"run_id": 0, "seed": 0, "src": "ib9", "norm_v5": 18.02, "cpu": 0},
    {"run_id": 0, "seed": 0, "src": "cbcre1", "norm_v5": 14.78, "cpu": 0},
    {"run_id": 0, "seed": 0, "src": "cbcre2", "norm_v5": 20.87, "cpu": 0},
]

rows = []
for f in sorted(glob.glob("norm_batch_*.jsonl")):
    for line in open(f):
        line = line.strip()
        if not line:
            continue
        d = json.loads(line)
        rows.append(d)

print(f"== normtool rows: {len(rows)} ==")
for r in rows:
    print(json.dumps({k: r.get(k) for k in
        ("run_id", "verdict", "m1_state", "cpu_index", "tps_med", "tps_exp_v5",
         "norm_v5", "n_polls_valid", "poll_source",
         "aioobe_biome", "aioobe_other", "biomes_exempt_applied")},
        ensure_ascii=False))
    sc = r.get("spark_crosscheck") or {}
    print(f"   spark: norm_spark={sc.get('norm_spark')} div={sc.get('divergence_pp')} "
          f"polls={r.get('raw_polls_bottlenecks')}")

# Пул сертификационных легов №24: только CLEAN NORM-COMPUTED (host-ценз и UNKNOWN не считаются)
mine = [{"run_id": r["run_id"], "norm_v5": r.get("norm_v5"), "verdict": r["verdict"],
         "cpu": r.get("cpu_index"), "m1": r.get("m1_state"), "src": "AG-34-x517"}
        for r in rows if r.get("verdict") == "NORM-COMPUTED" and r.get("m1_state") == "CLEAN"
        and r.get("norm_v5") is not None]

pool = KNOWN + mine
pool_sorted = sorted(pool, key=lambda x: -x["norm_v5"])
print("\n== Пул №24 (все известные леги, топ-12) ==")
for p in pool_sorted[:12]:
    print(f"  {p['norm_v5']:+7.2f}  {p.get('src','?')}  run={p['run_id']} cpu={p.get('cpu')}")

top3 = pool_sorted[:3]
top3min = min(p["norm_v5"] for p in top3) if len(top3) >= 3 else None
fire = [p for p in mine if p["norm_v5"] >= LEG_REQ]
print(f"\ntop-3 min = {top3min} vs leg_req {LEG_REQ}")
print(f"мои новые FIRE-леги (≥{LEG_REQ}, CLEAN): {len(fire)}")
if top3min is not None and top3min >= LEG_REQ:
    print("GATE-3 №24: ЗАКРЫТ (top-3 min ≥ leg_req) → MERGE-CANDIDATE-сертификат")
else:
    print(f"GATE-3 №24: НЕ закрыт (дефицит {round(LEG_REQ - top3min, 2) if top3min else 'n/a'}пп)")

with open("ag34_gate3_pool.json", "w") as fh:
    json.dump({"mine": mine, "pool_top12": pool_sorted[:12],
               "top3_min": top3min, "leg_req": LEG_REQ}, fh, ensure_ascii=False, indent=1)
print("\nsaved ag34_gate3_pool.json")
