# MEGA-ROUND-2 VERDICT (TASK-397, 2026-09-21, cron 401462)

ТОП-1 ботлнек (без изменений на master — раунд-1 без мержа): items/ItemEntity.tick
31.17% java (36051/115655, банк v4 fp=4) + item-driven broadphase 15.66%.
Формат: 10 сабагентов-турнир (A..J), каждый — уникальный механизм, своя ветка
round-397-*, свой CI-бенч world-bench-parallel.yml (банк v4, fp=4, 150k/seed42,
300s, band 6.0-9.5M), запрещён перекрёстный просмотр веток (читерство).

## РЕ-АНКЕР (новое знание тика)
Чистый банк-лег @ master 3c48a96: run 35543961438, runner 6918472 (band OK),
median 2.20 vs TPS_exp 2.26 = **−2.5% дрейф якорной линии** (GC Full=9/21.3s vs
справка Full=7/18.8s — тренд дрейфа подтверждён). Разброс раннеров против
2-точечной линии до +12% (не-armed лег @ 7232045 дал фейковые +12.0%) →
одиночные ноги <10% недостоверны, репликация обязательна.

## КАРТА ВЕКТОРОВ (Δ = median5 vs TPS_exp-интерполяция на своём runner; ARM-пруф = маркер патча в server-stdout)

| Вектор | Механизм | Ветка | Δ (валидные ноги) | ARM | Вердикт |
|---|---|---|---|---|---|
| **H items_oss** | порт Lithium item_entity_merging (QUERY_LIMIT=64 + dry-run) | round-396-h-items_oss | **+6.9% / +15.7% → медиана +11.3% min-of-2** | ✓✓ (28904→28621+serve) | **ПОБЕДИТЕЛЬ, реплицирован → МЕРЖ 4f927bd** |
| E sweep2 | sweep-line батч-мерж (ItemsSweepOps.tickMerge, eligibility-гейты ~97% reject) | round-397-e-sweep2 | **+8.1%** @6750203 (leg1 не-armed ложный +12% @7232045, leg2 band-out 9781272) | ✓ | GREEN-CAND 1 нога |
| C soa | SoA-плоские массивы горячих полей (ItemSoaOps, mergeWithNeighbours) | round-397-c-soa | **+7.5%** @7030445 | ✓ | GREEN-CAND 1 нога |
| G footprint | cache-line footprint: 48-site ретаргет + reorder, ItemFootprintOps/Unsafe-арена | round-397-g-footprint | **+7.2%** @7061762 | ✓ (28904→29142) | GREEN-CAND 1 нога |
| F oss_paper | Paper alt-item-merge (hook serve) | round-397-f-oss_paper | **+5.5%** @8804294 (items −2.47пп) | ✓ | MEASURED |
| D offthread | off-thread merge-скан на RegionTickOps-воркерах, детерминированный apply | round-397-d-offthread | **+3.5%** @6532380 | ✓ (28904→28976) | MEASURED |
| B stagger2 | фазовое расписание merge/move/nocollision N=4 | round-397-b-stagger_fix | **+0.5%** @7072026 (items −4.53пп — лейн режется, конверсии нет) | ✓ | MEASURED |
| A compose_ai | композиция index(A-396)+wakeup(I-396) | round-397-a-compose_ai | **−9.6%** @8287162 (items −2.02пп) | ✓ (28904→29304) | RED — ортогональность НЕ подтвердилась |
| H2 despawn_heap | heap-scheduled despawn (ItemLifetimeOps v1, despawn-only) | round-397-h-despawn_heap | +2.9% band-out @9585180; **−19.7%** @6853890 | ✓ (28904→28978) | RED — heap-оверхед на churn 150k |
| J subsys2 | полная замена подсистемы + встроенный grid-индекс | round-397-j-subsys2 | — | — | **SIGSEGV crussty::items_index::link** (unsafe-link без lifetime-guard) — crash-doc, фикс в след. раунде |
| I replicate | min-of-2 H + leg3 F-mono | round-396-* | H leg2 **+15.7%**; F-mono leg3 −4.5% | ✓ | H подтверждён; F-mono списан (+10.1/−4.5) |

## БЫЛО → СТАЛО (победитель items_oss, банк v4 fp=4, 150k, band)
- TPS: банк-интерполяция 2.16-2.30 (по runner'у) → **2.50 @6467076 (+15.7%)**,
  вторая нога 2.40-эквивалент (+6.9%); медиана репликации **+11.3%** ≥ +10% бар.
- CPU: items-лейн 31.17% → 30.64% (leg1); java_util 7.01→6.49-6.83%;
  мердж-трафик broadphase частично демпфирован QUERY_LIMIT=64.
- ОЗУ/GC: ParallelGC total 21.9s/Full=9 (leg2) — в норме семьи 20.0-24.9s/Full=9,
  heap-политика не деградировала, отдельной RAM-цены нет.
- Паритет: ванильная семантика мерджей (exact-vanilla fallback при
  truncated-walk re-query), superiority-отклонений нет.

## ИНФРА-УРОКИ ТИКА (§5-реестр)
1. **Task-инфра дедлайн при параллельных Task-вызовах**: 10 параллельно → 9
   «умерли», батч 4 → 4 «умерли»; НО агенты успевали ДОДЕЛАТЬ пайплайн
   (research/ветка/диспатч) до смерти репортинга — «умерший» агент = собирать
   артефакты (worktree log, runs API), не перезапускать слепо.
2. **ARM-верификация обязательна**: не-armed лег дал фейковый GREEN-CANDIDATE
   +12.0% (раннер-удача); «pristine sighting» без маркера патча = нога INFRA.
3. **Band-дисциплина**: runners 9585180/9781272 > band_max 9.5M — леги
   невалидны вне полосы независимо от ARM (прецедент продолжается).
4. **Диск 9.9G полный** (Errno 28): worktrees качали research-архивы (2.1G
   zips в agent-h) + cargo target/ + unz-дубли — чистка /research/gc-recon в
   worktrees, target/, unz/ = −4G. Правило: не качать архив истории в worktree.
5. Сигнатура краша rust-левера в server-stdout: `SIGSEGV ... _RNv...crussty
   ...items_index4link` — диагностируется без hs_err.

## NEXT (TASK-398)
1. Банк v5 = v4 + items_oss (lever_flag=items_oss) → ре-анкер банк-v5 лег →
   новая 2-точка якоря (текущая линия −2.5% дрейфует).
2. Репликации: sweep2 +8.1%, soa +7.5%, footprint +7.2% (по 2-й ноге;
   композиция soa+footprint/oss — после репликации).
3. J-subsys2: lifetime-guard в items_index::link (epoch/null-check) → ре-арм.
4. despawn_heap RED закрыт (v1); v2 (heap в rust + батч-применение) — только
   если sweep2/soa реплицируются неудачно.
