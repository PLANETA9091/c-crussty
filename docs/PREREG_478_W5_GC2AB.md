# PREREG 478-W5 — gc2-канал как стресс-заменитель gc3 (закон 14a/16)

База master 848d8f14 (тик ×478, WILD W5). CLAIM-фон (Л-478-A4/B1):
- A4-абсорб x2 run 36358029075: gc1 638ev/44.1s → gc2 151ev/19.4s (×0.24/×0.44,
  pause-target-артефакт), gc2 ∈ gc3-bank [18.3,25.6]s.
- B1-глубже: 3 фулла CodeCache-GC-Threshold = 6.28s из 8.42s full-времени —
  код-кэш доминант boot-STW (актуально для gc3-ног, gc2-фуллы смотрим отдельно).
- Свежий gc3-канон (B5-якоря, тот же вектор): a2 21.43s/122.8ms/9 fulls,
  a3 21.64s/118.2ms — в-банке.

## Гипотеза H-478-W5
gc_tune=2 (G1 IHOP35+8m+APT без pause-target — канонен, закон-5 чист) воспроизводит
STW-коридор gc3-канона на canon-векторе x466-C98 → годен как стресс-заменитель gc3.

## Диспатч
- A/B: A-плечо = ×2 реплики gc2 (corridor), B-плечо = gc3-банк (Л-478-A4 лестница
  [18.3,25.6]s + fresh B5-a2/a3 21.43/21.64s на том же векторе).
- Алиасы round-478-w5-gc2a / round-478-w5-gc2b @origin/master (0 код-дельт,
  docs-only-хвост), 1 диспатч = 1 ветка (Л188b), lever gc_tune=2 обе ноги,
  lever_flag/arg ПУСТО.
- Канон-вектор x466-C98 ЯВНЫМ JSON (урок C66-C72): 640/300s/fp4/gc2/ic1/fd1/rt4/
  bc1/pop150k/seed42/10G/xms4G/band[6000000,9500000].

## Гейты (закон 16, fast-fail)
1. cpu_index ∈ [6,000,000; 9,500,000] обе ноги (band-miss = free discard Л188c).
2. GATE-GC2 (A4-канон M1): STW_total ≤23.0s ∧ young ≤200 ev ∧ Full ≤12 — ОБЕ ноги.
3. Корридор: [min,max] ×2 реплик внутри/перекрыт с gc3-банком [18.3,25.6]s.
4. Валидность: NCDFE=0, AIOOBE=0, world afb3a0b3, canon-env ×12 полей,
   armed=∅ (vanilla-плечо), закон-5 запреты чисты (нет ZGC/tune4/THP/tune5/bitmask/
   fluid_dirty; gc2 канонен).
5. Runs >15 мин → вердикт DISPATCHED run-id (закон 18-iii), абсорб след. тиком.

## Вердикт
GC2-READY {числа ×2} при гейтах 1-4 PASS; иначе REFUTED_CENS с числами-потолком.
Парсинг: scripts/absorb_478_w5.py (gc.log Pause-канон Л-478-A1.1/B4, валидирован
бит-в-бит selftest W1-s7).
