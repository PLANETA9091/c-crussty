#!/usr/bin/env python3
"""pair452a.py — TASK-452-A pair-verdict helper.
Reads absorb JSON summaries (JSON lines in /tmp/absorb452.jsonl)
and computes: pair = leg_norm - anchor_norm for nearest fresh anchor (|d_runner| <= 50k,
anchor norm >= -2 depression gate), min-of-3 across legs. Vanilla-verify anchors.
Usage: python3 pair452a.py /tmp/absorb452.jsonl
JSON line schema: {"tag","run_id","branch","verdict","tps_med","tps_exp","delta_norm_pct","items_pct","runner","lever","markers","aioobe","threw"}
"""
import json, sys

def main(path):
    rows = [json.loads(l) for l in open(path) if l.strip()]
    anchors = [r for r in rows if r["branch"].startswith("round-452-anchor")
               and r["lever"] == "" and r["verdict"] in ("GREEN-CANDIDATE", "PARITY/LOW")]
    legs = [r for r in rows if "senseins" in r["branch"]
            and r["verdict"] in ("GREEN-CANDIDATE", "PARITY/LOW")]
    print(f"valid anchors: {len(anchors)}, valid legs: {len(legs)}")
    # vanilla-verify anchors
    for a in anchors:
        armed = [m for m in a.get("markers", []) if "armed" in m.lower() or "ARMED" in m]
        zero_items = (a.get("items_pct") == 0.0)
        ok = not armed and not zero_items
        print(f"  VANILLA-VERIFY {a['tag']}: lever='' armed_markers={len(armed)} items_lane={a.get('items_pct')} -> {'OK' if ok else 'GATE BROKEN'}")
    print()
    pairs = []
    for leg in legs:
        best = None
        for a in anchors:
            d = abs(leg["runner"] - a["runner"])
            if d <= 50_000 and a["delta_norm_pct"] >= -2.0:
                if best is None or d < best[1]:
                    best = (a, d)
        if best is None:
            print(f"  {leg['tag']}: norm {leg['delta_norm_pct']:+.1f} @ {leg['runner']} -> NO PAIR (no fresh anchor within 50k, valid)")
            continue
        a, d = best
        pair = leg["delta_norm_pct"] - a["delta_norm_pct"]
        verdict = "GREEN >= +20%" if pair >= 20.0 else "BELOW BAR"
        print(f"  {leg['tag']}: norm {leg['delta_norm_pct']:+.1f} @ {leg['runner']} (threw={leg.get('threw')} AIOOBE={leg.get('aioobe')} markers={len(leg.get('markers', []))}) PAIR {pair:+.1f}pp vs {a['tag']} (d={d}, anchor norm {a['delta_norm_pct']:+.1f}) -> {verdict}")
        pairs.append((leg["tag"], pair))
    if pairs:
        ms = sorted(p for _, p in pairs)
        print(f"\npairs collected: {[(t, round(p,1)) for t,p in pairs]}")
        if len(pairs) >= 3:
            print(f"min-of-3: {ms[0]:+.1f}pp -> {'CERT-BAR MET' if ms[0] >= 20.0 else 'BELOW BAR'}")
        else:
            print(f"min-of-N({len(pairs)}): {ms[0]:+.1f}pp (need 3 for min-of-3)")

if __name__ == "__main__":
    main(sys.argv[1])
