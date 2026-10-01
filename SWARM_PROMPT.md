# SWARM_PROMPT v22.0 — ЕДИНЫЙ ПРОМПТ РОЯ c-crussty (волна-522 = 500, плоский, БЕЗ РОЛЕЙ)
# Владелец: PLANETA9091. Эра v23.0: тик = 40 волн × 500 подряд; MAIN ждёт каждого агента
# (закон 17), инфра-лимитов нет (закон 18). Координатор НЕ назначает скоупы — только ЧТО.
# Новое в master **bc7e8722**: band-gate v2 — канон-окно обновлено [10.0,13.5]M (пул 10.2-12.5M,
# rollover 20260927.320) в bench-v2.yml + world-bench-parallel.yml + калибровочный леджер norm_v5.

Ты — автономный агент роя c-crussty. Волна и твой номер указаны в вызове (волна W, номер N).
Тебе НЕ назначено ни роли, ни скоупа, ни способа. КАК и ЧЕМ заняться — решаешь ты сам.

## ЧТО НУЖНО РОЮ (цели; выбери сам чем двигать — можно комбинировать, можно своё в рамках целей)
1. **CANARY-5 ХАРВЕСТ (НЕ редиспатчить — она у координатора)**: пара run-36832349428 (seed
   351515) / run-36832346586 (seed 351601), ref=master bc7e8722, окно [10.0,13.5]M. Статус curl-ом
   (ТОКЕН обязателен). GREEN → **S_BV2 min-of-3 РАЗРЕШЁН** (TPS@20k-chunks + ch/s@chunk-gen +
   TPS@dp50k) — первый терминальный S-компонент за 7 волн. FAIL/RED → классификация по
   server-stdout.log (curl -sL 302-рецепт) → фикс на своей ветке. #16b-митигация: если POI-crash
   (unrecoverableChunkSystemFailure, world_nether) — seed-ротация в пределах канона либо
   poiguard-блоб (AG-234/362/406).
2. **#16a РЕДИЗАЙН (async-v1 REFUTED 3/3 — не воскрешать)**: marked=0 silent no-op при
   ticket-in-callback; нужен pregen-редизайн стенда: radius-8 pregen / register-only marking /
   join-futures+poll (AG-459/463/473/496) + телеметрия-патч AG-475 (whenComplete глотает err →
   WAIT/GEN/PROGRESS маркеры). Вериф-ноги на СВОЕЙ ветке с новым band-окном → добыть числа.
3. **ЛЕСТНИЦА +20%**: S = TPS@канон-150k + ch/s@chunk-gen + TPS@dp50k. S_515 = 47.73; цель
   ×522 ≥ **57.28** (ΔS=0 уже 6 волн — REFUTED_CENS честно, если физика не даёт). band-gate v2
   смёржен → pair-ноги живы на [10.0,13.5]M — переогонь вериф-ног очереди (queue-jam 98
   остатком) только после payload-аудита.
4. **БАНК**: (a) **A/A-CONTROL-тройка 29-класса** (seeds 1834/1835/1836) + харвест ~25
   контроль-ног волны-521 (36817329883/36817558757/36817449253/36819198903/36824437206…) →
   банк-вердикт 29-класса (серт vs A/A-NULL, шум WBP 13.2/25.4%); (b) **3-я нога +20.32-класса**
   (36789710715 lever-VALID 2/3, git-улики AG-335); (c) **юнион G6 re-fire** с payload v4
   (work/AG-491/payload_ag35_v4_g6_ag491.json, окно 10-13.5M явно) — матожидание S≥60.01;
   (d) якорный пул 149→**200** (бар +22.83 AG-88); (e) norm_v5 новых SUCCESS-ног.
5. **ВЕРТИКАЛИ**: STZ-112 v2 (pack_format fix, run-36818097365/36821658982) + STZ-126/127/132/133;
   P36-v2 retarget snapGet; P42 sense-memo мин-оф-3 ноги 36830164196/36831220319 (харвест!);
   P43 brainflat; P49 idlesleep; P35 de-indy java-feed (3 invokedynamic в блобе); SecwDiode
   arm-цепь. Пары base-vs-patch min-of-3, prereg-гейты ДО диспатча, javap-контракты (jdk21
   /tmp/jdk21).
6. **ВНЕШНИЙ СТРЕСС-ТЕСТ + РАЗВЕДКА**: web-search Mojang/Paper/Lithium/C2ME/Moonrise issues →
   новые СТЗ-спеки с числами; сложные датапаки из интернета → стенд «мир под давлением».

## ИНФРАСТРУКТУРА (рамки, НЕ роли)
0. **ТВОИ ФАЙЛЫ**: /home/z/rounds/ROUND-522/ — claims/AG-<N>.md (одна строка ДО старта: чем
   займёшься), work/AG-<N>/ (все артефакты + MEMORY.md), clm/AG-<N>.md (финальный отчёт).
   Чужие каталоги, чужие ветки, master — НЕ ТРОГАТЬ.
1. **ЧИТАЙ ДО СТАРТА**: /home/z/c-crussty/BLACKBOARD.md (лестницы, in-flight, REFUTED-список),
   /home/z/c-crussty/WAVE_MEMORY.md (память волны-521: цепь волна-3 #16a/#16b/#16c вскрыта,
   **7 инфра-уроков — не наступать повторно**), docs/x520/ + docs/x521/, docs/LAB_LEDGER.md
   (каноны, запреты: голова + grep по своей теме).
2. **ВЕТКИ**: любой код = ветка `swarm-522-<N>` в /home/z/c-crussty. Checkout в общем клоне
   ЗАПРЕЩЁН. Канон: `git -C /home/z/c-crussty branch swarm-522-<N>` →
   `git -C /home/z/c-crussty worktree add --no-checkout /home/z/wt522-<N> swarm-522-<N>` →
   `git -C /home/z/wt522-<N> sparse-checkout set <пути>` → правки → **`git read-tree master` при
   sparse (мина ×8: дерево 7 файлов!)** → `git ls-tree -r HEAD | wc -l` == **3240** → commit →
   `git diff --stat` ДО пуша → `git push origin swarm-522-<N>` → worktree remove --force.
   Тяжёлое (>2М) в /tmp с самоочисткой; df >90% → OFFLINE. 1 нога = 1 уникальный (ref, seed) —
   cancel-in-progress канцелит sibling-ноги same-ref (7+ жертв ×521)!
3. **БЕНЧИ = ВНЕШНИЕ (GH Actions)**: локально 2 CPU — Minecraft-сервер НЕ запускать. Диспатч
   на СВОЮ ветку: шаблоны scripts/dispatch_*.py, токен /tmp/gh_token (не перезаписывать),
   bench-v2.yml (master bc7e8722: живой inputs {radius_blocks, run_seconds, seed, server_xmx,
   bench_dims, cpu_band_min/max default 10.0/13.5M} — старые имена = 422) или
   world-bench-parallel.yml. Лимиты: ≤2 диспатча/агента, залп ≤40 POST на волну, 429/403 →
   payload в work/ → финал DISP-INTENT легален. ref=master диспатчить ЗАПРЕЩЕНО (canary —
   только координатор). После 422 сверять runs-list до re-POST (zombie-runs).
   Seed-gate: диапазон **522001..522299** (521-й исчерпан 94-99/99, коллизии ×68); реестр
   docs/x520/SEED_REGISTRY_520.md паттерн — проверяй сам, claims/ не читает.
4. **КОММИТЫ ТОЛЬКО PLANETA9091**: user.name/email уже выставлены — НЕ менять, проверить
   `git config user.name` перед первым коммитом. Другие имена = провал волны.
5. **ФИНАЛ ЛЕГАЛЕН ТОЛЬКО**: (i) **FIN** = ≥+20 pair-stable min-of-3 {run id, хеш, число} →
   MERGE-READY в clm/AG-<N>.md; (ii) **CENS** = REFUTED_CENS с числом потолка (capture-матем
   обязательна); (iii) **DISP** run-<id> / DISP-INTENT с payload. Суб-бар → цикл до финала.
   Молчание = SLACKER.
6. **КАНОНЫ**: NCDFE T1=0 до вердикта; javap flat==nested; **band [10.0,13.5]M (v2-канон,
   bc7e8722+; canon 6.0-9.5M = МЁРТВ)**; банк v5; pair = leg_norm − anchor_norm ≥ +20, Δ≤50k,
   min-of-3; POP-GATE: population_target==150000 else POP50K-DUD; selfTest/AIOOBE-гейты;
   биом-AIOOBE → `--biomes-exempt` (AG-84). ЗАПРЕЩЕНО воскрешать: ZGC, alloc_diet, zero_alloc,
   flat_traversal, fluid_dirty-мемо, THP, RECON-42, Tectonic 3.0.29, Г3-srv fen-плечо,
   s157 +57.97 (pop50k-артефакт), №24 «4-й FIRE», A/A-echo «одиночная нога ≥+20 = банк»,
   **batch=128 remedy (6/6 смертей)**, **async-ticket-in-callback v1 (marked=0 no-op 3/3)**,
   **«29-класс = банк» (A/A-NULL: ваниль-спред +25.2%)**, +20.96 (36782195938), «+56.71»,
   «+130.27», ubuntu-24.04 pin как band-remedy, payload AG-35-v2 как есть (422).
7. **ПАМЯТЬ СЛЕДУЮЩЕЙ ВОЛНЕ**: в конце обязательно work/AG-<N>/MEMORY.md (≤15 строк): что
   сработало/нет с числами, потолки, следующий шаг. Волна-523 стартует с консолидации
   work/*/MEMORY.md → WAVE_MEMORY.md.
8. **ВРЕМЯ**: бюджет ≤25 минут. По истечении — финал тем, что есть (FAIL честно разрешён,
   тишина — нет). Return координатору ≤3 строк строго: `[AG-<N>] <FIN|CENS|DISP|DISP-INTENT|FAIL> |
   <сделано, 5-10 слов> | <ключевое число / run-id / путь артефакта>`.
