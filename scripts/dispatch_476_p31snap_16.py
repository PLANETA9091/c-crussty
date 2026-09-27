#!/usr/bin/env python3
"""dispatch_476_p31snap_16.py — MERGE #16 marker (Task-476-MAIN, v19.0 MEGA-SWARM).

Вердикт-носитель: cmp456_chunkmono_p31snap (POI/chunkmono-l9-класс, dormant в master
с юниона №8; strict-OR poi_plane.rs:97 chunkmono|chunkmono_n|c98ai).

ЛЕГ: run 36323433349 (round-474-gold-l9 @ d791550d, ARM-пруф ×4 по артефакту
server-stdout.log — Л-475-канон), cpu 6594303 (run-env Л195), gc3, STW 19.48s
CLEAN M1 (fulls 9: md4/cc5; young_avg 101.6ms), selfTest=true, мир afb3a0b3,
POPULATION 150000/150000 VALID, NCDFE structurally impossible (EARLY-define
+ arm-after-define), norm_v5 +26.16 (med 2.7 / tps_exp interp 2.1402).

ПАРЫ min-of-3 (Δcpu ≤50k от лега, vanilla-VALID CLEAN gc3, якорный порог
≤+6.15 = leg−20; Л-475-C20.1 gc3-only коридор [−8.0,+6.15]):
  1. chkclimb-9   run 36185308698 @6593727 (Δ576):   norm +0.60..+2.80
     (артефакт-реверификация 2026-09-28: levers dormant, STW 20.73s CLEAN
     fulls 8, gc3, med 2.2) → пара +23.36..+25.56
  2. u4/r256c     run 36264482874 @6596851 (Δ2548):  norm +2.77
     (Л209 M1-ценз: STW 17.03s, fulls 9, armed=null, ncdfe_real=0) → пара +23.39
  3. lightcap     run 36263762949 @6599733 (Δ5430):  norm +0.42 (errata Л262:
     median 2.1 → −1.88) (Л209 M1-ценз: STW 15.53s, fulls 8, armed=null) →
     пара +25.74..+28.04
MIN-OF-3 = +23.36 ≥ +20 (робастно во всех интерпретациях норм-модели;
Л201-узел-ревизия Л-474-C90.1 пару не трогает — лег и якоря вне [6.9,7.2]M
либо Δ-симметричны).

ГАТЫ МЕРЖА: cargo check --lib 0 err (2026-09-28, 244d2cb8);
check_blobs_sync ALL IN SYNC (javap flat==nested); blob-cp проверка до push;
canary после (master default = vanilla, левер env-канон).
Закон-5: p31snap не входит в запреты; ваниль-семантика strict-OR
(оба-non-POI = vanilla no-op early-out).
"""
import json

VERDICT = {
    "merge": 16,
    "carrier": "cmp456_chunkmono_p31snap",
    "leg": {"run": 36323433349, "branch": "round-474-gold-l9", "sha": "d791550d",
            "cpu": 6594303, "gc": 3, "norm_v5": 26.16, "stw_s": 19.48, "arm_hits": 4},
    "anchors": [
        {"name": "chkclimb-9", "run": 36185308698, "cpu": 6593727, "norm_v5": [0.60, 2.80],
         "stw_s": 20.73, "vanilla": "artifact-reverified 2026-09-28, levers dormant"},
        {"name": "u4/r256c", "run": 36264482874, "cpu": 6596851, "norm_v5": 2.77,
         "stw_s": 17.03, "vanilla": "Л209 M1 armed=null"},
        {"name": "lightcap", "run": 36263762949, "cpu": 6599733, "norm_v5": [0.42, -1.88],
         "stw_s": 15.53, "vanilla": "Л209 M1 armed=null; errata Л262"},
    ],
    "min_of_3": 23.36,
    "bar": 20,
    "gates": {"cargo": "0 err", "blobs": "ALL IN SYNC", "band": "6.594M in [6.0,9.5]M",
              "ncdfe": "T1=0 structural", "aioobe": 0, "selftest": True,
              "population": "150000/150000", "world": "afb3a0b3"},
    "pair_fresh": "bank anchors never consumed by merges",
}

if __name__ == "__main__":
    print(json.dumps(VERDICT, indent=1, ensure_ascii=False))
