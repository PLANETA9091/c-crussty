#!/usr/bin/env python3
"""RECON-37: worker-дисбаланс I из THREADED wall-профиля (P2-pre-gate, RECON-36 §4).

Вход: run-dir (или путь к world3-bench.zip) с wall-collapsed.txt, снятым
thread-separated сессией (`asprof start -t -e wall`, TASK-363 harness change).
Формат строки: "<thread>;frame1;...;frameN COUNT" (имя потока = токен 0).

Метрики:
  - WORKER = thread ~ ^crussty-region-worker- ; MAIN = "Server thread".
  - duty_w = ACTIVE/total, ACTIVE = стеки с tickBucket БЕЗ CyclicBarrier
    (worker исполняет entity-работу); PARKED = стеки с CyclicBarrier
    (GO/DONE барьеры RegionTickOps); прочее = пул-idle/прочие фазы.
  - I = max(duty_w)/avg(duty_w) по воркерам.
  - main: park-доля (DONE-wait residual) + активная entity-доля (region_steal).

Развилка RECON-36 (прегистр): I<=1.15 -> OFFLOAD-READY (P2 = слив main-бакетов
+ offload остатков, потолок +25..33% > ДВОЙНОГО БАРА); I>=1.3 -> REBALANCE
(REGION_CHUNKS=8 -> мельче / WORKERS вверх); между -> GRAY (повторный лег).
"""
import os, re, sys, zipfile

WORKER_RX = re.compile(r"^(?:\[)?crussty-region-worker-(\d+)(?:\])?$")
MAIN_RX = re.compile(r"^(?:\[)?Server thread(?:\])?$")


def load_threaded(path):
    out = []
    for ln in open(path, errors="ignore"):
        ln = ln.rstrip("\n")
        if not ln:
            continue
        m = re.match(r"^(.*) (\d+)$", ln)
        if not m:
            continue
        parts = m.group(1).split(";")
        out.append((parts[0].strip(), parts, int(m.group(2))))
    return out


def analyze(path):
    rows = load_threaded(path)
    if not rows:
        print("EMPTY profile")
        return 3
    threads = {}
    for tname, parts, w in rows:
        threads.setdefault(tname, []).append((parts, w))

    # формат-гвард: threaded или старый merged (первый токен java/lang/...)
    threaded = any(WORKER_RX.match(t) or MAIN_RX.match(t) for t in threads)
    total_all = sum(r[2] for r in rows)
    print(f"wall-collapsed: {len(rows)} стеков, {total_all} сэмплов, "
          f"{len(threads)} уникальных имён потоков")
    if not threaded:
        print("VERDICT: TOOL-FAIL — threaded-формат НЕ обнаружен (первый токен "
              "не worker/Server thread); asprof -t не применился, повторить лег")
        return 4

    workers = {t: v for t, v in threads.items() if WORKER_RX.match(t)}
    mains = {t: v for t, v in threads.items() if MAIN_RX.match(t)}

    print("\n-- WORKERS (duty = tickBucket-активность / все сэмплы потока):")
    duties = {}
    for t in sorted(workers, key=lambda x: int(WORKER_RX.match(x).group(1))):
        v = workers[t]
        tot = sum(w for _, w in v)
        act = sum(w for p, w in v if any("tickBucket" in f for f in p)
                  and not any("CyclicBarrier" in f for f in p))
        park = sum(w for p, w in v if any("CyclicBarrier" in f for f in p))
        duty = act / tot if tot else 0
        slot = WORKER_RX.match(t).group(1)
        duties[slot] = duty
        print(f"  {t:28s} tot={tot:6d} active={act:6d} ({100*duty:5.1f}%) "
              f"park={park:6d} ({100*park/tot if tot else 0:4.1f}%)")
        # топ-3 листьев активной фазы — sanity тел бакетов
        leaf = {}
        for p, w in v:
            if any("tickBucket" in f for f in p) and not any("CyclicBarrier" in f for f in p):
                leaf[p[-1].split("/")[-1]] = leaf.get(p[-1].split("/")[-1], 0) + w
        for k, w in sorted(leaf.items(), key=lambda kv: -kv[1])[:3]:
            print(f"      {w:6d}  {k}")

    if len(duties) < 3:
        print(f"VERDICT: INCOMPLETENESS — worker-потоков {len(duties)} < 3 "
              f"(ожидалось 4); повторить лег")
        return 5
    ds = list(duties.values())
    avg = sum(ds) / len(ds)
    I = max(ds) / avg if avg else 0
    print(f"\n-- ДИСБАЛАНС I = max/avg duty = {max(ds):.3f}/{avg:.3f} = ** {I:.2f} **")

    for t, v in mains.items():
        tot = sum(w for _, w in v)
        park = sum(w for p, w in v if any("CyclicBarrier" in f for f in p))
        act = sum(w for p, w in v if any("tickBucket" in f for f in p)
                  and not any("CyclicBarrier" in f for f in p))
        print(f"-- MAIN ({t}): tot={tot} active-entity={act} "
              f"({100*act/tot if tot else 0:.1f}%) park={park} "
              f"({100*park/tot if tot else 0:.1f}%) — DONE-wait residual")

    print("\n-- РАЗВИЛКА RECON-36 (прегистр):")
    if I <= 1.15:
        print(f"   I={I:.2f} <= 1.15 -> ** OFFLOAD-READY **: P2 = слив main-бакетов "
              f"в workers + offload main-остатков; потолок +25..33% wall-clock")
        return 0
    if I >= 1.30:
        print(f"   I={I:.2f} >= 1.30 -> ** REBALANCE **: сначала REGION_CHUNKS=8 -> "
              f"мельче / WORKERS вверх; offload отложен до GREEN-геометрии")
        return 1
    print(f"   I={I:.2f} в серой зоне (1.15..1.30) -> повторный threaded лег "
          f"(min-of-2 по знаку развилки)")
    return 2


def main():
    arg = sys.argv[1] if len(sys.argv) > 1 else ""
    p = os.path.join(arg, "wall-collapsed.txt")
    if not os.path.exists(p) and os.path.isdir(arg):
        z = os.path.join(arg, "world3-bench.zip")
        if os.path.exists(z):
            os.makedirs("/tmp/recon37", exist_ok=True)
            p = "/tmp/recon37/wall-collapsed.txt"
            if not os.path.exists(p):
                os.system(f'unzip -o -q "{z}" wall-collapsed.txt -d /tmp/recon37')
    if not os.path.exists(arg):
        print(f"usage: recon37_worker_balance.py <run-dir|zip-path> (got: {arg})")
        return 9
    return analyze(p) if os.path.exists(p) else (print(f"нет wall-collapsed.txt в {arg}") or 8)


if __name__ == "__main__":
    sys.exit(main())
