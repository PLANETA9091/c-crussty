# RECON-14 — свежий ТОП (s7169) + закрытие ТОП-1 chunk-parse + разморозка лейна курсора — 2026-09-19 ~19:1x +08 (TASK-330, тик 18:33)

Инструменты: `recon14_fresh_top.py` (свежий ТОП трёх осей), `recon13f_parse_diet.py` (под-лейны parse), census `chunk-parse-diag.txt` (s7169). База: s7169 (run 35437243128, банк v3 + parse_diag=1 + recon_diag=1 — самый свежий профиль).

## 1. Census s7169 — кэш REFUTED ЖЁСТКО, parse-лейн закрыт
- Фикс арма (a93324a) работает: "dump task ARMED -> .../chunk-parse-diag.txt" в stdout; census доставлена: **total_loads=160, unique=160, repeats=0, repeat_share=0.0%** (span 275s, consistency PASS: row_sum == total).
- repeat_share 0.0% < 10% → **lever #12 (decode-cache) REFUTED** — лейн = first-loads при ticket-churn, повторных parse НЕТ вообще.
- Свежий профиль s7169: parse-лейн = **0.00% alloc / 0.02% CPU** (в s7165 было 45.04% — burst эпизодичен, окно-зависим). По директиве «ТОП-1 ОБЯЗАН УПАСТЬ» лейн сдался: **исчез из свежего профиля** + решающий census-вердикт.
- 160 parse'ов на soak = ~64 ap-сэмпла на parse — объясняет «45% окна» при эпизодических волнах; steady-state вклад лейна ничтожен.

## 2. Свежий ТОП (s7169, три оси)
| ось | #1 | #2 | #3 |
|---|---|---|---|
| CPU (deepest-frame) | **entity-tick-core 34.04%** | world-tick misc 25.31% | remset/card-set (G1) 24.26% |
| ALLOC | **entity-tick-core 68.20%** (Vec3 21.06 + AABB 18.87 + BlockPos-семья ~13.6) | world-tick misc 16.78% | navigation 1.13% |
| WALL | libc-idle 80% (не узкое место) | — | — |
Кросс-ран vs s7165: entity-tick-core 36.09→34.04% CPU (стабилен), remset 25.61→24.26%, navigation 6.58→1.15% (эффект inside_cache), **chunk-parse 0.00→0.02% (исчез)**.

## 3. Новый ТОП-1 = entity-tick-core; крупнейший атакуемый под-лейн = КУРСОР
- Квантификация лейна курсора ( needles: BlockPos$6/betweenCornersInDirection/forEachBlockIntersectedBetween) на свежих профилях: **s7169: 29.18% alloc / 6.42% CPU; s7165: 9.72% alloc / 7.56% CPU** — старая заморозка «1.5% ap» снята, лейн ≥5% С ТО ЗАПАСОМ.
- Механика (cp-скан jar): BlockGetter.forEachBlockIntersectedBetween → на каждый вызов fresh BlockPos$6 + MutableBlockPos; addCollisionsAlongTravel аллоцирует НОВЫЙ итератор на КАЖДЫЙ шаг travel. Единственная точка порождения — синтетическая `lambda$betweenCornersInDirection$8` (22B → new BlockPos$6).
- Референсы уже-REFUTED рычагов лейна: zero_alloc_inside #10 REFUTED-BY-ECONOMICS (fluid-push +15.2%), flat_traversal #9 FAIL PG4a — курсор не покрыт ни одним из них.

## 4. Рычаг #11 v1 (CRUSSTY_ZERO_CURSOR) — реализован В ЭТОМ ТИКЕ
- `entityinside/net/minecraft/core/ZeroCursorIter.java` — бит-точная реплика BlockPos$6 (свой Iterator, без guava-состояния); `ZeroCursorOps.lambda8` — ThreadLocal-кольцо ×4, reset на вызов.
- Верификация: **CursorLockstepHarness 350,000 сценариев / 31,761,141 позиция бит-в-бит PASS** (против РЕАЛЬНОЙ ванильной lambda через рефлексию).
- `redirect_static_method_body_to_static` (classfile.rs, +4 теста на реальном fixture BlockPos_real.class); `zero_cursor.rs` (хук pristine/serve + define обоих мостов в kernel loader + retransform); workflow input zero_cursor (fluid_free дропнут — лимит 25), run_world3.sh CRUSSTY_ZERO_CURSOR.
- cargo 182/182 PASS; release PASS. c-crussty 28539f3. **ЛЕГ s7170: run 35440054574 @ 28539f3 (19:25:07 +08)**.

## 5. Прегистер-гейты s7170 (полный текст в dispatch_s7170.py)
PG-Z2 доставка (маркеры pristine/defined/computed redirect Retargeted{sites:1}/armed rc=0, NCDFE=0, pop VALID); PG-Z3 TPS ≥1.60 + дельта vs 1.80; PG-Z4a cursor-alloc лейн ≥−50% (ожидание −80..−95%); PG-Z4b young ≤154, 0 Full; PG-Z4c cursor-CPU ≥−30%; CRASH-FREE. Банкинг: PASS → CUMULATIVE v4 = v3 + zero_cursor=1; FAIL → REFUTED + rollback.
