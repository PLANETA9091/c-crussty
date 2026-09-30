# STZ-110 «С2 датапак-давление» — стенд «мир под давлением» v1 (wave-519, AG-80)

Источник: LEDGER Л-466-C94.3 ТОП-5 СТРЕСС-ТЗ, приоритет 1: С2 «датапак-давление» —
5k mcfunction (200 каждотиковых) → lt_drain/fn-плоскость под давлением (G-D1-класс,
деплой-порог ≥3.0% Л62/Л123). Прецеденты полного цикла спека→код→релиз→нога за 1 тик:
СТЗ-3v2 A14 (sha 16fa1a32, 704 fn, 35k leaves/тик) и СТЗ-82 (v506-stz82, 15 fn, 3 ноги re2).

## Что в v1
- **5,000 mcfunction** (тик-теги: 200 корней `c2:t0..t199` каждотик; каждый корень вызывает
  24 детей `c2:f<i>_<j>`; каждый ребёнок = 1 лист) = **9,800 ops/тик** (~15% op-cap 65,536).
- **STATELESS-листья** `data get storage stz_c2:buf k`: command-storage read-only — 0
  персистенции, 0 entity-сканов (НЕ повторяем случай СТЗ-3v2-селекторов O(N)) → ожидаемый
  world-diff block-parity 0.0000% (гейт П-2 Л-466-C96.3).
- pack_format 88 dual [48,88] (Л-466-C89.1: datapack-format 1.21.10 = 88) → патчи mcmeta не нужны.
- Fixture: stz110-c2-fixture.zip **881,182B, sha256 79a0f122c2c77d61a32601ff85ecc98baaf263b738691ef583b4ddbdc301b248**,
  release **v519-stz110-c2**, генератор `scripts/gen_stz110_c2_fixture.py` (детерминированный zip,
  фиксированные mtime → sha воспроизводим).

## Preregistered гейты (ДО диспатча; полный список в dispatch_519_ag80_stz110.py)
G0 band [6.0,9.5]M + DP-INSTALLED sha256==79a0f122; G1 fn/тик ≥5000; G2 dp-core ≥1% ALL-CPU
(reopen-класс ×485; прогноз 9800 ops × 0.2-0.5µs = 2-5ms/тик = 4-10% MSPT по Л-482-C13.2);
G3 M1 CLEAN STW ≤23.0, NCDFE=0, AIOOBE=0, lever ∅; G4 world-diff 0.0000%; G5 вердикт =
dp-класс class-gate F1 → dp-кривая, НЕ банк (СТЗ-82-прецедент).

## Экономика и статус
Очередь сатуратед (98 queued в первых 100 ранов, 2026-10-01; canary-пара 36788080912/83370
queued на 3-й тик) → POST НЕ производить: финал AG-80 = **DISP-INTENT**, payload готов
(ветка swarm-519-80 + release-asset + dispatch-скрипт с queue-guard). Fire: очередь <40 queued
И canary GREEN → `ALLOW_STZ110_FIRE=1 python3 scripts/dispatch_519_ag80_stz110.py`.
Дальше (волна-520): если G2 закрыт ≥1% — v2 добавляет стрессоры С2-спеки: /reload×5
(внешний триггер runner-а) и noise default_block=air (C2ME #592-паттерн, требует fresh-gen
фикстуру — отдельная нога), после чего СТЗ-110 становится несущим стендом dp-класса.
