#!/usr/bin/env python3
"""absorb_482_c75_fresh.py — COMMANDER C75 absorb: fresh-gen контроль × new-master.

Run 36431208887 @round-482-c75-fresh (master full sha 3666a79317…, МЕРЖ №19).
Мир: fresh-gen фикстура 9b3a3f09 (world479-f2-stz-v1.zip, 19861 B, level.dat +
ПУСТОЙ region + dp-драйвер forceload duty 0.2). Канон-парсер = bank v5
(наследие absorb_482_main.py: norm_v5 interp по V5-сетке, gc.log deep-parse).

ПРЕГИСТ (C93-класс, пороги v5-FROZEN):
  M1: stw_total ≤23.0s ∧ young_avg ≤200ms
  P1: young_n 200-280 (банк pregen 109-123) ∧ avg 40-80ms ∧ young-масса [12.5,16.5]s
  P2: stw_total ∈ [18,23]s
  P3: norm ВНЕ коридора [−8,+1.5] сверху (+2..+6) → НЕ банк-фид (сцена-конфаунд C94)
  P4: worldgen/noise тик-сэмплы 0.0000% (gen off-tick)
"""
import bisect, json, os, re, shutil, statistics, subprocess, sys, time, zipfile

REPO = "PLANETA9091/c-crussty"
OUT = "/home/z/rounds/ROUND-482/c75_fresh"
RUN_ID = 36431208887
BRANCH = "round-482-c75-fresh"
PIN = "3666a7931703e24a36af4887c41a585918667b7a"
WORLD_FRESH = "9b3a3f09"
V5 = [(6.5, 2.1252), (7.0, 2.2047), (7.5, 2.3271), (8.2, 2.4732), (8.7, 2.5981), (9.0, 2.6280)]
STRICT_LO, STRICT_HI = 6.9e6, 7.2e6
BAND = (6.0e6, 9.5e6)
STW_LIMIT_S, YOUNG_AVG_LIMIT_MS = 23.0, 200.0
CORRIDOR = (-8.0, 1.5)

RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")
RE_GEN = re.compile(r"net/minecraft/world/level/levelgen|NoiseChunk|ChunkStatus|Worldgen")


def tok():
    return open("/tmp/gh_token").read().strip()


def api(t, path):
    p = subprocess.run(["curl", "-s", "-H", f"Authorization: token {t}",
                        "-H", "Accept: application/vnd.github+json",
                        f"https://api.github.com/repos/{REPO}/{path}"],
                       capture_output=True, timeout=90)
    return json.loads(p.stdout.decode() or "{}")


def wait_run(t, max_min=70):
    t0 = time.time()
    while time.time() - t0 < max_min * 60:
        r = api(t, f"/actions/runs/{RUN_ID}")
        st, cc = r.get("status"), r.get("conclusion")
        print(f"[{int(time.time()-t0)}s] status={st} conclusion={cc}", flush=True)
        if st == "completed":
            return cc
        time.sleep(120)
    raise SystemExit("TIMEOUT wait")


def fetch_artifact(t):
    arts = api(t, f"/actions/runs/{RUN_ID}/artifacts")["artifacts"]
    a = next((x for x in arts if x["name"] == "world3-bench"), None)
    if not a:
        raise SystemExit("no world3-bench artifact")
    zpath = f"{OUT}/a.zip"
    url = f"https://api.github.com/repos/{REPO}/actions/artifacts/{a['id']}/zip"
    for _ in range(3):
        p = subprocess.run(["curl", "-sL", "-H", f"Authorization: token {t}", url,
                            "-o", zpath], capture_output=True, timeout=900)
        if p.returncode == 0 and os.path.exists(zpath) and open(zpath, "rb").read(2) == b"PK":
            break
        time.sleep(5)
    d = os.path.join(OUT, "art")
    os.makedirs(d, exist_ok=True)
    with zipfile.ZipFile(zpath) as z:
        for m in ("run-env.txt", "server-stdout.log", "gc.log", "cpu-collapsed.txt",
                  "BOTTLENECKS_3.md"):
            names = [n for n in z.namelist() if n.endswith(m)]
            if names:
                with z.open(names[0]) as src, open(os.path.join(d, m), "wb") as dst:
                    shutil.copyfileobj(src, dst)
    os.remove(zpath)
    return d


def parse_gc(text):
    pauses = []
    for line in text.splitlines():
        if "Pause" not in line or "[gc,phases" in line:
            continue
        m = PAUSE_COMPL.search(line)
        if not m:
            continue
        um, cm = UPTIME.search(line), CAUSE.search(line)
        pauses.append((float(m.group(1)), cm.group(1) if cm else "?",
                       float(um.group(1)) if um else 0.0))
    young = [p for p in pauses if p[1] == "Young"]
    fulls = [p for p in pauses if p[1] == "Full"]
    return {"stw_total_s": round(sum(p[0] for p in pauses)/1000, 2),
            "young_n": len(young),
            "young_avg_ms": round(sum(p[0] for p in young)/len(young), 2) if young else 0.0,
            "young_mass_s": round(sum(p[0] for p in young)/1000, 2),
            "fulls": len(fulls),
            "full_mass_s": round(sum(p[0] for p in fulls)/1000, 2),
            "max_ms": round(max((p[0] for p in pauses), default=0.0), 1)}


def tps_exp_v5(cpu):
    xs = [p[0] for p in V5]; ys = [p[1] for p in V5]; c = cpu / 1e6
    if c <= xs[0]:
        return ys[0] + (ys[1]-ys[0])/(xs[1]-xs[0])*(c-xs[0]), "extrap-low"
    if c >= xs[-1]:
        return ys[-1] + (ys[-1]-ys[-2])/(xs[-1]-xs[-2])*(c-xs[-1]), "extrap-high"
    i = bisect.bisect_left(xs, c)
    return ys[i-1] + (ys[i]-ys[i-1])/(xs[i]-xs[i-1])*(c-xs[i-1]), "interp"


def main():
    os.makedirs(OUT, exist_ok=True)
    t = tok()
    cc = wait_run(t)
    print(f"conclusion={cc}")
    d = fetch_artifact(t)
    env = open(os.path.join(d, "run-env.txt"), errors="replace").read()
    log = open(os.path.join(d, "server-stdout.log"), errors="replace").read()
    gct = open(os.path.join(d, "gc.log"), errors="replace").read()
    cpu_m = re.search(r"runner_cpu_index:\s*(\d+)", env)
    cpu = int(cpu_m.group(1)) if cpu_m else None
    world_m = re.search(r"world_sha256:\s*([0-9a-f]{8})", env)
    world = world_m.group(1) if world_m else None
    tps_all = [float(x) for x in RE_TPS.findall(log)]
    bench = [x for x in tps_all if x < 15]
    med = statistics.median(bench[:5]) if bench else None
    g = parse_gc(gct)
    e, tag = (tps_exp_v5(cpu) if cpu else (None, "no-cpu"))
    norm = round(100*(med/e - 1), 2) if (med and e) else None
    # P4: worldgen/noise tick-samples share of cpu-collapsed
    gen_pct = None
    cp = os.path.join(d, "cpu-collapsed.txt")
    if os.path.exists(cp):
        tot, gen = 0, 0
        for line in open(cp, errors="replace"):
            try:
                stack, n = line.rsplit(" ", 1)
                n = int(n)
            except ValueError:
                continue
            tot += n
            if RE_GEN.search(stack):
                gen += n
        gen_pct = round(100*gen/tot, 4) if tot else None
    inject = re.search(r"POPULATION INJECT DONE.*elapsedMs=(\d+)", log)
    v = {
        "run": RUN_ID, "branch": BRANCH, "pin": PIN, "conclusion": cc,
        "world_sha256": world, "world_is_fresh": world == WORLD_FRESH,
        "cpu": cpu, "band": bool(cpu and BAND[0] <= cpu <= BAND[1]),
        "strict": bool(cpu and STRICT_LO <= cpu <= STRICT_HI), "exp_tag": tag,
        "exp": round(e, 5) if e else None, "polls5": bench[:5], "n_polls": len(bench),
        "med": med, "norm_v5": norm,
        "corridor": bool(norm is not None and CORRIDOR[0] <= norm <= CORRIDOR[1]),
        "ncdfe": log.count("NoClassDefFoundError"),
        "aioobe": log.count("ArrayIndexOutOfBounds"),
        "inject_elapsedMs": int(inject.group(1)) if inject else None,
        "gen_tick_pct": gen_pct,
        **g,
    }
    v["M1"] = bool(v["stw_total_s"] <= STW_LIMIT_S and v["young_avg_ms"] <= YOUNG_AVG_LIMIT_MS)
    v["P1_morph"] = bool(200 <= v["young_n"] <= 280 and 40 <= v["young_avg_ms"] <= 80)
    v["P1_mass"] = bool(12.5 <= v["young_mass_s"] <= 16.5)
    v["P2"] = bool(18 <= v["stw_total_s"] <= 23)
    v["P3_breach"] = bool(norm is not None and norm > CORRIDOR[1])
    v["P4_gen_offtick"] = bool(gen_pct is not None and gen_pct < 0.01)
    verdict = "HOLD" if (v["M1"] and v["P1_morph"] and v["P1_mass"] and v["P2"]
                         and v["P3_breach"] and v["P4_gen_offtick"]) else \
              ("HOLD-PARTIAL" if v["M1"] else "DIRTY")
    v["verdict"] = verdict
    out = os.path.join(OUT, "verdict_482_c75.json")
    json.dump(v, open(out, "w"), indent=1, ensure_ascii=False)
    print(json.dumps(v, indent=1, ensure_ascii=False))
    print(f"verdict -> {out}")


if __name__ == "__main__":
    main()
