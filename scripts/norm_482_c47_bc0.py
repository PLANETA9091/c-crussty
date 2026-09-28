#!/usr/bin/env python3
"""norm_482_c47_bc0.py — COMMANDER C47 ×482: run-id → канон-парсер банк v5
(absorb_482_main.process) + сравнение с прецедентом Л-480-C47 bc0 −0.25
(run 36391524670 @686f2258, cpu 7,002,989, STRICT-IN-POINT, M1 clean).

Вопрос миссии: −0.25 стабилен или дрейфует на compo-носителе 3666a793 (МЕРЖ №19)?

Usage: norm_482_c47_bc0.py <run_id> <branch>
"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from absorb_482_main import tok, process  # канон-парсер банк v5

PRECEDENT = {"run": 36391524670, "carrier": "686f2258 (×479)", "norm": -0.25,
             "cpu": 7002989, "class": "STRICT-IN-POINT"}


def main():
    t = tok()
    args = sys.argv[1:]
    out = {"precedent_480_c47": PRECEDENT}
    for i in range(0, len(args), 2):
        rid, br = int(args[i]), args[i + 1]
        tag, v = process(t, rid, "world-bench-parallel", br, "2026-09-28T13:00:00Z")
        out[tag] = v
        keys = ("run", "cpu", "norm", "med", "exp", "m1", "stw_total_s", "young_n",
                "young_avg_ms", "fulls", "band", "corridor", "class", "world",
                "ncdfe", "aioobe", "armed")
        print(json.dumps({k: v.get(k) for k in keys}, ensure_ascii=False), flush=True)
        if v.get("norm") is not None:
            d = round(v["norm"] - PRECEDENT["norm"], 2)
            print(f"  C47-EVAL: bc0 norm={v['norm']}% vs precedent −0.25% → "
                  f"Δ{d:+.2f}пп on compo-carrier 3666a793 "
                  f"({'СТАБИЛЕН (суб-шум)' if abs(d) <= 1.0 else 'ДРЕЙФУЕТ (шум-бар 1пп)'})",
                  flush=True)
    p = f"/home/z/rounds/ROUND-482/c47_bc0/norm_482_c47_bc0.json"
    os.makedirs(os.path.dirname(p), exist_ok=True)
    json.dump(out, open(p, "w"), indent=1, ensure_ascii=False, default=str)
    print(f"saved {p}", flush=True)


if __name__ == "__main__":
    main()
