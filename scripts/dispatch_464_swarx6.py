#!/usr/bin/env python3
"""TASK-464-39 marker: swarx-6 re-roll leg 2 (H07, original WIP as-is).

Base: round-463-swarx-6 @ 6198445e (round-464-swarx-h07 was NOT pushed by
TASK-464-37 at dispatch time, so this leg re-rolls the original WIP with
lever_flag=cmp463_swar_hilbert, lever_arg=1).

Wiring check performed before dispatch:
- FLAG18 = "cmp463_swar_hilbert" declared (ColpushOps.java:113) and wired into
  the STRICT-OR lever acceptance predicate (ColpushOps.java:129).
- CP marker "cmp463_swar_hilbert" present in both compiled artifacts:
  colpush/build/ColpushOps.class and
  colpush/build/net/minecraft/world/entity/ColpushOps.class
  (verified via `strings` count=1 on each; javap unavailable in sandbox, exit 127).

Dispatch: world-bench-parallel.yml, radius=640, seconds=300, fake_players=4,
population_target=150000, population_seed=42, server xmx=10G/xms=4G,
cpu band 6000000..9500000, fluid_guard=1, gc_tune=3, inside_cache=1,
flush_diet=1, region_threads=4, batch_collector=1.
"""
