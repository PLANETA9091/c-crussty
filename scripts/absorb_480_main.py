#!/usr/bin/env python3
"""absorb_480_main.py — ROUND-480 mass absorb (закон 21: абсорб in-flight).

Волны ×479 к абсорбу: V1 STRICT ×12 (s04 in flight), V3 якоря ×10, V4 безумия ×8,
master-волна SUCCESS (~40 vanilla), s1-s4 A4, bn-retry, СТЗ-2, та-якоря y1-y3.
Классификация: STRICT-in-point (банк-фид), ваниль-VALID §3, HOST-CENS, corridor-breach.
Артефакт → 3 файла → parse → server-stdout.log удаляется (маркеры ДО пурджа).
"""
import json, os, re, shutil, statistics, subprocess, sys, time, zipfile, bisect
from concurrent.futures import ThreadPoolExecutor, as_completed

REPO = "PLANETA9091/c-crussty"
OUT = "/home/z/rounds/ROUND-480/absorb"
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

WAVES = {
    "V1-STRICT": {
        "s01": 36380947446, "s02": 36380966574, "s03": 36381428411, "s04": 36381449578,
        "s05": 36381472779, "s06": 36381494193, "s07": 36381515013, "s08": 36381535337,
        "s09": 36381555616, "s10": 36381574962, "s11": 36381593873, "s12": 36381614348},
    "V3-ANCHORS": {
        "y01": 36381318797, "y02": 36381324303, "y03": 36381329821, "y04": 36381335476,
        "y05": 36381565316, "y06": 36381347086, "y07": 36381353197, "y08": 36381570415,
        "y09": 36381364434, "y10": 36381369817},
    "V4-WILD": {
        "x03": 36381691677, "x04": 36381712486, "x05": 36381732204, "x06": 36381751782,
        "x08": 36381790416, "x09": 36381810117, "x10": 36381831203, "r07": 36382015159,
        "x02": 36381671806, "r01": 36382003326},
    "TAIL": {
        "a4-s1": 36374666298, "a4-s2": 36374667469, "a4-s3": 36374668962, "a4-s4": 36374670462,
        "bn": 36374776545, "stz2": 36379253638,
        "ta-y1": 36382804495, "ta-y2": 36382822966, "ta-y3": 36382840757, "ta-y4": 36382857522},
    "MASTER": {
        "m-01": 36380906369, "m-02": 36381184647, "m-03": 36381252652, "m-04": 36381384900,
        "m-05": 36381403197, "m-06": 36381407892, "m-07": 36381537465, "m-08": 36381540301,
        "m-09": 36381589178, "m-10": 36381611133, "m-11": 36381614625, "m-12": 36381638939,
        "m-13": 36381663511, "m-14": 36381671994, "m-15": 36381693159, "m-16": 36381743556,
        "m-17": 36381762843, "m-18": 36381766230, "m-19": 36381768543, "m-20": 36381818127,
        "m-21": 36381825877, "m-22": 36382129386, "m-23": 36382139203, "m-24": 36382249025,
        "m-25": 36382306869, "m-26": 36382313239, "m-27": 36382320470, "m-28": 36382380314,
        "m-29": 36382418570, "m-30": 36382494999, "m-31": 36382524964, "m-32": 36382548879,
        "m-33": 36382562224, "m-34": 36382565701, "m-35": 36382565805, "m-36": 36382580820,
        "m-37": 36382607297, "m-38": 36382679193, "m-39": 36382686965, "m-40": 36382696359,
        "m-41": 36382701100, "m-42": 36382741224, "m-43": 36382763961, "m-44": 36382839800,
        "m-45": 36382873657, "m-46": 36382919648, "m-47": 36382978116, "m-48": 36383009553,
        "m-49": 36383136384},
}


def tok():
    return open("/tmp/gh_token").read().strip()


def api(t, path):
    path = path.lstrip("/")
    p = subprocess.run(["curl", "-s", "-H", f"Authorization: token {t}",
                        "-H", "Accept: application/vnd.github+json",
                        f"https://api.github.com/repos/{REPO}/{path}"],
                       capture_output=True, timeout=90)
    return json.loads(p.stdout.decode() or "{}")


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
    # классификация
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


def process(t, tag, rid, wave):
    r = api(t, f"/actions/runs/{rid}")
    st, cc = r.get("status"), r.get("conclusion")
    if st != "completed":
        return tag, {"run": rid, "wave": wave, "status": st, "branch": r.get("head_branch", "")}
    if cc != "success":
        return tag, {"run": rid, "wave": wave, "conclusion": cc,
                     "branch": r.get("head_branch", "")}
    arts = api(t, f"/actions/runs/{rid}/artifacts")["artifacts"]
    a = next((x for x in arts if x["name"] == "world3-bench"), None)
    if not a:
        return tag, {"run": rid, "wave": wave, "conclusion": cc, "err": "no artifact",
                     "branch": r.get("head_branch", "")}
    zpath = f"{OUT}/{tag}_a.zip"
    url = f"https://api.github.com/repos/{REPO}/actions/artifacts/{a['id']}/zip"
    for attempt in range(2):
        p = subprocess.run(["curl", "-sL", "-H", f"Authorization: token {t}", url, "-o", zpath],
                           capture_output=True, timeout=900)
        if p.returncode == 0 and os.path.exists(zpath) and open(zpath, "rb").read(2) == b"PK":
            break
        time.sleep(5)
    else:
        return tag, {"run": rid, "wave": wave, "err": "download fail",
                     "branch": r.get("head_branch", "")}
    d = f"{OUT}/{tag}"
    os.makedirs(d, exist_ok=True)
    try:
        with zipfile.ZipFile(zpath) as z:
            for m in ("run-env.txt", "server-stdout.log", "gc.log"):
                names = [n for n in z.namelist() if n.endswith(m)]
                if not names:
                    return tag, {"run": rid, "wave": wave, "err": f"missing {m}"}
                with z.open(names[0]) as src, open(os.path.join(d, m), "wb") as dst:
                    shutil.copyfileobj(src, dst)
    finally:
        os.remove(zpath)
    try:
        v = norm_of(d)
    except Exception as ex:
        return tag, {"run": rid, "wave": wave, "err": f"parse: {ex}"}
    v.update(run=rid, wave=wave, tag=tag, branch=r.get("head_branch", ""))
    # маркеры сохранены в v; server-stdout.log purge (диск)
    so = os.path.join(d, "server-stdout.log")
    if os.path.exists(so):
        os.remove(so)
    return tag, v


def main():
    os.makedirs(OUT, exist_ok=True)
    t = tok()
    only = set(sys.argv[1].split(",")) if len(sys.argv) > 1 else None
    jobs = [(tg, rid, wv) for wv, mp in WAVES.items() if not only or wv in only
            for tg, rid in mp.items()]
    res = {}
    print(f"ABSORB-480: {len(jobs)} runs", flush=True)
    with ThreadPoolExecutor(max_workers=6) as ex:
        futs = {ex.submit(process, t, tg, rid, wv): (tg, rid) for tg, rid, wv in jobs}
        done = 0
        for fut in as_completed(futs):
            tg, rid = futs[fut]
            try:
                _, v = fut.result()
            except Exception as ex:
                v = {"run": rid, "err": str(ex)[:200]}
            res[tg] = v
            done += 1
            brief = v.get("class") or v.get("conclusion") or v.get("status") or v.get("err", "?")
            print(f"[{done}/{len(jobs)}] {tg} {rid}: {brief} "
                  f"cpu={v.get('cpu')} norm={v.get('norm')} m1={v.get('m1')}", flush=True)
    # merge с предыдущими волнами (JSON — аккумулятор между запусками)
    acc_path = f"{OUT}/absorb_480.json"
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
    # сводка
    summary = {}
    for wv in WAVES:
        sub = {k: v for k, v in res.items() if v.get("wave") == wv}
        cls = {}
        for k, v in sub.items():
            c = v.get("class") or v.get("conclusion") or v.get("status") or "ERR"
            cls[c] = cls.get(c, 0) + 1
        summary[wv] = cls
    strict_hits = [k for k, v in res.items() if v.get("class") == "STRICT-IN-POINT(банк-фид)"]
    print("SUMMARY:", json.dumps(summary, ensure_ascii=False), flush=True)
    print(f"STRICT-HITS: {strict_hits}", flush=True)
    print(f"saved {OUT}/absorb_480.json", flush=True)


if __name__ == "__main__":
    main()
