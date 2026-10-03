# BENCHV2 — AG-433 wave-515 heavy stand (AG-496 x522: radius-aware gates, pregen-v3)

- ch/s (drain-def: marked chunks / (drain_ts − first_ts)): **9.70**
- forceload-marked chunks total: **20449** (expect ≥19426 = 0.95×1×20449; radius-blocks side=143)
- MSPT: idle≈1.9, sustain-median≈50.8 (spark mspt samples n=808)
- TPS samples (spark tps): n=809, min=11.18, last=20.0
- entity-tick share: see sparkprofile artifact (offline analysis)
- NCDFE=0 (canon T1=0 gate: PASS), AIOOBE=0
- G3 datapacks-enabled markers: 4/4 (PASS)
- G4 marked≥95%: PASS; G5 drain: PASS
- drain window: first_ts=1790992719 drain_ts=1790994828
# BENCHV2 PER-DIM CENSUS — AG-342 (wave-515) leg fake_players=?

- overworld: census rounds n=150, median entities=3266.5, max=3431
- the_nether: census rounds n=150, median entities=3266.5, max=3431
- the_end: census rounds n=150, median entities=3266.5, max=3431
- TOTAL across dims: median=9799.5 (spread 675)
- G6-FPV2 verdict: **SPAWN-LANES-ACTIVE (leg-B total=9799.5 >= 500)**


## AG-12 canon gates x516
G-DIM: ov=21609 ne=0 en=0 total=21609 (gate per-dim>=19000 total>=60000)
G-HB: heartbeat_lines=5595 (gate >=60, wall-clock async sampler)
G-TECTONIC: 3.0.25 sha512 7b3c5dee repinned (FATAL-alias 3.0.29 purged)
G-FP: fake_players=0 (0=canon vacuum byte-identical)
