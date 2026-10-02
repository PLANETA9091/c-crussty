#!/usr/bin/env python3
"""absorb_496_main.py — ROUND-496 mass absorb (закон 21 + NEXT ×496 п.3/6).

Абсорб волны ×494+×495: ~222 bench-рана из ABSORB-495-WAVE.json.
Приоритет: P1 verdict-ноги (an2/seed/c41/c42/c43/c76-c80/strict/w8c) →
P2 GLOB+anchor банк-фиды → P3 прочие (terr/tect/swarx5/w12/dp600/leak).
Ядро парсинга = absorb_480_main.py (TPS-медиана first5, GC, ARM-маркеры,
NCDFE/AIOOBE, band, norm_v5, классификация §3). server-stdout удаляется
после парса (маркеры ДО пурджа — урок ×425).
Выход: /home/z/rounds/ROUND-496/absorb/registry_496.json + ABSORB_496_SUMMARY.md
"""
import json, os, re, shutil, statistics, subprocess, sys, time, zipfile, bisect
from concurrent.futures import ThreadPoolExecutor, as_completed

REPO = "PLANETA9091/c-crussty"
OUT = "/home/z/rounds/ROUND-496/absorb"
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
    d = f"/tmp/abs496/{run_id}"
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
        env_p = os.path.join(d, "run-env.txt")
        log_p = os.path.join(d, "server-stdout.log")
        gc_p = os.path.join(d, "gc.log")
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
        v["vanilla_valid"] = bool(cpu and armed == [] and v["ncdfe"] == 0
                                  and v["aioobe"] == 0 and world == WORLD_SHA)
        v["class"] = classify(v)
        rec.update({"ok": True, "verdict": v})
        # маркеры ДО пурджа: ARM-маркеры уже в v['armed']; лог больше не нужен
        for f in ("server-stdout.log", "gc.log"):
            fp = os.path.join(d, f)
            if os.path.exists(fp):
                os.unlink(fp)
    except Exception as ex:
        rec["err"] = f"{type(ex).__name__}: {ex}"
    finally:
        # чистим распакованное, кроме fp.json (верdict-ногам нужен deeper-разбор)
        if os.path.isdir(d):
            for f in os.listdir(d):
                fp = os.path.join(d, f)
                if f not in ("fp.json",) and os.path.isfile(fp):
                    os.unlink(fp)
    return rec


def prio(branch):
    b = branch or ""
    if any(k in b for k in ("an2", "c43-strict", "c41", "c42", "seed", "s43", "s44", "s45", "c76", "c77", "c78")):
        return 0
    if "-g" in b or "-a" in b:
        return 1
    return 2


def main():
    os.makedirs(OUT, exist_ok=True)
    wave = json.load(open("/home/z/rounds/ROUND-496/ABSORB-495-WAVE.json"))
    runs = [(e["bench_run_id"], e["branch"]) for e in wave["bench_runs"]
            if e["conclusion"] == "success" and e["branch"] != "master"]
    # уникальность run_id
    seen, uniq = set(), []
    for rid, br in runs:
        if rid not in seen:
            seen.add(rid)
            uniq.append((rid, br))
    uniq.sort(key=lambda x: (prio(x[1]), -x[0]))
    print(f"bench-рана к абсорбу: {len(uniq)} (success, не master)")
    registry = {"tool": "absorb_496_main", "n_runs": len(uniq), "results": []}
    t0 = time.time()
    done = 0
    with ThreadPoolExecutor(max_workers=8) as ex:
        futs = {ex.submit(absorb_run, rid, br): (rid, br) for rid, br in uniq}
        for fut in as_completed(futs):
            rec = fut.result()
            registry["results"].append(rec)
            done += 1
            if done % 20 == 0 or rec.get("err"):
                print(f"[{done}/{len(uniq)}] {rec['branch']} {rec['run_id']} "
                      f"{'OK norm=' + str(rec.get('verdict', {}).get('norm')) if rec.get('ok') else 'ERR ' + str(rec.get('err'))[:80]} "
                      f"({time.time()-t0:.0f}s)", flush=True)
            if done % 50 == 0:
                with open(f"{OUT}/registry_496.json", "w") as f:
                    json.dump(registry, f, indent=1, ensure_ascii=False)
    with open(f"{OUT}/registry_496.json", "w") as f:
        json.dump(registry, f, indent=1, ensure_ascii=False)

    # сводка
    from collections import Counter
    cls = Counter(r.get("verdict", {}).get("class", "ERR:" + str(r.get("err"))[:40]) for r in registry["results"])
    lines = ["# ABSORB ×496 SUMMARY (волна ×494/×495)", "",
             f"ранов: {registry['n_runs']}; время: {time.time()-t0:.0f}s", "", "## Классы", ""]
    for k, v in cls.most_common():
        lines.append(f"- {k}: {v}")
    lines.append("")
    lines.append("## Вердикт-ноги (P1, norm ≠ None)")
    for r in registry["results"]:
        v = r.get("verdict", {})
        if v.get("norm") is not None and prio(r["branch"]) == 0:
            lines.append(f"- {r['branch']} | {r['run_id']} | cpu {v['cpu']} | med {v['med']} | norm {v['norm']} | {v['class']} | STW {v['stw_total_s']}s fulls {v['fulls']} | armed {v['armed'] or '∅'} | world {v['world']}")
    lines.append("")
    lines.append("## Банк-фиды (P1+P2, VANILLA-VALID)")
    for r in registry["results"]:
        v = r.get("verdict", {})
        if v.get("class", "").startswith("VANILLA-VALID"):
            lines.append(f"- {r['branch']} | {r['run_id']} | cpu {v['cpu']} | norm {v['norm']}")
    with open(f"{OUT}/ABSORB_496_SUMMARY.md", "w") as f:
        f.write("\n".join(lines) + "\n")
    print("SUMMARY:", f"{OUT}/ABSORB_496_SUMMARY.md")
    print("CLASSES:", dict(cls))


if __name__ == "__main__":
    main()
