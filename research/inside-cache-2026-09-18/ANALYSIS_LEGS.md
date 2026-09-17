# S7-135b — ABSORB legs #1/#2 INSIDE-CACHE (runs 35282003292 / 35284069355)

## Leg #1 (35282003292, master 1bd7f52): ИНЕРТЕН ПО ПОСТРОЕНИЮ
- ARMED живьём: pristine 205458B (=фикстуре), defined InsideBlockOps(+Recorder), computed patch Retargeted{1}, hook serve 205522B, retransform rc=0; fixture VALID 150000/150000; ap.log чист (4×started)
- Gate ВЫЗЫВАЛСЯ (15 CPU-сэмплов — патченная копия живая), НО: (1) capture недостижим — пустой слот всегда → ваниль, mirror был только на инвалидации несуществующего кэша; (2) delta-чек не проходил — покоящиеся предметы несут гравитационный остаток deltaMovement
- Лейны = база в шуме (LongOpenHashSet 451→478, BlockPos$6 385→372, Vec3.add 702→510 −27% — межрановый шум S7-96d)
- Вторичное наблюдение: двойное pristine sighting Entity (205458 ядро + 204078 — другая копия, не тикает); retarget попал в живую копию (CPU-след gate)

## Bridge v2 (d8453b3): фикс двух дефектов
- статик-детектор: xo==x && yo==y && zo==z (from==to==pos; deltaMovement удалён)
- bootstrap capture: пустой слот + статик ⇒ mirror (serve+capture) вместо ванили
- офлайн OFFLINE PASS повторён, rust 106 ✓

## Leg #2 (35284069355, master d8453b3): РАН НЕВАЛИДЕН (инфра/сцена)
- Сцена коллапсировала: F4 total 67-74k живых (база 148k!), item ~49k (база ~100k), hostiles вначале ~×0.3 базы с ростом к концу — при этом FIXTURE-VALIDITY: VALID (INJECT DONE 150000/150000 маркер в отчёте) — популяция РАСПАЛАСЬ ПОСЛЕ инъекции
- Crawl с момента инъекции: TPS [22.8 → 1.5, 1.7, 2.0, 2.6, 3.3], MSPT avg 430ms (база 85-125)
- Артефакт НЕПОЛОН: server-stdout.log отсутствует (маркеры ARMED/injection/topup неверифицируемы)
- Bridge в CPU = 0.4% (212/51063 сэмплов) — самовзрыва моста НЕТ; фазовая разбивка структурно нормальна; НО per-entity стоимость entity-фазы ~9× (238ms CPU/тик / 70k сущностей vs 0.38μs/сущность в базе) — причина НЕ изолирована
- Главный подозреваемый (не доказан): ping-pong слотов — 150k сущностей с последовательными id в 131072 слотах ⇒ ~19k пар (id, id+131072) делят слот; каждый тик оба «пусто» ⇒ оба mirror ⇒ перезапись друг друга вечно (mirror ≈ ваниллен по стоимости, но +Recorder-аллокации + card-marks записей в old-gen массивы слотов — урок §155). Второй: runner-контеншн (LCG-индекс 8.86M высокий, но это single-thread скорость; wall-профиль: 84.5% libc-idle — треды ЖДАЛИ)

## Вердикт: lever INSIDE-CACHE остаётся ARMED-CODE-BANKED (default 0 fail-closed). NEXT (S7-136):
1. Ping-pong hardening: пустой-слот bootstrap capture ТОЛЬКО если SLOT_EID[slot]==0 (слот, занятый другой сущностью ⇒ ваниль, без перезаписи — ping-pong уничтожен; ~13% сущностей живут ванилью при 150k>131072)
2. Чистый re-run leg #2' (та же кодовая база + hardening)
3. Если лейны снова не сдвинутся при живом HIT — STEP-0 ДжИТ-проверка (деопт-телеметрия retransform)
