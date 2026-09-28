#!/usr/bin/env python3
"""absorb_482_late.py — точечный абсорб живых ног тика (C53/C15/C39/C31/C36/C35/C16/C45/C77/C81/C32...)."""
import json, os, sys
sys.path.insert(0, "/home/z/c-crussty/scripts")
import absorb_482_main as A

LIVE = {
    "c53-w8compo": 36427166966,
    "c15-noise-van": 36420598597,
    "c15-noise-arm": 36420632053,
    "c39-stz5": 36424183726,
    "c31-p205g6": 36423259025,
    "c34-strat2": 36423268715,
    "c36-r960g6": None,       # C36 задиспатчил 36424183726? нет — это C39; C36 DISPATCHED run неизвестен
    "c16-rt2-200k": 36419473548,
    "c16-rt6-200k": 36419476672,
}

def main():
    os.makedirs(A.OUT, exist_ok=True)
    t = A.tok()
    acc_path = f"{A.OUT}/absorb_482.json"
    acc = json.load(open(acc_path)) if os.path.exists(acc_path) else {}
    res = {}
    jobs = [(tag, rid) for tag, rid in LIVE.items() if rid]
    for tag, rid in jobs:
        if f"late_{tag}" in acc:
            print(f"skip {tag} (уже)")
            continue
        try:
            _, v = A.process(t, rid, "world-bench-parallel", f"late-{tag}", "")
            v["wave"] = "LATE-482"
            res[f"late_{tag}"] = v
            brief = v.get("class") or v.get("conclusion") or v.get("status") or v.get("err", "?")
            print(f"{tag} {rid}: {brief} cpu={v.get('cpu')} norm={v.get('norm')} m1={v.get('m1')} "
                  f"armed={v.get('armed')} ncdfe={v.get('ncdfe')}", flush=True)
        except Exception as ex:
            print(f"{tag} {rid}: ERR {str(ex)[:150]}", flush=True)
    acc.update(res)
    json.dump(acc, open(acc_path, "w"), indent=1, ensure_ascii=False)
    print(f"saved {acc_path} total={len(acc)}")

if __name__ == "__main__":
    main()
