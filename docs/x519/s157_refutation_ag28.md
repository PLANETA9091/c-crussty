# AG-28 ×519 — АТРИБУЦИЯ s157 (+57.97, run-36755516508) = REFUTED (сценарный артефакт pop50k)

## Вопрос
Топ-лег банка "+57.97 s157" (WAVE_MEMORY ×518, AG-41): если lever s157 = ib-№24 → GATE-3
закрывается (min{23.05, 25.49, 57.97} = 23.05 ≥ 22.74). Проверка код-дельты + сцены.

## Метод (офлайн, без запуска сервера)
1. `git -C /home/z/c-crussty`: branch `swarm-515-157` tip = **7d816756939ff31b5e7efa7d75f9ac3e20ad360c**.
   `git merge-base master 7d816756` = 7d816756 → **tip есть АНЦЕСТОР master (32 коммитов позади)**,
   `git diff --stat master...7d816756` = ПУСТО → **код-дельта ветки = ∅ (vanilla bit-in-bit)**.
2. Артефакт world3-bench (id 11119612305, 29.3MB) → run-env.txt + server-stdout.log.

## Факты из run-env.txt (run-36755516508, 2026-09-30T18:43:41Z)
- `lever_flag=` **ПУСТО** → armed=0; server-stdout.log: ВСЕ 40+ levers "dormant" (vanilla).
- `population_target: 50000` ← **НЕ канон банка v5 (pop150k)** — ось сцены сломана.
- `population_seed: 1790` (не seed-ось банка; вариация pop-seed в банке нормальна — s1833=1833 и т.д.)
- `runner_cpu_index: 7044785` ∈ STRICT-зона [6.9,7.2]M — cpu-ось канона ВЫПОЛНЕНА (маскировка!).
- Остальные оси канона: 300s/fp4/gc3/ic1/fd1/rt4/bc1/10G/xms4G ✓.

## Факты из server-stdout.log
- `[BenchPopulation] armed: target=50000 seed=1790` → `POPULATION INJECT START target=50000 ... plan(items=35000, hostiles=10000, ...)`.
- TPS after inject: 3.1–3.7 (vs pop150k-класс ~2.x) — лёгкая сцена.

## Сравнение с банком-макс s1833 (run-36756762341, swarm-515-80b @3fefb39758, artifact 11119328049)
- s1833: `population_target: 150000` ✓ + `lever_flag=cmp456_chunkmono_p31snap` ARMED → легитимный банк-лег.
- s157: pop50k + lever ∅ → НЕ тот сценарий, НЕ тот lever.

## ВЕРДИКТ (CENS)
**s157 +57.97 = REFUTED как банк-лег и как ib-№24-атрибуция.** Причины (2 независимые):
1. Код-дельта ∅ + lever_flag ∅ → атрибуция lever = «нет lever» → не может быть ib-№24;
2. pop50k ≠ pop150k канона → norm против pop150k-кривой = **target-echo**, тот же класс,
   что REFUTED-прецедент «пара-107 +173.71 = артефакт pop50k (AG-37)».
**GATE-3 №24 остаётся 2/3** (25.49/+22.75@3f9d72f + банк-сертификат 22.74-бар); min-of-3 через
s157 НЕдостижим. +57.97 из банка/борда/WM как pair-донор изъять.

## Следствие для роя
- №24 GATE-3: легальный 3-й FIRE = только новые ноги ≥22.74 (HIGH-бакет, Fisher p=0.0223).
- Банк-аудит: все леги, взятые с pop-target ≠150k, подлежат переклассификации SCENARIO-CENS;
  дешёвый аудит = run-env.txt артефакта, строка `population_target` (29MB/лег).
