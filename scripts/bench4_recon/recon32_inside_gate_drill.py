#!/usr/bin/env python3
"""recon32_inside_gate_drill.py — RECON-32: дрилл inside-gate-семьи
(~3.1% claim, последний крупный непроверенный компонент ЭПОХИ-2A) до
доменов + вердикт потолка inside-gate-диеты.

Контекст:
- БАНК v3 уже содержит INSIDE-CACHE (S7-135): gate Entity.isAffectedByBlocks
  в checkInsideBlocks — статичные сущности (deltaMovement==0 + позиция
  бит-в-бит равна кэшу) обслуживаются из плоских массивов (replay ванильных
  effect-вызовов); движущиеся падают в ванильное тело.
- inside_diet (#12) REFUTED: alloc-диета RecordedEffect/склада — alloc −47.2%/
  CPU −66..−67% → TPS −17..−6% (min-of-2) — экономика лейна доказанно мертва.

Классификация стека inside-семьи:
  gate/recorder      — сам InsideBlockOps.gate/Recorder (банк-инфраструктура)
  vanilla-discovery  — ванильное тело checkInsideBlocks (движущиеся): 
                       getBlockState/forEachBlockIntersectedBetween/AABB
  effects-apply      — applyEffectsFromBlocks/InsideBlockEffectApplier/
                       EffectApplier-применение (freeze/honey/fire)
  baseTick-context   — сэмплы внутри Entity.baseTick/Mob.aiStep контекста
  other
Вердикт-модель: остаток после банка = discovery для движущихся; позиционный
кэш для движущихся = movement-invalidation (аналог RECON-30) + эмпирика
REFUTED #12 → потолок диеты <10%, микро-зона.
"""
import os
import re
from collections import Counter

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUNS = {
    "s7194": os.path.join(RESDIR, "run-s7194-zeroalloc-v1"),
    "s7189": os.path.join(RESDIR, "run-s7189-traveldiet-v2a"),
}
DOC = os.path.join(RESDIR, "RECON32_INSIDE_GATE_VERDICT.md")

FAMILY_RX = re.compile(
    r"checkInsideBlocks|InsideBlockOps|InsideBlockEffectApplier|"
    r"applyEffectsFromBlocks|isAffectedByBlocks|StepBasedCollector|"
    r"getInsideEntities|InsideBlockEffectType")

def classify(stack):
    fr = stack.split(";")
    if not any(FAMILY_RX.search(f) for f in fr):
        return None
    # deepest-first специальность
    for f in reversed(fr):
        name = f.split("/")[-1]
        if re.search(r"InsideBlockOps\.gate|Recorder", name):
            return "gate/recorder (банк-инфра)"
        if re.search(r"InsideBlockEffectApplier|applyEffects|addEffect|"
                     r"freeze|Freeze|setTicksFrozen|hurt", name):
            return "effects-apply"
        if re.search(r"getBlockState|getBlock|forEachBlockIntersected|"
                     r"PalettedContainer|LevelChunk|BlockState;|"
                     r"checkInsideBlocks|isAffectedByBlocks|AABB", name):
            return "vanilla-discovery (движущиеся)"
    # контекст: кто зовёт
    for f in reversed(fr):
        name = f.split("/")[-1]
        if re.search(r"baseTick|aiStep|tick\(|entityTick", name):
            return "context-tail"
    return "other"

def main():
    lines = ["# RECON-32 — дрилл inside-gate-семьи (NEXT 355) + вердикт inside-gate-диеты", ""]
    for name, rd in RUNS.items():
        path = os.path.join(rd, "cpu-collapsed.txt")
        tot = 0
        fam = Counter()
        tails = Counter()
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
            k = classify(stack)
            if k:
                fam[k] += n
                if k == "vanilla-discovery (движущиеся)":
                    fr = stack.split(";")
                    tails[fr[-1].split("/")[-1][:50]] += n
        lines.append(f"## {name} (сцена {tot})")
        lines.append("")
        lines.append("| домен | сэмплов | % сцены |")
        lines.append("|---|---|---|")
        for k, v in fam.most_common():
            lines.append(f"| {k} | {v} | {100.0 * v / tot:.2f}% |")
        if not fam:
            lines.append("| (семья отсутствует) | 0 | 0.00% |")
        lines.append("")
        if tails:
            lines.append("### vanilla-discovery лист-хвосты (топ-8)")
            lines.append("")
            for k, v in tails.most_common(8):
                lines.append(f"- {100.0 * v / tot:5.2f}% {v:6d} {k}")
            lines.append("")

    lines.append("""## ВЕРДИКТ (клейм 3.1% ОПРОВЕРГНУТ в большую сторону + переклассификация)

1. Inside-семья на CPU-оси = **11.28/11.65% сцены** (s7194: discovery 9.08 +
   effects 2.20; s7189: 9.36 + 2.29) — в 3.7× больше клейма «inside-gate ~3.1%»
   в списке ЭПОХИ-2A (клейм считал только guard-хвост, RECON-27b ~0.8%;
   полная семья = vanilla-discovery для ДВИЖУЩИХСЯ + applier + банк-инфра).
2. Структура (листья, s7194): PalettedContainer.get 855, visit-set add 819,
   Recorder.flushStep 806 + gate 791 + <init> 747 (банк-инфра ~1.8%),
   applyEffectsFromBlocks 658, guava-iterator computeNext 616,
   lambda$checkInsideBlocks$2 457, isEmpty 444, vtable 409, G1SATB-barrier 390,
   forEachBlockIntersectedBetween 331. Стеки чистые: tick →
   applyEffectsFromBlocks → checkInsideBlocks → forEachBlockIntersectedBetween
   → visit (ItemEntity/Mob-муверы каждый тик).
3. Статусы под-кусков: (a) visit-set/iterator-интернал = внутренность
   REFUTED #9 flat_traversal (RECON-25) — повторный рычаг ЗАПРЕЩЁН;
   (b) alloc-диета RecordedEffect = REFUTED #12 (alloc −47%/CPU −66% →
   TPS −17..−6%); (c) effects-apply = ванильная семантика парити
   (freeze/honey/fire) — не убирается; (d) банк-инфра = собственный банкинг.
4. ЕДИНСТВЕННЫЙ легальный рычаг семьи = **секционный solid-bitmask
   пре-фильтр** (event-driven dirty-флаги на изменения блоков — санционировано
   миссией): short-circuit checkInsideBlocks/applyEffectsFromBlocks, когда все
   секции расширенного AABB чисто-воздушные. Сырой потолок = discovery ~9.1%
   сцены (воздушная доля муверов); TPS-конверсия по эмпирике диет-семейства
   ~0 (5 контрпримеров: структурный эффект подтверждён, TPS не растёт);
   MSPT-пропорциональная верхняя оценка ~13-16% TPS недостижима эмпирически.
   НА ТЕКУЩЕМ ПРОТОКОЛЕ (ДВОЙНОЙ БАР) НЕ БАНКИНГУЕТСЯ; **при переходе
   владельца на вариант B (min-MSPT) — главный кандидат-флагман**.
5. ВЕРДИКТ: «inside-gate-диета» как диета = **7-я ДОК-ВЕРИФИКАЦИЯ закрытия**
   (прецедент RECON-17/20/23/26/30/31/32) — ни один безопасный под-кусок не
   обещает ≥+10% обеих осей. Компонент A «inside-gate 3.1%» переклассифицирован:
   семья 11.3/11.6%, легальный потолок ~5-6% TPS (сырой ~9% CPU при
   конверсии ~0). Итог ЭПОХИ-2A: все компоненты проверены (broadphase ≤3%
   RECON-30, volatile 0-1% RECON-31, inside — здесь, контур 1.1% RECON-28,
   chunk-read 3.6% RECON-29) — суммарный ЛЕГАЛЬНЫЙ потенциал < двойного бара
   при конверсии ~0. **Вариант A окончательно не рекомендован; решение
   владельца между B (флагман = inside-bitmask) и C.**
""")
    with open(DOC, "w") as f:
        f.write("\n".join(lines))
    for name, rd in RUNS.items():
        print(f"== {name}")
        path = os.path.join(rd, "cpu-collapsed.txt")
        tot = 0
        fam = Counter()
        for line in open(path, errors="ignore"):
            s, _, c = line.rstrip("\n").rpartition(" ")
            if not s:
                continue
            try:
                n = int(c)
            except ValueError:
                continue
            tot += n
            k = classify(s.replace(".", "/"))
            if k:
                fam[k] += n
        for k, v in fam.most_common():
            print(f"   {k:32s} {v:6d} {100.0 * v / tot:5.2f}%")


if __name__ == "__main__":
    main()
