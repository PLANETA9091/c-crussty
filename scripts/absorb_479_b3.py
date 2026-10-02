#!/usr/bin/env python3
"""absorb_479_b3.py — [479-B3] collision E2 dedup pair-class re-roll absorb.

RUNS: arm 36376926391 (@b24f4876, lever cmp401_collide; attempt-1 36376791815
band-dead fast-fail 04:13:29Z → W3 free re-roll ×1 израсходован) /
van 36376817322 (@b3e2f4f4, lever ∅). Прегист-гейты G1-G6 в
scripts/dispatch_479_b3_pair.py (закон 16, до диспатча).
"""
import json, os, re, shutil, statistics, subprocess, sys, time, zipfile, bisect

REPO = "PLANETA9091/c-crussty"
RUNS = {"arm": (36376926391, "round-479-b3-arm"),
        "van": (36376817322, "round-479-b3-van")}
V5 = [(6.5, 2.1252), (7.0, 2.2047), (7.5, 2.3271), (8.2, 2.4732), (8.7, 2.5981), (9.0, 2.6280)]
LOCAL_70, LOCAL_LO, LOCAL_HI = 2.1293, 6.9e6, 7.2e6
BAND = (6.0e6, 9.5e6)
STW_LIMIT_S, YOUNG_AVG_LIMIT_MS = 23.0, 200.0
LEG_LEVER = "cmp401_collide"
OUT = "/home/z/rounds/ROUND-479/B3"

RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
RE_ARMED = re.compile(r"(cmp[0-9a-z_]+): ARMED")
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")


def tok():
    return open("/tmp/gh_token").read().strip()


def api(t, path):
    path = path.lstrip("/")  # fix: двойной слэш в URL → 404 (B3-фикс, поведение API не меняет)
    p = subprocess.run(["curl", "-s", "-H", f"Authorization: token {t}",
                        "-H", "Accept: application/vnd.github+json",
                        f"https://api.github.com/repos/{REPO}/{path}"],
                       capture_output=True, timeout=90)
    return json.loads(p.stdout.decode() or "{}")


def run_status(t, rid):
    r = api(t, f"/actions/runs/{rid}")
    return r.get("status"), r.get("conclusion")


def tps_exp_v5(cpu):
    if LOCAL_LO <= cpu <= LOCAL_HI:
        return LOCAL_70, "Л201-local"
    xs = [p[0] for p in V5]; ys = [p[1] for p in V5]; c = cpu / 1e6
    if c <= xs[0]:
        return ys[0] + (ys[1]-ys[0])/(xs[1]-xs[0])*(c-xs[0]), "extrap-low"
    if c >= xs[-1]:
        return ys[-1] + (ys[-1]-ys[-2])/(xs[-1]-xs[-2])*(c-xs[-1]), "extrap-high"
    i = bisect.bisect_left(xs, c)
    return ys[i-1] + (ys[i]-ys[i-1])/(xs[i]-xs[i-1])*(c-xs[i-1]), "interp"


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
            "fulls": len(fulls),
            "max_ms": round(max((p[0] for p in pauses), default=0.0), 1)}


def fetch_artifact_members(t, rid, tag):
    arts = api(t, f"/actions/runs/{rid}/artifacts")["artifacts"]
    a = next((x for x in arts if x["name"] == "world3-bench"), None)
    if not a:
        return None, "no world3-bench artifact"
    zpath = f"{OUT}/{tag}_a.zip"
    url = f"https://api.github.com/repos/{REPO}/actions/artifacts/{a['id']}/zip"
    p = subprocess.run(["curl", "-sL", "-H", f"Authorization: token {t}", url, "-o", zpath],
                       capture_output=True, timeout=600)
    if p.returncode != 0 or open(zpath, "rb").read(2) != b"PK":
        return None, "download/PK fail"
    d = f"{OUT}/{tag}"
    os.makedirs(d, exist_ok=True)
    with zipfile.ZipFile(zpath) as z:
        for m in ("run-env.txt", "server-stdout.log", "gc.log"):
            names = [n for n in z.namelist() if n.endswith(m)]
            if not names:
                return None, f"missing {m}"
            with z.open(names[0]) as src, open(os.path.join(d, m), "wb") as dst:
                shutil.copyfileobj(src, dst)
    os.remove(zpath)
    return d, ""


def norm_of(d):
    env = open(os.path.join(d, "run-env.txt"), errors="replace").read()
    log = open(os.path.join(d, "server-stdout.log"), errors="replace").read()
    # B3-фикс: world-маркер живёт в run-env.txt (world_sha256: afb3a0b3...), stdout его не содержит
    gct = open(os.path.join(d, "gc.log"), errors="replace").read()
    cpu = int(re.search(r"runner_cpu_index:\s*(\d+)", env).group(1))
    tps_all = [float(x) for x in RE_TPS.findall(log)]
    bench = [x for x in tps_all if x < 15]
    first5 = bench[:5]
    med = statistics.median(first5) if first5 else None
    med_all = statistics.median(bench) if bench else None
    armed = sorted(set(RE_ARMED.findall(log)))
    g = parse_gc(gct)
    e, tag = tps_exp_v5(cpu)
    v = {"cpu": cpu, "polls5": first5, "med": med, "med_all": med_all,
         "exp": round(e, 5), "exp_tag": tag,
         "norm": round(100*(med/e - 1), 2) if med else None,
         "norm_all": round(100*(med_all/e - 1), 2) if med_all else None,
         "armed": armed, "ncdfe": log.count("NoClassDefFoundError"),
         "aioobe": log.count("ArrayIndexOutOfBounds"),
         "world": (re.search(r"world_sha256:\s*([0-9a-f]{8})", env).group(1)
                   if re.search(r"world_sha256:\s*([0-9a-f]{8})", env) else
                   (re.search(r"world sha[: ]*([0-9a-f]{8})", log).group(1)
                    if re.search(r"world sha[: ]*([0-9a-f]{8})", log) else None)),
         "tick_behind": (lambda m: int(m.group(1)) if m else None)(
             re.search(r"tick[- ]behind[: ]*(\d+)", log)),
         **g}
    v["m1"] = v["stw_total_s"] <= STW_LIMIT_S and v["young_avg_ms"] <= YOUNG_AVG_LIMIT_MS
    v["band"] = BAND[0] <= cpu <= BAND[1]
    return v


def main():
    os.makedirs(OUT, exist_ok=True)
    t = tok()
    res, deadline = {}, time.time() + (int(sys.argv[1]) if len(sys.argv) > 1 else 1500)
    while time.time() < deadline and len(res) < len(RUNS):
        for tag, (rid, br) in RUNS.items():
            if tag in res:
                continue
            st, cc = run_status(t, rid)
            if st == "completed":
                print(f"{tag} {rid} completed: {cc}", flush=True)
                if cc != "success":
                    res[tag] = {"run": rid, "conclusion": cc}
                    continue
                d, err = fetch_artifact_members(t, rid, tag)
                if d:
                    v = norm_of(d); v.update(run=rid, branch=br, conclusion=cc)
                    res[tag] = v
                    print(f"{tag}: cpu={v['cpu']} polls={v['polls5']} med={v['med']} "
                          f"exp={v['exp']} norm={v['norm']} (all {v['norm_all']}) "
                          f"STW={v['stw_total_s']}s young={v['young_avg_ms']}ms fulls={v['fulls']} "
                          f"armed={v['armed']} m1={v['m1']} band={v['band']}", flush=True)
                else:
                    res[tag] = {"run": rid, "conclusion": cc, "err": err}
        if len(res) < len(RUNS):
            time.sleep(60)
    # вердикт-математика (пегист G1-G6, закон 16)
    if "arm" in res and "van" in res and "norm" in res.get("arm", {}) and "norm" in res.get("van", {}):
        a, v = res["arm"], res["van"]
        pair = round(a["norm"] - v["norm"], 2)
        dcpu = abs(a["cpu"] - v["cpu"])
        validity = (a["ncdfe"] == 0 and a["aioobe"] == 0 and v["ncdfe"] == 0 and v["aioobe"] == 0
                    and a["world"] == "afb3a0b3" and v["world"] == "afb3a0b3"
                    and (a["pop"] if "pop" in a else True))
        arms_ok = LEG_LEVER in a["armed"] and v["armed"] == []
        gates = {"band_both": a["band"] and v["band"], "m1_both": a["m1"] and v["m1"],
                 "validity": validity, "arm_proof": arms_ok,
                 "dcpu_le_50k": dcpu <= 50000}
        gates["pair_ge_20"] = pair >= 20.0
        verdict = "PAIR" if all(gates.values()) else "REFUTED_CENS"
        res["_verdict"] = {"pair": pair, "dcpu": dcpu, "gates": gates, "verdict": verdict,
                           "dedup_merge_gate_ge2": pair >= 2.0,
                           "legA_corridor_-8..+3": -8.0 <= a["norm"] <= 3.0,
                           "min_of_basis": "2 (банк-гейт min-of-3 — 3-й якорь след. тик)"}
        print(json.dumps(res["_verdict"], indent=1, ensure_ascii=False), flush=True)
    with open(f"{OUT}/absorb_479_b3.json", "w") as f:
        json.dump(res, f, indent=1, ensure_ascii=False)
    print(f"saved {OUT}/absorb_479_b3.json", flush=True)


if __name__ == "__main__":
    main()
