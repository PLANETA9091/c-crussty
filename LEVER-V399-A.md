# LEVER-V399-A — cmp399_batch (TASK-399-A, agent A): BATCH-JNI transport

База: origin/round-398-j-subsys2 (J). Вектор: per-item JNI-протокол J
(idx_insert/idx_set_cell/idx_query/idx_remove ≈ 33k переходов/с на сцене 150k)
заменён фазовым батчем: java (ItemEntityManager) копит за фазу тик-бакета все
grid-ops и merge-запросы ПОТОКА в плоский int[] (op-код, id, lid, cx, cy, cz;
для query — окно бокса, проскейленное java-side двойной математикой), затем
ОДИН нативный вызов `idxBatchApply(int[] ops, int opLen, int[] out)` на фазу
на поток (tickBucket-finally + forEach-start + post-phase-4-drain на main).
Rust (items_index::idx_batch_apply): ОДИН захват RwLock на батч, ОДИН
GetPrimitiveArrayCritical на вход, ОДИН на выход; кандидаты merge-запросов
возвращаются одним плоским массивом (out[1]=Q; пары offset+len; блоб id).
Порядок обхода клеток и chain-порядок кандидатов = per-call idx_query;
точные фильтры (level/isMergable/AABB.intersects/walls-clip/tryToMerge)
остаются java-side в ванильном порядке.

## Гейты

- J-arm (items_manager.rs lever_flag_matches): пропускает `items_subsys2` И
  любой флаг с префиксом `cmp399_` (composition contract п.2).
- Вектор (rust batch_flag_exact + java BATCH_MODE): ТОЛЬКО точный
  `cmp399_batch`; `idxBatchApply` регистрируется отдельным RegisterNatives
  только на этом флаге (на J-ногах класс его не объявляет — J-композиция
  не тронута). ARM-маркеры: rust eprintln
  `[crussty-plugin] cmp399_batch: ARMED batch_apply=idx_batch_apply (...)` +
  java LOG.info `[crussty-plugin] cmp399_batch: ARMED batch transport (...)`.
- Fail-closed: любой rc<0 / UnsatisfiedLinkError / рассинхрон заголовка →
  indexBroken=true → весь merge-путь в ваниль (как per-call ERR_STRUCT в J);
  незаведённый флаг/пустой → 100% ваниль.

## ПАРИТЕТ (что гарантированно идентично J)

- Семантика каждой op-записи — 1:1 реплика per-call натива (insert=unlink+link,
  setCell с no-op-branch, remove с never-inserted-branch; fail-closed коды).
- Стрим = temporal order записей данного потока (ops и queries в одном
  массиве), натив обрабатывает последовательно → каждый merge-запрос видит
  ровно те предшествующие мутации, что видел бы per-call протокол потока.
- Окно запроса: java-side `floor(min)-1 .. floor(max)+1` той же двойной
  математики, что per-call rust `f64::floor` — окно байт-в-байт.
- Порядок кандидатов: cz→cy→cx вложенные, chain order per cell = idx_query.
- Точные фильтры и ванильный порядок tryToMerge/break-on-removed — java-side.
- ERR_RANGE (>6 клеток по оси) → ванильный merge немедленно, как в J.
- OVERFLOW out-массива → java растит (×4) и переигрывает ВЕСЬ батч (ops
  идемпотентны; результаты первого прогона не потребляются).

## Superiority-отклонения (все ≤1 фаза, осознанные)

1. **Merge-экшн отложен до конца фазы** (запрос копится, tryToMerge вызывается
   на flushBatch в tickBucket-finally). Между гейтом и flush сам может
   деспавниться (age-гейт того же тика после merge-окна) — тогда мерджи
   пропускаются (applyMergeResults ре-чекает isRemoved/isMergable). В стационаре
   (осевшая популяция) позиции/стеки между гейтом и flush инвариантны → исход
   тот же; дельта — суб-тик.
2. **Межпоточная видимость мутаций** (setCell/insert чужого региона-воркера)
   видна на следующем барьеpe фазы, а не мгновенно. В J порядок межпоточных
   переходов и так был аппаратно-недетерминированным (общий RwLock per call);
   перекрёстные пары у границ регионов разрешаются фильтрами на flush.
3. **id-reuse отложен**: freed-id возвращается в freeIds только после применения
   батча (flush), иначе insert нового item с reused-id мог бы обогнать remove
   старого на чужом батче (в J per-call это невозможно). Задержка ≤1 фаза.
4. **Батч-отказ**: структурный отказ натива на фазе → indexBroken сразу +
   ванильные мерджи для ВСЕХ запросов батча (в J отказ был бы per-call
   точечным). Это деградация в уже-ванильный режим, а не изменение исходов.
5. **Throw на flush-мердже** (tryToMerge бросил — патология): guardEntityTick-
   семантика (лог severe + discard self) применяется на flush вместо тика
   сущности.

## Что НЕ трогает

items-lane J-композиции (tickOne остаётся инлайном RegionTickOps.tickBucket,
java items/ItemEntity.tick остаётся 0.00%), fluid-lane, broadphase-мобов,
nav_ai, harness inputs (radius 640 / seconds 300 / band 6M-9.5M / population
150000 / seed 42 / region_threads 4 / region_steal 0).
