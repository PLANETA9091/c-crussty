#!/usr/bin/env python3
"""absorb_497_main.py — ROUND-497 абсорб волны ×496 (закон 21).

Ценз из /tmp/bench497_census.json (159 ног). Приоритет:
P1 ARM-код-ноги (sb1/sb2/snap/p41/p42/p43/p44/p45/4arg/burstA) →
P2 лестницы (165k/205k/p250/p300/leak2/terr/tect/tow/r960/w12) →
P3 dp-стенды (BACAP/dp900) → P4 STRICT-хиты (sq3/st1/an2c9) → P5 банк-фиды (gNN/anchor/wild/an2/poi).
Ядро = absorb_496_main.py (first5-медиана, GC, ARM-маркеры, band, norm_v5, §3).
Выход: /home/z/rounds/ROUND-497/absorb/registry_497.json + ABSORB_497_SUMMARY.md
"""
import json, os, re, shutil, statistics, subprocess, sys, time, zipfile, bisect
from concurrent.futures import ThreadPoolExecutor, as_completed

REPO = "PLANETA9091/c-crussty"
OUT = "/home/z/rounds/ROUND-497/absorb"
V5 = [(6.5, 2.1252), (7.0, 2.2047), (7.5, 2.3271), (8.2, 2.4732), (8.7, 2.5981), (9.0, 2.6280)]
STRICT_LO, STRICT_HI = 6.9e6, 7.2e6
BAND = (6.0e6, 9.5e6)
STW_LIMIT_S, YOUNG_AVG_LIMIT_MS = 23.0, 200.0
CORRIDOR = (-8.0, 1.5)
RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
RE_ARMED = re.compile(r"(cmp[0-9a-z_]+)[^\n]{0,40}?\bARMED\b")
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")
WORLD_SHA = "afb3a0b3"


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


def absorb_run(run_id, branch):
    d = f"/tmp/abs497/{run_id}"
    os.makedirs(d, exist_ok=True)
    rec = {"run_id": run_id, "branch": branch, "ok": False}
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
        v["vanilla_valid"] = bool(cpu and armed == [] and v["ncdfe"] == 0 and v["aioobe"] == 0 and world == WORLD_SHA)
        v["class"] = classify(v)
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
    if any(k in b for k in ("c03-sb1", "c04-sb2", "c62-snap", "c63-p41", "c64-p42", "c65-p43", "c66-p44", "c67-p45", "c85-w12", "c85-w10", "c69-w8d1", "p31")):
        return 0
    if any(k in b for k in ("c70", "c71", "c72", "c73", "c74", "c77", "c79", "c76-r960", "c75-w12")):
        return 1
    if any(k in b for k in ("c26-dpstand", "c93-dp900")):
        return 2
    if any(k in b for k in ("c88-sq", "c95-st", "c87-an2", "c96-pa", "c92-a", "c90-wa", "c99-s48")):
        return 3
    return 4


def main():
    os.makedirs(OUT, exist_ok=True)
    census = json.load(open("/tmp/bench497_census.json"))
    runs = [(e["id"], e["branch"]) for e in census if e["conclusion"] == "success"]
    seen, uniq = set(), []
    for rid, br in runs:
        if rid not in seen:
            seen.add(rid)
            uniq.append((rid, br))
    uniq.sort(key=lambda x: (prio(x[1]), x[0]))
    print(f"success-ног к абсорбу: {len(uniq)}")
    registry = {"tool": "absorb_497_main", "n_runs": len(uniq), "results": []}
    t0 = time.time()
    done = 0
    with ThreadPoolExecutor(max_workers=8) as ex:
        futs = {ex.submit(absorb_run, rid, br): (rid, br) for rid, br in uniq}
        for fut in as_completed(futs):
            rec = fut.result()
            registry["results"].append(rec)
            done += 1
            if done % 10 == 0 or rec.get("err"):
                print(f"[{done}/{len(uniq)}] {rec['branch']} {rec['run_id']} "
                      f"{'OK norm=' + str(rec.get('verdict', {}).get('norm')) if rec.get('ok') else 'ERR ' + str(rec.get('err'))[:70]} "
                      f"({time.time()-t0:.0f}s)", flush=True)
            if done % 30 == 0:
                with open(f"{OUT}/registry_497.json", "w") as f:
                    json.dump(registry, f, indent=1, ensure_ascii=False)
    with open(f"{OUT}/registry_497.json", "w") as f:
        json.dump(registry, f, indent=1, ensure_ascii=False)
    from collections import Counter
    cls = Counter(r.get("verdict", {}).get("class", "ERR") for r in registry["results"])
    lines = ["# ABSORB ×497 SUMMARY (волна ×496)", "", f"ранов: {registry['n_runs']}; время: {time.time()-t0:.0f}s", "", "## Классы", ""]
    for k, v in cls.most_common():
        lines.append(f"- {k}: {v}")
    lines.append("")
    lines.append("## ARM/ladder/STRICT-ноги (P0-P3)")
    for r in sorted(registry["results"], key=lambda x: prio(x["branch"])):
        v = r.get("verdict", {})
        if prio(r["branch"]) <= 3 and v:
            lines.append(f"- {r['branch']} | {r['run_id']} | cpu {v['cpu']} | med {v['med']} | norm {v['norm']} | {v['class']} | armed {v['armed'] or '∅'} | STW {v['stw_total_s']} fulls {v['fulls']} | world {v['world']}")
    lines.append("")
    lines.append("## Банк-фиды VANILLA-VALID (P4+)")
    for r in registry["results"]:
        v = r.get("verdict", {})
        if v.get("class", "").startswith(("VANILLA-VALID", "STRICT-IN")):
            lines.append(f"- {r['branch']} | {r['run_id']} | cpu {v['cpu']} | norm {v['norm']}")
    with open(f"{OUT}/ABSORB_497_SUMMARY.md", "w") as f:
        f.write("\n".join(lines) + "\n")
    print("SUMMARY:", f"{OUT}/ABSORB_497_SUMMARY.md")
    print("CLASSES:", dict(cls))


if __name__ == "__main__":
    main()
