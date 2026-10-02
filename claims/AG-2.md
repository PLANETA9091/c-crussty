# AG-2 PREREG (wave-526, 2026-10-02) — fg0 pre-guard A/B + pop400k-мид WBP dp3v2 zero-code

## Гипотезы/вилки
- leg-A fg0: fluid_guard=0 (pre-guard A/B нога канона fluid_guard=1, волна-1 TASK-80/S7-128).
  Клетка девственна x525: grep доски \bfg0\b/pre-guard = 0, OFF-ноги lever-канонов брали
  AG-247 (ic0+fd0) и AG-261 (bc0) — guard-канон последним не тронут. Ценность: lever-атрибуция
  guard-плоскости на пост-№19 банке (dp3v2, pop150k). Вердикты: |norm|≤σ когорты → guard
  не-несущий на текущем мастере (метод-FACT, компо-легален); norm снятия ≥ +2σ → guard = налог
  на pop-плоскости (FAIL-ценность: re-approve guard перед компо); negative → guard несущий.
- leg-B pop400k: pop-ось верх-брекет 350-450k (мид), канон pop150k; сосед pop500k contested
  (AG-241 клейм). Доза TPS(pop) к injector/heap-клиффу (AG-277 probe-класс). OOM/dead =
  REFUTED-INFRA честно (потолок pop-кривой при xmx10G), не FAIL гипотезы.

## Рецепт (verbatim WBP-канон AG-257/261/269)
- world-bench-parallel.yml @PIN e49e8984 (AG-257 WBP-носитель x525; tree ≥3200 FULL,
  wbp-yml blob 7c021f41*, вериф API до POST; fallback — ABORT, не fallback-пин).
- Канон-вектор бан-EXACT: 640/300s/fp4/guard1/gc3/ic1/fd1/fdi0/fb0/rt4/bc1/sbb0/steal0/
  bu0/pop150k/pseed42/xmx10G/xms4G/dp3v2(v484 stz3v2-fixture)/band 5.5-13.5M, lever пусто.
- leg-A swarm-526-2: дельта ТОЛЬКО fluid_guard=0. leg-B swarm-526-2b: дельта ТОЛЬКО
  population_target=400000. population_seed=42 в обоих (pair к канон-когорте seed42).
- refs POST /git/refs FULL-sha @PIN; воркри нет, локальных коммитов 0 (Д1-Д5); 2-й POST
  через 31s (FAIL AG-338); ≤2 POST; 429/403 → payload work/AG-2 → DISP-INTENT легален.

## Гейты при харвесте (канон WBP)
FIXTURE-VALIDITY (pop exact), NCDFE=0, threw=0, AIOOBE=0, selfTest=true, STW≤23s,
GC avg pause ≤200ms, band PASS, pairing |Δcpu|≤50k к когорте (Л147), min-of-3.
Cohort leg-A: канон-банк guard1 seed42 (AG-198/247/257 свежие ноги). Cohort leg-B:
pop350k (AG-269) + pop175k/250k (AG-277) dp50k-… нет — dp3v2-когорта: pop350k AG-269.

## Prereg вердиктов (до терминала)
1. leg-A: norm(fg0) ∈ [−σ, +σ] когорты (σ_pass 2.26пп AG-184) → guard не-несущий.
2. leg-A: norm(fg0) > +2.26пп → guard-налог; < −2.26пп → guard несущий-положительный.
3. leg-B: монотонное продолжение pop-кривой (350k→400k падение ≤ наклон 250→350k) →
  dose-факт; аномальный обвал/OOM → потолок-кандидат (CENS-матем при 3-х точках).

## Кап-матем
WBP-лега r640/300s ≈ 15-25 мин wall (AG-257 канон) << GH-кап; pop400k инжект ≤
POP_INJECT_TIMEOUT 900s? — 150k инжект ≈100 тик; 400k ≈ ×2.7 — риск таймаута инжекта
есть и ЧЕСТЕН (injector-клифф = целевой факт leg-B), отказ-класс REFUTED-INFRA.

## Сиды/дедуп
- population_seed=42 канон (не жгу 526-семейство). Ветки swarm-526-2[ab] = 0 remote.
- Живой GET race-чек перед CLAIM и перед POST; CAS-append; строки ≤120 симв.
