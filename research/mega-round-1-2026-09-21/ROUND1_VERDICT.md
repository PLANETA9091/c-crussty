# MEGA-ROUND-1 VERDICT (TASK-396, 2026-09-21, world-bench-parallel era)

ТОП-1 ботлнек: items/ItemEntity.tick 31.17% java (36051/115655, банк v4 fp=4,
BASELINE_LANES в absorb_s7207.py) + item-driven broadphase 15.66%.
Формат: 10 архитектурных векторов (A..J) на ItemEntity-мерж-механику, каждый —
своя ветка round-396-X с флаг-гейтом CRUSSTY_LEVER_FLAG, CI-бенч
world-bench-parallel.yml (банк v4: inside_cache=1 + flush_diet=1 +
region_threads=4 + batch_collector=1 + fluid_guard=1 + gc_tune=3, fp=4,
150k/seed42, 300s, band 6.0-9.5M), паритет-харнессы до диспатча.

## КАРТА ВЕКТОРОВ (Δ = median5 vs TPS_exp-интерполяция банка на своём runner)

| Вектор | Механизм | Ветка | Δ | ARMED | Вердикт |
|---|---|---|---|---|---|
| A items_index | 1.0-grid хеш-индекс соседей вместо getEntities (strict 2-site) | round-396-a-items_index | **+2.9%** | да | MEASURED <bar> |
| B items_stagger | фазовое расписание merge-сканов N=4 (3-site) | round-396-b-items_stagger | — | fail-closed dormant | compose strict-1 violated на gate-2 (move = invokespecial, не virtual); ре-роллы 2/2 исчерпаны |
| C items_sweep | sweep-line батч-мерж per bucket (co-defined ops) | round-396-c-items_sweep | −3.1% (runner 9.92M ВНЕ полосы — нога невалидна) | да | BAND-OUT, отрицательный |
| D items_soa | SoA-массивы горячих полей | — | — | — | ДРОП (недоимплементирован, скудный прогресс) |
| E items_offthread | off-thread merge-поиск на worker-ах | — | — | — | ДРОП (недоимплементирован) |
| F items_mono | девиртуализация megamorphic Entity.tick (type-test сплит) | round-396-f-items_mono | **+10.1%** leg1 / **+0.2%** leg2 | да/да | min-of-2 = +0.2% — НЕ реплицировался |
| G items_footprint | cache-line hot-объект (putfield-ретаргеты) | round-396-g-items_footprint | — | — | ДРОП (многосайтовый putfield-ретаргет без локального cargo — риск) |
| H items_oss | порт Lithium item_entity_merging (QUERY_LIMIT=64 + dry-run) | round-396-h-items_oss | **+6.9%** | да (single leg) | MEASURED, лучший подтверждённый |
| I items_wakeup | event-driven wakeup-лист (E1/E2/E3/E4) | round-396-i-items_wakeup | **+4.0%** | да | MEASURED, items-лейн −2.56пп |
| J items_manager | полная замена подсистемы ItemEntityManager | round-395-j-manager (параллельная сессия) | **+3.3%**, items-лейн 31.17%→0.00% | да (telemetry) | MEASURED: лейн снят ПОЛНОСТЬЮ, но конверсия в TPS +3.3% — work не на крит-пути тика |

## ВЕРДИКТ
МЕГА-РАУНД-1 = **КАРТА ВЕКТОРОВ, БЕЗ БАНКИНГА**: ни один вектор не дал
реплицированного ≥+10% (двойной бар) — лучший ARMED-лег F +10.1% на второй
ноге +0.2% (JIT-диспетч-выигрыш зависит от раннера/профиля), H +6.9% одна нога.
Master НЕ мержится (закон двойного бара; ложный мега-буст = худший исход).
items-лейн реально снимается (J: до 0.00%), но конверсия работы в TPS на банке
v4 ограничена: работа items в основном на worker-ах (W=4), крит-путь тика
сжимается меньше, чем доля java-сэмплов.

## БЫЛО → СТАЛО (банк v4 якорь 2.6@8551924 / 2.2@6653417; vanilla 0.6-0.8)
- TPS: банк-интерполяция на runner'е лега → лучшее ARMED-измерение:
  H 2.25→2.40 (+6.9%); F 2.18→2.40 (+10.1%) но повтор 2.29→2.30 (+0.2%);
  I 2.60→2.70 (+4.0%); J 2.23→2.30 (+3.3%); A 2.24→2.30 (+2.9%).
- CPU: items-лейн 31.17% → H 30.64 / I 28.61 / C 29.61 / J 0.00 (полная замена);
  broadphase 15.66% → C 14.55 / остальные флэт.
- ОЗУ/GC (ParallelGC, справка банка 18.8s/Full=7): все ARMED-леги
  20.0-24.9s total, Full=9 — без деградации heap-политики; отдельной RAM-цены
  рычаги не дали (J/manager — лёгкий оверхед собственного индекса).

## ИНФРА-УРОКИ ТИКА (§5-реестр)
1. 10 параллельных Task-агентов = context deadlineceeded у тул-инфра —
   выжившие воркфлоу-артефакты надо собирать, а не перезапускать слепо.
2. world-bench-parallel артефакт НЕ содержит world3-bench.log — TPS-поллы в
   server-stdout.log (абсорб-фоллбэк).
3. ItemEntity.tick зовёт Entity.move через **invokespecial** (super-call) —
   retarget_virtual_to_static ищет только invokevirtual → строгий census
   требует учёта invoke-опкода.
4. Классы bridges в root-crate: jni-типы = jvmti_bindings::jni::* (ре-экспорт),
   отдельной jni-зависимости нет.
5. Санбокс-фласки: JDK/fastutil/guava/joml/gson/dfu/paper-api пересобраны в
   /home/z/tools (javac21 путь в промптах устарел — tools/ не в гите).

## NEXT (TASK-397)
1. min-of-2 подтверждение H (+6.9%) и F на 3-й ноге — или списать F.
2. КОМПОЗИЦИЯ A+I (индекс решает ГДЕ искать, wakeup — КОГДА; архитектурно
   ортогональны, оба ARMED-проверены) на одной ветке = кандидат на ≥+10%.
3. B-фикс тривиален: gate-2 снять или invokespecial-паттерн добавить.
