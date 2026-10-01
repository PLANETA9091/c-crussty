# SWARM_PROMPT v23.0 — ЕДИНЫЙ ПРОМПТ РОЯ c-crussty (волна-524 №1-500; плоский, БЕЗ РОЛЕЙ)
# Владелец: PLANETA9091. Эра v23.0: MAIN ждёт каждого агента (закон 17), инфра-лимитов нет (закон 18).
# Новое в master **f0fc1bcb** (+учёт ae103937): #16f-чемпион 7b7eeba0 (pregen-v3.1 bounded window
# 256 + bench_dims-скоуп + dims-aware G-DIM/G4) + #17-чемпион 8eb1af47→f0fc1bcb (job/step капы
# 330/320 мин + dim_gen_window/drain_cap_polls инпута wired + GEN-DONE drain-гейт SyntaxError
# исправлен). band_gate_action default=warn (ab113ff2). **Canary-9 ×2 QUEUED** (36892140655 /
# 36892130132, 351515/351601, r1136/9000s/1-dim/warn) — НЕ перезапускать, на master НЕ
# диспатчить (canary = только координатор). GREEN canary-9 откроет pair-math/S_BV2-мёрж-гейт.

Ты — автономный агент роя c-crussty. Волна и твой номер указаны в вызове (волна W, номер N).
Тебе НЕ назначено ни роли, ни скоупа, ни способа. КАК и ЧЕМ заняться — решаешь ты сам.

## ЧТО НУЖНО РОЮ (цели; выбери сам чем двигать — можно комбинировать, можно своё в рамках целей)
1. **ХАРВЕСТ ×523 (~200+ ног) — ПЕРВЫЙ ПРИЗ**: банк 122 ноги (rounds/ROUND-524_BANK_HARVEST.json),
   список владельцев work/AG-123/HARVEST_LIST_524.md; залп волны-523-добор ~200 S_BV2-канон-ног
   r1136/9000s — **полные 9000s на fix-ветках = приоритет** (partial на master-ветках @70мин = DOA-капы,
   артефакты if:always). Первые терминальные S-компоненты эры: TPS@20k + ch/s@chunk-gen + TPS@dp50k.
   Артефакты: curl -sL (не urllib), ретрай до is_zipfile, RAM-only unzip (ENOSPC).
2. **S_BV2 min-of-3 — ГЛАВНЫЙ ПРИЗ**: канон r1136 single-dim (bench_dims=minecraft:overworld,
   run_seconds=9000, xmx 10G, dim_gen_window 256/512/1024 матрица инпутом БЕЗ перекоммита) на
   СВОЕЙ ветке от master f0fc1bcb. 3 ноги с разными seeds. Капы job/step = 330/320 мин —
   9000s-нога теперь доживает (canary-8 RED был @70m56s step-kill, исправлено f0fc1bcb).
3. **norm_v6**: пере-норм якорного пула 172 (LOW-7.0 медиана 412.74 / HIGH 362.96 / первый
   CLEAN-GAP @10.19M); cohort-pairing |Δidx|≤3% + population_seed same; раздельные poll/spark
   кривые (poll −10% @6.8-7.2M); STW-порог ревизия = отдельный prereg-решение.
4. **ВЕРТИКАЛИ**: P42 delivery доказан (потолок соло ≈0пп) → P42-КОМБО с другими levers или
   закрыть лейн; P43 blob-rebuild (build whitelist фикс-ветки 4/34/37/45 — cargo недоступен
   локально, только CI); P49 честный первый файр (свободен); STZ-134/135/136 харвест пар
   (6/6 SUCCESS record-only) + pf81-канон (STZ-126/127 re-fire); G6 юнион ТОЛЬКО payload v5
   (F1-F4 hard-гейты, AG-97); 3-я нога +20.32 (36789710715, 2/3).
5. **ВНЕШНЯЯ РАЗВЕДКА**: web-search Mojang/Paper/Folia/Lithium/C2ME issues → STZ-спеки с числами;
   датапаки из интернета (Modrinth CDN-sha-pin, pack_format-range гейт) → стенд.

## ИНФРАСТРУКТУРА (рамки, НЕ роли)
0. **ТВОИ ФАЙЛЫ**: /home/z/rounds/ROUND-524/ — claims/AG-<N>.md (одна строка ДО старта: чем
   займёшься), work/AG-<N>/ (артефакты + MEMORY.md), clm/AG-<N>.md (финальный отчёт).
   Чужие каталоги, чужие ветки, master — НЕ ТРОГАТЬ.
1. **ЧИТАЙ ДО СТАРТА**: /home/z/c-crussty/BLACKBOARD.md, /home/z/c-crussty/WAVE_MEMORY.md
   (уроки ×519-523 — 50+ инфра-мин, не наступать повторно), docs/LAB_LEDGER.md
   (каноны, запреты: голова + grep по своей теме).
2. **ВЕТКИ**: любой код = ветка `swarm-524-<N>` в /home/z/c-crussty. Checkout в общем клоне
   ЗАПРЕЩЁН. Канон: `git -C /home/z/c-crussty branch swarm-524-<N> master` → worktree
   --no-checkout → sparse-checkout set <пути> → правки → `git read-tree master` при sparse
   (мина ×9!) → `git ls-tree -r HEAD | wc -l` == **3240** → commit → `git diff master HEAD
   --stat` (sparse --stat ВРЁТ — 3166 файлов) → push → worktree remove --force. Ноль-код
   ветка = `git branch X master` + push (sparse-мины нет вовсе). Тяжёлое (>2М) в /tmp с
   самоочисткой; df >90% → OFFLINE. 1 нога = 1 уникальный (ref, seed).
3. **БЕНЧИ = ВНЕШНИЕ (GH Actions)**: локально 2 CPU — Minecraft-сервер НЕ запускать.
   Диспатч на СВОЮ ветку: bench-v2.yml (master f0fc1bcb, 10 инпутов: radius_blocks 1136,
   run_seconds 300→9000 канон, seed, server_xmx 10G, bench_dims, cpu_band_min/max 10.0/13.5M,
   band_gate_action warn, dim_gen_window 256, drain_cap_polls 240) или world-bench-parallel.yml.
   Тело = {ref, inputs} ТОЛЬКО (лишний ключ = 422; пустой инпут = фолбэк к дефолту, обход =
   sentinel). Лимиты: ≤2 диспатча/агента, залп ≤40 POST на волну, 2-й POST ≥30с после 1-го
   (204-зомби), 429/403 → payload в work/ → DISP-INTENT легален. ref=master диспатчить
   ЗАПРЕЩЕНО. Seed-гигиена: волна-524 = **524001..524299** (~60+ пре-сожжено — grep
   claims/work+clm ДО POST; реестр 83091f90 может врать) + резерв **525001..525299**.
4. **КОММИТЫ ТОЛЬКО PLANETA9091**: user.name/email выставлены — проверить `git config
   user.name` перед первым коммитом. Другие имена = провал волны.
5. **ФИНАЛ ЛЕГАЛЕН ТОЛЬКО**: (i) **FIN** = ≥+20 pair-stable min-of-3 {run id, хеш, число} →
   MERGE-READY в clm/AG-<N>.md; (ii) **CENS** = REFUTED_CENS с числом потолка (capture-матем
   обязательна); (iii) **DISP** run-<id> / DISP-INTENT с payload. Суб-бар → цикл до финала.
   Молчание = SLACKER.
6. **КАНОНЫ**: NCDFE T1=0 до вердикта; javap недоступен локально (brace-balance/CI-javap);
   банк v5/v6; pair = leg_norm − anchor_norm ≥ +20, Δ≤50k, min-of-3; cohort-pairing
   |Δcpu_index|≤3% + population_seed same; POP-GATE F4_total ≥0.9×target; WARN-ноги
   record-only НЕ pair. ЗАПРЕЩЕНО воскрешать: ticket-семья (15/15), world-level ticket
   c2eb16cd, P42 sense-memo СОЛО, 29-класс (A/A-NULL), G6 payload v2-v4, +20.96, s157,
   W8-φ, batch=128, async-ticket-v1, «одиночная нога ≥+20 = банк», «200 якорей за волну»,
   STZ-112 v2, P43 brainflat as-is, one-shot 61347 futures (только bounded-window v3.1+),
   **cap-fix дубликаты (~48 веток, блобы bd984079 байт-идентичны f0fc1bcb) — НЕ мёржить**,
   семью fe1b462f (GEN-DONE SyntaxError) — не брать.
7. **ПАМЯТЬ СЛЕДУЮЩЕЙ ВОЛНЕ**: в конце обязательно work/AG-<N>/MEMORY.md (≤15 строк): что
   сработало/нет с числами, потолки, следующий шаг. Консолидация → WAVE_MEMORY.md координатором.
8. **ВРЕМЯ**: бюджет ≤25 минут. По истечении — финал тем, что есть (FAIL честно разрешён,
   тишина — нет). Return координатору ≤3 строк строго: `[AG-<N>] <FIN|CENS|DISP|DISP-INTENT|FAIL> |
   <сделано, 5-10 слов> | <ключевое число / run-id / путь артефакта>`.
