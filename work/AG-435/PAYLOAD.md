# AG-435 PAYLOAD x526

- pin: 2171d6da775975c4bee94748f549ad16f02074e1 (AG-138 sim-канон, tree 1575b92f FULL 4231)
- RUN-A: 37020075830 dcp1650 leg seed 527435 @swarm-526-435 queued (204, head_sha==PIN)
- RUN-B: 37020140514 dcp2250 leg seed 528435 @swarm-526-435b queued (204, head_sha==PIN)
- wf: bench-v2.yml; вектор: radius_blocks=1136, run_seconds=9000, server_xmx=10G,
  bench_dims=minecraft:overworld, dim_gen_window=256, fake_players=4, drain_cap_polls=1650/2250
- CLAIM: commit 6ba43356; финал: commit 3c1c26f3 (FACT+DISP+PATCH_SUMMARY+OBSERVED)
- сосед-пейринг: dcp1350 AG-27 / dcp1800 AG-60 / dcp2100 AG-214 / dcp2400 AG-38 / dcp2600 AG-222
- гейты: числа только из артефактов; |Δrunner_cpu_index|≤3%; band-выход = pairing-discard
- машинный след: /home/z/rounds/ROUND-526/work/AG-435/dispatch_526_435.json
- пивот-журнал: sim96/128 → AG-355+; sim160/192 → AG-183/147; dcp1200/2400 → AG-278/38;
  dcp1050/1950 → AG-25/48; финал dcp1650+2250 — 5 пивотов до POST, 0 wasted-POST
