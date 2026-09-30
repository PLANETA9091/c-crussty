#!/usr/bin/env python3
"""absorb_511_main.py — ROUND-511 абсорб волны ×511 (закон 21).

Census: /tmp/wbp_runs.json (2 страницы, round-511-* + хвосты ×510).
Ядро = absorb_510 core (first5-медиана, V5-эксп, band, M1, corridor, §3-классы) + спеки:
  round-511-p31ib* → P31-IB hist-строки (flushes/thresh/ents/maxn/popcnt/errs) + ARM + THRESH → гейт МЕРЖ №24
  round-511-dp7/dp8 → dp-стенды 50k: NO-TPS-файры (G-B2 k-файлы)
  round-511-stz93/94 → датапак-стрессы CLM-C33 (мед/поллы; failure-причина)
  round-511-sp1-5 → band-gap пробы [7.5,8.0]M (fleet-gap гипотеза)
  pz06 → AIOOBE шум-гейт N=2
Банк-фиды §3: VANILLA-VALID CLEAN (armed ∅, NCDFE=0, AIOOBE=0, world ok, M1, band, corridor,
  norm one-sided ≤ +15 LEGAL), только population_target=150k.
Выход: /home/z/rounds/ROUND-511/absorb/registry_511.json + консоль-сводка
"""
import json, os, re, shutil, statistics, subprocess, sys, time, zipfile, bisect
from concurrent.futures import ThreadPoolExecutor, as_completed

REPO = "PLANETA9091/c-crussty"
OUT = "/home/z/rounds/ROUND-511/absorb"
V5 = [(6.5, 2.1252), (7.0, 2.2047), (7.5, 2.3271), (8.2, 2.4732), (8.7, 2.5981), (9.0, 2.6280)]
STRICT_LO, STRICT_HI = 6.9e6, 7.2e6
BAND = (6.0e6, 9.5e6)
STW_LIMIT_S, YOUNG_AVG_LIMIT_MS = 23.0, 200.0
CORRIDOR = (-8.0, 1.5)
LEGAL_MAX = 15.0
WORLD_SHA = "afb3a0b3"
RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
RE_ARMED = re.compile(r"(cmp[0-9a-z_]+)[^\n]{0,40}?\bARMED\b")
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")
RE_POP = re.compile(r"population_target:\s*(\d+)")
RE_HIST = re.compile(r"inside_batch: hist ([A-Za-z0-9_ =.]+)$")
RE_STZ59 = re.compile(r"STZ59-PROBE stale_discards=(\d+) torn=(\d+) reader_ops=(\d+) arm=(\w+)")


def tok():
    return open("/tmp/gh_token").read().strip()


def api(path):
    p = subprocess.run(["curl", "-s", "-H", f"Authorization: token {tok()}",
                        "-H", "Accept: application/vnd.github+json",
                        f"https://api.github.com/repos/{REPO}/{path.lstrip('/')}"],
                       capture_output=True, timeout=90)
    return json.loads(p.stdout.decode() or "{}")


def dl(url, dest):
    for attempt in range(3):
        p = subprocess.run(["curl", "-sL", "-H", f"Authorization: token {tok()}", "-o", dest,
                            "-w", "%{http_code}", url], capture_output=True, text=True, timeout=300)
        if p.stdout.strip() == "200":
            return True
        time.sleep(2 * (attempt + 1))
    return False


def parse_gc(text):
    pauses = []
    for line in text.splitlines():
        if "Pause" not in line or "[gc,phases" in line:
            continue
        m = PAUSE_COMPL.search(line)
        if not m:
            continue
        um, cm = UPTIME.search(line), CAUSE.search(line)
        pauses.append((float(m.group(1)), cm.group(1) if cm else "?", float(um.group(1)) if um else 0.0))
    young = [p for p in pauses if p[1] == "Young"]
    fulls = [p for p in pauses if p[1] == "Full"]
    return {"stw_total_s": round(sum(p[0] for p in pauses)/1000, 2), "young_n": len(young),
            "young_avg_ms": round(sum(p[0] for p in young)/len(young), 2) if young else 0.0,
            "fulls": len(fulls), "max_ms": round(max((p[0] for p in pauses), default=0.0), 1)}


def tps_exp_v5(cpu):
    xs = [p[0] for p in V5]; ys = [p[1] for p in V5]; c = cpu / 1e6
    if c <= xs[0]:
        return ys[0] + (ys[1]-ys[0])/(xs[1]-xs[0])*(c-xs[0]), "extrap-low"
    if c >= xs[-1]:
        return ys[-1] + (ys[-1]-ys[-2])/(xs[-1]-xs[-2])*(c-xs[-1]), "extrap-high"
    i = bisect.bisect_left(xs, c)
    return ys[i-1] + (ys[i]-ys[i-1])/(xs[i]-xs[i-1])*(c-xs[i-1]), "interp"


def classify(v):
    if not v["band"]:
        return "BAND-DEAD(free)"
    if v["strict"] and v["m1"] and v["vanilla_valid"] and v["corridor"]:
        return "STRICT-IN-POINT(банк-фид)"
    if v["m1"] and v["vanilla_valid"] and v["corridor"]:
        return "VANILLA-VALID-§3(банк-фид)"
    if not v["m1"]:
        return "HOST-CENSORED"
    if not v["vanilla_valid"]:
        return "NOT-VANILLA(armed/ncdfe/world)"
    return "CORRIDOR-BREACH"


def parse_hist(log):
    rows = []
    for line in log.splitlines():
        if "inside_batch: hist" not in line:
            continue
        kv = {}
        for m in re.finditer(r"(\w+)=([\d.]+)", line.split("hist", 1)[1]):
            try:
                kv[m.group(1)] = float(m.group(2))
            except ValueError:
                pass
        if kv:
            rows.append(kv)
    return rows


def is_spec(branch):
    b = branch or ""
    return (b.startswith("round-511-p31ib") or b in ("round-511-dp7", "round-511-dp8")
            or b.startswith("round-511-stz9") or b.startswith("round-511-sp")
            or b.startswith("round-511-fen") or b.startswith("round-511-unf")
            or b.startswith("round-510-pz06"))


def absorb_run(run_id, branch, conclusion):
    d = f"/tmp/abs511/{run_id}"
    os.makedirs(d, exist_ok=True)
    rec = {"run_id": run_id, "branch": branch, "conclusion": conclusion, "ok": False}
    try:
        arts = api(f"actions/runs/{run_id}/artifacts")
        a = next((x for x in arts.get("artifacts", []) if x["name"] == "world3-bench"), None)
        if not a:
            rec["err"] = "no world3-bench artifact"
            return rec
        zpath = f"{d}/wb.zip"
        if not dl(a["archive_download_url"], zpath):
            rec["err"] = "artifact download fail"
            return rec
        keep = {"server-stdout.log", "run-env.txt", "gc.log", "fp.json"}
        with zipfile.ZipFile(zpath) as zf:
            for name in zf.namelist():
                base = os.path.basename(name)
                if base in keep:
                    with zf.open(name) as src, open(os.path.join(d, base), "wb") as dst:
                        shutil.copyfileobj(src, dst)
        os.unlink(zpath)
        env_p, log_p, gc_p = (os.path.join(d, f) for f in ("run-env.txt", "server-stdout.log", "gc.log"))
        if not (os.path.exists(env_p) and os.path.exists(log_p) and os.path.exists(gc_p)):
            rec["err"] = f"missing files {sorted(os.listdir(d))}"
            return rec
        env = open(env_p, errors="replace").read()
        log = open(log_p, errors="replace").read()
        gct = open(gc_p, errors="replace").read()
        mc = re.search(r"runner_cpu_index:\s*(\d+)", env)
        cpu = int(mc.group(1)) if mc else None
        mp = RE_POP.search(env)
        pop = int(mp.group(1)) if mp else None
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
             "aioobe": log.count("ArrayIndexOutOfBounds"), "world": world, "pop": pop, **g}
        v["m1"] = (v["stw_total_s"] <= STW_LIMIT_S and v["young_avg_ms"] <= YOUNG_AVG_LIMIT_MS)
        v["band"] = bool(cpu and BAND[0] <= cpu <= BAND[1])
        v["strict"] = bool(cpu and STRICT_LO <= cpu <= STRICT_HI)
        v["corridor"] = bool(v["norm"] is not None and CORRIDOR[0] <= v["norm"] <= CORRIDOR[1])
        v["vanilla_valid"] = bool(cpu and armed == [] and v["ncdfe"] == 0 and v["aioobe"] == 0 and world == WORLD_SHA)
        v["class"] = classify(v)
        # --- спека: P31-IB ---
        if branch and branch.startswith("round-511-p31ib"):
            hist = parse_hist(log)
            v["hist_rows"] = len(hist)
            if hist:
                agg = {}
                for k in ("flushes", "thresh", "ents", "maxn", "popcnt", "errs", "batches", "mask_rows"):
                    vals = [r[k] for r in hist if k in r]
                    if vals:
                        agg[k] = {"first": vals[0], "last": vals[-1], "max": max(vals)}
                v["hist"] = agg
                v["hist_last"] = hist[-1]
            v["ib_armed"] = bool(armed) and any("p31snap" in x for x in armed)
            v["ib_thresh512"] = " thresh=512" in log or "THRESH=512" in log
            v["stz59_rows"] = len(RE_STZ59.findall(log))
        # --- спека: Г3 phantom-INDUCE (еслиfen/unf попадут в абсорб) ---
        if branch and (branch.startswith("round-511-fen") or branch.startswith("round-511-unf")):
            rows = [{"sd": int(s), "torn": int(t), "ops": int(o), "arm": am}
                    for s, t, o, am in RE_STZ59.findall(log)]
            v["stz59_rows"] = len(rows)
            if rows:
                v["stz59_last"] = rows[-1]
                v["stz59_sd_max"] = max(r["sd"] for r in rows)
                v["stz59_ops_last"] = rows[-1]["ops"]
        # --- спека: dp-стенды ---
        if branch in ("round-511-dp7", "round-511-dp8"):
            v["dp_notps"] = (v["n_polls"] == 0)
            v["dp_med"] = med
            v["dp_norm"] = v["norm"]
        rec.update({"ok": True, "verdict": v})
        for f in ("server-stdout.log", "gc.log"):
            fp = os.path.join(d, f)
            if os.path.exists(fp):
                os.unlink(fp)
    except Exception as ex:
        rec["err"] = f"{type(ex).__name__}: {ex}"
    finally:
        if os.path.isdir(d):
            for f in os.listdir(d):
                fp = os.path.join(d, f)
                if f not in ("fp.json",) and os.path.isfile(fp):
                    os.unlink(fp)
    return rec


def prio(branch):
    b = branch or ""
    if b.startswith("round-511-p31ib"):
        return 0
    if b in ("round-511-dp7", "round-511-dp8") or b.startswith("round-511-stz9") or b.startswith("round-511-sp"):
        return 1
    if b.startswith("round-510-pz06") or b in ("round-507-ax26",):
        return 2
    return 4


def main():
    os.makedirs(OUT, exist_ok=True)
    census = json.load(open("/tmp/wbp_runs.json"))["workflow_runs"]
    seen = set()
    uniq = []
    for r in census:
        if r["id"] in seen or r["status"] != "completed":
            continue
        seen.add(r["id"])
        uniq.append((r["id"], r["head_branch"], r["conclusion"]))
    uniq.sort(key=lambda x: (prio(x[1]), x[0]))
    succ = [u for u in uniq if u[2] == "success"]
    fail = [u for u in uniq if u[2] == "failure"]
    canc = [u for u in uniq if u[2] == "cancelled"]
    print(f"census: {len(uniq)} terminal = {len(succ)} succ / {len(fail)} fail / {len(canc)} canc")
    registry = {"tool": "absorb_511_main", "n_runs": len(uniq), "results": [], "cancelled": []}
    t0 = time.time()
    done = 0
    for rid, br, con in canc:
        registry["cancelled"].append({"run_id": rid, "branch": br, "created": None})
    todo = succ + fail
    with ThreadPoolExecutor(max_workers=8) as ex:
        futs = {ex.submit(absorb_run, rid, br, con): (rid, br) for rid, br, con in todo}
        for fut in as_completed(futs):
            rec = fut.result()
            registry["results"].append(rec)
            done += 1
            if done % 15 == 0:
                print(f"[{done}/{len(todo)}] {time.time()-t0:.0f}s", flush=True)
                with open(f"{OUT}/registry_511.json", "w") as f:
                    json.dump(registry, f, indent=1, ensure_ascii=False)
    with open(f"{OUT}/registry_511.json", "w") as f:
        json.dump(registry, f, indent=1, ensure_ascii=False)
    print(f"DONE {time.time()-t0:.0f}s → {OUT}/registry_511.json")


if __name__ == "__main__":
    main()
