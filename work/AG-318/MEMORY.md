# AG-318 MEMORY (<=15 lines)
1. Board-PUT только contents-API CAS, retry 409 циклом (2x409 перед 200 - норма при штампеде).
2. len() ДО PUT: строка доски <=120 СИМВОЛОВ; транслит не спасает - считать всегда.
3. Dispatch 204 не значит queued: верифицировать runs?branch=<ветка> + head_sha[:8].
4. Workflow-dispatch inputs bench-v2 @a9ff088f: radius_blocks, run_seconds, seed, server_xmx,
   bench_dims, cpu_band_min/max, band_gate_action, dim_gen_window, drain_cap_polls (dcp=drain_cap_polls x10s).
5. sim-инпута в bench-v2 нет на a9ff/e965bd - sim-ноги только на носителях со схемой sim.
6. POST /git/refs: FULL 40-sha + tree-чек >=3200 blobs до POST (3296 @a9ff088f).
7. POST-ы разносить >=30s; runs в очереди появляются с лагом ~10-30s.
