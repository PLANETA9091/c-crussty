#!/usr/bin/env python3
"""absorb_480_c43.py — [480-C43] финишер: чистая rt8+STEAL пара (run 36388810838).

Прегист-гейты из /home/z/rounds/ROUND-480/c43/PREREG_480_C43_RT8STEAL.md (закон 14a/16):
  G1 delivery: run-env region_threads:8 + region_steal:1 + bu_defer:0 + canon x466-C98
     + world afb3a0b3;
  G2 NPE-FREE (урок s7176, bu_defer=0 риск): 0 NPE/unexpected/threw, boot Done,
     AIOOBE=0, NCDFE=0, >=4 валид-поллов — NPE = честный ценз STEAL-v1 на rt8-фазе;
  G3 M1 HOST-ценз: STW_total <=23.0s AND young_avg <=200ms (v5-FROZEN);
  G4 barrier-share: CyclicBarrier once-per-stack доля wall-collapsed vs G6-база 1.579%
     (967/61,254, 100% RegionTickOps; rt4+steal+bu_defer run 36376470764);
     классы: <=1.2 steal-усилен / (1.2,2.0] C29-коридор / >2.0 rt8-анти-эффект;
  G5 capture >=2пп? (прегист-ответ НЕТ): потолок = main-barrier (1:1) +
     worker-skew/8 (симметрия-дивизор rt8);
  G6 pair: norm_v5 >= +20 AND in-band AND G2 AND G3 (honest P<=2%).

Usage: absorb_480_c43.py [--run-id ID] [--workdir DIR] [--json OUT.json]
"""
import argparse, bisect, collections, json, os, re, shutil, statistics, subprocess, sys, zipfile

REPO = "PLANETA9091/c-crussty"
RUNS = {"c43": (36388810838, "round-480-c43-rt8steal")}
V5 = [(6.5e6, 2.1252), (7.0e6, 2.2047), (7.5e6, 2.3271), (8.2e6, 2.4732),
      (8.7e6, 2.5981), (9.0e6, 2.6280)]
BAND = (6.0e6, 9.5e6)
STW_MAX_S, YOUNG_MAX_MS = 23.0, 200.0
G6_BARRIER_PCT = 1.579           # Л-479-G6: 967/61,254 wall, 100% RegionTickOps
PAIR_BAR = 20.0
RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
RE_IDX = re.compile(r"runner_cpu_index[:=]\s*(\d+)")
TPS_MAX_VALID = 15.0
CANON_ENV = {"seconds": "300", "fake_players": "4",
             "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
             "fluid_dirty": "0", "fluid_bitmask": "0",
             "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
             "population_target": "150000", "population_seed": "42"}
NPE_MARKS = ("NullPointerException", "unexpected exception", "Exception in thread",
             "threw exception", "Watchdog")
AIOOBE_MARKS = ("ArrayIndexOutOfBoundsException", "IndexOutOfBounds",
                "ObjectOpenCustomHashSet", "OutOfCoords")


def token():
    return open("/tmp/gh_token").read().strip()


def api(t, url):
    out = subprocess.run(["curl", "-s", "-H", f"Authorization: token {t}",
                          f"https://api.github.com{url}"],
                         capture_output=True, text=True, timeout=90).stdout
    return json.loads(out)


def fetch_artifact(t, rid, workdir, tag):
    arts = api(t, f"/repos/{REPO}/actions/runs/{rid}/artifacts").get("artifacts", [])
    a = next((x for x in arts if x["name"] == "world3-bench"), None)
    if not a:
        return None, "no world3-bench artifact"
    zpath = os.path.join(workdir, f"{tag}_a.zip")
    subprocess.run(["curl", "-sL", "-H", f"Authorization: token {t}",
                    f"https://api.github.com/repos/{REPO}/actions/artifacts/{a['id']}/zip",
                    "-o", zpath], capture_output=True, timeout=600)
    if open(zpath, "rb").read(2) != b"PK":
        return None, "download/PK fail"
    d = os.path.join(workdir, tag)
    os.makedirs(d, exist_ok=True)
    with zipfile.ZipFile(zpath) as z:
        names = z.namelist()
        for want in ("run-env.txt", "server-stdout.log", "gc.log",
                     "wall-collapsed.txt", "cpu-collapsed.txt"):
            srcs = [n for n in names if n.endswith(want)]
            if not srcs:
                if want in ("wall-collapsed.txt", "cpu-collapsed.txt"):
                    continue
                return None, f"missing {want}"
            with z.open(srcs[0]) as src, open(os.path.join(d, want), "wb") as dst:
                shutil.copyfileobj(src, dst)
    os.remove(zpath)
    return d, ""


def tps_exp_v5(cpu):
    xs = [p[0] for p in V5]; ys = [p[1] for p in V5]
    if cpu <= xs[0]:
        return ys[0] + (ys[1]-ys[0])/(xs[1]-xs[0])*(cpu-xs[0]), "extrap-low"
    if cpu >= xs[-1]:
        return ys[-1] + (ys[-1]-ys[-2])/(xs[-1]-xs[-2])*(cpu-xs[-1]), "extrap-high"
    i = bisect.bisect_left(xs, cpu)
    return ys[i-1] + (ys[i]-ys[i-1])/(xs[i]-xs[i-1])*(cpu-xs[i-1]), "interp"


def parse_gc(text):
    stw = {"total_ms": 0.0, "max_ms": 0.0, "young": 0, "full": 0,
           "young_sum_ms": 0.0}
    for line in text.splitlines():
        if "Pause" not in line or "[gc,phases" in line:
            continue
        m = re.search(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$", line)
        if not m:
            continue
        dur = float(m.group(1))
        stw["total_ms"] += dur
        stw["max_ms"] = max(stw["max_ms"], dur)
        if "Pause Full" in line:
            stw["full"] += 1
        else:
            stw["young"] += 1
            stw["young_sum_ms"] += dur
    stw["young_avg_ms"] = stw["young_sum_ms"]/stw["young"] if stw["young"] else 0.0
    stw["total_s"] = stw["total_ms"]/1000.0
    return stw


def tgroup(head):
    if 'region-worker' in head: return 'region-worker'
    if head.startswith('[Server thread'): return 'server-main'
    if 'Netty' in head or 'netty' in head: return 'netty-io'
    if 'Worker-Main' in head or 'worker-main' in head.lower(): return 'worker-main-pool'
    if 'Craft Scheduler' in head or 'scheduler' in head.lower(): return 'craft-sched'
    if 'Finalizer' in head or 'Reference Handler' in head or 'Signal Dispatcher' in head:
        return 'jvm-aux'
    if 'Timer' in head: return 'jvm-aux'
    return 'other'


def barrier_decomp(run_dir):
    """c17-канон: once-per-stack vs per-frame inclusive (урок Л-479-F1 x2) + thread-группы."""
    p = os.path.join(run_dir, "wall-collapsed.txt")
    pc = os.path.join(run_dir, "cpu-collapsed.txt")
    if not os.path.isfile(p):
        return None
    WT = CT = 0
    bar_stack = bar_frame = 0
    bar_threads = collections.Counter()
    bar_caller = collections.Counter()
    rt_only = 0
    for line in open(p, errors="replace"):
        stack, _, cnt = line.rstrip("\n").rpartition(" ")
        if not stack:
            continue
        try:
            n = int(cnt)
        except ValueError:
            continue
        WT += n
        fr = stack.split(';')
        head = fr[0]
        if any('CyclicBarrier' in f for f in fr):
            bar_stack += n
            bar_threads[tgroup(head)] += n
            bar_frame += n * sum(1 for f in fr if 'CyclicBarrier' in f)
            if all('RegionTickOps' in f or 'CyclicBarrier' in f or not
                   (f.startswith('net/minecraft') or f.startswith('ca/spottedleaf') or
                    f.startswith('java') or f.startswith('jdk'))
                   for f in fr):
                pass  # разбор сайтов ниже
            i0 = next(i for i, f in enumerate(fr) if 'CyclicBarrier' in f)
            caller = next((f for f in reversed(fr[:i0])
                           if f.startswith('net/minecraft') or f.startswith('ca/spottedleaf')),
                          'JUC-only')
            bar_caller[(tgroup(head), caller.split('(')[0][:80])] += n
            if 'RegionTickOps' in stack:
                rt_only += n
    if os.path.isfile(pc):
        for line in open(pc, errors="replace"):
            stack, _, cnt = line.rstrip("\n").rpartition(" ")
            try:
                n = int(cnt)
            except (ValueError, TypeError):
                continue
            CT += n
    if WT == 0:
        return None
    main_bar = bar_threads.get('server-main', 0)
    work_bar = bar_threads.get('region-worker', 0)
    rt8_div = 8
    ceiling_pp = 100.0*(main_bar + work_bar/rt8_div)/WT
    pct = round(100.0*bar_stack/WT, 3)
    if pct <= 1.2:
        klass = "STEAL-УСИЛЕН-НА-RT8-ФАЗЕ (<=1.2)"
    elif pct <= 2.0:
        klass = "C29-КОРИДОР (1.2,2.0] — суб-шум"
    else:
        klass = "RT8-АНТИ-ЭФФЕКТ (>2.0) — ось steal@rt8 REFUTED-кандидат"
    return {
        "wall_total": WT, "cpu_total": CT,
        "once_per_stack": bar_stack, "once_per_stack_pct": pct,
        "per_frame_inclusive": bar_frame,
        "per_frame_pct": round(100.0*bar_frame/WT, 3),
        "x2_factor": round(bar_frame/max(bar_stack, 1), 3),
        "regiontickops_site_pct": round(100.0*rt_only/WT, 3),
        "thread_groups": dict(bar_threads),
        "await_sites": [{"thread": t, "caller": c, "count": n}
                        for (t, c), n in bar_caller.most_common(8)],
        "main_barrier_pct_of_wall": round(100.0*main_bar/WT, 3),
        "worker_barrier_pct_of_wall": round(100.0*work_bar/WT, 3),
        "skew_symmetry_divisor_rt8": rt8_div,
        "critical_path_ceiling_pp": round(ceiling_pp, 3),
        "C29_canon_corridor_pp": 0.60,
        "hypothesis_2pp": bool(ceiling_pp >= 2.0),
        "g4_class": klass,
    }


def env_num(v):
    m = re.match(r"^\s*([\d.]+)", str(v))
    return m.group(1) if m else str(v).strip()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-id", type=int, default=RUNS["c43"][0])
    ap.add_argument("--workdir", default="/home/z/rounds/ROUND-480/c43")
    ap.add_argument("--json", default="/home/z/rounds/ROUND-480/c43/verdict_480_c43.json")
    args = ap.parse_args()
    t = token()
    rid = args.run_id
    os.makedirs(args.workdir, exist_ok=True)
    st = api(t, f"/repos/{REPO}/actions/runs/{rid}")
    print(f"run {rid}: status={st.get('status')} conclusion={st.get('conclusion')} "
          f"branch={st.get('head_branch')} head={str(st.get('head_sha'))[:8]}")
    if st.get("status") != "completed":
        sys.exit(3)
    d, err = fetch_artifact(t, rid, args.workdir, "c43")
    if err:
        print("ARTIFACT-FAIL:", err); sys.exit(5)
    env = open(os.path.join(d, "run-env.txt"), errors="replace").read()
    log = open(os.path.join(d, "server-stdout.log"), errors="replace").read()
    gc = open(os.path.join(d, "gc.log"), errors="replace").read()
    env_kv = {}
    for l in env.splitlines():
        kv = l.split(":", 1)
        if len(kv) == 2:
            env_kv[kv[0].strip()] = kv[1].strip()

    # ---- G1 delivery
    g1 = {}
    for k, want in (("region_threads", "8"), ("region_steal", "1"), ("bu_defer", "0")):
        got = env_num(env_kv.get(k, "?"))
        g1[k] = (got == want, got)
    canon_miss = [k for k, v in CANON_ENV.items() if env_num(env_kv.get(k)) != str(v)]
    g1["canon_miss"] = canon_miss
    world_ok = env_kv.get("world_sha256", "").startswith("afb3a0b3")
    g1["world_afb3a0b3"] = world_ok
    g1_ok = all(v[0] for v in g1.values() if isinstance(v, tuple)) and not canon_miss and world_ok
    print("G1 delivery:", g1, "->", "PASS" if g1_ok else "FAIL")

    # ---- G2 NPE-free / AIOOBE / NCDFE / fixture
    npe = [m for m in NPE_MARKS if m in log]
    aioobe = [m for m in AIOOBE_MARKS if m in log]
    ncdfe = log.count("NoClassDefFoundError")
    boot_done = bool(re.search(r"Done \(\d+\.\d+s\)", log))
    soak_polls = [float(x) for x in RE_TPS.findall(log) if float(x) < TPS_MAX_VALID]
    g2_ok = not npe and not aioobe and ncdfe == 0 and boot_done and len(soak_polls) >= 4
    print(f"G2 NPE-free: NPE={npe} AIOOBE={aioobe} NCDFE={ncdfe} boot_done={boot_done} "
          f"polls={len(soak_polls)} -> {'PASS' if g2_ok else 'FAIL (ценз оси, honest)'}")

    # ---- G3 M1
    stw = parse_gc(gc)
    m1_ok = stw["total_s"] <= STW_MAX_S and stw["young_avg_ms"] <= YOUNG_MAX_MS
    print(f"G3 M1: STW {stw['total_s']:.2f}s young_avg {stw['young_avg_ms']:.1f}ms "
          f"Full {stw['full']} -> {'CLEAN' if m1_ok else 'HOST-CENS'}")

    # ---- G4 barrier-share vs G6-база 1.579% + G5 capture-матем
    bd = barrier_decomp(d)
    if bd:
        print(f"G4 barrier: {bd['once_per_stack_pct']}% thread-wall "
              f"({bd['once_per_stack']}/{bd['wall_total']}, RegionTickOps-сайт "
              f"{bd['regiontickops_site_pct']}%) vs G6-base {G6_BARRIER_PCT}% -> "
              f"Δ {bd['once_per_stack_pct']-G6_BARRIER_PCT:+.3f}пп [{bd['g4_class']}]")
        print(f"G5 capture: потолок main {bd['main_barrier_pct_of_wall']}% + worker/8 "
              f"{bd['worker_barrier_pct_of_wall']}% = {bd['critical_path_ceiling_pp']}пп "
              f"-> hypothesis_2pp = {bd['hypothesis_2pp']}")
    else:
        print("G4 barrier: wall-collapsed отсутствует/пуст — N/A")

    # ---- G6 pair math
    cpu = int(RE_IDX.search(env).group(1)) if RE_IDX.search(env) else 0
    med = statistics.median(soak_polls) if soak_polls else 0.0
    exp, tag = tps_exp_v5(cpu)
    norm = 100*(med/exp - 1) if med else float("nan")
    in_band = BAND[0] <= cpu <= BAND[1]
    pair = norm >= PAIR_BAR
    print(f"G6 norm: cpu {cpu} in_band={in_band} polls-med {med} exp {exp:.4f}({tag}) "
          f"norm_v5 {norm:+.2f} -> pair {'>=+20 LEGAL' if pair and in_band and g2_ok and m1_ok else 'нет (honest)'}")

    out = {"run_id": rid, "branch": st.get("head_branch"),
           "head_sha": st.get("head_sha"), "cpu_index": cpu, "in_band": in_band,
           "G1_delivery": g1, "G1_ok": g1_ok, "G2_npe_free": g2_ok,
           "npe_marks": npe, "aioobe_marks": aioobe, "ncdfe": ncdfe,
           "boot_done": boot_done, "G3_M1": stw, "m1_ok": m1_ok,
           "G4_barrier": bd, "g6_barrier_base_pct": G6_BARRIER_PCT,
           "barrier_delta_pp": (bd["once_per_stack_pct"] - G6_BARRIER_PCT) if bd else None,
           "polls": soak_polls, "polls_median": med, "tps_exp": exp, "norm_v5": norm,
           "pair_legal": bool(pair and in_band and g2_ok and m1_ok)}
    with open(args.json, "w") as f:
        json.dump(out, f, indent=1)
    print("verdict-json:", args.json)


if __name__ == "__main__":
    main()
