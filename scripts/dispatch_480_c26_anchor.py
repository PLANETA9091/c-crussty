#!/usr/bin/env python3
"""dispatch_480_c26_anchor.py — COMMANDER C26 (round-480): ваниль-якорь STRICT-зоны ×1.

CLAIM (банк §3, Л-480-C26): STRICT-зона [6.9,7.2]M (Л201-узел 2.1293), банк
n=29/30, дефицит 1. Задача: 1 ваниль-диспатч world-bench-parallel @master
686f225830a40570fd7dddcf77c2e1e64e4ecb88 (0 код-дельт, ваниль-нога бит-идентична
— закон-5), алиас round-480-c26-a1, canon x466-C98 ЯВНЫМ JSON (640/300s/fp4/
gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G, lever="" ваниль), band GLOB
[6000000,9500000] fast-fail; окно пост-хок (НЕ в гейте, канон Л-470-S31.1).
STRICT-hit ⇔ cpu(run-env) ∈ [6.9,7.2]M ∧ CLEAN ∧ ваниль-VALID ∧ norm ∈
коридор [−8.0,+1.5] → банк n→30. Band-miss → 1 ре-ролл (закон W3).
Прегист закон 14a/16 — этот docstring; LEDGER Л-480-C26 (board/CLM-C26.md).
"""
import json, os, re, shutil, statistics, subprocess, sys, time, urllib.request, urllib.error, zipfile, bisect

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ALIAS = "round-480-c26-a1"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master FULL sha (686f2258)
BASE_CLAIM = "686f2258"  # CLAIM-база == pin: 0 код-дельт по определению миссии
OUT = "/home/z/rounds/ROUND-480/absorb"

# canon x466-C98 — ЯВНЫЙ JSON, yml-дефолты = merge-поверхность (урок C73)
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}

V5 = [(6.5, 2.1252), (7.0, 2.2047), (7.5, 2.3271), (8.2, 2.4732), (8.7, 2.5981), (9.0, 2.6280)]
STRICT_LO, STRICT_HI = 6.9e6, 7.2e6
BAND = (6.0e6, 9.5e6)
STW_LIMIT_S, YOUNG_AVG_LIMIT_MS = 23.0, 200.0
CORRIDOR = (-8.0, 1.5)
RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
RE_ARMED = re.compile(r"(cmp[0-9a-z_]+): ARMED")
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=90, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def ensure_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    cur = r.get("object", {}).get("sha")
    if cur == sha:
        print(f"{name}: GET-proof exists @ {sha} (Л188a)")
        return
    if cur:
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        print(f"{name}: moved -> {sha}")
    else:
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{name}", "sha": sha})  # FULL sha (S20: short=422)
    v = api(tok, f"/repos/{REPO}/git/ref/heads/{name}").get("object", {}).get("sha")
    assert v == sha, f"alias verify failed: {v} != {sha}"
    print(f"{name}: pinned FULL-sha @ {sha}, GET-verified (Л188a)")


def dispatch(tok, ref):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": INPUTS})
    print(f"dispatch {ref} pop=150000 -> {'204 OK' if r == {} else r}", flush=True)
    return r == {}


def latest_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"]}
    return None


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


def absorb(tok, rid, tag):
    """Ждёт завершения run rid, тянет артефакт, парсит → вердикт-JSON."""
    t0 = time.time()
    while True:
        r = api(tok, f"/repos/{REPO}/actions/runs/{rid}")
        st, cc = r.get("status"), r.get("conclusion")
        if st == "completed":
            break
        if time.time() - t0 > 3600:
            return {"run": rid, "status": st, "timeout": True}
        time.sleep(60)
        print(f"  poll {int((time.time()-t0)/60)}m status={st}", flush=True)
    base = {"run": rid, "conclusion": cc, "sha": r.get("head_sha", "")[:12],
            "branch": r.get("head_branch", "")}
    if cc != "success":
        # band fast-fail приходит как failure за ~40-60s
        base["class"] = "BAND-FAST-FAIL" if (time.time() - t0 < 240) else f"FAILED({cc})"
        return base
    arts = api(tok, f"/repos/{REPO}/actions/runs/{rid}/artifacts")["artifacts"]
    a = next((x for x in arts if x["name"] == "world3-bench"), None)
    if not a:
        base["class"], base["err"] = "NO-ARTIFACT", "world3-bench missing"
        return base
    os.makedirs(OUT, exist_ok=True)
    zpath = f"{OUT}/{tag}_a.zip"
    url = f"{API}/repos/{REPO}/actions/artifacts/{a['id']}/zip"
    for attempt in range(2):
        req = urllib.request.Request(url, headers={
            "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
        try:
            with urllib.request.urlopen(req, timeout=900) as resp, open(zpath, "wb") as f:
                f.write(resp.read())
            if open(zpath, "rb").read(2) == b"PK":
                break
        except Exception as ex:
            print(f"  dl attempt {attempt}: {ex}", file=sys.stderr)
        time.sleep(5)
    d = f"{OUT}/{tag}"
    os.makedirs(d, exist_ok=True)
    try:
        with zipfile.ZipFile(zpath) as z:
            for m in ("run-env.txt", "server-stdout.log", "gc.log"):
                names = [n for n in z.namelist() if n.endswith(m)]
                if not names:
                    base["class"], base["err"] = "NO-ART", f"missing {m}"
                    return base
                with z.open(names[0]) as src, open(os.path.join(d, m), "wb") as dst:
                    shutil.copyfileobj(src, dst)
    finally:
        if os.path.exists(zpath):
            os.remove(zpath)
    v = norm_of(d)
    base.update(v)
    return base


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "dispatch"
    tok = token()
    if mode == "dispatch":
        br = api(tok, f"/repos/{REPO}/branches/master")
        live = br.get("commit", {}).get("sha", "")
        print(f"origin/master live = {live}")
        pin = live if live.startswith(PIN[:12]) else PIN
        # drift-страж: CLAIM-база → pin не должен трогать bench-поверхности
        stat = subprocess.run(
            ["git", "-C", "/home/z/c-crussty", "diff", "--name-only",
             BASE_CLAIM, pin[:12]], capture_output=True, text=True).stdout
        hot = [ln for ln in stat.splitlines()
               if ("src/" in ln or ".github/" in ln or "Cargo" in ln or "pom" in ln
                   or "native/" in ln)]
        if hot:
            print("DRIFT-HOT files:", *hot, sep="\n", file=sys.stderr)
            sys.exit(3)
        n_files = len(stat.strip().splitlines()) if stat.strip() else 0
        print(f"drift {BASE_CLAIM}->{pin[:8]}: docs/scripts-only OK ({n_files} files)")
        ensure_alias(tok, ALIAS, pin)
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        if not dispatch(tok, ALIAS):
            sys.exit(2)
        time.sleep(30)
        run = latest_run(tok, ALIAS, mc)
        if not run:
            print("RUN-NOT-FOUND", file=sys.stderr)
            sys.exit(4)
        print("C26-DISPATCH-JSON " + json.dumps(run, indent=1))
        with open(f"{OUT}/c26_dispatch.json", "w") as f:
            json.dump({"alias": ALIAS, **run, "pin": pin}, f, indent=1)
        return
    # mode == "wait <run_id> [tag] [allow_reroll]"
    rid = int(sys.argv[2])
    tag = sys.argv[3] if len(sys.argv) > 3 else "c26-a1"
    reroll = len(sys.argv) > 4 and sys.argv[4] == "reroll"
    v = absorb(tok, rid, tag)
    print("C26-RESULT-JSON " + json.dumps(v, ensure_ascii=False, indent=1))
    miss = v.get("class") in ("BAND-FAST-FAIL", "BAND-DEAD(free)") or (
        v.get("cpu") is not None and not v.get("strict"))
    if miss and reroll:
        print("BAND/STRICT-MISS -> 1 ре-ролл (закон W3)", flush=True)
        ensure_alias(tok, ALIAS, PIN)
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        if dispatch(tok, ALIAS):
            time.sleep(30)
            r2 = latest_run(tok, ALIAS, mc)
            if r2:
                print(f"re-roll run-id = {r2['id']}", flush=True)
                v2 = absorb(tok, r2["id"], tag + "-r2")
                print("C26-RESULT2-JSON " + json.dumps(v2, ensure_ascii=False, indent=1))
                with open(f"{OUT}/c26_result2.json", "w") as f:
                    json.dump(v2, f, ensure_ascii=False, indent=1)
    with open(f"{OUT}/c26_result.json", "w") as f:
        json.dump(v, f, ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
