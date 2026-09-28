#!/usr/bin/env python3
"""dispatch_478_merge18.py — MERGE #18 marker (Task-478-MAIN, v19.0 MEGA-SWARM, тик 2026-09-28 07:43+08).

Вердикт-носитель: climb5-p32-1 (composite cmp456_chunkmono_p31snap, dormant в master;
InsideSnapRegistryOps java+class блобы master==round-464-climb5-p32-1 байт-идентичны —
resident-пруф 15bde2f9/4d28053b; strict-OR гейты inside_batch STRICT eq +
inside_snap_registry cmp459|composite|env).

НОСИТЕЛЬ: climb5-p32-1 norm_v5 +22.99 @cpu 8784563 (ветка round-464-climb5-p32-1
@1b67c215, TASK-464-29; javap 7b0e8e305fdc/6072B/13 полей/0 native).

ПАРЫ min-of-3 (окно [8734563,8834563] ≤+2.99, BANK_V5_FREEZE §2 v5-FROZEN):
  1. aD1  +34.55 (исторический, ROUND-464 эра)
  2. W10  +26.96
  3. W1-s7 +31.05 — run 36357571554 (vanilla-якорь @b491d7f4), cpu 8,824,280
     ∈ окно [8734563,8834563], anchor_norm −8.06 ≤ +2.99, Δleg 39,717 ≤50k
     pair-fresh, STW 21.85s CLEAN M1 (young 108.0ms / fulls 10 / armed=[] /
     NCDFE=0 / AIOOBE=0 / tick-behind 0), med-polls канон C55 13/13
MIN-OF-3 = +26.96 ≥ +20 → БАР взят, закон 2: немедленный --no-ff.

Норм-вердикт окна (478-A1): W3-c5 8,925,412 POI MISS (norm +6.85 > −1.99,
pair +11.16 → POI 2/3 остаётся); W3-c8 8,747,610 HOST-censored (STW 23.24s > 23).

ГАТЫ МЕРЖА: cargo check --lib 0 err; check_blobs_sync ALL IN SYNC (javap
flat==nested); blob-cp cmp456_poi позиции; canary после (master default =
vanilla, левер env-канон).
Закон-5: climb5/p31snap не входит в запреты; ваниль-семантика strict-OR.
"""
import json

VERDICT = {
    "merge": 18,
    "carrier": "climb5-p32-1 (composite cmp456_chunkmono_p31snap, dormant-master-resident)",
    "carrier_branch": "round-464-climb5-p32-1 @1b67c215",
    "carrier_norm_v5": 22.99,
    "window": "[8734563,8834563] <= +2.99 (BANK_V5_FREEZE 2, v5-FROZEN)",
    "pairs": [
        {"tag": "aD1", "pair": 34.55},
        {"tag": "W10", "pair": 26.96},
        {"tag": "W1-s7", "pair": 31.05, "run": 36357571554, "cpu": 8824280,
         "anchor_norm": -8.06, "delta_leg": 39717, "stw_s": 21.85, "clean_m1": True},
    ],
    "min_of_3": 26.96,
    "verdict": "MERGE #18 climb5-p32-1 -> master, pair min-of-3 +26.96 >= +20",
    "tick": "x478 2026-09-28 07:43+08 trace 1a0dc7e6662ff26d-cron-agent-loop-202609280743",
    "author_verdict_agent": "478-A1",
}

if __name__ == "__main__":
    print(json.dumps(VERDICT, ensure_ascii=False, indent=1))
