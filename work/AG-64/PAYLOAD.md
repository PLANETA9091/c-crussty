# AG-64 PAYLOAD (cycle-2, волна 526)
- dispatch JSON: work/AG-64/dispatch_526_64.json
- Leg A fp44@sim32 bench-v2: branch swarm-526-64 @2171d6da (report 762ceee8
  known-mine канон), seed 526064, run 36990152603, inputs r1136/9000s/w256/dcp900/
  xmx10G/1-dim/fp44/sim32
- Leg B rt18@pop150k WBP: branch swarm-526-64b @9c87f36c (tree 4245, WBP-yml
  blob 7c021f41 канон), popseed 527064, run 36990210274, inputs dp3v2/pop150k/
  region_threads=18/band 5.5-13.5M
- Race: rt18 vs AG-42 (их нога раньше, s42) — независ. leg-2, канон AG-190/263
- Соседние клетки: fp40 AG-250 36982647693, fp48/64 AG-216, fp92 AG-42
  36990096741; rt16 AG-226 36981461182, rt20 AG-26 36987601570
- Вердикт: harvest x527+ по artifact BENCHV2.md (job=failure не вердикт)
