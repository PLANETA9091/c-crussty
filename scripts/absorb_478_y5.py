#!/usr/bin/env python3
"""absorb_478_y5.py — [478-Y5] ваниль-якорь-квота: norm A1-метод + банк §3 классификация.

Реиспользует валидированный (bit-exact selftest B5-HB1, Л-478-A1.1) A1-absorb из
b5_norm_verdict.py: polls-median C55 (<15.0, poll-first-of-window), tps_exp_v5 interp
BANK_V5_FREEZE §2, HOST-ценз M1 (STW_total ≤23.0s ∧ young_avg ≤200ms, completion-only
без gc,phases), vanilla-гейт (canon-env ×12 + world afb3a0b3 + NCDFE=0 + AIOOBE=0).

Y5-обёртка (пороги НЕ двигать, BANK §5):
  verdict: BAND-DEAD | FIXTURE-INVALID | HOST-CENSORED | ANCHOR-VALID
  ANCHOR-VALID = in_band ∧ fixture-valid ∧ vanilla (в-точка в банк §3);
  HOST-CENSORED = VALID в-точка, из фитов исключена (CLEAN-first §3.3);
  window-статус report-only: climb5 [8734563,8834563] hit norm ≤ +2.99;
  POI [8907260,9007260] hit norm ≤ −1.99; мишень-зона 8.5-9.3M, мишень 8.9M.
Usage: absorb_478_y5.py --run-id ID [--run-id ID ...] [--workdir DIR]
"""
import argparse, json, os, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from b5_norm_verdict import absorb  # валидированный A1-метод (bit-exact)

BAND = (6_000_000, 9_500_000)
WIN_CLIMB5 = (8_734_563, 8_834_563)   # порог norm ≤ +2.99 (v5-FROZEN §2)
WIN_POI = (8_907_260, 9_007_260)      # порог norm ≤ −1.99 (v5-FROZEN §2)
TARGET = 8_900_000
PRIORITY = (8_500_000, 9_300_000)     # банк §3.2 приоритет-зона


def classify(r):
    if r.get("verdict") == "BAND-DEAD":
        return "BAND-DEAD", {}
    cpu = r["cpu_index"]
    norm = r.get("norm_v5")
    base = r.get("verdict")
    clean = bool(r.get("clean_M1")) and not r["clean_M1"]["host"]
    if base == "FIXTURE-INVALID":
        return "FIXTURE-INVALID", {}
    win = None
    hit = None
    if WIN_CLIMB5[0] <= cpu <= WIN_CLIMB5[1]:
        win, hit = "climb5[8734563,8834563]", (norm is not None and norm <= 2.99)
    elif WIN_POI[0] <= cpu <= WIN_POI[1]:
        win, hit = "POI[8907260,9007260]", (norm is not None and norm <= -1.99)
    y5 = {
        "verdict_y5": "HOST-CENSORED" if not clean else "ANCHOR-VALID",
        "clean_M1_flag": "CLEAN" if clean else "HOST-CENSORED",
        "window": win, "window_hit": hit,
        "in_target_8.5-9.3M": PRIORITY[0] <= cpu <= PRIORITY[1],
        "dist_to_8.9M": cpu - TARGET,
        "bank_point_v5": base in ("ANCHOR-LEGAL", "MISS-NORM", "HOST-CENSORED"),
    }
    return y5["verdict_y5"], y5


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-id", type=int, action="append", required=True)
    ap.add_argument("--workdir", default="/home/z/rounds/ROUND-478/Y5/art")
    a = ap.parse_args()
    for rid in a.run_id:
        r = absorb(rid, a.workdir)
        y5v, y5 = classify(r)
        r.update(y5)
        r["verdict_y5_final"] = y5v
        print(json.dumps(r, ensure_ascii=False))
