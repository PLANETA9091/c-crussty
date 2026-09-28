#!/usr/bin/env python3
"""absorb_478_y1.py — [478-Y1] ваниль-якорь STRICT-зоны [6.9,7.2]M: norm A1-метод + банк §3.

Реиспользует валидированный (bit-exact selftest B5-HB1/W3-c5, Л-478-A1.1) A1-absorb из
b5_norm_verdict.py: polls-median C55 (<15.0), tps_exp_v5 interp BANK_V5_FREEZE §2,
HOST-ценз M1 (STW_total ≤23.0s ∧ young_avg ≤200ms, completion-only без gc,phases),
vanilla-гейт (canon-env ×12 + world afb3a0b3 + NCDFE=0 + AIOOBE=0), cpu ТОЛЬКО из
run-env.txt (Л195).

Y1-обёртка (пороги НЕ двигать, BANK_V5_FREEZE §5):
  verdict: BAND-DEAD | FIXTURE-INVALID | HOST-CENSORED | STRICT-HIT | STRICT-MISS
  STRICT-HIT     = in STRICT [6.9e6,7.2e6] ∧ vanilla-valid ∧ CLEAN M1 → в-точка банк §3 (n→+1);
  STRICT-MISS    = vanilla-valid ∧ CLEAN, но cpu вне STRICT (банк-фид band [6.0,9.5]M, report-only);
  HOST-CENSORED  = CLEAN-first §3.3: валидная в-точка исключена из фитов;
  BAND-DEAD/FIXTURE-INVALID = discard (не раны, закон W3).
  target ~7.0M; dist_to_7.0M report-only.
Usage: absorb_478_y1.py --run-id ID [--run-id ID ...] [--workdir DIR]
"""
import argparse, json, os, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from b5_norm_verdict import absorb  # валидированный A1-метод (bit-exact)

BAND = (6_000_000, 9_500_000)
STRICT_LO, STRICT_HI = 6_900_000, 7_200_000
TARGET = 7_000_000


def classify(r):
    if r.get("verdict") == "BAND-DEAD":
        return "BAND-DEAD", {"in_strict": False}
    cpu = r["cpu_index"]
    norm = r.get("norm_v5")
    base = r.get("verdict")
    clean = bool(r.get("clean_M1")) and not r["clean_M1"]["host"]
    if base == "FIXTURE-INVALID":
        return "FIXTURE-INVALID", {"in_strict": STRICT_LO <= cpu <= STRICT_HI}
    in_strict = STRICT_LO <= cpu <= STRICT_HI
    y1 = {
        "in_strict": in_strict,
        "strict_window": "[6900000,7200000]",
        "clean_M1_flag": "CLEAN" if clean else "HOST-CENSORED",
        "dist_to_7.0M": cpu - TARGET,
        "bank_point_v5": base in ("ANCHOR-LEGAL", "MISS-NORM", "HOST-CENSORED"),
        "bank_strict_inpoint": bool(in_strict and clean and
                                    base in ("ANCHOR-LEGAL", "MISS-NORM")),
    }
    if not clean:
        v = "HOST-CENSORED"
    elif in_strict:
        v = "STRICT-HIT"
    else:
        v = "STRICT-MISS"
    return v, y1


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-id", type=int, action="append", required=True)
    ap.add_argument("--workdir", default="/home/z/rounds/ROUND-478/Y1/art")
    a = ap.parse_args()
    os.makedirs(a.workdir, exist_ok=True)
    for rid in a.run_id:
        r = absorb(rid, a.workdir)
        y1v, y1 = classify(r)
        r.update(y1)
        r["verdict_y1_final"] = y1v
        print(json.dumps(r, ensure_ascii=False))
