#!/usr/bin/env python3
"""recon31_volatile_diet_drill.py — RECON-31: дрилл volatile/VarHandle-лейна
(~3.4% сцены, компонент ЭПОХИ-2A) до доменов + вердикт потолка volatile-диеты.

Классификация стеков, содержащих volatile-семью:
  1. chunk-map concurrent (ConcurrentLong2Reference + getChunk* клиенты) —
     часть ИНФРАСТРУКТУРЫ region-threads (банк v3), конкурентность обязательна
  2. SynchedEntityData accessor-чтения (getHealth/getSharedFlag/isNoGravity/
     getAirSupply/getItem/getValue) — visibility обязательна (tracker-поток
     sendChanges) + intra-tick мутации (AI пишет и читает свои же поля)
  3. прочее

Вердикт-модель: volatile-read на x86 = plain load (acquire бесплатен);
стоимость = потеря hoisting/CSE, а hoisting на этих полях ЛОМАЕТ семантику
(tracker-visibility / intra-tick порядок) → честный потолок диеты ~0-1% сцены,
hard ceiling <5% = микро-зона чартера.
"""
import os
import re
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUNS = {
    "s7194": os.path.join(RESDIR, "run-s7194-zeroalloc-v1"),
    "s7189": os.path.join(RESDIR, "run-s7189-traveldiet-v2a"),
}
DOC = os.path.join(RESDIR, "RECON31_VOLATILE_DIET_VERDICT.md")

LANE_RX = re.compile(
    r"VarHandle|getVolatile|VarHandleGuards|getValueVolatile|SynchedEntityData|"
    r"getAcquire|getAtIndexAcquire|getTableAcquire|SectionVolatile|getNextVolatile")

CHUNK_MAP_RX = re.compile(
    r"ConcurrentLong2Reference|getNode|getTableAcquire|getChunkHolder|"
    r"getChunkNow|getChunkAtIfLoadedImmediately|getChunkIfPresentUnchecked|"
    r"ServerChunkCache/getChunk|getSectionVolatile|getRegion;")

SYNC_DATA_RX = re.compile(
    r"SynchedEntityData|getSharedFlag|getHealth|isNoGravity|getAirSupply|"
    r"getSleepingPos|DataItem|packDirty|sendDirtyEntityData")


def classify(stack):
    fr = stack.split(";")
    has_chunk = any(CHUNK_MAP_RX.search(f) for f in fr)
    has_sync = any(SYNC_DATA_RX.search(f) for f in fr)
    # приоритет: что глубже в стеке (последняя позиция)
    pos_chunk = max((i for i, f in enumerate(fr) if CHUNK_MAP_RX.search(f)), default=-1)
    pos_sync = max((i for i, f in enumerate(fr) if SYNC_DATA_RX.search(f)), default=-1)
    if pos_chunk > pos_sync:
        return "chunk-map-concurrent"
    if pos_sync >= 0:
        return "synced-entity-data"
    return "other-volatile"


def sync_site(stack):
    """лист-домен внутри SynchedEntityData-семьи"""
    fr = stack.split(";")
    for f in reversed(fr):
        name = f.split("/")[-1]
        if re.search(r"getSharedFlag|getHealth|isNoGravity|getAirSupply|"
                     r"getSleepingPos|isFallFlying|isCrouching|isSprinting|"
                     r"hasImpulse|getItem|getValue|getValueVolatile|packDirty|"
                     r"sendDirtyEntityData|isPassenger|isOnFire|isinvisible", name, re.I):
            return name[:45]
    return "?"


def main():
    lines = ["# RECON-31 — дрилл volatile-лейна (NEXT 354) + вердикт volatile-диеты", ""]
    res = {}
    for name, rd in RUNS.items():
        path = os.path.join(rd, "cpu-collapsed.txt")
        tot = 0
        lane = Counter()
        sync_sites = Counter()
        for line in open(path, errors="ignore"):
            s, _, c = line.rstrip("\n").rpartition(" ")
            if not s:
                continue
            try:
                n = int(c)
            except ValueError:
                continue
            stack = s.replace(".", "/")
            tot += n
            if not LANE_RX.search(stack):
                continue
            k = classify(stack)
            lane[k] += n
            if k == "synced-entity-data":
                sync_sites[sync_site(stack)] += n
        res[name] = (tot, lane, sync_sites)
        lines.append(f"## {name}: lane-итоги (сцена {tot})")
        lines.append("")
        lines.append("| домен | сэмплов | % сцены |")
        lines.append("|---|---|---|")
        for k in ("chunk-map-concurrent", "synced-entity-data", "other-volatile"):
            v = lane.get(k, 0)
            lines.append(f"| {k} | {v} | {100.0 * v / tot:.2f}% |")
        lines.append("")
        lines.append("### SynchedEntityData под-сайты")
        lines.append("")
        for k, v in sync_sites.most_common(10):
            lines.append(f"- {100.0 * v / tot:5.2f}% {v:6d} {k}")
        lines.append("")

    # вердикт
    lines.append("""## ВЕРДИКТ (модель потолка volatile-диеты)

1. Лейн 3.51/3.40% сцены распадается на две семьи (кросс-раннер стабильно):
   - synced-entity-data 1.94/2.00% сцены: доступ к SynchedEntityData
     (getItem/get/getValue — javap-контракт: itemsById[accessor.id()] AALOAD
     → DataItem.value GETFIELD — ПУТЬ ПОЛНОСТЬЮ PLAIN, НИ ОДНОЙ volatile-
     инструкции; волатильность у DataItem отсутствует и в поле value);
     sub-сайты: generic getValue 1.06/1.08%, getItem 0.78/0.85%,
     packDirty/sendDirtyEntityData ~0.08% — tracker-семейство;
   - chunk-map-concurrent 1.52/1.34% сцены: ConcurrentLong2ReferenceChainedHashTable
     ($TableEntry.getValueVolatile guard_L_L + getAtIndexAcquire guard_LI_L)
     — конкурентная chunk-карта ServerChunkCache (getChunk/getChunkNow/
     getChunkHolder/getChunkAtIfLoadedImmediately) — часть ИНФРАСТРУКТУРЫ
     region-threads (компонент БАНКА v3): регион-воркеры читают chunk-карту
     конкурентно, acquire/volatile-семантика обязательна; диета = атака на
     собственный банк;
   - other-volatile 0.05% — шум.
2. ВЫВОД-КОРРЕКТИРОВКА: «volatile-диета» на этой сцене НЕ ИМЕЕТ ПРЕДМЕТА:
   (a) sync-семья уже plain — убирать нечего, любая выгода = только
   редизайн двухуровневой косвенности itemsById→DataItem.value (структурный
   редизайн SynchedEntityData ради ≤2% = микро-зона, запрещён);
   (b) chunk-семья конкурентна по необходимости (банк v3) — снятие
   acquire/volatile ломает thread-safety чтения чанков воркерами.
   Честный потолок диеты ≈ 0-1% сцены даже при частичной легальности.
3. Hard ceiling лейна 3.4-3.5% < ДВОЙНОГО БАРА +10%; обе семьи залочены
   (plain-по-конструкции / конкурентность-по-необходимости) →
   **volatile-диета = 6-я ДОК-ВЕРИФИКАЦИЯ закрытия лейна (прецедент
   RECON-17/20/23/26/30), патчер не реализуется**.
4. Для эскалации: компонент A «volatile-диета ~3.4%» реально стоит ~0-1%
   → потенциал ЭПОХИ-2 пересчитан: 10-13% → **~9-12% сцены** (верхняя
   граница, TPS-конверсия по эмпирике ~0). Рекомендация B/C усилена.
""")
    with open(DOC, "w") as f:
        f.write("\n".join(lines))
    for name, (tot, lane, sync_sites) in res.items():
        print(f"== {name} tot={tot}")
        for k in ("chunk-map-concurrent", "synced-entity-data", "other-volatile"):
            v = lane.get(k, 0)
            print(f"   {k:24s} {v:6d} {100.0 * v / tot:5.2f}%")
        for k, v in sync_sites.most_common(5):
            print(f"     sync-site {100.0 * v / tot:5.2f}% {k}")


if __name__ == "__main__":
    main()
