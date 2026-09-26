#!/usr/bin/env python3
"""TASK-464-32 chkclimb-12 iter-2 dispatcher marker.

Leg: chkclimb-12 VALID +17.64pp v5 @8183686, carrier cmp456_chunkmono_p31quant
(P31 + P34-quant K-tick rest, run 36201153518). Baseline-to-bar: -2.4pp.

ITER-2 PLAN (RE-DISPATCH of the same branch, no composite arm):
  lever_flag = cmp456_chunkmono_p31quant  (carrier lever, STRICT-OR across 22 rust sites)
  lever_arg  = 2                          (stagger variation N=2)

HYPOTHESIS (stagger2): quant K-tick rest + stagger N=2 yields +1.5..+2.5pp
over the +17.64pp leg. lever_arg=2 retargets push/goal stagger offsets —
a real behavioral delta (not a no-op flag), because stagger width changes
the per-entity push/goal retarget cadence inside the INSIDE-BATCH bridge,
composing with P34-quant K-tick dwell classification (K=20) without a
composite lever arm (leverEnabled() is single-flag: two-shoulder path is
impossible; re-dispatch with varied arg is the legal climb move).

Bench params: radius=640, seconds=300, fake_players=4, fluid_guard=1,
gc_tune=3, inside_cache=1, flush_diet=1, region_threads=4,
batch_collector=1, population_target=150000, seed=42, xmx=10G, xms=4G,
cpu band 6.0M..9.5M.
"""
