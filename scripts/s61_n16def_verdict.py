#!/usr/bin/env python3
"""s61_n16def_verdict.py — ROUND-470 S61 (ИМПЛ): авто-вердикт пина №11 (n16def-verify).

Поллит run 36268090827 (ветка round-470-n16def-verify @ МЕРЖ №11 b3853246, yml lever_arg=""):
- run в полёте → печать **DISPATCHED <run-id>** (канон 12e), exit 0;
- completed → артефакт world3-bench → вердикт-ЧИСЛО (norm v5, пп).

ГЕЙТЫ ПИНА (preregister, закон 14a; канон BLACKBOARD IN-FLIGHT «epoch ok n=16 при пустом
LEVER_ARG = пин работает»):
- G1 EPOCH: строка «cmp466_c98ai: epoch ok tick=N windowLen=W n=16» в server-stdout.log
  (mobai-window эхо пина; N≥1) — при пустом LEVER_ARG решает дефолт блоба.
- G2 ARM-БАННЕРЫ «default 16» ×2: AI-лейн cmp406_aibatch («N from env CRUSSTY_AI_N/LEVER_ARG
  default 16») + PUSH-лейн cmp466_c98ai stagger («N from env CRUSSTY_STAGGER_N/LEVER_ARG
  default 16») — оба лейна пина на дефолте 16 (AI16/PUSH16, Л216).
- G3 STW-ЦЕНЗ M1 (Л-466-C15): stw_total ≤23.0s ∧ young_avg ≤200ms (gc.log-канон
  completion-строк LEDGER-118; boot-MD-сплит по uptime≤30s/live<600M — репорт обоих).
- G4 NORM v5 (Л195+Л201): cpu ТОЛЬКО из run-env.txt (kill-класс gate-echo Л195);
  tps_exp_v5 с Л201-локальным узлом [6.9,7.2]M=2.1293 (узел вшит, вне окна = interp);
  norm_pp = 100*(median_bench/tps_exp − 1); median = медиана «TPS from last 5s» <15.
- ВАЛИДНОСТЬ (репорт, не гейт — нога ARMED ≠ ваниль-в-точка): ncdfe_real / aioobe /
  selftest_fail / err_struct счётчики + POPULATION INJECT DONE.

ВЕРДИКТ-ЧИСЛО = norm_pp (≥+20 = пара-канон-класс якоря-n16; pin_ok = G1∧G2∧G3∧band).
Нога ARMED (cmp466_c98ai, n=16) → в банк v5 НЕ идёт (§3 требует armed=null∧n=null) —
это ARM-пруф-нога МЕРЖа №11. Итог-канон: DISPATCHED run-id если артефакт не готов.

Канон-техника (вендор прецедентов): curl -sL для artifact-zip (urllib-403 урок, PK-магия);
urllib только api.github.com JSON; избирательный unzip 3 малых членов (диск-фругал);
--selftest офлайн. Usage:
  s61_n16def_verdict.py [--run-id 36268090827] [--workdir DIR] [--offline --fixture-dir DIR]
"""
import argparse, bisect, json, os, re, statistics, subprocess, sys, tempfile, urllib.request

REPO = "PLANETA9091/c-crussty"
DEFAULT_RUN = 36268090827
ARTIFACT_NAME = "world3-bench"
MEMBERS = ["run-env.txt", "server-stdout.log", "gc.log"]   # избирательно: без 42MB cpu-collapsed
PIN_BRANCH = "round-470-n16def-verify"
PIN_SHA = "b385324677ac76f4f9f8ef942a3835c8004aef78"        # МЕРЖ №11

# ---- норм-модель v5 + Л201-локальный узел (вендор absorb_next.py, BANK_V5_FREEZE) ----
V5 = [(6.5, 2.1252), (7.0, 2.2047), (7.5, 2.3271), (8.2, 2.4732), (8.7, 2.5981), (9.0, 2.6280)]
LOCAL_70 = 2.1293                     # Л201-локальный узел [6.9,7.2]M (узел 2.2047 дефектен, Л213)
LOCAL_LO, LOCAL_HI = 6.9e6, 7.2e6
BAND = (6.0e6, 9.5e6)                 # канон band
STW_LIMIT_S = 23.0                    # HOST-ценз M1
YOUNG_AVG_LIMIT_MS = 200.0

# ---- gc.log M1-канон (вендор gc_parse_canon.py: completion-строки, boot/bench сплит) ----
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")
HEAP = re.compile(r"\d+G?M->(\d+)M\((\d+)M\)")
BOOT_MD_MAX_UPTIME_S = 30.0
BOOT_MD_MAX_LIVE_M = 600

# ---- server-stdout паттерны пина ----
RE_EPOCH = re.compile(r"cmp466_c98ai: epoch ok tick=(\d+) windowLen=(\d+) n=(\d+)")
RE_BANNER16 = re.compile(r"N from env (?:CRUSSTY_AI_N|CRUSSTY_STAGGER_N)/LEVER_ARG default (\d+)")
RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
RE_ARMED = re.compile(r"(cmp[0-9a-z_]+): ARMED")


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, path):
    """urllib ТОЛЬКО api.github.com JSON (S3-редирект = curl-канон, urllib-403 урок)."""
    req = urllib.request.Request(f"https://api.github.com/repos/{REPO}/{path}",
                                 headers={"Authorization": f"token {tok}",
                                          "Accept": "application/vnd.github+json"})
    with urllib.request.urlopen(req, timeout=90) as r:
        return json.load(r)


def curl_download(tok, url, dest):
    """Канон скачивания: curl -sL strip-ает auth на cross-host S3-редиректе; PK-магия."""
    p = subprocess.run(["curl", "-sL", "-H", f"Authorization: token {tok}",
                        "-H", "Accept: application/vnd.github+json", url, "-o", dest],
                       capture_output=True, timeout=600)
    if p.returncode != 0:
        return False, f"curl rc={p.returncode}"
    with open(dest, "rb") as f:
        if f.read(2) != b"PK":
            return False, "not-a-zip (S3-error body?)"
    return True, ""


def tps_exp_v5(cpu):
    """v5-кривая + Л201-локальный узел (гейт обязан потреблять узел в окне, Л213)."""
    if LOCAL_LO <= cpu <= LOCAL_HI:
        return LOCAL_70, "Л201-local"
    xs = [p[0] for p in V5]
    ys = [p[1] for p in V5]
    c = cpu / 1e6
    if c <= xs[0]:
        return ys[0] + (ys[1] - ys[0]) / (xs[1] - xs[0]) * (c - xs[0]), "extrap-low"
    if c >= xs[-1]:
        return ys[-1] + (ys[-1] - ys[-2]) / (xs[-1] - xs[-2]) * (c - xs[-1]), "extrap-high"
    i = bisect.bisect_left(xs, c)
    return ys[i - 1] + (ys[i] - ys[i - 1]) / (xs[i] - xs[i - 1]) * (c - xs[i - 1]), "interp"


def norm_pp(median, exp):
    return round(100 * (median / exp - 1), 2)


def _lane(cause):
    c = cause.strip()
    return "md" if c.startswith("Metadata") else ("cc" if c.startswith("CodeCache") else "erg")


def _is_boot_md(p):
    if p["uptime_s"] is not None and p["uptime_s"] <= BOOT_MD_MAX_UPTIME_S:
        return True
    return p["post_live_m"] is not None and p["post_live_m"] < BOOT_MD_MAX_LIVE_M


def parse_gc(text):
    """M1-канон: completion-строки Pause (не gc,start/[gc,phases]); boot-MD сплит."""
    pauses = []
    for line in text.splitlines():
        if "Pause" not in line or "[gc,phases" in line:
            continue
        m = PAUSE_COMPL.search(line)
        if not m:
            continue
        um, cm, hm = UPTIME.search(line), CAUSE.search(line), HEAP.search(line)
        pauses.append({"uptime_s": float(um.group(1)) if um else None,
                       "dur_ms": float(m.group(1)),
                       "kind": cm.group(1) if cm else "?",
                       "lane": _lane(cm.group(2)) if cm else "erg",
                       "post_live_m": int(hm.group(1)) if hm else None})
    first_nonmd = next((p["uptime_s"] for p in pauses if p["lane"] != "md"), None)
    md_boot = [p["uptime_s"] for p in pauses
               if p["kind"] == "Full" and p["lane"] == "md" and _is_boot_md(p)]
    cands = [t for t in (first_nonmd, max(md_boot) if md_boot else None) if t is not None]
    boot_end = max(cands) if cands else None
    s = {"young": 0, "full": 0, "boot_md": 0, "bench_md": 0, "full_cc": 0, "full_erg": 0,
         "total_ms": 0.0, "boot_stw_ms": 0.0, "bench_stw_ms": 0.0,
         "young_sum_ms": 0.0, "max_ms": 0.0}
    for p in pauses:
        d, u = p["dur_ms"], p["uptime_s"]
        in_boot = (u is not None and boot_end is not None and u < boot_end) or \
                  (p["kind"] == "Full" and p["lane"] == "md" and _is_boot_md(p))
        s["total_ms"] += d
        s["max_ms"] = max(s["max_ms"], d)
        s["boot_stw_ms" if in_boot else "bench_stw_ms"] += d
        if p["kind"] == "Full":
            s["full"] += 1
            if p["lane"] == "md":
                s["boot_md" if in_boot else "bench_md"] += 1
            elif p["lane"] == "cc":
                s["full_cc"] += 1
            else:
                s["full_erg"] += 1
        else:
            s["young"] += 1
            s["young_sum_ms"] += d
    s["young_avg_ms"] = round(s["young_sum_ms"] / s["young"], 2) if s["young"] else 0.0
    return s


def parse_stdout(text):
    """G1 epoch-n16, G2 баннеры default-16, armed, bench-TPS-медиана, валидность."""
    v = {"epoch_ok": False, "epoch_tick": None, "epoch_window_len": None, "n": None,
         "banners16": [], "armed": [], "tps_rows": 0, "median": None,
         "ncdfe_real": 0, "aioobe": 0, "selftest_fail": 0, "err_struct": 0,
         "inject_done": False}
    m = RE_EPOCH.search(text)
    if m:
        v["epoch_ok"] = True
        v["epoch_tick"] = int(m.group(1))
        v["epoch_window_len"] = int(m.group(2))
        v["n"] = int(m.group(3))
    v["banners16"] = RE_BANNER16.findall(text)
    v["armed"] = sorted(set(RE_ARMED.findall(text)))
    tps = [float(x) for x in RE_TPS.findall(text)]
    v["tps_rows"] = len(tps)
    bench = [x for x in tps if x < 15]                      # boot-полл >15 мимо (канон absorb)
    if bench:
        v["median"] = statistics.median(bench)
    v["ncdfe_real"] = len(re.findall(r"NoClassDefFoundError", text))
    v["aioobe"] = len(re.findall(r"ArrayIndexOutOfBounds", text))
    v["selftest_fail"] = len(re.findall(r"selftest FAIL", text))
    v["err_struct"] = len(re.findall(r"ERR_STRUCT", text))
    v["inject_done"] = "POPULATION INJECT DONE" in text
    return v


def parse_runenv(text):
    """Л195: cpu ТОЛЬКО из run-env.txt."""
    m = re.search(r"runner_cpu_index:\s*(\d+)", text)
    mt = re.search(r"^gc_tune:\s*(\d+)", text, re.M)
    mp = re.search(r"^population_target:\s*(\d+)", text, re.M)
    return {"cpu": int(m.group(1)) if m else None,
            "gc_tune": int(mt.group(1)) if mt else None,
            "pop": int(mp.group(1)) if mp else None}


def verdict_from_dir(d, rid, meta=None):
    """Собрать вердикт из каталога с MEMBERS. pin_ok = G1∧G2(≥2)∧G3∧band."""
    v = {"claim": "S61", "run": rid, "branch": PIN_BRANCH, "sha": PIN_SHA[:8]}
    if meta:
        v["conclusion"] = meta.get("conclusion")
    env = open(os.path.join(d, "run-env.txt"), errors="replace").read()
    log = open(os.path.join(d, "server-stdout.log"), errors="replace").read()
    gct = open(os.path.join(d, "gc.log"), errors="replace").read()
    v.update(parse_runenv(env))
    v.update(parse_stdout(log))
    gc = parse_gc(gct)
    v["stw_total_s"] = round(gc["total_ms"] / 1000, 2)
    v["bench_stw_s"] = round(gc["bench_stw_ms"] / 1000, 2)
    v["young_avg_ms"] = gc["young_avg_ms"]
    v["full"] = gc["full"]
    v["full_lanes"] = f"boot_md={gc['boot_md']}/bench_md={gc['bench_md']}/cc={gc['full_cc']}/erg={gc['full_erg']}"
    # гейты
    v["band"] = v["cpu"] is not None and BAND[0] <= v["cpu"] <= BAND[1]
    v["g1_epoch_n16"] = v["epoch_ok"] and v["n"] == 16
    v["g2_banners16_n"] = len(v["banners16"])
    v["g2_banners_default16"] = v["g2_banners16_n"] >= 2
    v["g3_stw_clean"] = v["stw_total_s"] <= STW_LIMIT_S and v["young_avg_ms"] <= YOUNG_AVG_LIMIT_MS
    if v["cpu"] is not None and v["median"] is not None:
        e, tag = tps_exp_v5(v["cpu"])
        v["tps_exp"] = round(e, 4)
        v["exp_tag"] = tag
        v["norm_pp"] = norm_pp(v["median"], e)
        v["host_cens"] = not v["g3_stw_clean"]
    v["pin_ok"] = bool(v["g1_epoch_n16"] and v["g2_banners_default16"]
                       and v["g3_stw_clean"] and v["band"])
    v["bank_note"] = "ARM-пруф-нога: armed!=null → банк §3 НЕ пополняет (armed/n != null)"
    v["verdict"] = v.get("norm_pp")
    return v


def emit(v):
    print(json.dumps(v, ensure_ascii=False))
    num = v.get("verdict")
    if num is not None:
        sign = "+" if num >= 0 else ""
        bar = ">=+20 (пара-канон-класс)" if num >= 20 else "<+20"
        print(f"VERDICT {sign}{num} пп [{bar}] pin_ok={v['pin_ok']} "
              f"(epoch_n16={v['g1_epoch_n16']}, banners16={v['g2_banners16_n']}, "
              f"STW={v['stw_total_s']}s/{'CLEAN' if v['g3_stw_clean'] else 'CENS'}, "
              f"cpu={v['cpu']}/{v.get('exp_tag')})")


def poll_and_verdict(rid, workdir):
    tok = token()
    try:
        meta = api(tok, f"actions/runs/{rid}")
    except Exception as e:
        print(f"ERROR api: {e}")
        return 2
    status, concl = meta.get("status"), meta.get("conclusion")
    branch, sha = meta.get("head_branch"), (meta.get("head_sha") or "")[:8]
    if status != "completed":
        # канон: в полёте = DISPATCHED run-id (12e), вердикт-числа нет
        print(f"DISPATCHED {rid} status={status} branch={branch}@{sha} — артефакт не готов, "
              f"вердикт-число добьёт завершение (canon 12e)")
        return 0
    if concl != "success":
        print(f"ERROR run {rid} conclusion={concl} — fail-closed, вердикта нет")
        return 1
    arts = api(tok, f"actions/runs/{rid}/artifacts")
    aid = next((a["id"] for a in arts.get("artifacts", []) if a["name"] == ARTIFACT_NAME), None)
    if not aid:
        print(f"ERROR no {ARTIFACT_NAME} artifact (completed, concl={concl})")
        return 1
    os.makedirs(workdir, exist_ok=True)
    have = all(os.path.isfile(os.path.join(workdir, m)) for m in MEMBERS)
    if not have:
        z = os.path.join(workdir, "a.zip")
        ok, err = curl_download(tok, f"https://api.github.com/repos/{REPO}/actions/artifacts/{aid}/zip", z)
        if not ok:
            print(f"ERROR download: {err}")
            return 1
        for m in MEMBERS:
            subprocess.run(["unzip", "-o", "-q", z, m, "-d", workdir], capture_output=True)
        os.remove(z)                                       # диск-фругал: zip 28MB сразу
    miss = [m for m in MEMBERS if not os.path.isfile(os.path.join(workdir, m))]
    if miss:
        print(f"ERROR artifact members missing: {miss}")
        return 1
    v = verdict_from_dir(workdir, rid, meta={"conclusion": concl})
    emit(v)
    with open(os.path.join(workdir, "verdict.json"), "w") as f:
        json.dump(v, f, ensure_ascii=False, indent=1)
    return 0


# ---------------- selftest (офлайн, сеть не трогается) ----------------
def selftest():
    fails = []

    def check(name, cond):
        print(f'  [{"PASS" if cond else "FAIL"}] {name}')
        if not cond:
            fails.append(name)

    print("selftest s61_n16def_verdict.py:")
    # К1: Л201-узел + v5-кривая
    e, tag = tps_exp_v5(7_000_000)
    check("К1 Л201-узел 7.0M → 2.1293/Л201-local", abs(e - 2.1293) < 1e-9 and tag == "Л201-local")
    check("К1 границы окна 6.9M/7.2M → узел",
          tps_exp_v5(6_900_000)[1] == "Л201-local" and tps_exp_v5(7_200_000)[1] == "Л201-local")
    check("К1 вне окна 8.844662M → interp-кривая", tps_exp_v5(8_844_662)[1] == "interp")
    check("К1 knot 8.2M → 2.4732", abs(tps_exp_v5(8_200_000)[0] - 2.4732) < 1e-9)
    # К2: norm-математика
    check("К2 norm 3.3/2.6125 → +26.32", norm_pp(3.3, 2.6125) == 26.32)
    check("К2 norm отрицательный", norm_pp(2.1, 2.2047) == -4.75)
    # К3: epoch + баннеры + TPS фикс-трина
    fx = ("[x] [crussty-plugin] cmp466_c98ai: ARMED stagger (push=retargeted 1 site, goal=retargeted 1 site, "
          "N from env CRUSSTY_STAGGER_N/LEVER_ARG default 16)\n"
          "[x] [crussty-plugin] cmp406_aibatch: ARMED mob-ai-window (… N from env CRUSSTY_AI_N/LEVER_ARG "
          "default 16; rust aiEpoch …)\n"
          "[20:11:29 INFO]: [crussty-plugin] [crussty-plugin] cmp466_c98ai: epoch ok tick=24 windowLen=0 n=16 "
          "(bulk JNI 1/tick over soa population)\n"
          "TPS from last 5s, 1m, 5m, 15m: 23.7, 23.7, 23.7, 23.7\n"
          "TPS from last 5s, 1m, 5m, 15m: 2.5, 2.2, 1.8, 1.8\n"
          "TPS from last 5s, 1m, 5m, 15m: 2.8, 2.6, 2.0, 2.0\n"
          "TPS from last 5s, 1m, 5m, 15m: 3.3, 3.0, 2.2, 2.2\n"
          "TPS from last 5s, 1m, 5m, 15m: 3.4, 3.3, 2.6, 2.4\n"
          "TPS from last 5s, 1m, 5m, 15m: 3.3, 3.3, 2.9, 2.6\n"
          "biomes selftest FAIL (throwable java.lang.ArrayIndexOutOfBoundsException: Index 1)\n")
    p = parse_stdout(fx)
    check("К3 epoch ok tick=24 n=16", p["epoch_ok"] and p["epoch_tick"] == 24 and p["n"] == 16)
    check("К3 баннеры default 16 ×2 (AI+PUSH лейны)", len(p["banners16"]) == 2)
    check("К3 median bench = 3.3 (boot 23.7 отфильтрован)", p["median"] == 3.3 and p["tps_rows"] == 6)
    check("К3 selftest_fail/aioobe счётчики", p["selftest_fail"] == 1 and p["aioobe"] == 1)
    # К4: gc M1-канон — boot-каскад vs bench
    gcf = "\n".join([
        "[t][2.100s][info][gc] GC(0) Pause Young (Metadata GC Threshold) 898M->13M(3925M) 5.258ms",
        "[t][2.189s][info][gc] GC(1) Pause Full (Metadata GC Threshold) 13M->13M(3925M) 16.171ms",
        "[t][13.439s][info][gc] GC(3) Pause Young (Allocation Failure) 1295M->315M(3925M) 26.693ms",
        "[t][47.712s][info][gc] GC(5) Pause Full (Metadata GC Threshold) 1063M->953M(5815M) 1244.026ms",
        "[t][100.000s][info][gc] GC(9) Pause Young (Allocation Failure) 1296M->347M(3925M) 41.628ms",
        "[t][100.500s][info][gc,start] GC(10) Pause Full (CodeCache GC Threshold)",
    ])
    g = parse_gc(gcf)
    check("К4 boot-MD=1 (каскад 1 Full), bench-MD=1 (953M@47.7s за окном), cc не считан по start-строке",
          g["boot_md"] == 1 and g["bench_md"] == 1 and g["full_cc"] == 0)
    check("К4 totals: 1333.776ms, young_avg 24.53", abs(g["total_ms"] - 1333.776) < 0.01
          and g["young_avg_ms"] == 24.53)
    # К5: runenv парсер (Л195)
    r = parse_runenv("runner_cpu_index: 8844662 (iters/s fixed 6M-step LCG loop)\ngc_tune: 3 (…)\n"
                     "population_target: 150000 (…)\n")
    check("К5 cpu 8844662/gc_tune 3/pop 150000 из run-env",
          r == {"cpu": 8844662, "gc_tune": 3, "pop": 150000})
    # К6: вердикт-сборка на синтетике — пин OK
    with tempfile.TemporaryDirectory() as td:
        open(os.path.join(td, "run-env.txt"), "w").write(
            "runner_cpu_index: 8844662 (x)\ngc_tune: 3 (x)\npopulation_target: 150000 (x)\n")
        open(os.path.join(td, "server-stdout.log"), "w").write(fx + "POPULATION INJECT DONE target=150000\n")
        open(os.path.join(td, "gc.log"), "w").write(gcf)
        v = verdict_from_dir(td, DEFAULT_RUN)
    check("К6 pin_ok=True (G1∧G2∧G3∧band), verdict-число +26.3x",
          v["pin_ok"] is True and v["verdict"] is not None and 20 < v["verdict"] < 30)
    check("К6 inject_done репортится", v["inject_done"] is True)
    neg = dict(v)
    neg["stw_total_s"] = 27.1
    neg["g3_stw_clean"] = False
    neg["pin_ok"] = bool(neg["g1_epoch_n16"] and neg["g2_banners_default16"]
                         and neg["g3_stw_clean"] and neg["band"])
    check("К6 STW 27.1s → g3 CLEAN=False → pin_ok=False (HOST-ценз)",
          neg["g3_stw_clean"] is False and neg["pin_ok"] is False)

    print(f'selftest: {"ALL PASS" if not fails else "FAIL: " + ", ".join(fails)}')
    return 0 if not fails else 1


def main():
    ap = argparse.ArgumentParser(description="S61 n16def-verify авто-вердикт пина")
    ap.add_argument("--run-id", type=int, default=DEFAULT_RUN)
    ap.add_argument("--workdir", default="/tmp/s61_verdict")
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args()
    if args.selftest:
        return selftest()
    return poll_and_verdict(args.run_id, args.workdir)


if __name__ == "__main__":
    sys.exit(main())
