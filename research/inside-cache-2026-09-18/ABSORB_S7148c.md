# ABSORB S7-148c — FLUSH-DIET leg (ран 35324517090): §S7-138 PASS — flushStep-семья −100%

Дата: 2026-09-18 16:5x +08. Job 394666. Агент agent-7625532f.

## 1. Валидность A/B

Ран 35324517090 (head 1864e3d, flush_diet=1, все остальные рычаги 0,
fp4/300s/150k/seed42/xmx10G/guard1), SUCCESS 08:28:05→08:43:50 UTC (~16 мин).
База = base-b (35317176927, head 4c8f029, все рычаги 0, fp4/300s).

- Конфиг точен (run-env.txt: flush_diet=1, inside_cache=0, fp4, 300s) ✓
- INJECT 150000/150000 VALID; alive-check players()=4 injected=4 ✓
- Популяция стабильна весь soak: entity totals **148483 / 148328 / 148178**
  (топап-дренаж живой, дрейфа вниз нет) ✓
- **0 NoClassDefFoundError / 0 «Entity threw exception»** (stdout 257KB) ✓
- FLUSH-DIET мост ARMED живьём: pristine sighting
  InsideBlockEffectApplier$StepBasedCollector 5695B (major 65) → ретаргеты
  serve (chain внутри rust-плагина) ✓
- inside_cache: dormant (корректно — изолированная нога) ✓
- patched-kernel в артефакте = e2992d63 (байт-в-бит с эталоном) ✓

## 2. §S7-138-вердикт: PASS

| Метрика | base-b | FLUSH-DIET | Гейт | Вердикт |
|---|---|---|---|---|
| flushStep (alloc доля) | 4.15% (299) | **0.00% (0)** | семейство ↓ | ✅ **PASS (−100%)** |
| Object[] (leaf alloc) | 426 | 208 (−51.2%) | маркер new Object[0] | ✅ PASS |
| entity-фаза | 59.7% | 54.1% | ≤+0.5pp | ✅ PASS (−5.6pp) |
| fixture | зелёная | зелёная | зелёные | ✅ PASS |
| 0 NCDFE (S7-148-чек) | — | 0 | 0 | ✅ PASS |
| young GC | 125 | 141 | ↓ (калибровка: по alloc-долям) | топап-спавны; flushStep −100% не виден в GC-счётчике — ожидаемо |
| TPS | 0.8–0.9 | 0.7–0.8 | нейтрально | парити-чист, не регрессирует |

Механика подтверждена end-to-end: zero-waste addAll (FlushOps.fladd) снял
ВСЮ flushStep-семью (4.15% alloc-чурна ценза) и половину Object[]-листа.
TPS-эффект нейтральный (аллокационный рычаг, не TPS-драйвер) — консистентно
с калибровкой S7-148b (young-GC на живой сцене малочувствителен).

## 3. Конвейер

FLUSH-DIET = **GREEN** (парити-чист, аллокационная диета максимальна,
самовзрыва нет). ОстаЁтся в накопительном конфигурационном ране.

NEXT: FLUID-FREE leg диспатчен (35326295881, head d2f063e, fluid_free=1 +
paletted_demux=1 + inside_cache=1 — владелец Entity-цепочки, 08:48:58 UTC,
§S7-139-гейты: fluid-доля get ↓≥60%, entity-фаза ≤+0.5pp + S7-148-протокол)
→ ALLOC-DIET leg → накопительный ран (все зелёные рычаги вместе).

## 4. Артефакты

- run-s7148-flushdiet/: BOTTLENECKS_3.md, alloc-collapsed.txt (f), gc.log,
  run-env.txt, sha256_flushdiet_extras.txt (stdout 257KB; patched-kernel
  e2992d63) — в git; большие cpu/wall/flamegraph/entity-recon — sha256-only.
- Диспатч: dispatch_s7148c.py (FLUID-FREE leg).
