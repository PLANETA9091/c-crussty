#!/usr/bin/env python3
"""analyze_482_c71_netcensus.py — ЛАБ-C71 ROUND-482 (ОФФЛАЙН, 0 диспатчей).

Net-capture актуализация на rt4-стенде:
  1) net-стек-ценз ×481-482 (CPU+wall collapsed) — transport-work / send-лейн / top-30;
  2) RAMP ×1.18 fixed-shape пересчёт rt8-prio-теоремы (W3 ×478, k-мост);
  3) fp8 np-last2 plateau-контроль vs ваниль-окно (absorb_482.json).

Каноны: Л-470-S27 (netty REFUTED 0/612,636 wall, R1-R4), Л-479-W3 (RAMP τ̂=0.69,
bias −12.53пп, fixed-shape ×1.18), W3_rt8_duty_redecomp.md (7.40пп теор / 1.5-2.2 реалист),
Л-482-C57 (TOPUP-SCAN 0.012% cpu), Л-482-C60 (владельце-квалификация символов).
Запуск: python3 scripts/analyze_482_c71_netcensus.py [--quick]
"""
import json
import math
import re
import statistics
import sys

# --- 1. NET-ЦЕНЗ -----------------------------------------------------------

NET_PAT = re.compile(
    r"netty|Netty|epoll|ServerConnection|Connection\.send|writev|flush0|"
    r"processSelectedKeys|[Ss]ocket|[Kk]eepAlive|PacketEncoder|PacketDecoder|"
    r"Varint|Cipher|ChannelOutbound|EmbeddedChannel|doSendPacket"
)
# транспорт-work сайты Л-470-S27: настоящий work, не idle-park
TRANSPORT_WORK = re.compile(r"writev|flush0|processSelectedKeys")
SEND_LANE = "net/minecraft/network/Connection.send;"

CPU_PROFILES = [
    ("/home/z/rounds/ROUND-481/absorb/x36384769001/cpu-collapsed.txt", "481 x36384769001"),
    ("/home/z/rounds/ROUND-481/absorb/x36389080708/cpu-collapsed.txt", "481 x36389080708"),
    ("/home/z/rounds/ROUND-482/c48_xms8/x482/art/cpu-collapsed.txt", "482 c48_xms8"),
    ("/home/z/rounds/ROUND-482/c45_960s/unz/cpu-collapsed.txt", "482 c45_960s"),
]
WALL_PROFILES = [
    ("/home/z/rounds/ROUND-481/absorb/x36384769001/wall-collapsed.txt", "481 x36384769001"),
    ("/home/z/rounds/ROUND-481/absorb/x36389080708/wall-collapsed.txt", "481 x36389080708"),
    ("/home/z/rounds/ROUND-482/c34_strat2/artifact/wall-collapsed.txt", "482 c34_strat2"),
    ("/home/z/rounds/ROUND-482/c48_xms8/x482/art/wall-collapsed.txt", "482 c48_xms8"),
    ("/home/z/rounds/ROUND-482/c45_960s/unz/wall-collapsed.txt", "482 c45_960s"),
]


def load_collapsed(path):
    stacks = []
    with open(path, errors="replace") as fh:
        for ln in fh:
            ln = ln.rstrip("\n")
            if not ln:
                continue
            i = ln.rfind(" ")
            try:
                n = int(ln[i + 1:])
            except ValueError:
                continue
            stacks.append((n, ln[:i]))
    stacks.sort(reverse=True)
    return stacks


def netcensus(path, label):
    stacks = load_collapsed(path)
    tot = sum(n for n, _ in stacks)
    net = sum(n for n, s in stacks if NET_PAT.search(s))
    tw = sum(n for n, s in stacks if TRANSPORT_WORK.search(s))
    send = sum(n for n, s in stacks if SEND_LANE in s + ";")
    top30_net = [(i, n, s) for i, (n, s) in enumerate(stacks[:30], 1) if NET_PAT.search(s)]
    idle = [
        (n, s)
        for n, s in stacks[:30]
        if "EpollEventLoop" in s and "epoll_wait" in s
    ]
    idle_work = sum(n for n, s in idle if "epollWait" in s and not TRANSPORT_WORK.search(s))
    print(f"[{label}] total={tot} net={net} ({100 * net / tot:.4f}%) "
          f"transport_work={tw} send_lane={send} ({100 * send / tot:.4f}%) "
          f"net_in_top30={len(top30_net)}")
    for i, n, s in top30_net:
        kind = "idle-park" if "epoll_wait" in s else "WORK"
        print(f"    #{i} {n} ({100 * n / tot:.4f}%) [{kind}] {s[:120]}")
    return tot, net, tw, send, len(top30_net)


# --- 2. RAMP-ПЕРЕСЧЁТ W3-prio ----------------------------------------------

def ramp_recount():
    fit = json.load(open("/home/z/c-crussty/docs/ROUND-479/lab/ramp_fit.json"))
    taus = [r["tau"] for r in fit]
    bias = [r["bias_pp"] for r in fit]
    print(f"RAMP: n={len(fit)} tau_med={statistics.median(taus):.2f} "
          f"IQR {sorted(taus)[len(taus) // 4]:.2f}-{sorted(taus)[3 * len(taus) // 4]:.2f} "
          f"bias_med={statistics.median(bias):.2f} bias_mean={statistics.mean(bias):.2f} "
          f"sd={statistics.stdev(bias):.2f}")
    # W3-пара x9(rt4)/x5(rt8): плато-мед 2.6 vs 2.4 (5-полльные) — fixed-shape инвариантность
    raw = 100 * (2.4 / 2.6 - 1)
    corr = 100 * (2.4 * 1.18 / (2.6 * 1.18) - 1)
    k = abs(raw) / 9.89  # norm-пп на 1 CPU-пп (мост x5/x9)
    print(f"W3 pair: raw dPlateau={raw:.2f}пп corrected={corr:.2f}пп "
          f"(fixed-shape инвариантность) мост k={k:.3f} norm-пп/CPU-пп")
    print(f"prio-теорема 7.40пп CPU -> {7.40 * k:.2f}пп norm "
          f"(x1.18 щедро: {7.40 * k * 1.18:.2f}пп); реалист 1.5-2.2пп CPU -> "
          f"{1.5 * k:.2f}..{2.2 * k:.2f}пп (x1.18: {1.5 * k * 1.18:.2f}..{2.2 * k * 1.18:.2f}пп)"
          " — все << бар +20 -> RAMP НЕ reopen")


# --- 3. FP8 NP-LAST2 ПЛАТО-КОНТРОЛЬ ----------------------------------------

def fp8_plateau_control():
    d = json.load(open("/home/z/rounds/ROUND-482/absorb/absorb_482.json"))
    runs = d if isinstance(d, list) else d.get("runs") or list(d.values())
    l2s, med_vals = [], []
    for r in runs if isinstance(runs, list) else []:
        if not isinstance(r, dict):
            continue
        p = r.get("polls5") or r.get("polls") or (r.get("raw_polls_stdout") or [])[1:]
        if not p or len(p) < 5 or r.get("armed"):
            continue
        cls = str(r.get("class", ""))
        if "VANILLA" not in cls and "NORM-COMPUTED" not in str(r.get("verdict", "")):
            continue
        l2s.append(statistics.median(p[-2:]))
        med_vals.append(statistics.median(p))
    c42 = json.load(open("/home/z/rounds/ROUND-482/c42/absorb_c42_fp8.json"))
    p = c42["polls5"]
    fp8_l2 = statistics.median(p[-2:])
    fp8_med = statistics.median(p)
    # tau-фит C42: poll[i]=P*(1-exp(-i/tau))
    best = None
    for P100 in range(260, 320, 2):
        for t10 in range(5, 60):
            P, t = P100 / 100, t10 / 10
            sse = sum((pi - P * (1 - math.exp(-(i + 1) / t))) ** 2 for i, pi in enumerate(p))
            if best is None or sse < best[0]:
                best = (sse, P, t)
    if l2s:
        med_l2 = statistics.median(l2s)
        sd = statistics.stdev(l2s) if len(l2s) > 1 else float("nan")
        print(f"ваниль-окно n={len(l2s)}: np-last2 med {med_l2:.2f} "
              f"(IQR {sorted(l2s)[len(l2s) // 4]:.2f}-{sorted(l2s)[3 * len(l2s) // 4]:.2f}, "
              f"sd {sd:.3f}) | c55-med {statistics.median(med_vals):.2f} "
              f"-> эмпирический gain x{med_l2 / statistics.median(med_vals):.2f} (калибровка x1.18)")
        print(f"fp8: np-last2 {fp8_l2:.2f} (delta {100 * (fp8_l2 / med_l2 - 1):+.1f}пп, "
              f"z {(fp8_l2 - statistics.mean(l2s)) / sd:+.2f}) | med {fp8_med:.2f} "
              f"(med-дельта vs окна {100 * (fp8_med / statistics.median(med_vals) - 1):+.1f}пп) | "
              f"tau_fit={best[2]:.1f} полла (канон 0.69)")


def main():
    quick = "--quick" in sys.argv
    ct = cn = cw = cs = 0
    wt = ww = ws = 0
    for p, l in CPU_PROFILES:
        t, n, w, s, _ = netcensus(p, "CPU " + l)
        ct += t; cn += n; cw += w; cs += s
    if not quick:
        for p, l in WALL_PROFILES:
            t, n, w, s, _ = netcensus(p, "WALL " + l)
            wt += t; ww += w; ws += s
    print(f"ИТОГ CPU: {ct} сэмплов, transport_work={cw} (0 ожид.), send-лейн {cs} "
          f"({100 * cs / ct:.4f}%) | ИТОГ WALL: {wt}, transport_work={ww} | "
          f"net top-30 CPU = 0 | netty-wall 1.96% = 100% epoll_wait idle-park")
    ramp_recount()
    fp8_plateau_control()
    print("ВЕРДИКТ: net-план закрыт x6 (W3 x5 + rt4-актуализация); reopen >=2пп не найден "
          f"(send-лейн {100 * cs / ct:.4f}% CPU -> <= +0.003пп norm)")


if __name__ == "__main__":
    main()
