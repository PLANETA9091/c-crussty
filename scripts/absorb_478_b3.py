#!/usr/bin/env python3
"""absorb_478_b3.py — [478-B3] chk-14 verdict-closure absorb (A1-метод, канон C55).

ПРЕГИСТ (закон 14a/16, гейты G1-G6 из scripts/dispatch_478_b3_chk14.py, BANK_V5_FREEZE §2 v5-FROZEN):
  leg_v5 = +23.22 FROZEN, порог якоря norm ≤ +3.22, окно GLOB [6.0,9.5]M,
  pair = 23.22 − anchor_norm ≥ +20.00.
  Δcpu(якорь, нога-носитель) ≤ 50k pair-fresh; Distant-нога: frozen-leg cpu
  8687055, окно [8637055,8737055].
  HOST-ценз M1: STW_total ≤23.0s ∧ young_avg ≤200ms (gc.log completion-строки,
  без gc,phases) — иначе HOST-CENSORED (не-якорь, в-точка §3.3).
  Норм A1: polls-median C55 = медиана FIRST-числа строк "TPS from last 5s..." <15.0
  окна 300s (валидация bit-exact W3-c5 +6.85 / W1-s7 selftest Л-478-A1.1);
  norm_v5 = 100·(med/tps_exp_v5(cpu)−1), interp узлов §2, экстраполяция c42.
  Robust-вариант (НЕ вердикт-носитель): c42-Л201 узел [6.9,7.2]M=2.1293.
  Ваниль-валидность: lever_flag=∅, armed(cmp*)=∅, NCDFE=0, AIOOBE=0,
  world afb3a0b3, FIXTURE-VALIDITY VALID (BOTTLENECKS_3.md).
  Ноги (G6): armed-баннер cmp456_chunkmono обязателен; in-band CLEAN → свежий
  leg_norm кросс-чек frozen +23.22 (MAE 5.91пп бюджет ±3.2-3.7пп).
  В-точки vanilla-VALID → банк §3 (пороги НЕ двигать §5).

Usage: absorb_478_b3.py [--run-id ID ...] [--workdir DIR] [--json OUT.json]
"""
import argparse, bisect, json, os, re, shutil, statistics, subprocess, zipfile

REPO = "PLANETA9091/c-crussty"
RUNS = {"a3":   (36367348171, "round-478-b3-a3"),
        "a1r2": (36367509586, "round-478-b3-a1r2"),
        "a2r2": (36367591813, "round-478-b3-a2r2"),
        "leg1": (36367349398, "round-478-b3-leg1"),
        "leg2": (36367350635, "round-478-b3-leg2")}
ANCHORS = ("a3", "a1r2", "a2r2")
V5 = [(6.5e6, 2.1252), (7.0e6, 2.2047), (7.5e6, 2.3271), (8.2e6, 2.4732),
      (8.7e6, 2.5981), (9.0e6, 2.6280)]
LOCAL_70, LOCAL_LO, LOCAL_HI = 2.1293, 6.9e6, 7.2e6   # Л201 robust, не вердикт
BAND = (6.0e6, 9.5e6)
THRESH, LEG_V5 = 3.22, 23.22                            # chk-14 FROZEN §2
FROZEN_LEG_CPU, LEG_WIN = 8_687_055, (8_637_055, 8_737_055)
LEG_LEVER = "cmp456_chunkmono"
STW_MAX_S, YOUNG_MAX_MS, PAIR_FRESH = 23.0, 200.0, 50_000
WORLD_SHA = "afb3a0b3"

RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
RE_IDX = re.compile(r"runner_cpu_index[:=]\s*(\d+)")
RE_ARMED = re.compile(r"(cmp[0-9a-z_]+): ARMED")
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
TPS_MAX_VALID = 15.0
CANON_ENV = {"gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
             "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
             "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
             "region_steal": "0", "population_target": "150000", "fake_players": "4"}


def tps_exp_v5(cpu, local_node=False):
    if local_node and LOCAL_LO <= cpu <= LOCAL_HI:
        return LOCAL_70, "Л201-local"
    xs = [p[0] for p in V5]; ys = [p[1] for p in V5]
    if cpu <= xs[0]:
        return ys[0] + (ys[1]-ys[0])/(xs[1]-xs[0])*(cpu-xs[0]), "extrap-low"
    if cpu >= xs[-1]:
        return ys[-1] + (ys[-1]-ys[-2])/(xs[-1]-xs[-2])*(cpu-xs[-1]), "extrap-high"
    i = bisect.bisect_left(xs, cpu)
    return ys[i-1] + (ys[i]-ys[i-1])/(xs[i]-xs[i-1])*(cpu-xs[i-1]), "interp"


def parse_gc(text):
    stw = {"total_ms": 0.0, "max_ms": 0.0, "pauses": 0, "full": 0, "young": 0,
           "full_cc": 0, "full_md": 0, "full_other": 0,
           "full_sum_ms": 0.0, "young_sum_ms": 0.0}
    for line in text.splitlines():
        if "Pause" not in line or "[gc,phases" in line:
            continue
        m = PAUSE_COMPL.search(line)
        if not m:
            continue
        dur = float(m.group(1))
        stw["total_ms"] += dur; stw["pauses"] += 1
        stw["max_ms"] = max(stw["max_ms"], dur)
        if "Pause Full" in line:
            stw["full"] += 1; stw["full_sum_ms"] += dur
            if "CodeCache" in line: stw["full_cc"] += 1
            elif "Metadata" in line: stw["full_md"] += 1
            else: stw["full_other"] += 1
        else:
            stw["young"] += 1; stw["young_sum_ms"] += dur
    return stw


def fetch_members(t, rid, workdir, tag):
    """Артефакт → in-place zipfile извлечение 4 членов (Л-478-A1.2d, диск 94%)."""
    arts = json.loads(subprocess.run(
        ["curl", "-s", "-H", f"Authorization: token {t}",
         f"https://api.github.com/repos/{REPO}/actions/runs/{rid}/artifacts"],
        capture_output=True, text=True, timeout=90).stdout).get("artifacts", [])
    a = next((x for x in arts if x["name"] == "world3-bench"), None)
    if not a:
        return None, "no world3-bench artifact"
    zpath = os.path.join(workdir, f"{tag}_a.zip")
    p = subprocess.run(["curl", "-sL", "-H", f"Authorization: token {t}",
                        f"https://api.github.com/repos/{REPO}/actions/artifacts/{a['id']}/zip",
                        "-o", zpath], capture_output=True, timeout=600)
    if p.returncode != 0 or open(zpath, "rb").read(2) != b"PK":
        return None, "download/PK fail"
    d = os.path.join(workdir, tag)
    os.makedirs(d, exist_ok=True)
    with zipfile.ZipFile(zpath) as z:
        names = z.namelist()
        for want in ("run-env.txt", "server-stdout.log", "gc.log", "BOTTLENECKS_3.md"):
            srcs = [n for n in names if n.endswith(want)]
            if not srcs:
                if want == "BOTTLENECKS_3.md":
                    continue
                return None, f"missing {want}"
            with z.open(srcs[0]) as src, open(os.path.join(d, want), "wb") as dst:
                shutil.copyfileobj(src, dst)
    os.remove(zpath)
    return d, ""


def norm_of(d, tag):
    env = open(os.path.join(d, "run-env.txt"), errors="replace").read()
    log = open(os.path.join(d, "server-stdout.log"), errors="replace").read()
    gct = open(os.path.join(d, "gc.log"), errors="replace").read()
    bot = ""
    bp = os.path.join(d, "BOTTLENECKS_3.md")
    if os.path.exists(bp):
        bot = open(bp, errors="replace").read()
    env_kv = {}
    for l in env.splitlines():
        kv = l.split(":", 1)
        if len(kv) == 2:
            env_kv[kv[0].strip()] = kv[1].strip()
    cpu = int(re.search(RE_IDX, env).group(1)) if re.search(RE_IDX, env) else 0
    tps_all = [float(x) for x in RE_TPS.findall(log)]
    polls = [x for x in tps_all if x < TPS_MAX_VALID]
    med = statistics.median(polls) if polls else 0.0        # C55 вердикт-носитель
    med5 = statistics.median(polls[:5]) if polls else 0.0   # robust first-5
    exp, exp_tag = tps_exp_v5(cpu)
    exp_c42, _ = tps_exp_v5(cpu, local_node=True)
    norm = 100 * (med / exp - 1) if med else float("nan")
    norm_c42 = 100 * (med / exp_c42 - 1) if med else float("nan")
    norm5 = 100 * (med5 / exp - 1) if med5 else float("nan")
    s = parse_gc(gct)
    n = s["pauses"]
    all_avg = s["total_ms"] / n if n else 0.0
    young_avg = s["young_sum_ms"] / s["young"] if s["young"] else 0.0
    host = bool(n) and (s["total_ms"] > STW_MAX_S * 1000 or
                        (s["young"] and young_avg > YOUNG_MAX_MS))
    armed = sorted(set(RE_ARMED.findall(log)))
    ncdfe = "NoClassDefFoundError" in log
    aioobe = "ArrayIndexOutOfBoundsException" in log
    valid = "FIXTURE-VALIDITY: VALID" in bot
    canon_bad = [k for k, v in CANON_ENV.items()
                 if not env_kv.get(k, "").startswith(v)]
    world_ok = WORLD_SHA in env_kv.get("world_sha256", "")
    is_anchor = tag in ANCHORS
    lever_env = env_kv.get("lever_flag", "")
    if is_anchor:
        vanilla_ok = (not canon_bad and world_ok and not armed
                      and lever_env in ("", "null", "None") and not ncdfe and not aioobe)
    else:
        vanilla_ok = world_ok and not ncdfe and not aioobe
    in_band = BAND[0] <= cpu <= BAND[1]
    clean = bool(n) and not host
    if not in_band:
        verdict = "BAND-DEAD"
    elif not valid:
        verdict = "FIXTURE-INVALID"
    elif is_anchor and not vanilla_ok:
        verdict = "FIXTURE-INVALID"
    elif host:
        verdict = "HOST-CENSORED"
    elif is_anchor:
        verdict = "ANCHOR-LEGAL" if norm <= THRESH else "MISS-NORM"
    else:
        verdict = "LEG-OK" if LEG_LEVER in armed else "LEG-ARM-MISSING"
    pair = LEG_V5 - norm if med else float("nan")
    return {
        "tag": tag, "run_id": RUNS[tag][0], "branch": RUNS[tag][1],
        "verdict": verdict, "cpu_index": cpu, "in_band": in_band,
        "n_polls": len(polls), "tps_polls": polls, "tps_med": round(med, 4),
        "tps_med_first5": round(med5, 4), "tps_exp_v5": round(exp, 4),
        "exp_tag": exp_tag, "tps_exp_c42_robust": round(exp_c42, 4),
        "norm_v5": round(norm, 2), "norm_c42_robust": round(norm_c42, 2),
        "norm_first5_robust": round(norm5, 2), "threshold": THRESH,
        "pair_conv": round(pair, 2),
        "clean_M1": {"stw_total_s": round(s["total_ms"] / 1000, 2),
                     "all_avg_ms": round(all_avg, 1),
                     "young_avg_ms": round(young_avg, 1),
                     "max_pause_ms": round(s["max_ms"], 1),
                     "young_n": s["young"], "full_n": s["full"],
                     "full_cc": s["full_cc"], "full_md": s["full_md"],
                     "host": host},
        "fixture_valid": valid, "ncdfe": ncdfe, "aioobe": aioobe,
        "armed": armed, "lever_flag_env": lever_env,
        "vanilla": {"canon_env_bad": canon_bad, "world_sha_ok": world_ok},
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-id", type=int, action="append")
    ap.add_argument("--workdir", default="/home/z/rounds/ROUND-478/B3/art")
    ap.add_argument("--json", dest="outjson", default=None)
    a = ap.parse_args()
    os.makedirs(a.workdir, exist_ok=True)
    t = open("/tmp/gh_token").read().strip()
    tags = RUNS.keys() if not a.run_id else [
        k for k, (rid, _) in RUNS.items() if rid in a.run_id]
    res = []
    for tag in tags:
        rid = RUNS[tag][0]
        st = json.loads(subprocess.run(
            ["curl", "-s", "-H", f"Authorization: token {t}",
             f"https://api.github.com/repos/{REPO}/actions/runs/{rid}"],
            capture_output=True, text=True, timeout=90).stdout)
        if st.get("status") != "completed":
            print(f"{tag} {rid}: NOT-COMPLETED status={st.get('status')} — skip", flush=True)
            continue
        if st.get("conclusion") != "success":
            r = {"tag": tag, "run_id": rid, "verdict": f"FAIL-{st.get('conclusion')}"}
            print(json.dumps(r, ensure_ascii=False), flush=True)
            res.append(r)
            continue
        d, err = fetch_members(t, rid, a.workdir, tag)
        if not d:
            r = {"tag": tag, "run_id": rid, "verdict": "NO-ARTIFACT", "err": err}
        else:
            r = norm_of(d, tag)
            r["verdict_final"] = (
                "PAIR-ANCHOR" if r["verdict"] == "ANCHOR-LEGAL"
                and any(abs(r["cpu_index"] - lg.get("cpu_index", 0)) <= PAIR_FRESH
                        or LEG_WIN[0] <= r["cpu_index"] <= LEG_WIN[1]
                        for lg in res if lg["tag"].startswith("leg")) else None)
        print(json.dumps(r, ensure_ascii=False), flush=True)
        res.append(r)
    # PAIR-сводка (G5): pair = 23.22 − anchor_norm по КАЖДОМУ легальному якорю,
    # Δcpu(якорь, нога-носитель) ≤50k pair-fresh ИЛИ anchor в frozen-leg окне.
    leg_cpus = [r["cpu_index"] for r in res
                if r["tag"].startswith("leg") and "cpu_index" in r]
    anchors = []
    for r in res:
        if r["tag"] in ANCHORS and r.get("verdict") == "ANCHOR-LEGAL":
            dfresh = [abs(r["cpu_index"] - lc) for lc in leg_cpus]
            ok = min(dfresh) <= PAIR_FRESH if leg_cpus else \
                LEG_WIN[0] <= r["cpu_index"] <= LEG_WIN[1]
            anchors.append({**{k: r[k] for k in ("tag", "run_id", "cpu_index",
                                                 "norm_v5", "pair_conv")},
                            "delta_cpu_min": min(dfresh) if leg_cpus else None,
                            "pair_fresh_ok": ok,
                            "frozen_leg_window_ok": LEG_WIN[0] <= r["cpu_index"] <= LEG_WIN[1]})
    leg_cross = [{"tag": r["tag"], "run_id": r["run_id"], "cpu_index": r.get("cpu_index"),
                  "norm_v5": r.get("norm_v5"),
                  "cross_frozen_23.22_delta": (round(r["norm_v5"] - LEG_V5, 2)
                                               if r.get("norm_v5") == r.get("norm_v5") else None),
                  "cross_in_budget": (abs(r["norm_v5"] - LEG_V5) <= 3.7
                                      if r.get("norm_v5") == r.get("norm_v5") else None)}
                 for r in res if r["tag"].startswith("leg") and "norm_v5" in r]
    summary = {"leg_v5": LEG_V5, "threshold": THRESH, "bar": 20.0,
               "legal_anchors": anchors,
               "pair_values": [an["pair_conv"] for an in anchors if an["pair_fresh_ok"]],
               "pair_certified": any(an["pair_fresh_ok"] for an in anchors),
               "legs_cross_check": leg_cross,
               "bank_points_s3": [{"tag": r["tag"], "run_id": r["run_id"],
                                   "cpu_index": r.get("cpu_index"),
                                   "norm_v5": r.get("norm_v5"),
                                   "stw": r.get("clean_M1", {}).get("stw_total_s"),
                                   "young_avg": r.get("clean_M1", {}).get("young_avg_ms"),
                                   "class": ("HOST-CENSORED-VALID" if r.get("verdict") == "HOST-CENSORED"
                                             else "vanilla-VALID-CLEAN" if r.get("verdict") in ("ANCHOR-LEGAL", "MISS-NORM")
                                             else r.get("verdict"))}
                                  for r in res if r["tag"] in ANCHORS and "norm_v5" in r]}
    print("SUMMARY " + json.dumps(summary, ensure_ascii=False), flush=True)
    if a.outjson:
        with open(a.outjson, "w") as f:
            json.dump({"runs": res, "summary": summary}, f, ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
