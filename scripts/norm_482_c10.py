#!/usr/bin/env python3
"""norm_482_c10.py — post-hoc фильтр C10: run-id → артефакт+norm (absorb_482_main.process),
фильтр chkclimb-5 [6427199,6527199] norm ≤ −1.60 (v5-FROZEN; Л-470-S31.1 post-hoc).

Usage: norm_482_c10.py <run_id> <branch> [<run_id> <branch> ...]
"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from absorb_482_main import tok, process  # канон-парсер банк v5

WIN_CHK5 = (6427199, 6527199, -1.60)


def main():
    t = tok()
    args = sys.argv[1:]
    out = {}
    for i in range(0, len(args), 2):
        rid, br = int(args[i]), args[i + 1]
        tag, v = process(t, rid, "world-bench-parallel", br, "2026-09-28T12:07:00Z")
        out[tag] = v
        print(json.dumps({"tag": tag, **v}, ensure_ascii=False, default=str), flush=True)
        if v.get("class") in ("STRICT-IN-POINT(банк-фид)", "VANILLA-VALID-§3(банк-фид)"):
            cpu, norm = v.get("cpu"), v.get("norm")
            in_win = bool(cpu is not None and WIN_CHK5[0] <= cpu <= WIN_CHK5[1])
            thr = bool(norm is not None and norm <= WIN_CHK5[2])
            print(f"  CHK5-EVAL {tag}: cpu={cpu} norm={norm}% in_window={in_win} "
                  f"norm<=-1.60={thr} -> {'HIT (КЛИМБ-кандидат)' if in_win and thr else 'miss'}",
                  flush=True)
    json.dump(out, open(f"{OUT}/norm_482_c10.json", "w"), indent=1,
              ensure_ascii=False, default=str)
    print(f"saved {OUT}/norm_482_c10.json", flush=True)


if __name__ == "__main__":
    main()
