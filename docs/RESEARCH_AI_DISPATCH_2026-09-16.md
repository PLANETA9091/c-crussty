# RESEARCH — AI-DISPATCH / BRAIN-LENS дизайн + 20 TPS портфель (S7-96b, task165-доп)

> Комплементарная research-линия раунда S7-96 (дубль-агент): LLM+web leg миссии.
> Raw: `research/brainlens-2026-09-17/` (4 LLM-запроса, 2 из них thinking ON, + 6 web-поисков).
> Контекст запросов = верифицированная анатомия двух прогонок (run#11 + run#12), не гипотезы.
> Owner-директивы в контексте: NO FALLBACKS / NO CONFIG-WINS / 20 TPS north star.

## 1. BRAIN-LENS (следующий большой рычаг после GC-SHAPE-1) — вердикт дизайна

Данные: Brain.tick кластер 6.85%/6.28% (run#11/run#12); machinery-leaves ~1.6-1.7%
абсолютного CPU (itable stub 0.45-0.49 мегаморфный диспатч + итераторы LinkedHashMap/HashMap
~1.03 + getNode 0.18-0.23 + sequencedKeySet 0.11); startEachNonRunningBehavior 4.28-4.66%;
alloc кластера 10.2-11.3% тика. Parity-закон: LinkedHashMap insertion order стартов
поведений обязан сохраниться бит-точно.

| вариант | суть | потенциал | вердикт LLM (q1) | сведение с нашей дисциплиной |
|---|---|---|---|---|
| (1) JVM hot-patch `Brain.startEachNonRunningBehavior` | snapshot behaviors в плоский `Object[]` (insertion order) + running-flags bitmap + прямой цикл вместо LinkedHashMap-итерации | ~3.0% CPU | **РЕКОМЕНДОВАН** (лучший баланс выигрыш/риск/сложность) | совпадает с do-not-duplicate: логика поведений остаётся JVM, меняем только bookkeeping; требует parity-тестов порядка стартов (side effects canStart!) |
| (2) Rust-зеркало Brain + batch-JNI | один JNI-вызов на N мобов, Rust держит зеркала behavior-таблиц | теоретически высокий | **ОТКЛОНЁН**: cache-coherence зеркала + цена JNI + сложность синхронизации | согласен: мутации Brain из JVM делают зеркало опасным (это тот же класс ошибок, что per-get lens) |
| (3) alloc-only (Stream/Iterator → примитивы) | только уборка аллокаций кластера | 1-2% | частично; в отрыве от (1) бессмысленно | **вливается в GC-SHAPE-1 (task166)**: verified alloc sites уже включают Brain iterator/views |

Kill-criteria BRAIN-LENS (пререгистрация на будущий STEP-0): заменяемое ядро
(machinery-leaves + часть SELF) < 3% по анатомии → REFUTED pre-code; parity = порядок
стартов + отсутствие двойных canStart вызовов на тик.

## 2. BATCH-RNG — закрытие (независимое подтверждение рефутации)

Две независимые линии сошлись: (a) javap-контракт близнеца (optimiseRandomTick +
advanceSeed LCG, replaceable 1.6-1.9% << 3% gate), (b) наша стек-анатомия run#11/run#12
(RNG-only 1.57%/1.40% FAIL; upper 3.93%/3.13% недостижим из-за per-position upcalls).
LLM (q2) подтвердил арифметику: перенос pick-лупа в Rust с upcall'ами getBlockState
катастрофичен (660k позиций/тик), комбинированный «RNG-драйвер + sectSnapshot» не
проходит совместный гейт (batch-lens уже рефутнут на 3.3% потолке).

Банк на ре-открытие (если когда-нибудь len переоценят): jump-ahead для 48-bit LCG
java.util.Random (M=0x5DEECE66D, A=0xB, mod 2^48) через бинарное возведение
матрицы аффинного шага в степень K — O(log K) умножений по модулю 2^48; SIMD
ускоряет пакет прыжков НЕСКОЛЬКИХ генераторов (4-wide), но не один прыжок.

## 3. Путь к 20 TPS — портфельная математика (q3, red-teamed)

LLM-роадмап (5-6 раундов до ~50ms) ПО ФОРМЕ принят, ПО ЧИСЛАМ переведён на
верифицированные replaceable-ядра: его «Boats -3.5ms» противоречит измеренному
boat-owned 0.76% (~-0.64ms) — классическая путаница presence/owned. Честная
сумма ЗАКРЫТЫХ сегодня соло-рычагов ~5-7% — до цели далеко; нужны новые классы:
- **GC-SHAPE** (task166, пререгистрирован близнецом): verified alloc sites серией;
- **BRAIN-LENS** (этот док, §1): ~3% потолок hot-patch варианта;
- **ENTITY-LENS семейство**: minecarts STEP-0, sensor-стримы, Villager через Brain;
- **REDSTONE-LENS** (STEP-0 первым раундом, ~3.8% лейн);
- **time-slicing / phase parallelism** — новые классы рычагов (q3), требуют свежего
  recon тик-лупа (порядок фаз, где безопасно перекрытие);
- **bench-4 fake-players**: per-player spawning как у игроков — честная база A/B
  для ВСЕХ будущих рычагов (owner-условие; task166+).
Главный риск портфеля (q3): дрейф в микро-оптимизации «простых» лейнов вместо
фундаментальных (chunk, AI) — метрика раннего предупреждения: кумулятивный MSPT
бюджет в GOAL_20TPS_MINESSHIELD3.md, обновлять КАЖДЫЙ раунд.

## 4. MoE-диспатч → mob-AI (q4, owner «рисерч от дипсика»)

Изоморфизмы с вердиктами: (1) precomputed routing table вместо per-tick итерации —
это и есть вариант (1) §1, подтверждён; (2) shared-expert (общие memory-проверки раз
в тик на мир) — интересный STEP-0 вопрос: сколько из canStart предикатов world-invariant
в пределах тика; (3) MTP-спекуляция стартов поведений — ОПАСНО (side effects canStart),
отклонено; (4) paged bitmap behavior-статусов — вариант (1) покрывает; (5) DualPipe
overlap AI/chunk фаз — только после свежего recon тик-лупа, в бэклог.

## 5. Web-обзор (w1-w6, raw в research/brainlens-2026-09-17/)

- Lithium/мена фокусы AI-оптимизаций: sensor-tick batching, behavior-смежные патчи —
  подтверждает, что Brain-механика признана горячей в сообществе.
- ECS/AI-батчинг (Unity DOTS, EnTT): плоские массивы + битмапы состояний — та же
  морфология, что вариант (1) §1.
- JVM мегаморфный диспатч: itable stub уходит в мономорф после стабилизации
  профиля вызовов — hot-patch с прямым циклом снижает биморфность сайта.
- java.util.Random jump-ahead: классика (Дистер-и-др. для LCG skip-ahead) — банк §2.

## 6. Харнесс-баги, найденные этой линией (S7-96b) — починено

- `paper mspt` НЕ существует на Purpur 1.21.10: каждый полл с run#10 отвечал
  Usage-error → MSPT-данных «paper mspt» никогда не было; реальные MSPT шли от
  spark tickmonitor [⚡]. Fix: полл заменён на `paper mobcaps world` (спавн-обсервабилити),
  parse_mspt_windows получил fallback на [⚡]-строки (валидировано offline на run#12:
  Min 57.06 / Max 145.36 / Avg 75.62).
- `paper entity list` требует фильтр+мир: каждый полл с run#10 — Usage-error,
  0 данных по сущностям. Fix: `paper entity list * world`.
- **F4 entity spawn/despawn churn** добавлена в report_world3.py (owner-условие):
  polls, total min..max, churn %, top movers по типам, summon-счётчик, вердикт
  ACTIVE/STAGNANT. На run#10-12 честно скажет STAGNANT (данных нет) — данные
  появятся с run#15 (первый с починенными поллами).

## Файлы

- raw LLM/web: `research/brainlens-2026-09-17/` (INDEX.md, q1-q4, w1-w6)
- репортер: `bench/world3/report_world3.py` (F4 + mspt-fallback)
- харнесс: `bench/world3/run_world3.sh` (mobcaps + entity list * world)
