# AG-170 w528 — fork#3 (pop150k re-fire) narrowing + dup-guard — 2026-10-03T09:0xZ
Ветка не нужна (0 POST, 0 диспатчей): ценз-работа = API-верификация + capture-математика.

## 1. FACT: MERGE-READY AG-116 (6f6d8f0b) уже удовлетворён master'ом
- origin/master:bench/worldv2/report_benchv2.py содержит блок `AG-116 w528 (#16g-sibling)`
  (per-world MAX monotone PROGRESS-marked fallback, completion-лайн класс) — grep по
  реф-байтам = 2 hits. Доска AG-116 `MERGE-READY ... 0 POST` = закрыто чужим merge-exec;
  повторный мердж = dup-риск AG-120-класса. POST-ов не делал, ветку не создавал.

## 2. CENS: placebo-вариант MAIN fork#3 (pop150k re-fire) — REFUTED до диспатча
MAIN-430805-3 fork#3 `pop150k re-fire на пост-LIMBO-фикс базе` читается как «диспатчни
ванильную pop150k-ногу на текущем master». Это плацебо/дуп по трём независимым осям:
1) **bank-default dup**: world-bench-parallel.yml `population_target` default = '150000'
   (x466-C98 банк-канон) — каждая банк-нога УЖЕ pop150k; очередь (118 bench, AG-119)
   содержит свежие pop150k ваниль-якоря пост-фикс мастера непрерывно (срез MAIN 08:08Z:
   ch/s healthy-band 9.1-12.0). LIMBO-инъекция AG-43 = pop625k-клетка; pop150k s300
   SUCCESS канон (AG-498 gc6 run 37000385561).
2) **eindex-A/B = убитый класс**: cmp405_eindex counts-skip исторически PARITY (x410,
   LAB_LEDGER Л-466-C12.4); ARM-маркер шаг 4/4, seed failed(0) прецедент дорман-класса
   (Л115). Диспатч = плацебо №2-класс.
3) **capture-математика counts-skip**: capture = P(count==0) по rect-чанкам (superset-скип
   пустых). Банк 150k на ~9.2k чанк = 16.3 ent/чанк в среднем; кластеризация 9951 чанк /
   497 кластеров (Л202) оставляет пустые чанки, но getEntities-rect'ы живых запросов
   сидят в кластер-корах (datdpak-пробы 351 скан/тик @e, AG-19) → P(count==0) в
   посещаемых rect'ах ≈ 0 → realize-потолок = 0пп (подтверждено измерением x410).
   Capture селектор-лейна (70-95%, AG-19) принадлежит PER-TYPE индексу, не counts-skip.

## 3. Хэндофф: живой остаток fork#3
Единственная живая форма fork#3 = A/B per-type индекса на pop150k (коллапс-класс,
капчур 30-58пп AG-19) — lane AG-128 (rust substrate iter-1) + AG-187 (chains slice-1)
+ java iter-1 AG-160 (d5f0c767, dormant, selftest GREEN). Вилку НЕ трогаю (CLAIM|AG-x).

## Верификация
- `git cat-file -p origin/master:bench/worldv2/report_benchv2.py | grep -c 'AG-116 w528'` = 2
- `origin/master` = 3326d887..e5584ca9 (живой fetch 08:50-09:0xZ); blob-байты, не display.
- workflow default: `.github/workflows/world-bench-parallel.yml` `population_target: default '150000'`.
