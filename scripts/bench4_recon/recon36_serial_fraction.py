#!/usr/bin/env python3
"""RECON-36: serial-fraction измерение ЭПОХИ-2C P1 (спека S7-171 §6 P1 / §3).

Вопрос: какая доля CPU сцены = КРИТИЧЕСКИЙ ПУТЬ главного потока (serial
остаток), а какая уже распараллелена (region_threads workers / chunk pool /
GC)? Профили свёрнуты БЕЗ имён потоков → дискриминация по вход-пути стека:
  - WORKER-ENTITY: дно = RegionTickOps$$Lambda.run → lambda$ensureHelpers$4
    (region_threads worker, S7-156);
  - MAIN: дно = ...MinecraftServer$$Lambda.run → lambda$spin$2 (серверный тик-луп);
  - NATIVE/VM: дно = libc thread_native_entry → Thread::call_run (GC/JIT/VM);
  - TPE-WORKER: ThreadPoolExecutor$Worker.run; CHUNK-WORKER: PrioritisedQueue.

Фазы MAIN (первое совпадение, приоритет снизу-вверх по стеку): entity-tick
(RegionTickOps/tickBucket — main тоже исполняет бакеты, region_steal),
chunk-system, random-tick, fluid, network, world-tick-other, server-tail.

Потолок C (Амдал, CPU-рамка): wall ≈ main-CPU (main = крит.путь; DONE-park
≈0 после region_steal — хедер RegionTickOps). Offload X% работы main на
уже живой пул workers (duty ~14-15%/поток — headroom есть) →
speedup = 1/(1 - X*(1-k)), k = sync-overhead.

Гейт честности P1 (спека §6): serial >70% сцены → пауза C.

Данные: research/gc-recon-2026-09-19/run-{s7194-zeroalloc-v1,s7189-traveldiet-v2a}/
world3-bench.zip → cpu-collapsed.txt (авто-распаковка в /tmp/recon36).
"""
import os, re, sys, collections

BASE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
RR = os.path.join(BASE, "research", "gc-recon-2026-09-19")
RUNS = {"s7194": "run-s7194-zeroalloc-v1", "s7189": "run-s7189-traveldiet-v2a"}
TMP = "/tmp/recon36"

def collapse_path(rd, tag):
    p = os.path.join(rd, "cpu-collapsed.txt")
    if os.path.exists(p):
        return p
    tdir = os.path.join(TMP, tag)
    os.makedirs(tdir, exist_ok=True)
    p = os.path.join(tdir, "cpu-collapsed.txt")
    if not os.path.exists(p):
        z = os.path.join(rd, "world3-bench.zip")
        rc = os.system(f'unzip -o -q "{z}" cpu-collapsed.txt -d "{tdir}"')
        if rc != 0:
            sys.exit(f"FAIL unzip {z}")
    return p

def load(p):
    """→ [(frames:list[str], weight:int)]"""
    out = []
    with open(p) as f:
        for ln in f:
            ln = ln.rstrip("\n")
            if not ln:
                continue
            m = re.match(r"^(.*) (\d+)$", ln)
            if not m:
                continue
            path, w = m.group(1), int(m.group(2))
            out.append((path.split(";"), w))
    return out

def classify_thread(fr):
    for t in fr[:5]:
        if "RegionTickOps.lambda$ensureHelpers$4" in t:
            return "WORKER-ENTITY"
        if "MinecraftServer.lambda$spin$2" in t:
            return "MAIN"
        if "PrioritisedQueueExecutorThread" in t:
            return "CHUNK-WORKER"
        if "ThreadPoolExecutor$Worker.run" in t:
            return "TPE-WORKER"
    t0 = fr[0]
    if "libc.so.6" in t0 or "thread_native_entry" in t0 or t0.startswith("[") \
       or "barrier" in t0 or "adapters" in t0:
        return "NATIVE-VM"
    return "OTHER"

# фазы MAIN: (имя, регекс по полному стеку) — первое совпадение сверху списка
MAIN_PHASES = [
    ("entity-tick(main-бакеты+оркестрация)", r"RegionTickOps"),
    ("fluid-sim", r"updateFluidHeightAndDoFluidPushing|getFlow|hasSameAbove|FluidState\.tick|tickFluid|getFluidState"),
    ("random-tick", r"andom[Tt]ick|tickChunk"),
    ("block-tick-scheduler", r"LevelTicks|runBlockUpdates|BlockTickingTick"),
    ("chunk-system", r"ServerChunkCache\.tick|ChunkMap\.tick|ticket|Ticket|ChunkTaskScheduler|chunkSystem"),
    ("network/connection", r"Connection\.tick|Packet|packet"),
    ("world-border/weather/sleep", r"worldBorder|WorldBorder|sleepStatus|weather"),
    ("entity-другое(broadphase/tracker)", r"ChunkEntitySlices|getEntities|newTrackerTick|EntityTickList"),
    ("midTick-pump", r"midTickTasks|pollTask"),
    ("server-tail/прочее", r"."),
]

def main_phase(fr):
    full = "|".join(fr)
    for name, rx in MAIN_PHASES:
        if re.search(rx, full):
            return name
    return "server-tail/прочее"

def top_frames(stacks, k=12, skip=6):
    """взвешенный топ хвостов-фреймов (конец стека) для честности"""
    c = collections.Counter()
    for fr, w in stacks:
        for t in fr[-3:]:
            if len(fr) > skip:
                c[t] += w
    return c.most_common(k)

def amdal(X, k):
    return 1.0 / (1.0 - X * (1.0 - k))

for tag, rd_name in RUNS.items():
    rd = os.path.join(RR, rd_name)
    p = collapse_path(rd, tag)
    data = load(p)
    total = sum(w for _, w in data)
    print(f"\n{'='*78}\n{tag}: {p}\nСЭМПЛОВ ВСЕГО: {total}")

    threads = collections.OrderedDict()
    for fr, w in data:
        threads.setdefault(classify_thread(fr), []).append((fr, w))
    print("\n-- THREAD-SPLIT (взвешенный):")
    main_stacks = []
    for th, stacks in sorted(threads.items(), key=lambda kv: -sum(w for _, w in kv[1])):
        s = sum(w for _, w in stacks)
        print(f"  {th:16s} {s:7d}  {100*s/total:5.1f}%")
        if th == "MAIN":
            main_stacks = stacks

    # фазовая декомпозиция MAIN
    ms = sum(w for _, w in main_stacks)
    print(f"\n-- MAIN-ФАЗЫ (serial крит.путь, {ms} сэмплов = {100*ms/total:.1f}% сцены):")
    phases = collections.OrderedDict()
    for fr, w in main_stacks:
        phases.setdefault(main_phase(fr), []).append((fr, w))
    for ph, stacks in sorted(phases.items(), key=lambda kv: -sum(w for _, w in kv[1])):
        s = sum(w for _, w in stacks)
        print(f"  {ph:42s} {s:6d}  {100*s/total:5.1f}% сцены | {100*s/ms:5.1f}% main")
    print("\n  -- честность: топ-хвосты двух крупнейших main-фаз:")
    for ph, stacks in sorted(phases.items(), key=lambda kv: -sum(w for _, w in kv[1]))[:2]:
        print(f"   [{ph}]")
        for t, w in top_frames(stacks):
            print(f"      {w:6d}  {t}")

    # WORKER-ENTITY sanity (уже параллельно)
    we = threads.get("WORKER-ENTITY", [])
    wes = sum(w for _, w in we)
    if we:
        print(f"\n-- WORKER-ENTITY sanity: {wes} сэмплов ({100*wes/total:.1f}% сцены), "
              f"4 потока → duty ~{100*wes/total/4:.1f}%/поток (headroom пулов)")
        print("   топ-хвосты:", ", ".join(f"{t.split('/')[-1]}:{w}" for t, w in top_frames(we, 6)))

    # потолок Амдала для C: offload доли main (в единицах main-wall = 1.0)
    print("\n-- ПОТОЛОК C (Амдал, wall=main-CPU=1.0; speedup=1/(1-X*(1-k))):")
    for X in (0.15, 0.25, 0.35, 0.50):
        row = "  X=" + f"{X:.2f} main → " + "  ".join(
            f"k={k}: +{100*(amdal(X,k)-1):.0f}%" for k in (0.1, 0.3))
        print(row)

    # гейт честности P1
    sfrac = ms / total
    print(f"\n-- GATE P1: serial-fraction (main/сцена) = {100*sfrac:.1f}% "
          f"→ {'ПАУЗА C (>70%)' if sfrac > 0.70 else 'C ПРОДОЛЖАЕТСЯ (≤70%)'}")

print(f"\n{'='*78}\nПРИМЕЧАНИЯ ЧЕСТНОСТИ:")
print(" 1. CPU-профиль НЕ видит WAIT главного потока (барьеры/IO) — wall-доля main")
print("    может быть ВЫШЕ CPU-доли; DONE-park≈0 по хедеру RegionTickOps")
print("    (region_steal), но GO-arrival/переключения не наблюдаемы → резидуал P2.")
print(" 2. Профили = банк v3 + один рычаг (s7194 zero_alloc=1 / s7189 travel_diet=1);")
print("    REFUTED-ноги не меняют структуру фаз (RECON-26/27/28 кросс-стабильны).")
print(" 3. cpu_index раннеров 6.63M/6.95M — оба в широком банде 6.0..9.5M.")
