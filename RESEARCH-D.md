# TASK-400-D wakefix — root-cause r3i pop=INVALID (evidence-based)

## Симптом
run 35557524097 (round-399-i-wakeup, cmp399_wakeup, leg2): boot чистый, но
BENCH-4 fixture pop=INVALID. В server-stdout.log: массовые
`item spawn failed ... java.lang.ArrayIndexOutOfBoundsException: Index -1
out of bounds for length 16385` + те же на mob spawn — во время INJECT
(03:36:25, plan items=105000 + hostiles=30000 + passives=15000) → население
не достигло 150000 → fixture-gate FAIL.

## Root-cause (установлен этим тиком)
[crussty-plugin] items_subsys2: bridge ready (enabled=false) — при этом спавн
падает. Дифф ветки round-399-i-wakeup vs round-398-j-subsys2:
src/items_manager.rs lever_flag_matches() расширен до `starts_with("cmp399_")`
(контракт композиции раунда-3), НО java-гейт ENABLED не CP-патчится для
cmp399_wakeup → РАЗНОСЛОЙНОЕ полу-вооружение: rust определяет
ItemEntityManager + вооружает kernel-policy retargets, java остаётся
vanilla → каждый addEntity проходит через полу-инициализированный мост →
AIOOBE(-1,16385) → pop=INVALID. Урок: двойные гейты (rust+java) обязаны
переключаться синхронно; частичное расширение только rust-слоя = поломка
spawn-пути (не wakeup-логика!).

## Фикс (round-400-d-wakefix)
lever_flag_matches() возвращён к eq("items_subsys2") — cmp399_wakeup
вооружает ТОЛЬКО wakeup (Mob serverAiStep 2+2 retarget через AiWakeupOps,
passive-poll армирование 4140600 сохранено). Item-путь = чистая ваниль,
идентичная якорным ногам. Композит J+wakeup требует синхронного расширения
java CP-патча (отложено, см. LEVER-D.md).

## Upstream-прецеденты
Passive sighting poll: brainhook.rs (S7-114). Dual-gate дисциплина:
FluidBitmaskOps rust/java симметричные гейты. wakeup-семантика: Paper
«tick-suppression без изменения видимости сущностей».
