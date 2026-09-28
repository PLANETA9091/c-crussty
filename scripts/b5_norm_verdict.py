#!/usr/bin/env python3
"""b5_norm_verdict.py — [478-B5] chk-19 anchor norm-математика (A1-метод, канон C55).

CLAIM chk-19 (BANK_V5_FREEZE §2 v5-FROZEN, не двигать):
  leg_v5 = +16.11, порог якоря norm_v5 ≤ −3.89, окно GLOB 6.0–9.5M.
  Существующие легальные пары: a22 → 23.2, a53 → 23.3 (2/3).
  PAIR min-of-3 = min(23.2, 23.3, 16.11 − norm_i) по всем CLEAN ваниль-якорям norm ≤ −3.89.

hit/anchor-legal = cpu(run-env) ∈ band [6.0,9.5]M ∧ norm_v5 ≤ −3.89 ∧ CLEAN M1
  (STW_total ≤ 23.0s ∧ young_avg ≤ 200ms, completion-строки M1-канон без gc,phases)
  ∧ vanilla-valid (lever_flag=∅, NCDFE=0, AIOOBE=0, FIXTURE-VALIDITY: VALID).
norm_v5 = 100*(median_polls / tps_exp_v5(cpu) − 1);
  polls-median = медиана значений "TPS from last 5s" < 15.0 (канон C55);
  tps_exp_v5 = линейная интерполяция узлов §2 (6.5M→2.1252 … 9.0M→2.6280),
  вне узлов — линейная экстраполяция крайними (канон c42).
Robust-вариант (не вердикт-носитель): c42-Л201 локальный узел [6.9,7.2]M = 2.1293.
Usage: b5_norm_verdict.py --run-id ID [--run-id ID ...] [--workdir DIR]
"""
import argparse, json, os, re, statistics, subprocess, zipfile

REPO = "PLANETA9091/c-crussty"
V5 = [(6.5e6, 2.1252), (7.0e6, 2.2047), (7.5e6, 2.3271), (8.2e6, 2.4732),
      (8.7e6, 2.5981), (9.0e6, 2.6280)]
BAND = (6.0e6, 9.5e6)
THRESH, LEG_V5 = -3.89, 16.11
LEG_CPU = 6_733_439  # chk-19 лег +12.8@6733439 (эра ×460) — Δ report-only (окно GLOB)
EXISTING_PAIRS = [23.2, 23.3]  # a22, a53 (v5-ре-скор, BANK §2 "маржи пар 22.5→23.2/22.6→23.3")
STW_MAX_S, YOUNG_MAX_MS = 23.0, 200.0

RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
RE_IDX = re.compile(r"runner_cpu_index[:=]\s*(\d+)")
RE_MSPT = re.compile(r"MSPT.*?avg.*?([\d.]+)", re.I)
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
KIND_CC = re.compile(r"Pause Full \(CodeCache")
KIND_MD = re.compile(r"Pause Full \(Metadata")
TPS_MAX_VALID = 15.0
LOCAL_70, LOCAL_LO, LOCAL_HI = 2.1293, 6.9e6, 7.2e6  # c42 robust-вариант
WORLD_SHA = "afb3a0b3"
# x466-C98 canon-вектор (ваниль-якорь: lever=∅, все axes = банк-канон)
CANON_ENV = {"gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
             "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
             "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
             "region_steal": "0", "population_target": "150000", "fake_players": "4"}


def tps_exp_v5(idx, local_node=False):
    if local_node and LOCAL_LO <= idx <= LOCAL_HI:
        return LOCAL_70
    def interp(lo, hi, idx):
        f = (idx - lo[0]) / (hi[0] - lo[0])
        return lo[1] + f * (hi[1] - lo[1])
    if idx < V5[0][0]:
        return interp(V5[0], V5[1], idx)
    if idx > V5[-1][0]:
        return interp(V5[-2], V5[-1], idx)
    for i in range(len(V5) - 1):
        if V5[i][0] <= idx <= V5[i + 1][0]:
            return interp(V5[i], V5[i + 1], idx)
    return V5[-1][1]


def gc_canon(text):
    stw = {"total_ms": 0.0, "max_ms": 0.0, "pauses": 0, "full": 0, "young": 0,
           "full_cc": 0, "full_md": 0, "full_other": 0,
           "full_sum_ms": 0.0, "young_sum_ms": 0.0}
    for line in text.splitlines():
        if "Pause" not in line:
            continue
        m = PAUSE_COMPL.search(line)
        if not m or "[gc,phases" in line:
            continue
        dur = float(m.group(1))
        stw["total_ms"] += dur
        stw["pauses"] += 1
        stw["max_ms"] = max(stw["max_ms"], dur)
        if "Pause Full" in line:
            stw["full"] += 1
            stw["full_sum_ms"] += dur
            if KIND_CC.search(line):
                stw["full_cc"] += 1
            elif KIND_MD.search(line):
                stw["full_md"] += 1
            else:
                stw["full_other"] += 1
        else:
            stw["young"] += 1
            stw["young_sum_ms"] += dur
    return stw


def absorb(run_id, workdir):
    tok = open("/tmp/gh_token").read().strip()
    base = f"https://api.github.com/repos/{REPO}"
    os.makedirs(workdir, exist_ok=True)
    zpath = os.path.join(workdir, f"art_{run_id}.zip")
    if not os.path.exists(zpath):
        out = subprocess.run(["curl", "-s", "-H", f"Authorization: token {tok}",
                              f"{base}/actions/runs/{run_id}/artifacts"],
                             capture_output=True, text=True).stdout
        art = next((a for a in json.loads(out).get("artifacts", [])
                    if a["name"] == "world3-bench"), None)
        if not art:
            return {"run_id": run_id, "verdict": "NO-ARTIFACT"}
        subprocess.run(["curl", "-sL", "-H", f"Authorization: token {tok}",
                        "-o", zpath, art["archive_download_url"]], check=True)
    with zipfile.ZipFile(zpath) as z:
        names = z.namelist()
        env = z.read("run-env.txt").decode(errors="replace")
        log = z.read("server-stdout.log").decode(errors="replace")
        bot = z.read("BOTTLENECKS_3.md").decode(errors="replace") if "BOTTLENECKS_3.md" in names else ""
        gcn = next((n for n in names if n.endswith("gc.log")), None)
        gclog = z.read(gcn).decode(errors="replace") if gcn else ""

    m = RE_IDX.search(env)
    idx = int(m.group(1)) if m else 0
    env_kv = {}
    for l in env.splitlines():
        kv = l.split(":", 1)
        if len(kv) == 2:
            env_kv[kv[0].strip()] = kv[1].strip()
    canon_bad = [k for k, v in CANON_ENV.items() if not env_kv.get(k, "").startswith(v)]
    world_ok = WORLD_SHA in env_kv.get("world_sha256", "")
    tps = [float(x) for x in RE_TPS.findall(log) if float(x) < TPS_MAX_VALID]
    med = statistics.median(tps) if tps else 0.0
    exp = tps_exp_v5(idx)
    exp_c42 = tps_exp_v5(idx, local_node=True)
    norm = 100 * (med / exp - 1) if med else float("nan")
    norm_c42 = 100 * (med / exp_c42 - 1) if med else float("nan")
    pair = LEG_V5 - norm if med else float("nan")

    s = gc_canon(gclog) if gclog else None
    n = s["pauses"] if s else 0
    all_avg = (s["total_ms"] / n) if s and n else 0.0
    young_avg = (s["young_sum_ms"] / s["young"]) if s and s["young"] else 0.0
    host = bool(s) and (s["total_ms"] > STW_MAX_S * 1000 or
                        (s["young"] and young_avg > YOUNG_MAX_MS))
    valid = "FIXTURE-VALIDITY: VALID" in bot
    ncdfe = "NoClassDefFoundError" in log
    aioobe = "ArrayIndexOutOfBoundsException" in log

    in_band = BAND[0] <= idx <= BAND[1]
    clean = bool(s) and not host
    vanilla_ok = not canon_bad and world_ok
    if not in_band:
        verdict = "BAND-DEAD"
    elif not valid or ncdfe or aioobe or not vanilla_ok:
        verdict = "FIXTURE-INVALID"
    elif host:
        verdict = "HOST-CENSORED"
    elif med and norm <= THRESH:
        verdict = "ANCHOR-LEGAL"
    else:
        verdict = "MISS-NORM"
    return {
        "run_id": run_id, "verdict": verdict,
        "cpu_index": idx, "in_band": in_band, "delta_to_leg_cpu": idx - LEG_CPU,
        "n_tps_polls": len(tps), "tps_polls": tps, "tps_med": round(med, 4),
        "tps_exp_v5": round(exp, 4), "tps_exp_c42_robust": round(exp_c42, 4),
        "norm_v5": round(norm, 2), "norm_c42_robust": round(norm_c42, 2),
        "threshold": THRESH, "pair_conv": round(pair, 2),
        "clean_M1": {"stw_total_s": round(s["total_ms"] / 1000, 2) if s else None,
                     "all_avg_ms": round(all_avg, 1),
                     "young_avg_ms": round(young_avg, 1),
                     "max_pause_ms": round(s["max_ms"], 1) if s else None,
                     "young_n": s["young"] if s else 0,
                     "full_n": s["full"] if s else 0,
                     "full_cc": s["full_cc"] if s else 0, "full_md": s["full_md"] if s else 0,
                     "host": host},
        "fixture_valid": valid, "ncdfe": ncdfe, "aioobe": aioobe,
        "vanilla": {"canon_env_bad": canon_bad, "world_sha_ok": world_ok,
                    "lever_empty": vanilla_ok},
        "mspt_avg_in_report": (RE_MSPT.search(bot).group(1) if RE_MSPT.search(bot) else None),
    }


def min_of_3(results):
    pairs = list(EXISTING_PAIRS)
    for r in results:
        if r.get("verdict") == "ANCHOR-LEGAL":
            pairs.append(round(r["pair_conv"], 2))
    return {"qualifying": pairs, "min_of_3": min(pairs) if len(pairs) >= 3 else None,
            "bar": 20.0, "pair_certified": len(pairs) >= 3 and min(pairs) >= 20.0}


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-id", type=int, action="append", required=True)
    ap.add_argument("--workdir", default="/home/z/rounds/ROUND-478/scripts_b5/art")
    a = ap.parse_args()
    res = [absorb(rid, a.workdir) for rid in a.run_id]
    for r in res:
        print(json.dumps(r, ensure_ascii=False))
    print(json.dumps(min_of_3(res), ensure_ascii=False))
