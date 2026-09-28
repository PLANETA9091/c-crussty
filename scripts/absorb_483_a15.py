#!/usr/bin/env python3
"""absorb_483_a15.py — COMMANDER A15 ROUND-483: абсорб STRICT-волны ×2 + pair-фид ×2.

Канон-парсер = банк v5 (наследие absorb_483_a03 / absorb_482_main). Норм =
100*(med5/exp − 1), exp = V5-интерполяция по runner_cpu_index. STRICT =
ПОСТ-НОРМ классификация normtool (НЕ input): cpu ∈ [6.9M, 7.2M] ∧ M1 ∧
vanilla_valid ∧ corridor → STRICT-IN-POINT(банк-фид); иначе при M1+valid+corridor
→ VANILLA-VALID-§3(банк-фид) = pair-фид. Pair = leg_norm − anchor_norm (canon
bc1 vanilla @master acffa383, окно тика, min-of-3). M1: STW<=23s, young_avg<=200ms.
"""
import json, os, re, shutil, statistics, subprocess, sys, zipfile, bisect

REPO = "PLANETA9091/c-crussty"
OUT = "/home/z/rounds/ROUND-483/absorb_a15"
V5 = [(6.5, 2.1252), (7.0, 2.2047), (7.5, 2.3271), (8.2, 2.4732), (8.7, 2.5981), (9.0, 2.6280)]
STRICT_LO, STRICT_HI = 6.9e6, 7.2e6
BAND = (6.0e6, 9.5e6)
STW_LIMIT_S, YOUNG_AVG_LIMIT_MS = 23.0, 200.0
CORRIDOR = (-8.0, 1.5)
WINDOW_SINCE = "2026-09-28T15:00:00Z"
MY_RUNS = {36445134406, 36444960284, 36444994999, 36445030000, 36444924600}

LEGS = [
    ("round-483-a15-s1-r2", 36445134406),   # s1 оригинал 36444924600 = BAND-MISS fast-fail → ре-ролл 1
    ("round-483-a15-s2",    36444960284),
    ("round-483-a15-s3",    36444994999),
    ("round-483-a15-s4",    36445030000),
]

RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
RE_ARMED = re.compile(r"(cmp[0-9a-z_]+): ARMED")
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")


def tok():
    return open("/tmp/gh_token").read().strip()


def api(t, path):
    path = path.lstrip("/")
    for attempt in range(3):
        p = subprocess.run(["curl", "-s", "-H", f"Authorization: Bearer {t}",
                            "-H", "Accept: application/vnd.github+json",
                            f"https://api.github.com/repos/{REPO}/{path}"],
                           capture_output=True, timeout=90)
        try:
            d = json.loads(p.stdout.decode() or "{}")
        except json.JSONDecodeError:
            d = {}
        if "message" not in d or "artifacts" in d or "workflow_runs" in d:
            return d
        print(f"api({path}) attempt {attempt}: {d.get('message')}", file=sys.stderr)
        import time as _t
        _t.sleep(5)
    print(f"api({path}) -> {p.stdout[:200]!r}", file=sys.stderr)
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
            "fulls": len(fulls),
            "max_ms": round(max((p[0] for p in pauses), default=0.0), 1)}


def tps_exp_v5(cpu):
    xs = [p[0] for p in V5]; ys = [p[1] for p in V5]; c = cpu / 1e6
    if c <= xs[0]:
        return ys[0] + (ys[1]-ys[0])/(xs[1]-xs[0])*(c-xs[0]), "extrap-low"
    if c >= xs[-1]:
        return ys[-1] + (ys[-1]-ys[-2])/(xs[-1]-xs[-2])*(c-xs[-1]), "extrap-high"
    i = bisect.bisect_left(xs, c)
    return ys[i-1] + (ys[i]-ys[i-1])/(xs[i]-xs[i-1])*(c-xs[i-1]), "interp"


def norm_of(d):
    env = open(os.path.join(d, "run-env.txt"), errors="replace").read()
    log = open(os.path.join(d, "server-stdout.log"), errors="replace").read()
    gct = open(os.path.join(d, "gc.log"), errors="replace").read()
    mc = re.search(r"runner_cpu_index:\s*(\d+)", env)
    cpu = int(mc.group(1)) if mc else None
    tps_all = [float(x) for x in RE_TPS.findall(log)]
    bench = [x for x in tps_all if x < 15]
    first5 = bench[:5]
    med = statistics.median(first5) if first5 else None
    armed = sorted(set(RE_ARMED.findall(log)))
    g = parse_gc(gct)
    wm = re.search(r"world_sha256:\s*([0-9a-f]{8})", env)
    world = wm.group(1) if wm else None
    e, tag = (tps_exp_v5(cpu) if cpu else (None, "no-cpu"))
    v = {"cpu": cpu, "polls5": first5, "n_polls": len(bench), "med": med,
         "exp": round(e, 5) if e else None, "exp_tag": tag,
         "norm": round(100*(med/e - 1), 2) if (med and e) else None,
         "armed": armed, "ncdfe": log.count("NoClassDefFoundError"),
         "aioobe": log.count("ArrayIndexOutOfBounds"), "world": world, **g}
    v["m1"] = (v["stw_total_s"] <= STW_LIMIT_S and v["young_avg_ms"] <= YOUNG_AVG_LIMIT_MS)
    v["band"] = bool(cpu and BAND[0] <= cpu <= BAND[1])
    v["strict"] = bool(cpu and STRICT_LO <= cpu <= STRICT_HI)
    v["corridor"] = bool(v["norm"] is not None and CORRIDOR[0] <= v["norm"] <= CORRIDOR[1])
    v["vanilla_valid"] = bool(cpu and armed == [] and v["ncdfe"] == 0
                              and v["aioobe"] == 0 and world == "afb3a0b3")
    if not v["band"]:
        v["class"] = "BAND-DEAD(free)"
    elif v["strict"] and v["m1"] and v["vanilla_valid"] and v["corridor"]:
        v["class"] = "STRICT-IN-POINT(банк-фид)"
    elif v["m1"] and v["vanilla_valid"] and v["corridor"]:
        v["class"] = "VANILLA-VALID-§3(банк-фид)"
    elif not v["m1"]:
        v["class"] = "HOST-CENS(M1)"
    elif not v["corridor"]:
        v["class"] = "CORRIDOR-BREACH(инфра-ценз)"
    elif armed:
        v["class"] = "ARMED(leg/безумие)"
    else:
        v["class"] = "UNCLASS"
    return v


def process(t, rid, branch):
    tag = f"r{rid}"
    dd = os.path.join(OUT, tag)
    if os.path.exists(os.path.join(dd, "run-env.txt")):
        return tag, norm_of(dd)
    arts = api(t, f"/actions/runs/{rid}/artifacts")["artifacts"]
    a = next((x for x in arts if x["name"] == "world3-bench"), None)
    if not a:
        return tag, {"run": rid, "err": "no artifact", "branch": branch}
    zpath = f"{OUT}/{tag}_a.zip"
    url = f"https://api.github.com/repos/{REPO}/actions/artifacts/{a['id']}/zip"
    p = subprocess.run(["curl", "-sL", "-H", f"Authorization: token {t}", url, "-o", zpath],
                       capture_output=True, timeout=900)
    if p.returncode != 0 or not os.path.exists(zpath) or open(zpath, "rb").read(2) != b"PK":
        return tag, {"run": rid, "err": "download fail", "branch": branch}
    d = dd
    os.makedirs(d, exist_ok=True)
    try:
        with zipfile.ZipFile(zpath) as z:
            for m in ("run-env.txt", "server-stdout.log", "gc.log"):
                names = [n for n in z.namelist() if n.endswith(m)]
                if not names:
                    return tag, {"run": rid, "err": f"missing {m}", "branch": branch}
                with z.open(names[0]) as src, open(os.path.join(d, m), "wb") as dst:
                    shutil.copyfileobj(src, dst)
    finally:
        os.remove(zpath)
    v = norm_of(d)
    v.update(run=rid, tag=tag, branch=branch)
    return tag, v


ENV_CANON = {"BATCH_COLLECTOR": "1", "SERVER_XMX": "10G", "SERVER_XMS": "4G",
             "REGION_THREADS": "4", "FAKE_PLAYERS": "4"}


def canon_ok(env_text):
    for k, want in ENV_CANON.items():
        m = re.search(rf"{k}[:=](\S+)", env_text)
        if m and m.group(1).strip('"') != want:
            return False, k
    return True, None


def main():
    os.makedirs(OUT, exist_ok=True)
    t = tok()
    res = {}
    for br, rid in LEGS:
        tag, v = process(t, rid, br)
        res[br] = v
        print(f"{br} run {rid}: norm={v.get('norm')} cpu={v.get('cpu')} "
              f"med={v.get('med')} strict={v.get('strict')} m1={v.get('m1')} "
              f"class={v.get('class', v.get('err'))}", flush=True)
    # анкеры: canon bc1 vanilla @master acffa383 в окне тика (pair-база)
    runs, page = [], 1
    while page <= 6:
        d = api(t, f"/actions/runs?event=workflow_dispatch&per_page=100&page={page}")
        rs = d.get("workflow_runs", [])
        if not rs:
            break
        runs.extend(rs)
        if len(runs) >= d.get("total_count", 0):
            break
        page += 1
    cand = [r for r in runs if r["created_at"] >= WINDOW_SINCE
            and r["conclusion"] == "success"
            and r["head_sha"].startswith("acffa383")
            and r["id"] not in MY_RUNS]
    print(f"anchor-candidates on acffa383 since {WINDOW_SINCE}: "
          f"{[(r['head_branch'], r['id']) for r in cand]}", flush=True)
    anchors = {}
    for r in cand[:14]:
        tag, v = process(t, r["id"], r["head_branch"])
        if "err" in v:
            anchors[r["id"]] = v
            continue
        env = open(os.path.join(OUT, tag, "run-env.txt"), errors="replace").read()
        ok, bad = canon_ok(env)
        v["canon_ok"], v["canon_bad"] = ok, bad
        anchors[r["id"]] = v
        print(f"anchor {r['head_branch']} run {r['id']}: canon={ok}"
              f"{'' if ok else f'({bad})'} norm={v.get('norm')} cpu={v.get('cpu')} "
              f"class={v.get('class')}", flush=True)
    good = [v for v in anchors.values()
            if v.get("canon_ok") and v.get("norm") is not None
            and v.get("m1") and v.get("vanilla_valid")]
    anchor_norm = None
    if good:
        anchor_norm = statistics.median([v["norm"] for v in good])
        print(f"ANCHOR canon-bc1 median norm = {anchor_norm} (n={len(good)})")
        for br, v in res.items():
            if v.get("norm") is not None:
                print(f"PAIR {br}: {v['norm']} − {anchor_norm} = {round(v['norm'] - anchor_norm, 2)}")
    json.dump({"legs": res, "anchors": anchors, "anchor_norm": anchor_norm},
              open(f"{OUT}/absorb_a15.json", "w"), indent=1)
    print("A15-ABSORB-JSON " + json.dumps(
        {"legs": {k: {kk: vv for kk, vv in v.items() if kk != "polls5"}
                  for k, v in res.items()},
         "anchor_n": len(good), "anchor_norm": anchor_norm}))


if __name__ == "__main__":
    main()
