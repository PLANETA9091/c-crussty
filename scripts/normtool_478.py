#!/usr/bin/env python3
"""normtool_478.py — [478-G24] norm-скрипт автоматизации (A1-метод canon, MEGA-SWARM v19.0).

Вход {run id} → GitHub API артефакт world3-bench → in-place zip parse (Л-478-A1.2d,
диск 94%: без unzip) → BOTTLENECKS_3.md first-of-window polls → медиана (канон C55:
поллы <15.0 — pre-inject пустой мир 20.0-26.4 отсекается) → tps_exp_v5 interp
(BANK_V5_FREEZE §2 сэмплы 6.5M→2.1252 … 9.0M→2.6280 + Л201 robust-узел [6.9,7.2]M=2.1293,
report-only) → norm_v5 = 100*(median/tps_exp_v5(cpu)−1) → HOST-ценз M1
(STW_total ≤23.0s ∧ young_avg ≤200ms из gc.log completion-строк без gc,phases;
гейт-поле m1_clean top-level: fail-closed, gc.log не распарсен → False — [479-G1]
урок: потребители читают m1_clean/stw, None-класс = STW-check блокирован;
[482-C03.1] TRI-STATE m1_state CLEAN/CENS/UNKNOWN: UNKNOWN = gc.log нет/0
completion-строк/young_n==0 при stw_total>0 → m1_clean=False ∧ verdict
M1-UNKNOWN (НЕ PASS — Л-481-C22 fail-open дыра «gc.log есть ∧ 0 строк →
m1_clean=True» закрыта) → JSON.

[482-C03.2] biomes-exempt: CLI-флаг --biomes-exempt — биом-AIOOBE класс
(строка «biomes selftest FAIL (throwable …AIOOBE…» = fail-closed probe
ChunkParseOps.biomesSelftest, прецедент Л-474-C88.2: AIOOBE=2 selftest-проба
не гейт) exempt из FIXTURE-гейта; явные маркеры в JSON: aioobe_biome /
aioobe_other / biomes_exempt / biomes_exempt_applied. Без флага поведение
×481 бит-точное (любой AIOOBE → FIXTURE-INVALID). Реальные (не-probe) AIOOBE
exempt'ом НЕ снимаются (redstone/entity-классы → FIXTURE-INVALID как раньше).

Уроки-каноны (НЕ нарушать):
  A1.2a percentile-медианы НЕТ в артефактах (spark печатает Max/Min/Average only) —
       рабочая прокси = медиана TPS-polls;
  A1.2b polls-vs-spark дивергенция до 24.9пп — raw-поллы ОБЯЗАНЫ храниться в JSON
       (raw_polls_stdout / polls_first_of_window / polls_valid_c55) + spark-кросс-чек;
  B3   exp-юнит-баг (cpu/1e6 vs узлы-герцы) ловится selftest — selftest обязателен
       перед каждым вердиктом.

Пороги/окна BANK_V5_FREEZE §2 v5-FROZEN — только report-only, НЕ двигать (§5).
Usage:
  normtool_478.py --run-id ID [--run-id ID ...] [--workdir DIR] [--biomes-exempt]
  normtool_478.py --selftest [--workdir DIR]   # 3 канон-реконструкции [479-G1]
                                               # + 9 offline-фикстур [482-C03]
"""
import argparse, json, os, re, statistics, subprocess, zipfile

REPO = "PLANETA9091/c-crussty"
# BANK_V5_FREEZE §2 tps_exp_v5 сэмплы (FROZEN, не пересматривать)
V5 = [(6.5e6, 2.1252), (7.0e6, 2.2047), (7.5e6, 2.3271), (8.2e6, 2.4732),
      (8.7e6, 2.5981), (9.0e6, 2.6280)]
BAND = (6.0e6, 9.5e6)                                # GLOB 6.0-9.5M
LOCAL_70, LOCAL_LO, LOCAL_HI = 2.1293, 6.9e6, 7.2e6  # Л201 robust-узел (report-only)
TPS_MAX_VALID = 15.0                                 # канон C55
STW_MAX_S, YOUNG_MAX_MS = 23.0, 200.0                # HOST-ценз M1
# v5-FROZEN окна (report-only; BANK §2/§5 — пороги не двигать)
FROZEN_WINDOWS = {
    "climb5[8734563,8834563]": {"range": (8_734_563, 8_834_563), "thresh": 2.99},
    "POI[8907260,9007260]": {"range": (8_907_260, 9_007_260), "thresh": -1.99},
}

RE_POLLS_BOT = re.compile(r"TPS polls captured: (\d+), first-of-window values: \[([^\]]*)\]")
RE_TPS = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
RE_IDX = re.compile(r"runner_cpu_index[:=]\s*(\d+)")
RE_SPARK = re.compile(r"spark tick-monitor MSPT: avg \*?\*?([\d.]+)ms")
RE_GC_TOTAL = re.compile(r"total pause: \*?\*?([\d.]+) ms")
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
# [482-C03.2] биом-AIOOBE класс: fail-closed probe ChunkParseOps.biomesSelftest
# печатает one-line toString Throwable без стека → сигнатура одной строки.
RE_BIOME_AIOOBE = re.compile(r"biomes selftest FAIL \(throwable .*ArrayIndexOutOfBounds")


def tps_exp_v5(idx, local_node=False):
    """Линейная интерполяция узлов §2; вне узлов — линейная экстраполяция крайними
    (канон c42). ЮНИТЫ: idx и узлы в герцах (урок B3: cpu/1e6-баг = selftest-отлов)."""
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
    """M1-канон: completion-строки 'GC(n) Pause ... X.Xms' без [gc,phases]."""
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
            if "CodeCache" in line:
                stw["full_cc"] += 1
            elif "Metadata" in line:
                stw["full_md"] += 1
            else:
                stw["full_other"] += 1
        else:
            stw["young"] += 1
            stw["young_sum_ms"] += dur
    return stw


def parse_bundle(run_id, env, log, bot, gclog, biomes_exempt=False):
    """A1-конвейер поверх уже-распакованного бандла (offline-тестируемо, урок B3).
    norm_run = fetch-zip + parse_bundle; fixtures зовут parse_bundle напрямую."""
    idx_m = RE_IDX.search(env)
    idx = int(idx_m.group(1)) if idx_m else 0

    # --- polls: BOTTLENECKS first-of-window (первичный источник по CLAIM) ---
    raw_bot = []
    n_captured = None
    mb = RE_POLLS_BOT.search(bot)
    if mb:
        n_captured = int(mb.group(1))
        raw_bot = [float(x) for x in mb.group(2).split(",") if x.strip()]
    # --- raw-поллы из server-stdout (A1.2: хранить обязательно) ---
    raw_stdout = [float(x) for x in RE_TPS.findall(log)]
    source = "bottlenecks_first_of_window" if raw_bot else "stdout_fallback"
    raw_polls = raw_bot if raw_bot else raw_stdout
    polls = [x for x in raw_polls if x < TPS_MAX_VALID]  # канон C55
    med = statistics.median(polls) if polls else 0.0

    exp = tps_exp_v5(idx)
    exp_c42 = tps_exp_v5(idx, local_node=True)
    norm = 100 * (med / exp - 1) if med else None
    norm_c42 = 100 * (med / exp_c42 - 1) if med else None

    # --- spark-кросс-чек (дивергенция до 24.9пп — A1.2b) ---
    spark_mspt = float(RE_SPARK.search(bot).group(1)) if RE_SPARK.search(bot) else None
    spark_tps = round(1000.0 / spark_mspt, 4) if spark_mspt else None
    norm_spark = round(100 * (spark_tps / exp - 1), 2) if spark_tps and med else None
    div = round(norm_spark - norm, 2) if norm_spark is not None and norm is not None else None

    # --- HOST-ценз M1 (gc.log primary; BOTTLENECKS total-pause кросс-чек) ---
    s = gc_canon(gclog) if gclog else None
    n = s["pauses"] if s else 0
    young_avg = (s["young_sum_ms"] / s["young"]) if s and s["young"] else 0.0
    all_avg = (s["total_ms"] / n) if s and n else 0.0
    host = bool(s) and (s["total_ms"] > STW_MAX_S * 1000 or
                        (s["young"] and young_avg > YOUNG_MAX_MS))
    # --- M1-ГЕЙТ [479-G1] + [482-C03.1] TRI-STATE fail-closed ---
    # CENS: порог нарушен. UNKNOWN: gc.log нет / 0 completion-строк (Л-481-C22
    # fail-open дыра «gc.log есть ∧ 0 строк → m1_clean=True») / young_n==0 при
    # распарсенных паузах ≤23s (young-гейт неверифицируем — young_n==0 при
    # stw_total>0 НЕ может PASS по умолчанию). CLEAN: оба гейта проверены.
    # m1_clean остаётся boolean (потребители [479-G1]): UNKNOWN → False.
    if not s or s["pauses"] == 0:
        m1_state = "UNKNOWN"
    elif s["total_ms"] > STW_MAX_S * 1000.0:
        m1_state = "CENS"
    elif s["young"] == 0:
        m1_state = "UNKNOWN"
    elif s["young_sum_ms"] / s["young"] > YOUNG_MAX_MS:
        m1_state = "CENS"
    else:
        m1_state = "CLEAN"
    m1_clean = m1_state == "CLEAN"
    bot_total = float(RE_GC_TOTAL.search(bot).group(1)) if RE_GC_TOTAL.search(bot) else None

    valid = "FIXTURE-VALIDITY: VALID" in bot
    ncdfe = "NoClassDefFoundError" in log
    # --- [482-C03.2] AIOOBE-гейт c биом-exempt (прецедент Л-474-C88.2) ---
    aioobe_lines = [l for l in log.splitlines() if "ArrayIndexOutOfBoundsException" in l]
    aioobe_n = sum(l.count("ArrayIndexOutOfBoundsException") for l in aioobe_lines)
    aioobe_biome = sum(l.count("ArrayIndexOutOfBoundsException") for l in aioobe_lines
                       if RE_BIOME_AIOOBE.search(l))
    aioobe_other = aioobe_n - aioobe_biome
    aioobe = bool(aioobe_n)                      # compat-поле: полный AIOOBE-факт
    aioobe_gate = aioobe_other if biomes_exempt else aioobe
    biomes_exempt_applied = bool(biomes_exempt and aioobe_biome and not aioobe_other)
    in_band = BAND[0] <= idx <= BAND[1]

    if not in_band:
        verdict = "BAND-DEAD"
    elif not polls:
        verdict = "NO-TPS"
    elif not valid or ncdfe or aioobe_gate:
        verdict = "FIXTURE-INVALID"
    elif host:
        verdict = "HOST-CENSORED"          # в-точка VALID, из фитов CLEAN-first §3.3
    elif m1_state == "UNKNOWN":
        verdict = "M1-UNKNOWN"             # [482-C03.1] fail-closed: НЕ PASS
    else:
        verdict = "NORM-COMPUTED"          # norm-число готово; пороги — вне скоупа тулзы

    # --- v5-FROZEN окна report-only (НЕ двигать) ---
    win_report = {}
    if med:
        for name, w in FROZEN_WINDOWS.items():
            if w["range"][0] <= idx <= w["range"][1]:
                win_report[name] = {"norm_v5": round(norm, 2), "thresh_frozen": w["thresh"],
                                    "hit": norm <= w["thresh"]}

    return {
        "run_id": run_id, "tool": "normtool_478", "verdict": verdict,
        "m1_clean": m1_clean,                   # ГЕЙТ [479-G1]+[482-C03.1]: UNKNOWN→False
        "m1_state": m1_state,                   # [482-C03.1] CLEAN/CENS/UNKNOWN
        "cpu_index": idx, "in_band": in_band,
        "poll_source": source, "polls_captured": n_captured,
        "raw_polls_bottlenecks": raw_bot,          # A1.2: raw-поллы хранить
        "raw_polls_stdout": raw_stdout,            # A1.2: raw-поллы хранить
        "polls_valid_c55": polls,                  # после фильтра <15.0
        "n_polls_valid": len(polls),
        "tps_med": round(med, 4),
        "tps_exp_v5": round(exp, 5),
        "tps_exp_c42_robust": round(exp_c42, 4),
        "norm_v5": round(norm, 2) if norm is not None else None,
        "norm_c42_robust": round(norm_c42, 2) if norm_c42 is not None else None,
        "spark_crosscheck": {"mspt_avg": spark_mspt, "tps_avg": spark_tps,
                             "norm_spark": norm_spark, "divergence_pp": div},
        "host_M1": {"stw_total_s": round(s["total_ms"] / 1000, 4) if s else None,
                    "stw_limit_s": STW_MAX_S,
                    "all_avg_ms": round(all_avg, 1),
                    "young_avg_ms": round(young_avg, 1),
                    "young_limit_ms": YOUNG_MAX_MS,
                    "max_pause_ms": round(s["max_ms"], 1) if s else None,
                    "young_n": s["young"] if s else 0,
                    "full_n": s["full"] if s else 0,
                    "full_cc": s["full_cc"] if s else 0,
                    "full_md": s["full_md"] if s else 0,
                    "bottlenecks_total_pause_ms": bot_total,
                    "m1_clean": m1_clean, "host": host, "m1_state": m1_state},
        "fixture_valid": valid, "ncdfe": ncdfe, "aioobe": aioobe,
        "aioobe_biome": aioobe_biome, "aioobe_other": aioobe_other,
        "biomes_exempt": bool(biomes_exempt),
        "biomes_exempt_applied": biomes_exempt_applied,   # [482-C03.2] маркер
        "frozen_windows_report_only": win_report,
    }


def fetch_artifact(run_id, workdir):
    """Артефакт world3-bench → кэш-zip (download skip если уже есть)."""
    tok = open("/tmp/gh_token").read().strip()
    base = f"https://api.github.com/repos/{REPO}"
    os.makedirs(workdir, exist_ok=True)
    zpath = os.path.join(workdir, f"art_{run_id}.zip")
    if os.path.exists(zpath):
        return zpath
    out = subprocess.run(["curl", "-s", "-H", f"Authorization: token {tok}",
                          f"{base}/actions/runs/{run_id}/artifacts"],
                         capture_output=True, text=True).stdout
    art = next((a for a in json.loads(out).get("artifacts", [])
                if a["name"] == "world3-bench"), None)
    if not art:
        return None
    subprocess.run(["curl", "-sL", "-H", f"Authorization: token {tok}",
                    "-o", zpath, art["archive_download_url"]], check=True)
    return zpath


def norm_run(run_id, workdir, biomes_exempt=False):
    """Полный A1-конвейер одного run id → JSON-дикт (raw-поллы хранить обязательно)."""
    zpath = fetch_artifact(run_id, workdir)
    if not zpath:
        return {"run_id": run_id, "verdict": "NO-ARTIFACT"}
    with zipfile.ZipFile(zpath) as z:
        names = z.namelist()
        env = z.read("run-env.txt").decode(errors="replace")
        log = z.read("server-stdout.log").decode(errors="replace")
        bot = z.read("BOTTLENECKS_3.md").decode(errors="replace") if "BOTTLENECKS_3.md" in names else ""
        gcn = next((n for n in names if n.endswith("gc.log")), None)
        gclog = z.read(gcn).decode(errors="replace") if gcn else ""
    return parse_bundle(run_id, env, log, bot, gclog, biomes_exempt=biomes_exempt)


SELFTEST = [
    # (run_id, tag, {поля = бит-точные канон-числа Л-478-A1.1 / [478-A1] W3-c8 /
    #  [479-A1] canary; STW/M1-вердикты ×478-канон [479-G1]: CLEAN/CENS/CENS})
    (36357571554, "W1-s7", {"cpu_index": 8824280, "tps_med": 2.4, "tps_exp_v5": 2.61049,
                            "norm_v5": -8.06, "verdict": "NORM-COMPUTED", "m1_clean": True,
                            "host_M1": {"stw_total_s": 21.8517, "young_avg_ms": 108.0}}),
    (36357644226, "W3-c8", {"cpu_index": 8747610, "tps_med": 2.4, "tps_exp_v5": 2.60285,
                            "norm_v5": -7.79, "verdict": "HOST-CENSORED", "m1_clean": False,
                            "host_M1": {"stw_total_s": 23.2377, "young_avg_ms": 123.3}}),
    (36373375157, "canary", {"cpu_index": 7127362, "tps_med": 2.2, "tps_exp_v5": 2.23588,
                             "norm_v5": -1.6, "verdict": "HOST-CENSORED", "m1_clean": False,
                             "host_M1": {"stw_total_s": 25.2884, "young_avg_ms": 131.0}}),
]


def _get(d, path):
    for p in path.split("."):
        d = d[p]
    return d


def selftest(workdir):
    """Бит-точная реконструкция 3 канон-ранов + 9 offline-фикстур [482-C03]
    (урок B3: selftest перед вердиктом; фикстуры не требуют сети)."""
    ok = 0
    for run_id, tag, expect in SELFTEST:
        r = norm_run(run_id, workdir)
        fails = []
        for k, v in expect.items():
            got = _get(r, k)
            if isinstance(v, dict):
                for kk, vv in v.items():
                    if abs(_get(r, f"{k}.{kk}") - vv) > 1e-9:
                        fails.append(f"{k}.{kk}: got {got[kk]} want {vv}")
            elif got != v:
                fails.append(f"{k}: got {got} want {v}")
        status = "PASS" if not fails else "FAIL " + "; ".join(fails)
        ok += not fails
        print(f"[selftest] {tag} {run_id}: norm_v5={r['norm_v5']} med={r['tps_med']} "
              f"exp={r['tps_exp_v5']} -> {status}", file=__import__("sys").stderr)
        print(json.dumps(r, ensure_ascii=False))
    fx_ok, fx_n = _fixture_check()
    print(json.dumps({"selftest": f"{ok}/{len(SELFTEST)}",
                      "bit_exact": ok == len(SELFTEST),
                      "fixtures": f"{fx_ok}/{fx_n}",
                      "fixtures_ok": fx_ok == fx_n}, ensure_ascii=False))
    return ok == len(SELFTEST) and fx_ok == fx_n


# [482-C03] offline-фикстуры (урок B3: selftest без сети; env/bot канон-минимум:
# cpu 7.0M in-band + FIXTURE-VALIDITY VALID, TPS-полл 2.2 → stdout_fallback).
FX_ENV = "runner_cpu_index: 7000000\nworld_sha256: afb3a0b3"
FX_BOT = "FIXTURE-VALIDITY: VALID"
FX_LOG = "TPS from last 5s, 1m, 5m, 15m: 2.2, 2.2, 2.2, 2.2,\n"
FIXTURES = [
    # (tag, gclog, extra_log, biomes_exempt, {ожидания})
    ("fx-clean", "GC(3) Pause Young (Normal) 100.00ms\nGC(7) Pause Young (Normal) 110.00ms\n",
     "", False, {"verdict": "NORM-COMPUTED", "m1_state": "CLEAN", "m1_clean": True}),
    ("fx-no-gclog", "", "", False,
     {"verdict": "M1-UNKNOWN", "m1_state": "UNKNOWN", "m1_clean": False}),
    ("fx-zeropause", "[info][gc] Using G1\n", "", False,   # Л-481-C22 fail-open дыра
     {"verdict": "M1-UNKNOWN", "m1_state": "UNKNOWN", "m1_clean": False}),
    ("fx-full-only", "GC(9) Pause Full (CodeCache) 500.00ms\n", "", False,  # young_n==0 при stw_total>0
     {"verdict": "M1-UNKNOWN", "m1_state": "UNKNOWN", "m1_clean": False}),
    ("fx-stw-over", "GC(9) Pause Full (Metadata) 24000.00ms\n", "", False,
     {"verdict": "HOST-CENSORED", "m1_state": "CENS", "m1_clean": False}),
    ("fx-young-over", "GC(3) Pause Young (Normal) 250.00ms\nGC(7) Pause Young (Normal) 260.00ms\n",
     "", False, {"verdict": "HOST-CENSORED", "m1_state": "CENS", "m1_clean": False}),
    ("fx-biome-aioobe-noexempt", "GC(3) Pause Young (Normal) 100.00ms\n",
     "cmp420_chunk2: biomes selftest FAIL (throwable java.lang.ArrayIndexOutOfBoundsException: Index 1)\n",
     False, {"verdict": "FIXTURE-INVALID", "aioobe_biome": 1, "aioobe_other": 0,
             "biomes_exempt_applied": False}),
    ("fx-biome-aioobe-exempt", "GC(3) Pause Young (Normal) 100.00ms\n",
     "cmp420_chunk2: biomes selftest FAIL (throwable java.lang.ArrayIndexOutOfBoundsException: Index 1)\n",
     True, {"verdict": "NORM-COMPUTED", "aioobe_biome": 1, "aioobe_other": 0,
            "biomes_exempt_applied": True}),
    ("fx-real-aioobe-exempt", "GC(3) Pause Young (Normal) 100.00ms\n",
     "java.lang.ArrayIndexOutOfBoundsException: Index 6 out of bounds for length 6\n"
     "\tat net.minecraft.world.level.redstone.CollectingNeighborUpdater$MultiNeighborUpdate.runNext(CollectingNeighborUpdater.java:137)\n",
     True, {"verdict": "FIXTURE-INVALID", "aioobe_biome": 0, "aioobe_other": 1,
            "biomes_exempt_applied": False}),
]


def _fixture_check():
    """[482-C03] 9 offline-фикстур: tri-state m1 + biomes-exempt гейт без сети."""
    ok = 0
    for tag, gclog, extra, exempt, expect in FIXTURES:
        r = parse_bundle(0, FX_ENV, FX_LOG + extra, FX_BOT, gclog, biomes_exempt=exempt)
        fails = [f"{k}: got {r.get(k)} want {v}" for k, v in expect.items() if r.get(k) != v]
        ok += not fails
        print(f"[selftest-fx] {tag}{' +exempt' if exempt else ''}: {r['verdict']}/{r['m1_state']} -> "
              + ("PASS" if not fails else "FAIL " + "; ".join(fails)),
              file=__import__("sys").stderr)
    return ok, len(FIXTURES)


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-id", type=int, action="append")
    ap.add_argument("--workdir", default="/home/z/rounds/ROUND-478/G24/art")
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--biomes-exempt", action="store_true")   # [482-C03.2]
    a = ap.parse_args()
    if a.selftest:
        raise SystemExit(0 if selftest(a.workdir) else 1)
    if not a.run_id:
        ap.error("--run-id required (or --selftest)")
    for rid in a.run_id:
        print(json.dumps(norm_run(rid, a.workdir, biomes_exempt=a.biomes_exempt),
                         ensure_ascii=False))
