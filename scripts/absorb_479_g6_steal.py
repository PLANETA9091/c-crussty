#!/usr/bin/env python3
"""absorb_479_g6_steal.py — [479-G6] финишер: STEAL=1+bu_defer=1 @rt4 (w2-вариант).

Прегист-гейты из scripts/dispatch_479_g6_steal.py (закон 14a/16):
  G1 delivery: run-env region_steal:1 + bu_defer:1 + region_threads:4 + canon x466-C98;
  G2 NPE-FREE (урок s7176): 0 NPE/unexpected/threw, сервер жив в конце, AIOOBE=0,
     NCDFE=0, boot Done/FIXTURE VALID;
  G3 M1 HOST-ценз: STW_total ≤23.0s ∧ young_avg ≤200ms (gc.log completion-строки);
  G4 barrier-share: CyclicBarrier доля thread-wall в wall-collapsed vs A15-база
     2.03% (x8 900s/640 run 36358155978);
  G5 pair: norm_v5 = 100·(med/tps_exp_v5(cpu)−1) ≥ +20 → пара (honest: unlikely).

Usage: absorb_479_g6_steal.py [--run-id ID] [--workdir DIR] [--json OUT.json]
"""
import argparse, bisect, json, os, re, shutil, statistics, subprocess, sys, zipfile

REPO = "PLANETA9091/defunct-none"  # placeholder, переопределяется ниже
REPO = "PLANETA9091/c-crussty"
RUNS = {"g6": (36376470764, "round-479-g6-steal")}
V5 = [(6.5e6, 2.1252), (7.0e6, 2.2047), (7.5e6, 2.3271), (8.2e6, 2.4732),
      (8.7e6, 2.5981), (9.0e6, 2.6280)]
BAND = (6.0e6, 9.5e6)
STW_MAX_S, YOUNG_MAX_MS = 23.0, 200.0
A15_BARRIER_PCT = 2.03
PAIR_BAR = 20.0
RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
RE_IDX = re.compile(r"runner_cpu_index[:=]\s*(\d+)")
TPS_MAX_VALID = 15.0
# radius НЕ эхоится в run-env — канон-мир верифицируется world_sha256 afb3a0b3
CANON_ENV = {"seconds": "300", "fake_players": "4",
             "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
             "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
             "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
             "population_target": "150000", "fake_players": "4"}
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
                     "BOTTLENECKS_3.md", "wall-collapsed.txt"):
            srcs = [n for n in names if n.endswith(want)]
            if not srcs:
                if want in ("BOTTLENECKS_3.md", "wall-collapsed.txt"):
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


def wall_barrier(run_dir):
    """thread-wall: total всех сэмплов + CyclicBarrier-доля (+ RegionTickOps-сайт)."""
    p = os.path.join(run_dir, "wall-collapsed.txt")
    if not os.path.isfile(p):
        return None
    tot = bar = bar_rt = 0
    for line in open(p, errors="ignore"):
        stack, _, cnt = line.rstrip("\n").rpartition(" ")
        if not stack:
            continue
        try:
            n = int(cnt)
        except ValueError:
            continue
        tot += n
        if "CyclicBarrier" in stack:
            bar += n
            if "RegionTickOps" in stack:
                bar_rt += n
    if tot == 0:
        return None
    return {"total_samples": tot, "barrier_samples": bar,
            "barrier_rt_samples": bar_rt,
            "barrier_pct": round(100.0*bar/tot, 3),
            "barrier_rt_pct": round(100.0*bar_rt/tot, 3)}


def env_num(v):
    """run-env значения аннотированы («1 (CRUSSTY_REGION_STEAL; …)») — ведущее число."""
    m = re.match(r"^\s*([\d.]+)", str(v))
    return m.group(1) if m else str(v).strip()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-id", type=int, default=RUNS["g6"][0])
    ap.add_argument("--workdir", default="/home/z/rounds/ROUND-479/G6")
    ap.add_argument("--json", default="/home/z/rounds/ROUND-479/G6/verdict_479_g6.json")
    args = ap.parse_args()
    t = token()
    rid = args.run_id
    os.makedirs(args.workdir, exist_ok=True)
    st = api(t, f"/repos/{REPO}/actions/runs/{rid}")
    print(f"run {rid}: status={st.get('status')} conclusion={st.get('conclusion')} "
          f"branch={st.get('head_branch')} head={str(st.get('head_sha'))[:8]}")
    if st.get("status") != "completed":
        sys.exit(3)
    if st.get("conclusion") != "success":
        sys.exit(4)
    d, err = fetch_artifact(t, rid, args.workdir, "g6")
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
    for k, want in (("region_steal", "1"), ("bu_defer", "1"), ("region_threads", "4")):
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
          f"polls={len(soak_polls)} -> {'PASS' if g2_ok else 'FAIL'}")

    # ---- G3 M1
    stw = parse_gc(gc)
    m1_ok = stw["total_s"] <= STW_MAX_S and stw["young_avg_ms"] <= YOUNG_MAX_MS
    print(f"G3 M1: STW {stw['total_s']:.2f}s/{stw['pauses'] if 'pauses' in stw else ''} "
          f"young_avg {stw['young_avg_ms']:.1f}ms Full {stw['full']} -> "
          f"{'CLEAN' if m1_ok else 'HOST-CENS'}")

    # ---- G4 barrier-share vs A15 2.03%
    wb = wall_barrier(d)
    if wb:
        print(f"G4 barrier: {wb['barrier_pct']}% thread-wall "
              f"(RegionTickOps-сайт {wb['barrier_rt_pct']}%, {wb['barrier_samples']}/"
              f"{wb['total_samples']} сэмплов) vs A15-base {A15_BARRIER_PCT}% "
              f"→ Δ {wb['barrier_pct']-A15_BARRIER_PCT:+.3f}пп")
    else:
        print("G4 barrier: wall-collapsed отсутствует/пуст — N/A")

    # ---- G5 pair math
    cpu = int(RE_IDX.search(env).group(1)) if RE_IDX.search(env) else 0
    med = statistics.median(soak_polls) if soak_polls else 0.0
    exp, tag = tps_exp_v5(cpu)
    norm = 100*(med/exp - 1) if med else float("nan")
    in_band = BAND[0] <= cpu <= BAND[1]
    pair = norm >= PAIR_BAR
    print(f"G5 norm: cpu {cpu} in_band={in_band} polls-med {med} exp {exp:.4f}({tag}) "
          f"norm_v5 {norm:+.2f} → pair {'≥+20 LEGAL' if pair and in_band and g2_ok and m1_ok else 'нет (honest)'}")

    out = {"run_id": rid, "branch": st.get("head_branch"),
           "head_sha": st.get("head_sha"), "cpu_index": cpu, "in_band": in_band,
           "G1_delivery": g1, "G1_ok": g1_ok, "G2_npe_free": g2_ok,
           "npe_marks": npe, "aioobe_marks": aioobe, "ncdfe": ncdfe,
           "boot_done": boot_done, "G3_M1": stw, "m1_ok": m1_ok,
           "G4_barrier": wb, "a15_barrier_pct": A15_BARRIER_PCT,
           "polls_median": med, "tps_exp": exp, "norm_v5": norm,
           "pair_legal": bool(pair and in_band and g2_ok and m1_ok)}
    with open(args.json, "w") as f:
        json.dump(out, f, indent=1)
    print("verdict-json:", args.json)


if __name__ == "__main__":
    main()
