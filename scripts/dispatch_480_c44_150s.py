#!/usr/bin/env python3
"""dispatch_480_c44_150s.py — COMMANDER 480-C44: 150s-спринт ×1 (WILD, t_stab-прегист).

CLAIM (канон W7 t_stab [250,500]s — Л-479: плато к poll 9-11 ≈ 450-550s в 600s-ячейке):
150s-окно делает прегист-тест «стабильна ли норма до 250s?» → если норма 150s сравнима
с 300s-каноном ±C55-поправка (median-vs-plateau bias −12.53пп, τ̂=0.69 полла) —
CI-экономика: спринт-раны ×0.5 стоимости.

ЗАДАЧ: 1 диспатч world-bench-parallel @686f225830a40570fd7dddcf77c2e1e64e4ecb88
(0 код-дельт, ×479 MAIN-консолидация), алиас round-480-c44-150s, canon x466-C98
ЯВНЫМ JSON с seconds="150" (дефолты прочие 640/fp4/gc3/ic1/fd1/rt4/bc1/pop150000/
seed42/10G/xms4G, lever ∅ vanilla), band GLOB [6000000,9500000] fast-fail,
band-miss → 1 ре-ролл (закон W3).

ПРЕГИСТ (закон 14a/16, FROZEN — полный в PREREG_480_C44_150S.md):
  G1 run-id → DISPATCHED (12e/18-iii); G2 in-band иначе W3-ре-ролл ×1;
  G3 VALID ∧ NCDFE=0; G4 t_stab-150: n_valid∈[3,10] (меряем, middle≈6),
  плато-подпись last2≥med ∧ n≥4, |norm150−norm300(w2 36386470310/w4 36386521687,
  vanilla 300s того же пина)| ≤ 8пп plateau-полями (fallback med-vs-med ±15пп).
  PASS → спринт-150s GO (CI ×0.5); FAIL → канон 300s остаётся.
LEDGER Л-480-C44. Runs >15 мин → DISPATCHED run-id (закон 18-iii).
"""
import json, sys, time
sys.path.insert(0, "/home/z/c-crussty/scripts")
from dispatch_479_v1_swarm import api, token, ensure_alias, dispatch, INPUTS

REPO = "PLANETA9091/c-crussty"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"   # ×479 MAIN-консолидация
ALIAS = "round-480-c44-150s"

# canon x466-C98: ЯВНЫЙ JSON, seconds=150 (спринт-прегист), прочее канон-дефолты
INPUTS = dict(INPUTS)
INPUTS["seconds"] = "150"
assert INPUTS["population_target"] == "150000" and INPUTS["lever_flag"] == ""
assert INPUTS["cpu_band_min"] == "6000000" and INPUTS["cpu_band_max"] == "9500000"


def main():
    tok = token()
    live = api(tok, f"/repos/{REPO}/branches/master").get("commit", {}).get("sha", "")
    print(f"live master = {live}")
    pin = live if live.startswith(PIN[:8]) else PIN
    ensure_alias(tok, ALIAS, pin)
    time.sleep(3)
    ok = dispatch(tok, ALIAS)
    if not ok:
        time.sleep(20)
        ok = dispatch(tok, ALIAS)
    if not ok:
        sys.exit(2)

    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    run_id, status = None, None
    deadline = time.time() + 300
    while time.time() < deadline and run_id is None:
        time.sleep(15)
        runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
        for r in runs.get("workflow_runs", []):
            if r["head_branch"] == ALIAS and r["created_at"] > mc:
                run_id, status = r["id"], r["status"]
                break
    print("C44-DISPATCH-JSON " + json.dumps({
        "claim": "480-C44", "alias": ALIAS, "pin": pin[:12], "seconds": "150",
        "run_id": run_id, "status": status,
        "verdict": "DISPATCHED" if run_id else "POSTED-NO-RUN-ID",
    }, indent=1))


if __name__ == "__main__":
    main()
