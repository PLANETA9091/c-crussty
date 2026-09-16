# Benchmark 3.0 — real-world no-player load benchmark (GitHub CI)

Owner directive (2026-09-16): download the MineShield-3 world, force all
chunks to load, run with farms/mobs/entities ticking and ZERO players, all in
GitHub CI (never in the dev sandbox), with a VERY useful output: every
bottleneck ranked in detail. The ranked hotspots then drive LARGE research
rounds on Rust replacement / native bridging.

## What runs

1. `world-bench-3` workflow builds both Rust artifacts from source:
   - `libcrussty_runtime.so` (CRUSSTY engine, `runtime/` crate)
   - `libcrussty.so` (this module)
2. Downloads Purpur 1.21.10 + the MineShield-3 world + the closed-source
   natives (v0.1.0 release asset; graceful hotpatch-only mode when absent —
   the report LABELS the mode).
3. Boots the kernel with the module injected (the production `-agentpath`
   form; launcher.jar not required).
4. Console driver: forceload sweep (overworld tiles, ≤256 chunks per
   command), then a fixed no-player window (default 15 min) with TPS polls,
   `spark tickmonitor`, and optional summon sweeps.
5. Profiling: async-profiler CPU+alloc via `asprof` ATTACH (native frames
   including `libcrussty.so` visible) + spark JVM/event profiler.
6. `report_world3.py` turns the collapsed stacks into `BOTTLENECKS_3.md`:
   self-time by research bucket (worldgen / entities / chunk system / block
   entities / redstone / network / our Rust / JVM internals) + top-40 leaf
   frames + boot/TPS/warning stats.

## Inputs (workflow_dispatch)

| input | default | note |
|---|---|---|
| `world_url` | MineShield-3 **Min** (6.68 GB) | Full is 43.4 GB — dispatch from a larger/self-hosted runner with the Full URL |
| `radius` | 640 blocks | ~9.2k forceloaded chunks; raise on bigger runners |
| `seconds` | 900 | no-player measurement window |
| `summon_sweeps` | 0 | periodic zombie summons to exercise the spawn pipeline |
| `natives_url` | v0.1.0 release asset | empty string = hotpatch-only mode |

## Honesty notes

- **Natural mob spawning is player-proximity-gated** in vanilla/Paper. With
  zero players, forceloaded chunks still fully tick: redstone, villagers,
  item entities, existing mobs, block entities (farms built on those run).
  Spawner blocks idle past 16 blocks of a player; natural spawns idle.
  `summon_sweeps=1` documents a deviation that exercises spawn/tick anyway.
- **Diagnostic boot**: profilers are instruments. Per INJECTS-ONLY project
  rules, numbers from this workflow are NOT parity/A-B evidence — they rank
  hotspots. Parity claims stay on the sandbox rigs.
- Worldgen chunks vs saved chunks: MineShield is pregenerated; the first
  sweep loads saved regions (I/O-bound), later ticks are the steady state.

## Files

- `run_world3.sh` — the harness (assembly, sweep, profiling, shutdown)
- `report_world3.py` — collapsed-stacks -> bottleneck report
- `.github/workflows/world-bench.yml` — the CI entry point (manual dispatch)
