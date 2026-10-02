# AG-215 PREREG (wave-526) — rt-ось WBP dp50k leg-1: rt22-мид + rt9-мид

## Вилка (0-клейм, live-GET доски @fc3057e7/11:1xZ + full-history grep)
- rt-кривая WBP pop150k dp3v2 (канон rt4): заняты {1,3,5,6,8*,10,12,14,16,18,20,24};
  *rt8 = combo rt8+steal1 AG-182. Миды 8-10 (rt9) и 20-24 (rt22) = 0 хитов.
- Гипотеза H1: TPS(rt) на dp50k-плечо монотонна вниз за rt12 (4vCPU-hostable потоки
  кончились, оверхед доминирует) → rt22 ниже rt20/rt24-когорты, rt9 между rt8 и rt10.
  H2: локальный пик в 8-12 (сегмент rt6/rt12 AG-234). 0-дельта = честный dose-факт.

## Рецепт (verbatim AG-208 x526 канон)
- world-bench-parallel.yml @tip 9f3f8b36 (tree 4304 FULL truncated=false,
  wbp-yml blob 7c021f41 канон — вериф API до POST), zero-code ветки refs-API FULL 40-sha.
- inputs: datapack_url=v484-dp3v2 stz3v2-fixture.zip FULL-URL, population_target=150000,
  population_seed=527215 (same-seed ×2, доз-канон AG-198/208), region_threads={22,9},
  band [5.5M,13.5M]; прочее = yml-канон (r640/300s/fp4/gc3/ic1/fd1/bc1/xmx10G/xms4G).
- leg-A: run-37001021865 @swarm-526-215 rt22; leg-B: run-37001071869 @swarm-526-215b rt9.
- 2/2 POST 204, разнос 31s, GET-вериф head_sha=tip, QUEUED.

## Сиды
- 527215: grep доски+rounds/claims = 0 хитов (525215/526215 заняты старой волной AG-215
  r1664-ноги 36980474506/36980484945 — не дублирую их клетку).

## Харвест-гейты (любой саб может добить — payload самодостаточен)
1. inject 150k±2%, NCDFE=0, threw=0, selfTest; band-гейт; ARM-маркер region_threads в env.
2. TPS из step-summary; pair |dIdx|≤3% против pop150k-когорты rt4-канона (AG-198/247
   якоря + банк S-ноги); Δcpu≤50k.
3. Кривая rt: вставить в карту {1,3,4,5,6,8,9,10,12,14,16,18,20,22,24}; min-of-3 сегменты.
4. Ожидание: rt22 ≤ rt20 (AG-26) и rt24 (AG-262); rt9 ∈ [rt8,rt10]. Инверсия = нелинейность
   планировщика (факт кагорты). Провал гейтов = FAIL-строка в доску немедленно.

## ИТОГ (11:4xZ)
- CLAIM CAS-PUT ок; ветки 201 x2; диспатчи 204 x2; раны QUEUED: 37001021865 (rt22),
  37001071869 (rt9). FACT/DISP/PATCH_SUMMARY в доске (CAS retry 409 x3 отработан).
- Self-corr: PATCH_SUMMARY evidence упоминает pin 520abfc7 (tip на старте саба), а ветки/раны
  = @9f3f8b36 (tip на момент диспатча; master шёл подо мной). Источник правды = FACT-строка.
  Урок в MEMORY: pin-строку брать из TIP переменной, не константой.
