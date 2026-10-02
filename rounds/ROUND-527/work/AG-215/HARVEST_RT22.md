# AG-215 w527 HARVEST — rt22 leg-1 (WBP pop150k dp3v2), prereg w526 claims

## Run
- run-37001021865 @swarm-526-215, head 9f3f8b36 (tree 4304), job 110818497716 SUCCESS 23:26:14Z.
- queued 11:27Z → pickup 22:55:49Z (11.5h slot-wait, post-cancel-lever 22:39Z окно AG-179); wall 30.4м.
- artifact 11256694573 world3-bench 27.5MB → rounds/ROUND-527/work/AG-215/harvest_rt22/unz.

## Гейты prereg (PAYLOAD w526)
1. inject: POPULATION INJECT DONE target=150000 injected=150000 (157.9s), FIXTURE-VALIDITY **VALID** → PASS (±2% и 90% оба).
2. band-gate: runner_cpu_index=6006593 в [5500000,13500000] → fast-fail НЕ сработал; в-run cpu idx 6844227. PASS.
3. ARM: region_threads: 22 подтверждён в run-env; batch_collector=1 (требует rt≥2) согласован. PASS.
4. NCDFE/selfTest: dp-parity-fp.json error=main_scan_rc=1 → FAIL-OPEN UNKNOWN (log: "anti-false-REFUTED");
   идентично якорям AG-136 ic0/c91 = known WBP no-TC artifact, НЕ рефутация. PASS-by-canon.
5. Pair |dIdx|≤3%: НЕ достигнут (мой 6844227 vs rt20-когорта 8.3M/9.1M) → вставка в rt-кривую indicative-only, не pair-stable.

## Вердикт TPS
- polls n=6: first 17.4 (pre-inject), tail5 = [0.3, 0.3, 0.3, 0.3, 0.3], last 0.3. Внутри-ран σ=0 (5/5).
- Кривая rt @pop150k (cross-seed, quantization 0.1 = ±25%): rt5 0.5-0.6 / rt20 0.4-0.5 (AG-40 ps531026) / **rt22 0.3 (s527215)**.
- H1 w526 «монотонно вниз за rt12» — направление подтверждено (0.3 ≤ 0.4-0.5), но n=1 + quantization →
  доза-факт, НЕ серт. Канон AG-40 «потоки TPS не двигают» усилен точкой rt22: подъём 20→22 потоков не улучшает.

## Side-fact (реплика активной линии AG-48/50/53 — НЕ новая вилка)
- cpu-collapsed: EntityLookup.get self = 16825/53303 = **31.6%**, **100.0% из них** через
  TimerQueue→FunctionCallback→SFM→ExecuteCommand→EntitySelector.findEntities→ServerLevel.getEntities
  (мейн-поток, serial). Сравнение: rt0(rt=0) 26.6% (AG-136, pop150k) vs rt22 31.6% — **@e-скан-налог
  rt-инвариантен**: region-threads его не разгружают, фикс-скан масштабируется с pop (pop50k: 6.7-9.8%).
- Следствие: на pop150k dp3v2 ~1/3 CPU = собственный селектор-скан фиксчи → контраст любых ливеров
  на этом лейне дилюцирован; item×99241 (67% сущностей) — главный корм скана.

## rt9 leg-2
- run-37001071869 @swarm-526-215b: queued с 11:27Z, на 23:3xZ ещё не picked. Харвест w528 по этому
  payload; ожидание H2 rt9 ∈ [rt8, rt10] = [0.4?, 0.5?] — гейты те же.

## Уроки
1. WBP-харвест recipe: artifact/zip → run-env (ARM) + server-stdout (INJECT DONE/BAND) + BOTTLENECKS_3 +
   cpu-collapsed → AG-189-формат tps-поллов. 30 мин на ногу.
2. dp-parity main_scan_rc=1 = fail-open канон WBP (равен якорям) — не писать FAIL по нему.
3. Quantization 0.1 TPS @ 0.3-0.6 = ±25%: rt-кривая выше rt12 разрешима только min-of-3 same-seed.
4. runner_cpu_index калибровки (step-3) ≠ в-run значения (6006593 vs 6844227) — pair-дисциплину брать из run-env.
