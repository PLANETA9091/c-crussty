#!/usr/bin/env python3
"""dispatch_464_climb5c.py — TASK-464-31 (КЛИМБ chkclimb-5⊕P32 нога-3) — ВАРИАЦИЯ СЕМЕНТА.
Ветка round-464-climb5-p32-3 на базе 4bcabb2f (round-460-chkclimb-5, chkclimb-5 +18.40 v5;
round-464-climb5-p32-1 не найден — откат к chkclimb-5 носителю по протоколу).
ЛЕГЕНДА: population_seed=43 — снимает seed-специфичность фикстуры (ноги-1/2 cmp459_snapreg
в полёте на дефолтном сиде). Отдельная гипотеза-дельта: pair-робастность P32-композиции —
если дельта chkclimb-5(+18.40)⊕P32-snapreg устойчива к смене population_seed,
эффект не является артефактом конкретного сида генерации населения.
Диспатч: world-bench-parallel.yml, lever cmp459_snapreg=1, radius=640, seconds=300,
fake_players=4, population_target=150000, population_seed=43, xmx=10G/xms=4G,
cpu_band 6.0M-9.5M, guard-набор fluid_guard/gc_tune/inside_cache/flush_diet/
region_threads/batch_collector=1. УРОК ×447: маркер-коммит вместо live-диспатча из скрипта.
"""
import sys

if len(sys.argv) > 1 and sys.argv[1] == "--dry-run":
    print("preflight: marker-only (seed=43 leg-3); dispatch executed by orchestrator via API")
    sys.exit(0)

print(__doc__)
print("leg-3 seed-variation marker — see orchestrator dispatch log for run id")
