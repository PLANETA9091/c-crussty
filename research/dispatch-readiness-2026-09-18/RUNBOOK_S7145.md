# RUNBOOK S7-145 — REPRO-аудит v3: все 5 харнессов из ЧИСТОГО состояния

Дата: 2026-09-18 ~05:0x-05:2x +08. Тик 13:08 Job 393012 (11-й тик CREDS-BLOCKED).
Дополняет RUNBOOK_S7144 (research/fluid-free-2026-09-18/), где FLUID-FREE был
воспроизведён первым. Здесь закрыты остальные 4 (3 рычага + демукс-субстрат).

## Результат (все exit 0)
1. INSIDE-CACHE OFFLINE PASS — patched Entity defined (structural), InsideBlockOps
   ARMED=true (Unsafe-резолвы на реальном Entity), Recorder implements
   BlockStepVisitor, cache-массивы 131072×12 на месте.
2. FLUSH-DIET OFFLINE PASS — FlushOps.fladd linked (empty->false no-op,
   non-empty->ordered), patched + vanilla smoke (2000 пустых шагов, 5 эффектов).
3. ALLOC-DIET OFFLINE PASS — CollisionUtil retarget verified (EntityQueryOps в CP),
   mutablePos ring 8 инстансов zeroed, vanilla-MutableBlockPos anchor.
4. FLUID-FREE OFFLINE PASS — см. S7-144 (STRUCTURAL/ARMED/FREE-HIT/EVENT/RESTORE/
   SCATTERED-WATER).
5. PALETTED-DEMUX Parity ALL PASS — 20000 random ops lockstep L1/L2/model
   (getAndSet matched 4883/4883), snapshot published (snapGen=30009), 1000
   fast-path reads, re-materialization parity, concurrency smoke (3R+1W).

## Уроки REPRO (для будущих прогонов после WIPE)
1. **Харнессы в default package** (нет package-декларации; javac кладёт .class в
   корень build-dir) — запуск по ПРОСТОМУ имени класса (`java -cp …
   InsideCacheHarness`), НЕ по FQN. FluidFreeHarness — исключение: у него есть
   `package net.minecraft.world.entity`.
2. **FlushDiet nest-partner**: `StepBasedCollector$RecordedEffect.class`
   извлекается из kernel jar и подаётся 2-м аргументом (см. урок S7-137).
3. **Parity stub jar = same-runtime-package closure** (loader-scoped):
   - семейство `PalettedContainer*` из kernel (patched ЗАМЕНЯЕТ vanilla-копию),
     иначе LinkageError itable (lone patched class против parent-loader RO);
   - `Strategy*` + `Configuration*` тоже в stub, иначе IllegalAccessError на
     protected abstract `Strategy.getConfigurationForBitCount` (граница
     protected/abstract = «same package SAME loader»).
4. **libRoot ParityHarness** — 5-й аргумент (`/tmp/kernelmat/server/libraries`),
   старый default `/tmp/pdec/matsrv/libraries` невосстановим (WIPE).
5. Общий classpath-рецепт неизменен: shadow/bridge-dirs → PATCHED KERNEL →
   libraries (RUNBOOK_S7144).

## Артефакты
- scripts_s7145/repro_dispatch_harnesses.sh — 3 харнесса одной командой.
- scripts_s7145/repro_parity.sh — демукс-парити (stub jar собирается на лету).
- sha256 — artifact_hashes_s7145.txt (рядом).
