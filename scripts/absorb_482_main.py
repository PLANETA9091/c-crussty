#!/usr/bin/env python3
"""absorb_482_main.py — ROUND-482 mass absorb (закон 21: абсорб in-flight).

Окно: раны созданные >= 2026-09-28T09:40Z (burst 73 + canary-post 36411459754
+ c29/c35/c36/c38 + C03×2 + dp1 + 19 in_progress на момент старта тика).
Авто-обнаружение по API (все страницы), обработка только completed-success
с артефактом world3-bench. Канон-парсер = bank v5 (наследие absorb_480_main).
"""
import json, os, re, shutil, statistics, subprocess, sys, time, zipfile, bisect
from concurrent.futures import ThreadPoolExecutor, as_completed

REPO = "PLANETA9091/c-crussty"
OUT = "/home/z/rounds/ROUND-482/absorb"
V5 = [(6.5, 2.1252), (7.0, 2.2047), (7.5, 2.3271), (8.2, 2.4732), (8.7, 2.5981), (9.0, 2.6280)]
STRICT_LO, STRICT_HI = 6.9e6, 7.2e6
BAND = (6.0e6, 9.5e6)
STW_LIMIT_S, YOUNG_AVG_LIMIT_MS = 23.0, 200.0
CORRIDOR = (-8.0, 1.5)
WINDOW_SINCE = "2026-09-28T09:40:00Z"  # burst-73 стартовал ~09:43Z

RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
RE_ARMED = re.compile(r"(cmp[0-9a-z_]+): ARMED")
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")


def tok():
    return open("/tmp/gh_token").read().strip()


def api(t, path):
    path = path.lstrip("/")
    p = subprocess.run(["curl", "-s", "-H", f"Authorization: token {t}",
                        "-H", "Accept: application/vnd.github+json",
                        f"https://api.github.com/repos/{REPO}/{path}"],
                       capture_output=True, timeout=90)
    return json.loads(p.stdout.decode() or "{}")


def discover(t):
    """Все bench-раны окна (все страницы)."""
    runs, page = [], 1
    while page <= 6:
        d = api(t, f"/actions/runs?per_page=100&page={page}")
        rs = d.get("workflow_runs", [])
        if not rs:
            break
        runs.extend(rs)
        if len(runs) >= d.get("total_count", 0):
            break
        page += 1
    win = []
    for r in runs:
        if r["created_at"] >= WINDOW_SINCE and r["name"] in ("world-bench-round", "world-bench-parallel", "ci"):
            win.append((r["id"], r["name"], r["head_branch"], r["created_at"]))
    return runs, win


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
    med_all = statistics.median(bench) if bench else None
    armed = sorted(set(RE_ARMED.findall(log)))
    g = parse_gc(gct)
    wm = re.search(r"world_sha256:\s*([0-9a-f]{8})", env)
    world = wm.group(1) if wm else None
    e, tag = (tps_exp_v5(cpu) if cpu else (None, "no-cpu"))
    v = {"cpu": cpu, "polls5": first5, "n_polls": len(bench), "med": med, "med_all": med_all,
         "exp": round(e, 5) if e else None, "exp_tag": tag,
         "norm": round(100*(med/e - 1), 2) if (med and e) else None,
         "armed": armed, "ncdfe": log.count("NoClassDefFoundError"),
         "aioobe": log.count("ArrayIndexOutOfBounds"), "world": world, **g}
    v["m1"] = (v["stw_total_s"] <= STW_LIMIT_S and v["young_avg_ms"] <= YOUNG_AVG_LIMIT_MS)
    v["band"] = bool(cpu and BAND[0] <= cpu <= BAND[1])
    v["strict"] = bool(cpu and STRICT_LO <= cpu <= STRICT_HI)
    v["corridor"] = bool(v["norm"] is not None and CORRIDOR[0] <= v["norm"] <= CORRIDOR[1])
    if cpu:
        v["vanilla_valid"] = (armed == [] and v["ncdfe"] == 0 and v["aioobe"] == 0
                              and world == "afb3a0b3")
    else:
        v["vanilla_valid"] = False
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


def process(t, rid, name, branch, created):
    tag = f"r{rid}"
    r = api(t, f"/actions/runs/{rid}")
    st, cc = r.get("status"), r.get("conclusion")
    if st != "completed":
        return tag, {"run": rid, "wave": "WINDOW", "status": st, "branch": branch, "created": created}
    if cc != "success":
        return tag, {"run": rid, "wave": "WINDOW", "conclusion": cc,
                     "branch": branch, "created": created}
    arts = api(t, f"/actions/runs/{rid}/artifacts")["artifacts"]
    a = next((x for x in arts if x["name"] == "world3-bench"), None)
    if not a:
        return tag, {"run": rid, "wave": "WINDOW", "conclusion": cc, "err": "no artifact",
                     "branch": branch, "created": created}
    zpath = f"{OUT}/{tag}_a.zip"
    url = f"https://api.github.com/repos/{REPO}/actions/artifacts/{a['id']}/zip"
    for attempt in range(2):
        p = subprocess.run(["curl", "-sL", "-H", f"Authorization: token {t}", url, "-o", zpath],
                           capture_output=True, timeout=900)
        if p.returncode == 0 and os.path.exists(zpath) and open(zpath, "rb").read(2) == b"PK":
            break
        time.sleep(5)
    else:
        return tag, {"run": rid, "wave": "WINDOW", "err": "download fail",
                     "branch": branch, "created": created}
    d = f"{OUT}/{tag}"
    os.makedirs(d, exist_ok=True)
    try:
        with zipfile.ZipFile(zpath) as z:
            for m in ("run-env.txt", "server-stdout.log", "gc.log"):
                names = [n for n in z.namelist() if n.endswith(m)]
                if not names:
                    return tag, {"run": rid, "wave": "WINDOW", "err": f"missing {m}",
                                 "branch": branch, "created": created}
                with z.open(names[0]) as src, open(os.path.join(d, m), "wb") as dst:
                    shutil.copyfileobj(src, dst)
    finally:
        os.remove(zpath)
    try:
        v = norm_of(d)
    except Exception as ex:
        return tag, {"run": rid, "wave": "WINDOW", "err": f"parse: {ex}",
                     "branch": branch, "created": created}
    v.update(run=rid, wave="WINDOW", tag=tag, branch=branch, created=created, rname=name)
    so = os.path.join(d, "server-stdout.log")
    if os.path.exists(so):
        os.remove(so)
    return tag, v


def main():
    os.makedirs(OUT, exist_ok=True)
    t = tok()
    _, win = discover(t)
    print(f"WINDOW runs: {len(win)}", flush=True)
    json.dump(win, open(f"{OUT}/window.json", "w"), indent=1)
    res = {}
    print(f"ABSORB-482: {len(win)} runs", flush=True)
    with ThreadPoolExecutor(max_workers=6) as ex:
        futs = {ex.submit(process, t, rid, nm, br, cr): rid for rid, nm, br, cr in win}
        done = 0
        for fut in as_completed(futs):
            rid = futs[fut]
            try:
                _, v = fut.result()
            except Exception as ex:
                v = {"run": rid, "err": str(ex)[:200]}
            res[v.get("tag", f"r{rid}")] = v
            done += 1
            brief = v.get("class") or v.get("conclusion") or v.get("status") or v.get("err", "?")
            print(f"[{done}/{len(win)}] {rid}: {brief} "
                  f"cpu={v.get('cpu')} norm={v.get('norm')} m1={v.get('m1')}", flush=True)
    acc_path = f"{OUT}/absorb_482.json"
    acc = {}
    if os.path.exists(acc_path):
        try:
            acc = json.load(open(acc_path))
        except Exception:
            acc = {}
    acc.update(res)
    res = acc
    with open(acc_path, "w") as f:
        json.dump(res, f, indent=1, ensure_ascii=False)
    cls = {}
    for v in res.values():
        c = v.get("class") or v.get("conclusion") or v.get("status") or "ERR"
        cls[c] = cls.get(c, 0) + 1
    print("SUMMARY:", json.dumps(cls, ensure_ascii=False), flush=True)
    print(f"saved {acc_path}", flush=True)


if __name__ == "__main__":
    main()
