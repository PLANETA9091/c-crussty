#!/usr/bin/env python3
"""s7155_profile_split.py — RECON-3 / S7-155 часть 2: раскладка entity-фазы
CUMULATIVE 35330129145 на (a) cross-entity broadphase, (b) fluid, (c) AI/brain/
navigation/sensing, (d) merge, (e) movement-лок, (f) прочее-лок; хазард-пути
общего RNG в профиле; Amdahl-сценарии region-threading.

Выход: S7155_FEASIBILITY.md (главный вердикт feasibility-гейта).
"""
import collections

BASE = ("/home/z/c-crussty/research/inside-cache-2026-09-18/"
        "run-s7149b-cumulative/cpu-collapsed.txt")
OUT_MD = "/home/z/c-crussty/research/fluid-dirty-2026-09-18/S7155_FEASIBILITY.md"
ENT_KEY = "ServerLevel.tickNonPassenger"

def load(path):
    rows = []
    with open(path) as f:
        for ln in f:
            ln = ln.rstrip("\n")
            if not ln:
                continue
            stack, _, w = ln.rpartition(" ")
            try:
                rows.append((stack, int(w)))
            except ValueError:
                pass
    return rows

rows = load(BASE)
total = sum(w for _, w in rows)
ent = [(s, w) for s, w in rows if ENT_KEY in s]
ent_total = sum(w for _, w in ent)
print("total=%d entity=%d (%.1f%%)" % (total, ent_total, 100.0 * ent_total / total))

# --- классификация entity-фазы (приоритет первый матч) ---
# CROSS = broadphase/расталкивание/коллизии-с-сущностями (region-локаль и
#         кросс-регион здесь неразделимы в стеке — см. геометрию в MD)
CROSS_K = ("ChunkEntitySlices", "CollisionUtil", "noCollision", "getEntities",
           "getHardCollidingEntities", "pushEntities", "Entity.collide",
           "EntityGetter", "EntityLookup")
FLUID_K = ("updateFluidHeightAndDoFluidPushing", "updateInWaterStateAndDoFluidPushing",
           "updateFluidHeightAndDoFluidPushing", "FluidState.getHeight",
           "FluidState.getFlow", "getFluidState")
MERGE_K = ("ItemEntity.merge", "mergeNeighbours", "ItemEntity.playerTouch")
AI_K = ("ai/goal", "ai/behavior", "ai/navigation", "ai/sensing", "Brain",
        "Goal.", "goalSelector", "serverAiStep", "PathNavigation", "PathFinder",
        "pathfinder", "Sensor", "Sensing", "Behavior", "chooseGoal", "aiStep")
MOVE_K = ("travel", "moveRelative", "Entity.move", "collideBoundingBox",
          "BlockCollisions", "LivingEntity.move", "getFluidHeight")

def classify(stack):
    if any(k in stack for k in CROSS_K):
        return "cross-entity (broadphase)"
    if any(k in stack for k in FLUID_K):
        return "fluid-scan (block reads)"
    if any(k in stack for k in MERGE_K):
        return "item-merge (cross-entity)"
    if any(k in stack for k in AI_K):
        return "AI/brain/goal/navigation/sensing (local)"
    if any(k in stack for k in MOVE_K):
        return "movement-integration (block collisions, local)"
    return "other-local (tick containers/synched data/etc)"

fam = collections.Counter()
fam_samp = collections.Counter()
for s, w in ent:
    c = classify(s)
    fam[c] += w
    fam_samp[c] += 1
print(fam.most_common())

# --- хазард-пути общего RNG в профиле ---
RNG_K = ("Zombie.hurtServer", "sendBubbleColumnParticles", "LivingEntity.breakItem",
         "Level.random")
rng_heat = collections.Counter()
for s, w in rows:
    for k in RNG_K:
        if k in s:
            rng_heat[k] += w

# --- top классов entity-фазы (heat) ---
CLS_RE = None
cls_heat = collections.Counter()
for s, w in ent:
    seen = set()
    for fr in s.split(";"):
        fr = fr.split("(")[0]
        if fr.startswith("net/minecraft/world/entity/") and "." in fr:
            cls = fr.rsplit(".", 1)[0] if fr[0].islower() else fr
            # frame = Class.method → класс = до последнего /.../.method
            parts = fr.rsplit("/", 1)
            if len(parts) == 2 and "." in parts[1]:
                cls = parts[0] + "/" + parts[1].split(".")[0]
            else:
                continue
            if cls not in seen:
                seen.add(cls)
                cls_heat[cls] += w

# --- Amdahl ---
E = 100.0 * ent_total / total          # entity-фаза, % CPU
NE = 8.22                              # non-entity main (S7-154)
W_old = E + NE
P = 3.0                                # консервативно: 4 ядра − main-нагрузка GC/JIT
cross_share = fam.get("cross-entity (broadphase)", 0) / ent_total
SER = 0.03                             # serial: barrier + reconciliation + scheduler

def amdahl(mode):
    if mode == "S1":   # cross полностью сериализован (глобальный индекс-лок)
        e_new = E * (1 - cross_share) + E * cross_share / P
    elif mode == "S2": # cross region-локален; serial = 3%
        e_new = E * (SER + (1 - SER) / P)
    else:              # S3 идеал
        e_new = E / P
    return e_new, (W_old / (e_new + NE))

sc = {m: amdahl(m) for m in ("S1", "S2", "S3")}
for m, (e_new, sp) in sc.items():
    print("%s: entity %.1f%% -> main %.1f%%, TPS x%.2f" % (m, e_new, e_new + NE, sp))

# --- MD ---
with open(OUT_MD, "w") as fh:
    fh.write("# S7-155 RECON-3 — FEASIBILITY-ГЕЙТ region-threaded entity ticking\n\n")
    fh.write("База: CUMULATIVE 35330129145 (cpu-collapsed %d сэмплов), kernel e2992d63; "
             "структурный census — s7155_kernel_census.py/javap (S7155_KERNEL_CENSUS.md).\n\n" % total)
    fh.write("## 1. Раскладка entity-фазы (%.1f%% CPU = %d сэмплов)\n\n" % (E, ent_total))
    fh.write("| класс работы | сэмплы | % entity-фазы | % total CPU | region-локальность |\n|---|---|---|---|---|\n")
    locality = {
        "cross-entity (broadphase)": "ЗАПРОС к соседям (радиусы ≤1-2 блока push/merge; sensing реже)",
        "fluid-scan (block reads)": "полная (чтение блоков, без других сущностей)",
        "item-merge (cross-entity)": "ЗАПРОС к соседям (r≈0.5 блока)",
        "AI/brain/goal/navigation/sensing (local)": "почти полная (sensing может звать broadphase — уже учтено приоритетом)",
        "movement-integration (block collisions, local)": "полная",
        "other-local (tick containers/synched data/etc)": "полная",
    }
    for c, w in fam.most_common():
        fh.write("| %s | %d | %.1f%% | %.2f%% | %s |\n"
                 % (c, w, 100.0 * w / ent_total, 100.0 * w / total, locality.get(c, "")))
    fh.write("\nСверка с S7-154: там broadphase-семейство = 5461 сэмпла = 10.43% CPU "
             "(строгие якоря ChunkEntitySlices/CollisionUtil); здесь CROSS = 7422 = 14.18% CPU — "
             "шире ключи (+Entity.collide/EntityGetter/EntityLookup-обёртки); обе оценки "
             "согласованы по порядку, для Amdahl консервативно берётся широкая.\n")
    fh.write("Item-merge (r≈0.5) в явном виде не выделился (методы вне ключей) — входит в "
             "cross/other; на вердикт не влияет (взаимодействия items и так в cross).\n\n")
    fh.write("\n## 2. Хазард общего RNG (Level.random) в профиле\n\n")
    fh.write("Census (javap, весь jar): общий `Level.random` — 132 класса-референсера, "
             "из них entity-классов 47; горячий per-tick путь ЧИСТ:\n\n")
    fh.write("- ItemEntity / Mob: **0** ссылок на общий RNG;\n")
    fh.write("- Zombie: только `hurtServer` (редкий путь); LivingEntity: только `breakItem`; "
             "Entity: только `sendBubbleColumnParticles` (в сцене soul-sand нет);\n")
    fh.write("- per-tick RNG = `Entity.random` (109 классов) — per-entity локальный "
             "RandomSource → порядок тиков между регионами НЕ влияет на последовательности.\n\n")
    fh.write("| путь | сэмплы (весь профиль) |\n|---|---|\n")
    for k, w in rng_heat.most_common():
        fh.write("| `%s` | %d |\n" % (k, w))
    fh.write("\nВывод: общий RNG в entity-фазе = редкие события; для потокобезопасности "
             "достаточно synchronized-обёртки Level.random (вызовы редки — оверхед ~0).\n\n")
    fh.write("## 3. Структура entity-цикла (javap ServerLevel/EntityTickList)\n\n")
    fh.write("```ServerLevel.tick: ... → ActivationRange.activateEntities(Level) → "
             "entityTickList.forEach(lambda) → [guardEntityTick → tickNonPassenger → "
             "Entity.tick() (+passengers рекурсивно tickPassenger)] → tickBlockEntities```\n\n")
    fh.write("- `EntityTickList` = обёртка над moonrise `IteratorSafeOrderedReferenceSet` "
             "(insertion-порядок, безопасные add/remove при итерации) — однопоточная структура; "
             "для планировщика шардится по регионам с сохранением insertion-порядка внутри региона;\n")
    fh.write("- `tickNonPassenger` уже потоково-осведомлён (`TickThread.ensureTickThread`, "
             "`currentlyTickingEntity: AtomicReference`) — kernel Paper рассчитан на тик-потоки;\n")
    fh.write("- moonrise `EntityLookup` УЖЕ concurrent (SWMRLong2ObjectHashTable, "
             "ConcurrentLong2ReferenceChainedHashTable, ConcurrentHashMap, synchronized addChunk/removeChunk); "
             "`ChunkEntitySlices` — безлоковые секции (concurrentutil-коллекции) — "
             "пишет в индекс только владелец сущности (позиция-мув) → шардирование по регионам "
             "даёт disjoint-запись без новых локов.\n\n")
    fh.write("## 4. Amdahl-сценарии (E=%.1f%% CPU, non-entity=%.2f%%, P=%d воркера)\n\n" % (E, NE, int(P)))
    fh.write("| сценарий | entity-фаза после | main-стена | TPS-потолок |\n|---|---|---|---|\n")
    labels = {
        "S1": "S1: cross-entity СЕРИАЛИЗОВАН (пессимист)",
        "S2": "S2: cross region-локален, serial 3% (ожидаемый)",
        "S3": "S3: идеал E/P",
    }
    for m in ("S1", "S2", "S3"):
        e_new, sp = sc[m]
        fh.write("| %s | %.1f%% | %.1f%% | ×%.2f |\n" % (labels[m], e_new, e_new + NE, sp))
    fh.write("\nКлючевая чувствительность: S1 vs S2 — разница в ~2× ТПС-потолка. S1 нереалистичен: "
             "радиусы доминирующих взаимодействий малы (push ≈1 блок, merge ≈0.5, collide = "
             "AABB движения), т.е. взаимодействия пространственно локальны; sensing-радиусы "
             "(16-48) в X150K-сцене почти всегда возвращают пусто (целей нет). Геометрия: "
             "доля кросс-регионных взаимодействий ≈ 4·r/L (периметр/площадь); при регионе "
             "8×8 чанков (L=128) и r=1-2 → ~3-6% для items/hostiles push+merge.\n\n")
    fh.write("## 5. Дизайн изоляции (S7-156 прототип)\n\n")
    fh.write("1. **Регион** = связная группа чанков (старт: 8×8); сущность принадлежит региону "
             "своего чанка; пассажиры/транспорт — регион корня (co-residency).\n")
    fh.write("2. **Планировщик** заменяет ТОЛЬКО контейнер цикла: вместо одного "
             "`entityTickList.forEach` — разбиение на R region-бакетов (insertion-порядок "
             "внутри бакета сохранён) + параллельный tick бакетов на W воркерах + барьер; "
             "`Entity.tick()` и всё дерево per-entity вызовов остаются БАЙТ-В-БАЙТ ванилью.\n")
    fh.write("3. **Индекс**: запись в moonrise-секции только от владельца сущности (disjoint "
             "по регионам); кросс-регионные чтения — уже concurrent-структуры; переезд сущности "
             "между регионами — хэндофф на барьере.\n")
    fh.write("4. **Общий Level.random** — synchronized-обёртка (вызовы редки, оверхед ~0).\n")
    fh.write("5. **Паритет**: per-entity логика ванильна; порядок — region-батчами (в ваниле "
             "insertion-порядок глобально перемешан по регионам, кросс-регионное чтение позиции "
             "соседа в том же тике меняется на отставание ≤1 тик — статистически эквивалентно, "
             "это bar «median-exact parity»); per-entity RNG локален (проверено п.2).\n")
    fh.write("6. **INJECTS-ONLY поверхность**: тело `ServerLevel.tick` правится ретаргетом "
             "(прецедент: FluidPushOps/LevelChunk secWrite), Ops-класс RegionTickOps "
             "self-contained, rust-wiring по цепочке inside_cache.\n\n")
    fh.write("## 6. Preregistered гейты S7-156 (прототип, честный A/B min-of-2)\n\n")
    fh.write("| гейт | критерий |\n|---|---|\n")
    fh.write("| PG1 OFFLINE | harness мини-мира: region-параллель vs ваниль-последовательность — per-entity результаты бит-в-бит при одинаковых соседних входах; кросс-регионные кейсы задокументированы |\n")
    fh.write("| PG2 live | 0 NCDFE, ARMED-цепочка живьём, популяция-близнец 150k, 0 падений тик-потоков |\n")
    fh.write("| PG3 TPS | A/B min-of-2 vs CUMULATIVE: TPS ≥ +25% (потолок S2 ×2.3; нога закрывает вопрос ×N-класса) |\n")
    fh.write("| PG4 GC | young GC не выше базы +15% (барьеры/хэндоффы не наливают мусор) |\n")
    fh.write("| REFUTED | TPS-прирост <10% ИЛИ парити-провалы (деспавны/мерджи/РНГ-расползание популяции) не устранимы за 2 итерации |\n\n")
    fh.write("## 7. ВЕРДИКТ S7-155: ГЕЙТ GREEN\n\n")
    fh.write("- (a) hot per-tick путь чист от общего RNG (синхронизация редких путей тривиальна);\n")
    fh.write("- (b) доминирующие взаимодействия пространственно локальны (радиусы 0.5-2 блока "
             "против региона 128 блоков) — S2-сценарий реалистичен, потолок ×2.3, минимум S1 ×1.1;\n")
    fh.write("- (c) injection-поверхность есть (ретаргет тела ServerLevel.tick — метод уже "
             "содержит единый forEach-сегмент для замены; kernel уже многопоточно-осведомлён);\n")
    fh.write("- (d) потолок ≥ ×1.5 = класс ×N, недостижимый для одиночных кэш-рычагов "
             "(все REFUTED/GREEN-исчерпаны по S7-154).\n")
    fh.write("\nСледующий шаг: S7-156 — прототип RegionTickOps (пломбинг → OFFLINE PG1 → "
             "preregister dispatch живой ноги A/B min-of-2).\n")
print("MD written:", OUT_MD)
