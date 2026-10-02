# AG-117 PAYLOAD (волна-526) — σ_seed pop150k A/A-пара @af0c5cc2 (zero-code)

Ветки: swarm-526-117 = swarm-526-117b = master-TIP af0c5cc2 (tree 3316 FULL,
Д5-гейт пройден; wbp_yml blob 7c021f41). 0 локальных коммитов, 0 ворктри (Д1-Д5).
CLAIM 73796d0f; FACT/DISP/PSUM ab49bae0. API-only (urllib, CAS 8 ретраев).

POSTs (2/2 HTTP 204, world-bench-parallel.yml, 2026-10-02, разнос 31s):
  run-36992639088 leg-A ref=swarm-526-117  seed527117 sha=af0c5cc2 QUEUED
  run-36992692943 leg-B ref=swarm-526-117b seed528117 sha=af0c5cc2 QUEUED
Инпуты (Δseed-only; остальное = yml-дефолты = canon-вектор x466-C98):
  datapack_url=stz3v2-fixture.zip @v484-dp3v2, population_target=150000,
  population_seed=527117|528117, cpu_band [5.5M,13.5M] (strict-мина ×4 AG-6/74).

## Гипотеза (prereg = claims/AG-117.md)
σ_seed pop150k canon-лейна (primary WBP living-scene). AG-6/80 покрыли pop50k/
dp50k; pop150k — где живёт вся pair-математика роя — пуст. H0: |ΔTPS| ≤ бар/2
(seed-инвариантность, cross-seed pairing безопасен); H1: seed-чувствительность
entity-микса = скрытый noise-floor pop150k-пар → Δseed-гейт в pair-канон.

## Харвест (ETA 10-30h; очередь 1116-1319q/51ip, харвест-канон AG-42/82/122/173)
1. GET runs 36992639088 + 36992692943; report md5-гейт (FIX 2da1febc vs BUGGED
   762ceee8; при bugged-sha — log-flip протокол AG-74).
2. TPS-med/tail, MSPT-avg, census F4, RUNNER_CPU_INDEX; σ_seed = |ΔTPS|/√2.
3. Пара против canon-когорты s42 (Δidx≤3% cohort, pair-fresh).
4. Band-die/timeout → OBSERVED/CENS-число бесплатно.
