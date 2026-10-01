# MEMORY AG-185 (волна-523, HARVEST)
- P42 resurrection REFUTED ×2 delivery-gate: SUCCESS-ноги 36860335151 (AG-37@10b7b46) и 36860543697 (AG-4) — jar_md5 ОБОИХ = 83b6f9c9 (ваниль-алиас), GoalMemoOps в runtime-jar ОТСУТСТВУЕТ (0 entries / 9809 классов) → фиксы build-script НЕ доставляют класс в жареный jar; lever_arg=p42m в payload, но lever физически 0 → вклад ΔS = 0 (capture-math), потолок P42 0.70-0.79пп недостижим. leg-3 36861079477 FAILURE.
- ОБЯЗАТЕЛЕН pre-dispatch гейт: `unzip -l patched-kernel.jar | grep GoalMemoOps && md5 != 83b6f9c9` — он поймал бы оба фейла ДО диспатча.
- STZ-134 (AG-46) = A/A-CONTROL пара, ОБА leg vanilla (stz134_squeeze arg=0 + stz134_ctrl arg=0): TPS 2.5 vs 2.6 (7.50M/7.02M idx); Δidx 6.6% >3% + pop_seed 523046≠523047 → pair ILLEGAL; 2 ваниль-точки для norm_v6 LOW-кохорты.
- #16f pregen-v4 вериф: 0/5 завершённых SUCCESS (FAILURE: 36869235123/115, 36869768680+36869772481/118, 36869695219/145, 36868762311/146; бисект 36869607136/137, 36868906044/124 RED); in_progress: 106/114/120/132/142. Чемпион НЕ найден, canary-7/S_BV2 не разблокированы.
- STZ-516 DnT RED ×2 (36864015923/36864025995); STZ-126/127 v2 RED ×2 (36863227064/36863241535 — reject-риск pf подтвердился); STZ-136 in_progress (36870018656/36870028829).
- canary-6 GREEN-кандидаты подтверждены SUCCESS (36859437234/36860192558).
- STZ-135 (36863333652/36863342802) SUCCESS ×2 — артефакты НЕ скачаны (время) → re-harvest ×524 первым делом.
- Урок: harvest.py RAM-only сработал; stderr нельзя глушить (> /dev/null скрыл 404 retry).
- Следующий шаг ×524: (1) re-harvest STZ-135 пары; (2) P42: чинить CI build-path (grep GoalMemoOps в собираемом jar на этапе CI, не локально); (3) #16f ждать in_progress-ноги.
