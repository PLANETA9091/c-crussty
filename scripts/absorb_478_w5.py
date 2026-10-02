#!/usr/bin/env python3
"""absorb_478_w5.py — [478-W5] gc2-коридор absorb (A1/B4-канон).

Гейты (прегистрированы docs/PREREG_478_W5_GC2AB.md, до вердикта):
  1. cpu_index ∈ [6.0, 9.5]M обе ноги (band-miss = free discard Л188c).
  2. GATE-GC2 (A4-канон M1): STW_total ≤23.0s ∧ young ≤200 ev ∧ Full ≤12.
  3. Корридор ×2: [min,max] STW внутри/перекрыт с gc3-банком [18.3,25.6]s
     (Л-478-A4 лестница + fresh B5-a2/a3 21.43/21.64s, тот же вектор).
  4. Валидность: NCDFE=0 / AIOOBE=0 / world afb3a0b3 / canon-env ×12.
Вердикт: GC2-READY {числа ×2} | REFUTED_CENS | DISPATCHED (>15 мин, закон 18-iii).
"""
import json, os, re, shutil, subprocess, sys, time, zipfile

REPO = "PLANETA9091/c-crussty"
BRANCHES = {"gc2a": "round-478-w5-gc2a", "gc2b": "round-478-w5-gc2b"}
BAND = (6.0e6, 9.5e6)
GC3_BANK = (18.3, 25.6)          # gc3 STW-корридор (Л-478-A4 + B5-a2/a3)
STW_LIMIT_S, YOUNG_EV_LIMIT, FULL_LIMIT = 23.0, 200, 12
OUT = "/tmp/w5_absorb"

RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")


def tok():
    return open("/tmp/gh_token").read().strip()


def api(t, path):
    p = subprocess.run(["curl", "-s", "-H", f"Authorization: token {t}",
                        "-H", "Accept: application/vnd.github+json",
                        f"https://api.github.com/repos/{REPO}/{path}"],
                       capture_output=True, timeout=90)
    return json.loads(p.stdout.decode() or "{}")


def resolve_run(t, branch):
    """Свежайший workflow_dispatch-ран ветки (S31-урок атрибуции по head_branch)."""
    runs = api(t, f"/actions/runs?event=workflow_dispatch&per_page=40")
    for r in runs.get("workflow_runs", []):
        if r["head_branch"] == branch:
            return r["id"], r["status"], r["conclusion"]
    return None, None, None


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
                       cm.group(2) if cm else "?"))
    young = [p for p in pauses if p[1] == "Young"]
    fulls = [p for p in pauses if p[1] == "Full"]
    cause = {}
    for ms, kind, why in pauses:
        key = f"{kind}:{why.split()[0] if why else '?'}"
        cause[key] = [cause.get(key, (0, 0.0))[0] + 1, cause.get(key, (0, 0.0))[1] + ms]
    return {"stw_total_s": round(sum(p[0] for p in pauses)/1000, 2),
            "young_n": len(young),
            "young_avg_ms": round(sum(p[0] for p in young)/len(young), 2) if young else 0.0,
            "fulls": len(fulls),
            "full_ms": round(sum(p[0] for p in fulls), 1),
            "max_ms": round(max((p[0] for p in pauses), default=0.0), 1),
            "cause": {k: f"{v[0]}x/{v[1]:.0f}ms" for k, v in sorted(cause.items())}}


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
    gct = open(os.path.join(d, "gc.log"), errors="replace").read()
    cpu = int(re.search(r"runner_cpu_index:\s*(\d+)", env).group(1))
    gt = re.search(r"^gc_tune:\s*(\d+)", env, re.M)
    tps_all = [float(x) for x in RE_TPS.findall(log)]
    bench = [x for x in tps_all if x < 15]
    g = parse_gc(gct)
    v = {"cpu": cpu, "gc_tune": gt.group(1) if gt else None,
         "polls_bench": bench, "tps_med": (sorted(bench)[len(bench)//2] if bench else None),
         "ncdfe": log.count("NoClassDefFoundError"),
         "aioobe": log.count("ArrayIndexOutOfBounds"),
         "world": (re.search(r"world sha[: ]*([0-9a-f]{8})", log).group(1)
                   if re.search(r"world sha[: ]*([0-9a-f]{8})", log) else None),
         **g}
    v["band"] = BAND[0] <= cpu <= BAND[1]
    v["gate_gc2"] = (v["stw_total_s"] <= STW_LIMIT_S and v["young_n"] <= YOUNG_EV_LIMIT
                     and v["fulls"] <= FULL_LIMIT)
    v["in_gc3_bank"] = GC3_BANK[0] <= v["stw_total_s"] <= GC3_BANK[1]
    return v


def main():
    os.makedirs(OUT, exist_ok=True)
    t = tok()
    deadline = time.time() + (int(sys.argv[1]) if len(sys.argv) > 1 else 1500)
    res = {}
    while time.time() < deadline and len(res) < len(BRANCHES):
        for tag, br in BRANCHES.items():
            if tag in res:
                continue
            rid, st, cc = resolve_run(t, br)
            if not rid:
                print(f"{tag}: run not found yet", flush=True)
                continue
            if st != "completed":
                print(f"{tag} {rid} {st}", flush=True)
                continue
            print(f"{tag} {rid} completed: {cc}", flush=True)
            if cc != "success":
                res[tag] = {"run": rid, "conclusion": cc}
                continue
            d, err = fetch_artifact_members(t, rid, tag)
            if d:
                v = norm_of(d); v.update(run=rid, branch=br, conclusion=cc)
                res[tag] = v
                print(f"{tag}: cpu={v['cpu']} gc_tune={v['gc_tune']} tps_med={v['tps_med']} "
                      f"STW={v['stw_total_s']}s young={v['young_n']}ev/{v['young_avg_ms']}ms "
                      f"fulls={v['fulls']} max={v['max_ms']}ms gate={v['gate_gc2']} "
                      f"bank={v['in_gc3_bank']} cause={v['cause']}", flush=True)
            else:
                res[tag] = {"run": rid, "conclusion": cc, "err": err}
        if len(res) < len(BRANCHES):
            time.sleep(45)
    if {"gc2a", "gc2b"} <= res.keys() and all("stw_total_s" in res[k] for k in ("gc2a", "gc2b")):
        lo = min(res["gc2a"]["stw_total_s"], res["gc2b"]["stw_total_s"])
        hi = max(res["gc2a"]["stw_total_s"], res["gc2b"]["stw_total_s"])
        ready = all(res[k]["gate_gc2"] and res[k]["band"] for k in ("gc2a", "gc2b"))
        print(json.dumps({"corridor_s": [lo, hi], "gc3_bank_s": list(GC3_BANK),
                          "GATE_GC2_READY": ready,
                          "verdict": "GC2-READY" if ready else "REFUTED_CENS"},
                         ensure_ascii=False))
    print(json.dumps(res, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    main()
