#!/usr/bin/env python3
# RECON-11 (TASK-316): декомпозиция G1-фазы и аллок-давления на банковом v3.
# Ось G1: категории self-CPU (card-set/remset = кросс-регионные записи; oop-scan = живой граф; queue).
# Ось alloc: ТОП типов-листьев после фильтра сер-стеков концевого сейва (урок №7).
import re, collections

RUN = "/home/z/c-crussty/research/batch-collector-2026-09-19/run-s7162-leg2-artifact"

# ---------- 1. alloc-collapsed: листья-типы ----------
tot = 0
leaf_n = collections.defaultdict(int)
for line in open(f"{RUN}/alloc-collapsed.txt", encoding="utf-8", errors="replace"):
    m = re.match(r"^(.*) (\d+)$", line.rstrip("\n"))
    if not m:
        continue
    stack, n = m.group(1), int(m.group(2))
    tot += n
    leaf_n[stack.split(";")[-1]] += n
print(f"[ALLOC] всего {tot} сэмплов, уникальных листьев {len(leaf_n)}")
print("[ALLOC] ТОП-25 типов:")
for leaf, n in sorted(leaf_n.items(), key=lambda x: -x[1])[:25]:
    print(f"  {100*n/tot:6.2f}%  {leaf[:130]}")

# сер-стеки концевого сейва (урок №7)
SER = re.compile(r"NbtIo|DataFixer|ChunkSerializer|DataResult|NbtAccounter|NbtUtils|CompoundTag\.save|PalettedContainer\.write")
ser = 0
for line in open(f"{RUN}/alloc-collapsed.txt", encoding="utf-8", errors="replace"):
    m = re.match(r"^(.*) (\d+)$", line.rstrip("\n"))
    if not m:
        continue
    stack, n = m.group(1), int(m.group(2))
    if SER.search(stack):
        ser += n
print(f"[ALLOC] сер-стеки (сейв/сериализация, урок №7): {ser} = {100*ser/tot:.1f}%")

# ---------- 2. cpu-collapsed: G1-категории по ЛИСТЬЯМ ----------
G1RX = re.compile(r"G1CardSet|G1RemSet|OopOopIterate|oopDesc::size|G1ParScan|refine_card|trim_queue|RebuildRS|G1Scan|G1CMOop|G1ConcurrentMark|G1UpdateBuffer|G1Evac|G1Copy|G1Service|G1FreeCollection|G1FullGC|G1CollectedHeap|G1HeapRegion|G1Allocator|G1FromCardCache|G1Policy|G1Analytics|G1HotCardCache|G1GCPhaseTimes|G1HeapVerifier|G1ParallelGC|G1DirtyCardQueue|G1SATB|G1Barrier|G1Collected|G1ArchiveAllocator|G1Page|G1RegionsSmallerThanCommit|G1StringDedup|QuickSort.*G1|G1Committed|G1Monitoring|G1Tracer|WallClock")
CATS = [
    ("card-set/remset (кросс-регионные записи)", re.compile(r"G1CardSet|G1RemSet|G1ScanCardClosure|refine_card_concurrently|G1ConcurrentRefine|G1UpdateBuffer|G1DirtyCardQueue|G1HotCardCache")),
    ("oop-scan живого графа", re.compile(r"OopOopIterate|oopDesc::size|G1CMOopClosure|G1RebuildRemSetClosure|G1ScanHRForRegion|G1ScanSurvivor|G1ScanCardClosure::do_oop")),
    ("queue/evac (копирование выживших)", re.compile(r"trim_queue_to_threshold|G1ParScanThreadState|G1Evacuation|G1Copy")),
    ("rebuild/scrub", re.compile(r"RebuildRSAndScrub")),
    ("mark/concurrent", re.compile(r"G1ConcurrentMark(?!Oop)|G1CMTask|G1CMBitMap")),
    ("прочие G1", re.compile(r"G1")),
]
g1tot = 0
cpu_tot = 0
cat_n = collections.Counter()
other_leaves = collections.Counter()
for line in open(f"{RUN}/cpu-collapsed.txt", encoding="utf-8", errors="replace"):
    m = re.match(r"^(.*) (\d+)$", line.rstrip("\n"))
    if not m:
        continue
    stack, n = m.group(1), int(m.group(2))
    cpu_tot += n
    leaf = stack.split(";")[-1]
    if not G1RX.search(leaf):
        continue
    g1tot += n
    for cat, rx in CATS:
        if rx.search(leaf):
            cat_n[cat] += n
            break
    else:
        cat_n["неразобранные G1-листья"] += n
        other_leaves[leaf] += n
print(f"\n[CPU] база {cpu_tot}; G1-листья суммарно {g1tot} = {100*g1tot/cpu_tot:.1f}% self-CPU")
for cat, n in cat_n.most_common():
    print(f"  {cat:48s} {n:6d} = {100*n/cpu_tot:5.2f}%")
print("[CPU] ТОП неразобранных G1-листьев:")
for leaf, n in other_leaves.most_common(8):
    print(f"   {100*n/cpu_tot:6.3f}%  {leaf[:120]}")

# ---------- 3. gc.log: steady-state young ----------
starts = []
pauses = []
for line in open(f"{RUN}/gc.log", encoding="utf-8", errors="replace"):
    m = re.search(r"\[([0-9.]+)s\]\[info\]\[gc,start\s*\] GC\((\d+)\) Pause Young \(Normal\) \(G1 Evacuation", line)
    if m:
        starts.append(float(m.group(1)))
    m2 = re.search(r"GC\(\d+\) Pause Young \([A-Za-z ]+\) \(G1 Evacuation Pause\) \d+M->\d+M\(\d+M\) ([0-9.]+)ms", line)
    if m2 and "gc,start" not in line:
        pauses.append(float(m2.group(1)))
if starts:
    t0, t1 = starts[0], starts[-1]
    print(f"\n[GC] Normal-young: {len(starts)} шт за [{t0:.0f}..{t1:.0f}]s = {(t1-t0)/max(1,len(starts)-1):.2f}s средний интервал")
if pauses:
    ps = sorted(pauses)
    print(f"[GC] паузы young: n={len(ps)} медиана {ps[len(ps)//2]:.1f}ms p90 {ps[int(len(ps)*0.9)]:.1f}ms max {ps[-1]:.0f}ms суммарно {sum(ps)/1000:.1f}s")
