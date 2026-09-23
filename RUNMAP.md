# TASK-427-A2 run map (S3, dispatched 10:36-10:38 UTC 2026-09-23)

| tag | run_id | branch | lever | head sha |
|---|---|---|---|---|
| a2-l1 | 35849774567 | round-427-a2-mobsoa | cmp424_mobfeed | fe4ee57 |
| a2-l2 | 35849900480 (alt dup 35849920287 — same sha, concurrency dedupe) | round-427-a2-l2 | cmp424_mobfeed | fe4ee57 |
| a2-l3 | 35849929570 | round-427-a2-l3 | cmp424_mobfeed | fe4ee57 |
| a2-anchora | 35849783846 | round-427-a2-anchora | (vanilla) | 790dc2f |
| a2-anchorb | 35849793446 | round-427-a2-anchorb | (vanilla) | 790dc2f |
| a2-anchorc | 35849802726 | round-427-a2-anchorc | (vanilla) | 790dc2f |

Notes:
- first 2 leg dispatches on the SAME branch (round-427-a2-mobsoa) were concurrency-cancelled
  (35849755674, 35849765421) — fixed with per-leg alias branches (l2/l3), exact same head/inputs.
- anchors round-427-a2-anchor{a,b,c} pushed @790dc2f (fresh vanilla, same window).
