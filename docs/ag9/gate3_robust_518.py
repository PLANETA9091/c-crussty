#!/usr/bin/env python3
"""gate3_robust_518.py — AG-9 волна-518. Робастная pair-матем №24 GATE-3 на полном пуле
FIRE/leg-класса (bank norm_v5 + новые леги ×518) против anchor-пула 179 (AG-2 ⊕ bank_516).

Прeregistered гейты (SPEC-AG9-GATE3, волна-517):
  pair = leg_norm − anchor_norm ≥ +20, Δcpu ≤ 50k, min-of-3 (≥3 якоря в окне);
  best3_min  — lenient (стиль AG-2: топ-3 пары, их минимум);
  worst_min  — строгий worst-case (leg − max_anchor_norm в окне);
  ROBUST-серт = n_anchors≥3 ∧ worst_min ≥ 20 (все якоря окна дают пару ≥+20).
  leg_req №24 = 22.74 = 20 + max(anchor ax52 +2.74) (LEDGER ×514, AG-34).
"""
import json, os

A2 = "/home/z/rounds/ROUND-516/work/AG-2/"
A9 = "/home/z/rounds/ROUND-516/work/AG-9/"
G34 = "/home/z/c-crussty/docs/ag34/"
OUT = "/home/z/rounds/ROUND-518/work/AG-9/"

def norm_anchors(pool):
    out = {}
    if isinstance(pool, dict):
        pool = pool.get("anchors", [])
    for a in pool:
        if not isinstance(a, dict):
            continue
        rid = a.get("run_id") or a.get("anchor_run") or a.get("branch")
        cpu = a.get("cpu") or a.get("cpu_index") or a.get("anchor_cpu")
        nrm = a.get("norm") or a.get("norm_v5") or a.get("anchor_norm")
        if rid is None or cpu is None or nrm is None:
            continue
        out[rid] = {"cpu": cpu, "norm": nrm,
                    "src": a.get("src") or a.get("class") or a.get("branch") or "?"}
    return out

pool_a2 = json.load(open(A2 + "anchor_pool_510_513.json"))
pool_b516 = json.load(open(A9 + "bank_516.json"))
anchors = {**norm_anchors(pool_a2), **norm_anchors(pool_b516)}

# [×518 AG-9 fix] дедуп алиасов: «round-513-ax52» (branch) и 36723474488 (run_id) — ОДИН
# физический ран (cpu и norm совпадают до знака). Схлопываем по ключу (cpu, norm):
# иначе min-of-3 считается по дублям и ФАЛЬСИФИЦИРУЕТ сертификат (нашлось на leg
# 36765497995: n=3 «якоря» → реально 2 различных). Fail-closed в сторону честности.
seen, dedup = {}, {}
for rid, a in anchors.items():
    key = (round(a["cpu"]), round(a["norm"], 2))
    if key in seen:
        seen[key]["aliases"].append(rid)
        continue
    seen[key] = {**a, "aliases": [rid]}
    dedup[key] = True
anchors = {v["aliases"][0]: {"cpu": v["cpu"], "norm": v["norm"], "src": v["src"],
                             "aliases": v["aliases"]} for v in seen.values()}

legs = {}  # key -> leg dict
def add_leg(rid, cpu, norm, src, seed=None, m1="CLEAN", spark=None):
    """Fail-closed: при нескольких норм-чтениях одного run_id (C55 с/без first-poll,
    расхождение до 4.56пп) держим МИНИМАЛЬНЫЙ norm — консервативное чтение."""
    if cpu is None or norm is None or not cpu:
        return
    k = str(rid)
    if k in legs:
        old = legs[k]
        if norm < old["norm"]:
            legs[k] = {"run_id": rid, "cpu": cpu, "norm": norm, "src": src + "|min-read",
                       "seed": seed or old["seed"], "m1": m1, "spark_div": spark,
                       "norm_max_read": old["norm"]}
        else:
            old.setdefault("norm_max_read", max(norm, old["norm"]))
        return
    legs[k] = {"run_id": rid, "cpu": cpu, "norm": norm, "src": src,
               "seed": seed, "m1": m1, "spark_div": spark}

# 1) AG-2 certified legs (normtool-format)
for l in json.load(open(A2 + "cert_legs_final.json")):
    sp = (l.get("spark_crosscheck") or {}).get("divergence_pp")
    add_leg(l["run_id"], l.get("cpu_index"), l.get("norm_v5"), "AG-2-cert-x516",
            m1=l.get("m1_state"), spark=sp)

# 2) AG-34 gate3 pool: mine (12, x517 bank) + pool_top12 (partly без cpu)
g34 = json.load(open(G34 + "ag34_gate3_pool.json"))
for l in g34.get("mine", []):
    add_leg(l["run_id"], l.get("cpu"), l.get("norm_v5"), l.get("src", "AG-34-x517"), m1=l.get("m1"))
for l in g34.get("pool_top12", []):
    if l.get("cpu"):
        add_leg(l.get("run_id") or l.get("src"), l.get("cpu"), l.get("norm_v5"),
                l.get("src", "AG-34-top12"), seed=l.get("seed"))

# 2b) Полный leg-пул AG-2 (matrix 14 легов — все normtool-файлы, дедуп по run_id)
for fn in ["legs_n24_norm.json", "legs_n24_exempt.json", "legs_n24_retry.json", "legs_n24_retry_exempt.json"]:
    try:
        for l in json.load(open(A2 + fn)):
            sp = (l.get("spark_crosscheck") or {}).get("divergence_pp")
            add_leg(l["run_id"], l.get("cpu_index"), l.get("norm_v5"), "AG-2-" + fn[:-5],
                    m1=l.get("m1_state"), spark=sp)
    except Exception:
        pass

# 3) Новые леги ×518 (AG-9 харвест, normtool_478 selftest 3/3 + 9/9, --biomes-exempt)
for line in open(OUT + "legs_518_ag9_raw.jsonl"):
    line = line.strip()
    if not line:
        continue
    d = json.loads(line)
    sp = (d.get("spark_crosscheck") or {}).get("divergence_pp")
    add_leg(d["run_id"], d.get("cpu_index"), d.get("norm_v5"), "AG-9-x518-harvest",
            m1=d.get("m1_state"), spark=sp)

LEG_REQ = 22.74
rows = []
for key, leg in legs.items():
    cpu, ln = leg["cpu"], leg["norm"]
    window = []
    for arid, a in anchors.items():
        delta = abs(a["cpu"] - cpu)
        if delta <= 50_000:
            window.append({"anchor_run": arid, "anchor_cpu": a["cpu"],
                           "anchor_norm": a["norm"], "delta": delta,
                           "pair": round(ln - a["norm"], 2)})
    window.sort(key=lambda x: -x["pair"])
    n = len(window)
    best3 = min((w["pair"] for w in window[:3]), default=None)
    worst = min((w["pair"] for w in window), default=None)
    n_ge20 = sum(1 for w in window if w["pair"] >= 20)
    rows.append({
        "run_id": leg["run_id"], "src": leg["src"], "seed": leg["seed"],
        "cpu": cpu, "leg_norm_v5": ln, "m1": leg["m1"], "spark_div_pp": leg["spark_div"],
        "n_anchors_50k": n, "n_anchors_pair_ge20": n_ge20,
        "best3_min": best3, "worst_min": worst,
        "best3_cert": bool(n >= 3 and best3 is not None and best3 >= 20),
        "ROBUST_cert": bool(n >= 3 and worst is not None and worst >= 20),
        "fire_ge_req": ln >= LEG_REQ,
        "top_pair": window[0] if window else None,
        "worst_pair": window[-1] if window else None,
    })

fire_rows = [r for r in rows if r["fire_ge_req"]]
robust_fire = [r for r in rows if r["fire_ge_req"] and r["ROBUST_cert"]]
lenient_fire = [r for r in rows if r["fire_ge_req"] and r["best3_cert"]]
rows.sort(key=lambda r: -r["leg_norm_v5"])

summary = {
    "tool": "gate3_robust_518 (AG-9, волна-518)",
    "anchor_pool": "AG-2 anchor_pool_510_513 (84) ⊕ AG-9 bank_516 (95), дедуп run_id",
    "anchors_merged": len(anchors),
    "legs_total": len(rows),
    "leg_req_n24": LEG_REQ,
    "fire_ge_req": len(fire_rows),
    "fire_lenient_best3_cert": len(lenient_fire),
    "fire_ROBUST_cert": len(robust_fire),
    "verdict": None,
    "rows": rows,
}
top3 = sorted((r["leg_norm_v5"] for r in fire_rows), reverse=True)[:3]
summary["top3_fire_min"] = min(top3) if len(top3) == 3 else None
# Фин-кандидаты: leg с ROBUST_cert (worst-case min-of-3 ≥+20 по ≥3 РАЗЛИЧНЫМ якорям), CLEAN, с числовым run_id
def has_real_rid(r):
    return isinstance(r["run_id"], int) or str(r["run_id"]).isdigit()
fin_cands = [r for r in rows if r["ROBUST_cert"] and r["m1"] == "CLEAN" and has_real_rid(r)]
summary["fin_candidates_worst_case_minof3"] = [
    {"run_id": r["run_id"], "cpu": r["cpu"], "leg_norm_v5": r["leg_norm_v5"],
     "worst_pair": r["worst_min"], "n_anchors_distinct": r["n_anchors_50k"]}
    for r in fin_cands]
if len(robust_fire) >= 3:
    summary["verdict"] = ("GATE-3 №24 CLOSED-ROBUST: ≥3 FIRE-легов ≥22.74 с worst-case pair min-of-3 ≥+20 "
                          f"(n={len(robust_fire)}) → MERGE-READY (проверка freshness/pair-fresh за координатором)")
elif len(lenient_fire) >= 3:
    summary["verdict"] = ("GATE-3 №24 частично: FIRE≥22.74 формально ≥3 (top3_min %.2f), но ROBUST worst-case "
                          "min-of-3 = %d; ambiguous-лег(и) зависят от C55-чтения; НО fin-кандидатов pair-stable "
                          "≥+20 worst-case = %d %s" % (
                              summary["top3_fire_min"], len(robust_fire), len(fin_cands),
                              [(c["run_id"], c["worst_pair"]) for c in summary["fin_candidates_worst_case_minof3"]]))
else:
    summary["verdict"] = "GATE-3 №24 НЕ закрыт: <3 FIRE-легов ≥22.74"

json.dump(summary, open(OUT + "gate3_robust_518.json", "w"), indent=1, ensure_ascii=False)

print(f"anchors={len(anchors)} legs={len(rows)} FIRE≥{LEG_REQ}={len(fire_rows)} "
      f"lenient={len(lenient_fire)} ROBUST={len(robust_fire)} | top3_fire_min={summary['top3_fire_min']}")
print("VERDICT:", summary["verdict"])
print("\n-- топ-14 легов по norm_v5 --")
for r in rows[:14]:
    print(f"run-{r['run_id']} leg={r['leg_norm_v5']:>6.2f} cpu={r['cpu']} m1={r['m1']} "
          f"src={r['src']} | n50k={r['n_anchors_50k']} n_ge20={r['n_anchors_pair_ge20']} "
          f"best3={r['best3_min']} worst={r['worst_min']} spark={r['spark_div_pp']} "
          f"| {'ROBUST' if r['ROBUST_cert'] else ('BEST3' if r['best3_cert'] else 'FAIL')}"
          f"{' FIRE' if r['fire_ge_req'] else ''}")
