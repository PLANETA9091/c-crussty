#!/usr/bin/env python3
"""recon35_chunkread_consumer_drill.py — RECON-35: дрилл block-read
(chunk-read) семьи (клейм ~3.6% из RECON-29; последний несведённый
компонент ЭПОХИ-2A) до ДОМЕНОВ-ПОТРЕБИТЕЛЕЙ + вердикт существования
самостоятельного рычага.

Методика: deepest-block-read-frame-wins (PalettedContainer.*/readPalette*/
LevelChunk.getBlockState*/getFluidState/ChunkAccess-чтение), затем
атрибуция СТЕКА ближайшему сверху домену-потребителю:
  fluid-sim          — getFlow/hasSameAbove/updateFluidOnEyes/tryAddFrost/
                       FlowingFluid/FluidState-контекст
  inside-discovery   — checkInsideBlocks/InsideBlockOps/
                       applyEffectsFromBlocks/forEachBlockIntersected
  movement-collision — move/travel/getBlockSpeedFactor/getOnPos/
                       getFloorLevel/collide/moveSuper
  pathfinding        — getPathTypeFromState/navigation/path
  chunk-lookup       — getChunk/getBlockStateIfLoaded (системные lookup)
  entity-context     — baseTick/aiStep/tick( хвосты без спец. домена
  other
Вердикт-модель: семья = инфраструктура потребителей; самостоятельный
рычаг существует только если домен-потребитель ≥5% сцены И не является
уже отображённым лейном (fluid=REFUTED-зона #10, inside=dormant #15,
movement=REFUTED #14).
"""
import os
import re
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUNS = {
    "s7194": os.path.join(RESDIR, "run-s7194-zeroalloc-v1"),
    "s7189": os.path.join(RESDIR, "run-s7189-traveldiet-v2a"),
}
DOC = os.path.join(RESDIR, "RECON35_BLOCKREAD_CONSUMER_VERDICT.md")
TMP = "/tmp/recon35"

def ensure_collapsed(tag, rd):
    """cpu-collapsed.txt: из run-dir, иначе авто-распаковка из world3-bench.zip"""
    p = os.path.join(rd, "cpu-collapsed.txt")
    if os.path.exists(p):
        return p
    tdir = os.path.join(TMP, tag)
    p = os.path.join(tdir, "cpu-collapsed.txt")
    if not os.path.exists(p):
        os.makedirs(tdir, exist_ok=True)
        z = os.path.join(rd, "world3-bench.zip")
        rc = os.system(f'unzip -o -q "{z}" cpu-collapsed.txt -d "{tdir}"')
        if rc != 0 or not os.path.exists(p):
            raise SystemExit(f"не удалось распаковать {z}")
    return p

# сам блок-read кадр (лист семьи). NB: стеки предварительно dot->slash,
# разделитель класс/метод = '/'. BlockStateBase.getFluidState исключён
# (consumer-side геттер свойства, не chunk-read).
READ_RX = re.compile(
    r"PalettedContainer|readPalette|ChunkAccess[/]"
    r"|LevelChunk[/]get(BlockState|FluidState)"
    r"|Level[/]get(BlockState|FluidState)"
    r"|PathNavigationRegion[/]get(BlockState|FluidState)"
    r"|LevelChunkSection[/]getBlockState"
    r"|StarLightEngine[/]getBlockState|CraftBlockStates[/]getBlockState"
    r"|getBlockStateFinal|getBlockStateOnLegacy|getBlockStateIfLoaded")

DOM = [
    ("fluid-sim", re.compile(
        r"getFlow|hasSameAbove|updateFluidOnEyes|tryAddFrost|[Ff]luid")),
    ("inside-discovery", re.compile(
        r"checkInsideBlocks|InsideBlock|applyEffectsFromBlocks|"
        r"forEachBlockIntersected")),
    ("movement-collision", re.compile(
        r"move|travel|collid|getBlockSpeedFactor|getOnPos|getFloorLevel|"
        r"isFree|[Ll]adder|Climb|[Ff]all")),
    ("pathfinding", re.compile(
        r"Navigation|PathFinder|NodeEvaluator|getPathType|pathfind")),
    ("chunk-lookup", re.compile(
        r"getChunk|hasChunk|isLoaded")),
    ("entity-context", re.compile(
        r"^tick$|entityTick|aiStep|baseTick|tickBucket")),
]

def domain_of(fr):
    """первый спец-домен снизу вверх (после самого read-кадра)"""
    for f in reversed(fr):
        if READ_RX.search(f):
            continue
        n = f.split("/")[-1]
        for name, rx in DOM:
            if rx.search(n):
                return name
    return "other"

def main():
    lines = ["# RECON-35 — дрилл block-read семьи до доменов-потребителей "
             "+ вердикт chunk-read-диеты", ""]
    summary = {}
    for name, rd in RUNS.items():
        path = ensure_collapsed(name, rd)
        if not os.path.exists(path):
            raise SystemExit(f"нет {path}")
        tot = 0
        dom = Counter()
        leaves = Counter()
        callers = Counter()
        for line in open(path, errors="ignore"):
            s, _, c = line.rstrip("\n").rpartition(" ")
            if not s:
                continue
            try:
                n = int(c)
            except ValueError:
                continue
            st = s.replace(".", "/")
            fr = st.split(";")
            tot += n
            idx = [i for i, f2 in enumerate(fr) if READ_RX.search(f2)]
            if not idx:
                continue
            i = idx[-1]
            dom[domain_of(fr[:i + 1])] += n
            leaves[fr[i].split("/")[-1][:56]] += n
            # ближайший потребующий кадр над read-кадром (полное имя)
            for j in range(i - 1, -1, -1):
                if not READ_RX.search(fr[j]):
                    callers[fr[j].split("/")[-1][:64]] += n
                    break
        summary[name] = (tot, dom)
        lines.append(f"## {name} (сцена {tot} сэмплов)")
        lines.append("")
        lines.append("### домены-потребители block-read семьи")
        lines.append("")
        fam = sum(dom.values())
        lines.append(f"семья целиком: {fam} = {100.0 * fam / tot:.2f}% сцены")
        lines.append("")
        lines.append("| домен | сэмплов | % семьи | % сцены |")
        lines.append("|---|---|---|---|")
        for k, v in dom.most_common():
            lines.append(f"| {k} | {v} | {100.0 * v / fam:.1f}% | {100.0 * v / tot:.2f}% |")
        lines.append("")
        lines.append("### read-листья (топ-8)")
        lines.append("")
        for k, v in leaves.most_common(8):
            lines.append(f"- {100.0 * v / fam:5.2f}% семьи {v:6d} {k}")
        lines.append("")
        lines.append("### ближайшие вызывающие кадры (топ-10, полные имена)")
        lines.append("")
        for k, v in callers.most_common(10):
            lines.append(f"- {100.0 * v / fam:5.2f}% семьи {v:6d} {k}")
        lines.append("")

    lines.append("""## ВЕРДИКТ (клейм RECON-29 ~3.6% ЗАНИЖЕН ~2×; самостоятельного рычага НЕТ)

1. Block-read семья (PalettedContainer/* + Level[/]Chunk[/]
   getBlockState*/getFluidState + ChunkAccess/* + readPalette*,
   deepest-wins, полный кадр) = **7.36..7.54% сцены** — клейм RECON-29
   «chunk-read ~3.6%» занижен ~2×: узкие регексы пропускали
   Level-делегаторы (getBlockState/getFluidState на Level),
   внутренние кадры PalettedContainer/ChunkAccess (get/getSections/
   getMinY/getNonEmptyBlockCount) и getBlockStateFinal.
2. Семья = ЧИСТАЯ ИНФРАСТРУКТУРА потребителей (кросс-стабильно
   s7194/s7189): **fluid-sim ~50% семьи = 3.61..3.77% сцены**
   (updateFluidHeightAndDoFluidPushing 16.8% + getFlow 15.3% +
   hasSameAbove 13.6% + updateFluidOnEyes 2.5% — REFUTED-зона #10,
   ванильная семантика растекания/плавания, parity-критично);
   **movement/collision ~22% = 1.63..1.65%** (getCollisionsForBlocksOr
   WorldBorder 9.1% + move/travel/getBlockSpeedFactor — лейн #14 REFUTED);
   **entity-context ~17% = 1.24..1.25%**; **inside-discovery ~10% =
   0.77%** (уже покрыт dormant-флагманом #15 CRUSSTY_INSIDE_BITMASK,
   option B); **pathfinding ~1.4% = 0.10..0.11%**. НИ ОДИН потребитель
   ≥5% сцены вне уже отображённых/закрытых лейнов.
3. Чтение как таковое (листья): PalettedContainer.get ~3.49..3.47% сцены
   (4410..4464), getFluidState ~1.54..1.39%, getBlockState ~0.99..1.00%,
   readPalette ~0.63..0.61%, getBlockStateFinal ~0.53% (final-state кэш
   уже в ваниле), getSections/getMinY <0.3% (section-lookup инфра).
   Мемоизация чтений = movement-invalidation-аналог (RECON-30): потолок =
   полная ликвидация семьи <7.6% сцены < ДВОЙНОГО БАРА +10%; по эмпирике
   6 REFUTED-ног TPS-конверсия диет-семейства ~0 → микро-зона чартера.
4. ВЕРДИКТ: chunk-read-диета НЕ БАНКИНГУЕТСЯ — 9-е ДОК-ЗАКРЫТИЕ
   (RECON-17/20/23/26/30/31/32/33/35); последний несведённый компонент
   ЭПОХИ-2A сведён: вариант A исчерпан полностью, рекомендация владельцу
   B/C из RECON-29×32 усилена. При option B флагман #15 уже покрывает
   внутри-discovery-чтения (0.77% сцены + ~9.1% CPU всей inside-семьи);
   fluid-чтения = ванильная семантика (не убираются при парити).
   Честный резидуал: атрибуция доменов по стекам (JIT-инлайнинг может
   смазать границы потребителей), но кросс-лег-стабильность
   (7.54/7.36%) + чистые списки вызывающих дают уверенность в раскладе.

## Пины
- данные: run-s7194-zeroalloc-v1 / run-s7189-traveldiet-v2a
  world3-bench.zip → cpu-collapsed.txt (spark, cpu-окно; авто-распаковка
  в /tmp/recon35)
- скрипт: scripts/bench4_recon/recon35_chunkread_consumer_drill.py
- прецеденты: RECON-29 (клейм), RECON-30/31/32 (методика дриллов)
""")
    with open(DOC, "w") as f:
        f.write("\n".join(lines) + "\n")
    print(f"OK: {DOC}")
    for name, (tot, dom) in summary.items():
        fam = sum(dom.values())
        top = ", ".join(f"{k}={100.0*v/tot:.2f}%" for k, v in dom.most_common())
        print(f"{name}: сцена={tot} семья={fam} ({100.0*fam/tot:.2f}%) | {top}")

if __name__ == "__main__":
    main()
