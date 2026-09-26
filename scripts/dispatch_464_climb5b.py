#!/usr/bin/env python3
"""TASK-464-30 marker: chkclimb-5 ⊕P32 нога-2 (реплика с вариацией).

Нога-2 = реплика нога-1 (TASK-464-29, round-464-climb5-p32-1) с вариацией
lever_arg=2 — STAGGER N=2 для sidecar-плана ⊕P32 (snapreg, cmp459_snapreg).

База: chkclimb-5 (4bcabb2f "reroll-460 climb-5 wave-2"), план cmp459_snapreg,
контур round-464-climb5-p32-2, world-bench-parallel dispatch:
  lever_flag=cmp459_snapreg lever_arg=2 radius=640 seconds=300
  fake_players=4 fluid_guard=1 gc_tune=3 inside_cache=1 flush_diet=1
  region_threads=4 batch_collector=1 population_target=150000
  population_seed=42 server_xmx=10G server_xms=4G
  cpu_band_min=6000000 cpu_band_max=9500000

Гипотеза-дельта: STAGGER N=2 (lever_arg=2) против нога-1 (базовый lever_arg)
— проверка чувствительности chkclimb-5+P32 к stagger-глубине snapreg sidecar.
Запуск не выполняет — маркер-диспатчер, фиксированная реплика нога-2.
"""
LEVER_FLAG = "cmp459_snapreg"
LEVER_ARG = "2"
STAGGER_N = 2
BRANCH = "round-464-climb5-p32-2"
BASE_SHA = "4bcabb2f"

if __name__ == "__main__":
    print(f"TASK-464-30 marker: {BRANCH} base={BASE_SHA} "
          f"lever={LEVER_FLAG} lever_arg={LEVER_ARG} STAGGER_N={STAGGER_N}")
