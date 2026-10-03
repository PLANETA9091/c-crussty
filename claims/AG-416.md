# AG-416 w527 — bloom-capture re-scale pop150k-dp: prereg (0-POST, статика + локальные арты)

## ГИПОТЕЗА
Л116 bloom-лейн (64KB/k12 chunk-key pre-gate) прайсился на roar-3b при getEntities+EntitySelector
8.93% CPU × capture 10-30% = +0.9-2.7пп (суб-бар соло). На pop150k-dp когорте та же плоскость
(dp-stz3v2 self-sched selector-шторм, канон AG-41/48/50/53/76) = 43.5-43.75% CPU (x2 ноги,
σ≈0.0пп same-фикстура). Наивный re-scale = 43.5 × 10-30% = **+4.4..13.1пп** — климб-кандидат.

## ВЕРИФИКАЦИЯ БАЗЫ (свои числа, /home/z/rounds/ROUND-527/work/AG-250/gc6art/cpu-collapsed.txt)
- total 59180; selector-plane (EntitySelector.addEntities ⊂ tickTime) = **25741 = 43.50%**
  (реплицирует AG-353 43.75% в допуске substring-агрегации).
- Сплит: EntityType.tryCast subtree 4127 (7.0%); flat-map iter volatile (NodeIterator.next /
  ValueIterator.getValueVolatile / hasNext+findNext) 4275+43+30+26+166 ≈ 7.7%; остаток ≈ 28.8%
  = self-walk EntityLookup.get + лямбды addEntities.
- Канон-вилка AG-48 37-61% → кластер ~44% (AG-353), моя 3-я точка 43.50% — три independent
  run, σ<1пп. Плоскость высокодетерминированная → A/B судимая даже кросс-раннер (≥2σ гейт).

## КРИТИЧЕСКИЙ КАВЕТ (не воскрешать наивный re-scale!)
1. **Walk FLAT, не chunk-keyed**: pop150k стек = ServerLevel.getEntities → moonrise EntityLookup.get
   → ConcurrentLong2ReferenceChainedHashTable$NodeIterator — итерация ГЛОБАЛЬНОЙ entity-hash-таблицы
   (148k), НЕ обход секций. Л116 bloom ключует chunk-key'и → НАПРЯМУЮ НЕ ЛОЖИТСЯ (нет ключей).
   Ребайт-левер: per-EntityType index (add/remove хуки в EntityLookup.add/remove, байт-в-байт
   разделение add-plane) ИЛИ section-index rewrite. Capture зависит от type-селективности dp.
2. **Селективность dp НЕ верифицирована**: stz3v2 (707ф) локально отсутствует (dp_assets только
   stz93/94); grep stz94v2.mcfunction = 0 @e (не та пачка). Если @e[type=X] доминирует —
   type-index съедает tryCast+walk вплоть до верхней границы; если голый @e — capture→~0,
   единственный резерв = C2-профилирование итератора (см. Л116 k12 probes-цена).
   ДИСКРИМИНАТОР w528 (1 команда, 0 POST): unzip dp707-stz3v2 → rg -o '@e\[[^]]*\]' | sort | uniq -c.
3. Парити-закон: index/bloom = FP-fallthrough, бит-в-байт порядок выдачи обязателен (урок Л154/20d).

## PREREG ГЕЙТЫ (w528, диспатч после дискриминатора)
- same-boot A/B в 1 job (рецепт clm/AG-210: 2 bench в 1 VM, boots подряд) — σ_d~12пп
  кросс-раннер несудим, selector-плоскость σ≈0 судима.
- Инпуты банк-канон: 300s/fp4/gc3/pop150k/seed42/10G/xms4G/r640/ic1/fd1/rt4/bc1, band [6.0,9.5]M.
- Гейты: FIXTURE-VALIDITY VALID; NCDFE=0; AIOOBE=0; threw=0; items-gate; lever ARM-маркер
  (index-hit counter в stdout); parity lockstep ≥20k ops; min-of-3.
- Честный потолок при type-селективности: 43.5% × (30-100% capture) = **+13..+43пп CPU** —
  монстр-класc; при голом @e → REFUTED до диспатча (capture-матем обязательна, закон 14c).

## СТАТУС
0 POST. Диспатч ЗАПРЕЩЁН до дискриминатора п.2. Владелец темы w528: свободен (CLAIM|OPEN).
