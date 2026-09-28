#!/usr/bin/env python3
"""absorbv2.py — [482-C22] единый absorbv2-пайплайн: normtool_478 (исправленный,
round-482-c03-normtool @f86b240c: TRI-STATE M1-UNKNOWN + biomes-exempt) +
флаг-чекеры normtool-фиксов ×480 (plateau C06 @6a60c868 / adoption C22 @0305e71c /
mid-inject C87 @7ee4a0b8) + C05 full_n>9 M1-расширение (acc 84%).
[483-B11] RAMP-ГЕЙТ (report-only): shape-ковариата ramp/plateau/neither (C87.2 —
в гейт-семантику НЕ ставить); fixed-shape ×1.18 ТОЛЬКО по ramp-shape-детекту, не по
n>6 (C87.3: overshoot +4.80пп на plateau-форме; C89: plateau-фикс нужен только cap≤6);
shadow ×1.16 (C89: эмпирия gain ×1.1448/×1.1591, ×1.18 = +2.5пп перелёт);
mid-inject TP/FP-счётчики в сводке (пререгистрация FP ≤1/20 банк-фидов, TP ≥1).

АРХИТЕКТУРА (additive/report-only — вердикты и пороги normtool v5-FROZEN НЕ тронуты):
  parse   = normtool_478.parse_bundle()  — базовый A1-конвейер (verdict/m1/norm);
  G1 G-PLATEAU   [480-C06] norm_plateau (n≤6 → median(last-2), n>6 → ×1.18),
                 флаги plateau_low_n (@n<4) и plateau_div_gt2pp (|div|>2пп =
                 C55-дефект-сигнал сверх канон-резидуала ≤±2пп);
  G2 G-ADOPTION  [480-C22] strict_window_hit ([6.9,7.2]M Л201), spark-div в канон-
                 банде 11..25пп, m1_unknown_reason (zero-parse tri-state аудит);
  G3 G-MIDINJECT [480-C87] ts-гейт (poll ≤ POPULATION INJECT DONE) + ramp-сигнатура
                 max/median(last-2) > 1.6 → excl-поллы, alt-медиана, leak_pp;
  G4 G-M1EXT     [482-C05] counts-only предиктор M1-срыва full_n>9 (канон-матрица
                 TP19/FP5/TN49/FN8, acc 84.0%, honest non-circular v2).
CLI:
  absorbv2.py --dir DIR [--out FILE] [--biomes-exempt] [--compare-absorb JSON]
  absorbv2.py --selftest          # 5 offline-фикстур-директорий, сеть не нужна
"""
import argparse, json, os, re, statistics, sys, tempfile

_HERE = os.path.dirname(os.path.abspath(__file__))
if _HERE not in sys.path:
    sys.path.insert(0, _HERE)
import normtool_478 as nt  # исправленный normtool (C03 tri-state + biomes-exempt)

# --- health-guard: единый пайплайн имеет смысл только поверх эталонного normtool ---
def normtool_health():
    ok = (hasattr(nt, "parse_bundle") and hasattr(nt, "gc_canon")
          and nt.STW_MAX_S == 23.0 and nt.YOUNG_MAX_MS == 200.0
          and nt.TPS_MAX_VALID == 15.0 and nt.BAND == (6.0e6, 9.5e6))
    return {"import": "normtool_478", "frozen_constants_ok": bool(ok),
            "m1_tri_state": True, "biomes_exempt": hasattr(nt, "RE_BIOME_AIOOBE")}

# ================= [480-C06] G1 PLATEAU (report-only port) =================
PLATEAU_CAP, PLATEAU_MIN_N, PLATEAU_FIXED_SHAPE = 6, 2, 1.18
PLATEAU_RESIDUAL_BAND_PP = 2.0   # канон C06: известный residual ≤±2пп → >2пп = сигнал

# ---- [483-B11] ramp-гейт константы (канон C87/C89) ----
RAMP_FIXED_SHAPE_SHADOW = 1.16   # [482-C89] gain эмпирия ×1.1448/×1.1591; ×1.18 = +2.5пп перелёт
RAMP_TAU_HINT = 0.76             # [482-C89] τ̂ банк 0.695 → окно 0.72 → ×482 0.76 (ковариата)
SHAPE_QUANT_TPS = 0.1            # [482-C87.2] plateau = |Δ last-2| < 0.1 TPS (2×2-канон)
RAMP_SHAPE_RISE_TPS = 0.2        # [482-C86/C87.4] soft-rise ≥0.2 TPS (банк 23/23)


def shape_class(polls_valid):
    """[483-B11][482-C87.2] shape-ковариата ramp/plateau/neither — REPORT-ONLY,
    в гейт-семантику НЕ включать (порогочувствительность 26.5%↔69.9% при Δ0.1)."""
    p = polls_valid or []
    if len(p) < 2:
        return None
    if abs(p[-1] - p[-2]) < SHAPE_QUANT_TPS:
        return "plateau"
    if (all(p[i + 1] > p[i] for i in range(len(p) - 1))
            and (p[-1] - p[0]) >= RAMP_SHAPE_RISE_TPS):
        return "ramp"
    return "neither"


def plateau_checker(polls_valid, med, exp, norm_v5):
    n = len(polls_valid or [])
    sc = shape_class(polls_valid)
    if not n or not exp or not med:
        return {"computed": False, "norm_plateau": None, "method": None,
                "plateau_low_n": None, "divergence_pp": None, "div_gt2pp": None,
                "shape_class": sc, "gain_l2_over_med": None,
                "norm_fixed_shape_x118": None, "norm_fixed_shape_x116": None,
                "tau_hint": RAMP_TAU_HINT}
    if n <= PLATEAU_CAP:
        if n < PLATEAU_MIN_N:
            return {"computed": False, "norm_plateau": None, "method": "fail_closed",
                    "plateau_low_n": True, "divergence_pp": None, "div_gt2pp": None,
                    "shape_class": sc, "gain_l2_over_med": None,
                    "norm_fixed_shape_x118": None, "norm_fixed_shape_x116": None,
                    "tau_hint": RAMP_TAU_HINT}
        tps_pl, method = statistics.median(polls_valid[-2:]), "last2_median"
    else:
        # [483-B11][482-C87.3] ×1.18 ТОЛЬКО по ramp-shape-детекту (plateau-форма
        # overshoot +4.80пп); n>6 median-of-all рампу уже проходит (C89 bias·n 68.7→18.9).
        if sc == "ramp":
            tps_pl, method = med * PLATEAU_FIXED_SHAPE, "fixed_shape_x1.18"
        else:
            tps_pl, method = med, "med_all_no_shape_fix"
    norm_pl = 100 * (tps_pl / exp - 1)
    div = round(norm_pl - norm_v5, 2) if norm_v5 is not None else None
    l2 = statistics.median(polls_valid[-2:]) if n >= 2 else None
    gain = round(l2 / med, 4) if (l2 and med) else None
    return {"computed": True, "norm_plateau": round(norm_pl, 2), "method": method,
            "plateau_low_n": bool(method == "last2_median" and n < 4),
            "divergence_pp": div,
            "div_gt2pp": bool(div is not None and abs(div) > PLATEAU_RESIDUAL_BAND_PP),
            "shape_class": sc,           # [482-C87.2] report-only ковариата, НЕ гейт
            "gain_l2_over_med": gain,    # [482-C89] канон ×1.145-1.159
            "tau_hint": RAMP_TAU_HINT,   # [482-C89] report-only
            "norm_fixed_shape_x118": round(100 * (med * PLATEAU_FIXED_SHAPE / exp - 1), 2)
                                      if sc == "ramp" else None,
            "norm_fixed_shape_x116": round(100 * (med * RAMP_FIXED_SHAPE_SHADOW / exp - 1), 2)
                                      if sc == "ramp" else None}

# ================= [480-C22] G2 ADOPTION (report-only port) =================
SPARK_DIV_CANON = (11.0, 25.0)   # G24-канон дивергенции (Л-480-C22, честный флаг)

def adoption_checker(res, gclog_present):
    idx = res["cpu_index"]
    strict_hit = nt.LOCAL_LO <= idx <= nt.LOCAL_HI
    div = (res.get("spark_crosscheck") or {}).get("divergence_pp")
    if res["m1_state"] == "UNKNOWN":
        reason = ("no-gclog" if not gclog_present else
                  "zero-pause" if (res["host_M1"]["young_n"] == 0 and
                                   (res["host_M1"]["stw_total_s"] or 0) == 0) else
                  "young-n0" if res["host_M1"]["young_n"] == 0 else "other")
    else:
        reason = None
    return {"strict_window_hit": bool(strict_hit),
            "spark_divergence_pp": div,
            "spark_div_canon_band": bool(div is not None and
                                         SPARK_DIV_CANON[0] <= div <= SPARK_DIV_CANON[1]),
            "m1_unknown_reason": reason}

# ================= [480-C87] G3 MID-INJECT (report-only port) =================
MIDINJ_RATIO = 1.6               # мид-гэп clean ≤1.22 / утечка 2.15 (C87)
RE_TPS_TS = re.compile(r"\[(\d{2}:\d{2}:\d{2})[^\]]*\]: TPS from last 5s.*?: ([\d.]+),")
RE_INJ_DONE_TS = re.compile(r"\[(\d{2}:\d{2}:\d{2})[^\]]*\]: .*POPULATION INJECT DONE")

def _hms(s):
    h, m, sec = s.split(":")
    return int(h) * 3600 + int(m) * 60 + int(sec)

def mid_inject_checker(log, polls, raw_stdout, med, exp, norm_v5):
    m = RE_INJ_DONE_TS.search(log)
    done_s = m.group(1) if m else None
    ts_flagged = []
    if done_s:
        d = _hms(done_s)
        for mm in RE_TPS_TS.finditer(log):
            t, v = _hms(mm.group(1)), float(mm.group(2))
            if ((d - t) % 86400) < 43200 and v < nt.TPS_MAX_VALID:
                ts_flagged.append(v)
    plat = statistics.median(polls[-2:]) if len(polls) >= 2 else None
    ratio = (max(polls) / plat) if (plat and polls) else 0.0
    sig_suspect = bool(plat) and len(polls) >= 4 and ratio > MIDINJ_RATIO
    polls_excl, excl_vals = None, []
    if ts_flagged and [x for x in raw_stdout if x < nt.TPS_MAX_VALID] == polls:
        pool = list(polls)
        for v in ts_flagged:
            if v in pool:
                pool.remove(v)
                excl_vals.append(v)
        polls_excl = pool
    elif sig_suspect and plat:
        polls_excl = [x for x in polls if x > MIDINJ_RATIO * plat]
        excl_vals = list(polls_excl)
    if polls_excl is not None and len(polls_excl) == len(polls):
        polls_excl = None                      # degenerate: всё исключено
    alt_med = statistics.median(polls_excl) if polls_excl else med
    alt_norm = 100 * (alt_med / exp - 1) if (alt_med and med) else None
    leak = round(norm_v5 - alt_norm, 2) if (norm_v5 is not None and alt_norm is not None) else None
    suspect = bool(excl_vals) or sig_suspect
    return {"suspect": suspect, "threshold_ratio": MIDINJ_RATIO,
            "inject_done_ts": done_s, "ts_flagged_polls": ts_flagged,
            "plateau_last2": round(plat, 4) if plat is not None else None,
            "max_over_plateau": round(ratio, 3) if plat else None,
            "signature_suspect": bool(sig_suspect),
            "polls_excluded_mid_inject": excl_vals,
            "tps_med_excl_midinject": round(alt_med, 4),
            "norm_v5_excl_midinject": round(alt_norm, 2) if alt_norm is not None else None,
            "mid_inject_leak_pp": leak}

# ================= [482-C05] G4 M1-EXTENSION full_n>9 =================
M1EXT_FULLN9_THR = 9             # counts-only, honest v2: acc 0.84 (TP19/FP5/TN49/FN8)
M1EXT_CANON = {"acc": 0.84, "TP": 19, "FP": 5, "TN": 49, "FN": 8}

def m1_ext_checker(full_n, m1_state):
    pred = bool(full_n is not None and full_n > M1EXT_FULLN9_THR)
    return {"m1_ext_fulln9_gt9": pred, "threshold": M1EXT_FULLN9_THR,
            "m1_ext_canon_matrix": M1EXT_CANON,
            "m1_state_actual": m1_state,
            "agrees_with_cens": (pred == (m1_state == "CENS"))}


# ================= [483-B11] RAMP-ГЕЙТ: пререгистрация гейтов патча =================
B11_GATES = {"ramp_detect_acc_min": 0.95,    # accuracy ≥95% на корпусе ×482-113 (C87)
             "midinject_fp_rate_max": 0.05,  # FP ≤1/20 банк-фидов; эвиденс TP1/1 FP0/18 ×480 ∧ 0/23 ×482
             "midinject_tp_min": 1,          # TP ≥1 (×480-корпус)
             "selftest_min": "8/8",          # офлайн-фикстуры absorbv2
             "corpus": "/home/z/rounds/ROUND-482/c87_plateau/verdict_482_c87.json",
             "mode": "report-only; v5-FROZEN не тронуты; NO-GO живой-gate (C87.6)"}

# ================= пайплайн одной директории-рана =================
RUN_ENV = "run-env.txt"

def process_run(run_dir, run_id, biomes_exempt=False):
    env_p = os.path.join(run_dir, RUN_ENV)
    log_p = os.path.join(run_dir, "server-stdout.log")
    if not (os.path.exists(env_p) and os.path.exists(log_p)):
        return {"run": run_id, "dir": os.path.basename(run_dir),
                "verdict": "SKIPPED-EMPTY", "reason": "no run-env.txt/server-stdout.log"}
    env = open(env_p, errors="replace").read()
    log = open(log_p, errors="replace").read()
    bot_p = os.path.join(run_dir, "BOTTLENECKS_3.md")
    bot = open(bot_p, errors="replace").read() if os.path.exists(bot_p) else ""
    gcn = next((f for f in os.listdir(run_dir) if f.endswith("gc.log")), None)
    gclog = open(os.path.join(run_dir, gcn), errors="replace").read() if gcn else ""

    res = nt.parse_bundle(run_id, env, log, bot, gclog, biomes_exempt=biomes_exempt)
    polls = res["polls_valid_c55"]
    g1 = plateau_checker(polls, res["tps_med"], res["tps_exp_v5"], res["norm_v5"])
    g2 = adoption_checker(res, bool(gclog))
    g3 = mid_inject_checker(log, polls, res["raw_polls_stdout"],
                            res["tps_med"], res["tps_exp_v5"], res["norm_v5"])
    g4 = m1_ext_checker(res["host_M1"]["full_n"], res["m1_state"])
    return {"run": run_id, "dir": os.path.basename(run_dir), "verdict": res["verdict"],
            "m1_state": res["m1_state"], "m1_clean": res["m1_clean"],
            "cpu_index": res["cpu_index"], "in_band": res["in_band"],
            "norm_v5": res["norm_v5"], "tps_med": res["tps_med"],
            "full_n": res["host_M1"]["full_n"],
            "stw_total_s": res["host_M1"]["stw_total_s"],
            "young_avg_ms": res["host_M1"]["young_avg_ms"],
            "aioobe_biome": res["aioobe_biome"], "aioobe_other": res["aioobe_other"],
            "biomes_exempt_applied": res["biomes_exempt_applied"],
            "g_plateau": g1, "g_adoption": g2, "g_midinject": g3, "g_m1ext": g4}

def scan_dir(root, biomes_exempt=False, compare_absorb=None):
    ref = {}
    if compare_absorb and os.path.exists(compare_absorb):
        ref = json.load(open(compare_absorb))
    runs, skipped = [], 0
    single = all(os.path.exists(os.path.join(root, f))
                 for f in (RUN_ENV, "server-stdout.log"))
    entries = [root] if single else \
        sorted(os.path.join(root, d) for d in os.listdir(root)
               if os.path.isdir(os.path.join(root, d)))
    for d in entries:
        m = re.search(r"(\d{6,})", os.path.basename(d))
        rid = int(m.group(1)) if m else 0
        rec = process_run(d, rid, biomes_exempt=biomes_exempt)
        v = ref.get(f"r{rid}") or ref.get(str(rid)) or ref.get(rid)
        if isinstance(v, dict):
            rec["absorb_ref"] = {k: v.get(k) for k in
                                 ("class", "norm", "m1", "corridor", "wave", "conclusion")
                                 if k in v}
        runs.append(rec)
        skipped += rec["verdict"] == "SKIPPED-EMPTY"
    ok = [r for r in runs if r["verdict"] != "SKIPPED-EMPTY"]

    def cnt(pred):
        return sum(1 for r in ok if pred(r))
    summary = {
        "tool": "absorbv2", "version": "482-C22.1", "dir": root,
        "biomes_exempt": bool(biomes_exempt),
        "normtool_health": normtool_health(),
        "n_dirs": len(entries), "n_runs": len(ok), "n_skipped": skipped,
        "verdicts": dict((v, sum(1 for r in ok if r["verdict"] == v))
                         for v in set(r["verdict"] for r in ok)),
        "m1_states": dict((v, sum(1 for r in ok if r["m1_state"] == v))
                          for v in set(r["m1_state"] for r in ok)),
        "gates": {
            "plateau_computed": cnt(lambda r: r["g_plateau"]["computed"]),
            "plateau_low_n": cnt(lambda r: r["g_plateau"]["plateau_low_n"]),
            "plateau_div_gt2pp": cnt(lambda r: r["g_plateau"]["div_gt2pp"]),
            "mid_inject_suspect": cnt(lambda r: r["g_midinject"]["suspect"]),
            "m1_ext_fulln9_gt9": cnt(lambda r: r["g_m1ext"]["m1_ext_fulln9_gt9"]),
            "m1_ext_agrees_cens": cnt(lambda r: r["g_m1ext"]["agrees_with_cens"]),
            "strict_window_hit": cnt(lambda r: r["g_adoption"]["strict_window_hit"]),
            "spark_div_canon_band": cnt(lambda r: r["g_adoption"]["spark_div_canon_band"]),
            "biomes_exempt_applied": cnt(lambda r: r["biomes_exempt_applied"]),
            "m1_unknown": cnt(lambda r: r["m1_state"] == "UNKNOWN"),
            # [483-B11] ramp-гейт / mid-inject TP-FP учёт (report-only)
            "ramp_shape_ramp": cnt(lambda r: r["g_plateau"]["shape_class"] == "ramp"),
            "ramp_shape_plateau": cnt(lambda r: r["g_plateau"]["shape_class"] == "plateau"),
            "ramp_shape_neither": cnt(lambda r: r["g_plateau"]["shape_class"] == "neither"),
            "ramp_fixed_shape_applied": cnt(lambda r: r["g_plateau"]["method"] == "fixed_shape_x1.18"),
            "mid_inject_suspect_bank": cnt(lambda r: r["g_midinject"]["suspect"]
                                           and r["verdict"] == "NORM-COMPUTED")},
        "b11_gate_spec": B11_GATES,
        "runs": runs}
    return summary

# ================= selftest: 5 offline-фикстур-директорий =================
FX_ENV = "runner_cpu_index: 7000000\nworld_sha256: afb3a0b3"
FX_BOT = "FIXTURE-VALIDITY: VALID"
GC_CLEAN = "GC(3) Pause Young (Normal) 100.00ms\nGC(7) Pause Young (Normal) 110.00ms\n"

def _mkfix(base, name, env, log, bot, gc):
    d = os.path.join(base, name)
    os.makedirs(d, exist_ok=True)
    open(os.path.join(d, RUN_ENV), "w").write(env)
    open(os.path.join(d, "server-stdout.log"), "w").write(log)
    open(os.path.join(d, "BOTTLENECKS_3.md"), "w").write(bot)
    open(os.path.join(d, "bench.gc.log"), "w").write(gc)
    return d

TPS_LINE = "TPS from last 5s, 1m, 5m, 15m: 2.2, 2.2, 2.2, 2.2,\n" * 4  # 4 полла, построчно

def selftest():
    base = tempfile.mkdtemp(prefix="absorbv2_fx_")
    _mkfix(base, "r1001", FX_ENV, TPS_LINE, FX_BOT, GC_CLEAN)
    _mkfix(base, "r1002", FX_ENV, TPS_LINE,
           FX_BOT, "".join(f"GC({i}) Pause Full (Metadata GC Threshold) 2500.00ms\n"
                           for i in range(10)) +
           "GC(20) Pause Young (Normal) 120.00ms\n" * 3)
    _mkfix(base, "r1003", FX_ENV,
           "[00:34:34]: TPS from last 5s, 1m, 5m, 15m: 5.6, 2.2, 2.2, 2.2,\n"
           "[00:35:29]: POPULATION INJECT DONE\n"
           "[00:36:00]: TPS from last 5s, 1m, 5m, 15m: 2.2, 2.2, 2.2, 2.2,\n", FX_BOT, GC_CLEAN)
    _mkfix(base, "r1004", FX_ENV, TPS_LINE +
           "cmp420_chunk2: biomes selftest FAIL (throwable java.lang.ArrayIndexOutOfBoundsException: Index 1)\n",
           FX_BOT, GC_CLEAN)
    _mkfix(base, "r1005", FX_ENV, TPS_LINE, FX_BOT, "[info][gc] Using G1\n")
    _mkfix(base, "r1006", FX_ENV,
           "".join(f"[00:40:0{i}]: TPS from last 5s, 1m, 5m, 15m: {2.0 + 0.15*i:.2f}, 2.2, 2.2, 2.2,\n"
                   for i in range(5)), FX_BOT, GC_CLEAN)        # [483-B11] ramp-форма (монотонный рост)
    _mkfix(base, "r1007", FX_ENV, TPS_LINE, FX_BOT, GC_CLEAN)   # [483-B11] plateau-форма (flat)

    s = scan_dir(base)
    by = {r["dir"]: r for r in s["runs"]}
    checks = [
        ("fx-clean", by["r1001"]["verdict"] == "NORM-COMPUTED"
         and by["r1001"]["m1_state"] == "CLEAN"
         and by["r1001"]["g_plateau"]["computed"]
         and by["r1001"]["g_plateau"]["method"] == "last2_median"
         and by["r1001"]["g_plateau"]["divergence_pp"] == 0.0
         and not by["r1001"]["g_plateau"]["plateau_low_n"]
         and not by["r1001"]["g_midinject"]["suspect"]
         and not by["r1001"]["g_m1ext"]["m1_ext_fulln9_gt9"]),
        ("fx-cens-full10", by["r1002"]["verdict"] == "HOST-CENSORED"
         and by["r1002"]["m1_state"] == "CENS"
         and by["r1002"]["g_m1ext"]["m1_ext_fulln9_gt9"]
         and by["r1002"]["g_m1ext"]["agrees_with_cens"]),
        ("fx-midinject", by["r1003"]["g_midinject"]["suspect"]
         and by["r1003"]["g_midinject"]["ts_flagged_polls"] == [5.6]
         and by["r1003"]["g_midinject"]["tps_med_excl_midinject"] == 2.2
         and by["r1003"]["g_midinject"]["norm_v5_excl_midinject"] == -0.21
         and by["r1003"]["g_midinject"]["mid_inject_leak_pp"] is not None),
        ("fx-biome-noexempt", by["r1004"]["verdict"] == "FIXTURE-INVALID"
         and by["r1004"]["aioobe_biome"] == 1
         and not by["r1004"]["biomes_exempt_applied"]),
        ("fx-tristate-zeropause", by["r1005"]["verdict"] == "M1-UNKNOWN"
         and by["r1005"]["m1_state"] == "UNKNOWN"
         and by["r1005"]["g_adoption"]["m1_unknown_reason"] == "zero-pause"),
        ("fx-b11-ramp-shape", by["r1006"]["g_plateau"]["shape_class"] == "ramp"
         and by["r1006"]["g_plateau"]["method"] == "last2_median"
         and by["r1006"]["g_plateau"]["gain_l2_over_med"] > 1.0
         and by["r1006"]["g_plateau"]["norm_fixed_shape_x118"] is not None),
        ("fx-b11-plateau-shape", by["r1007"]["g_plateau"]["shape_class"] == "plateau"
         and by["r1007"]["g_plateau"]["norm_fixed_shape_x118"] is None
         and by["r1007"]["g_plateau"]["gain_l2_over_med"] == 1.0),
    ]
    # biomes-exempt пере-прогон r1004: биом-AIOOBE probe снимается exempt'ом
    s2 = scan_dir(base, biomes_exempt=True)
    by2 = {r["dir"]: r for r in s2["runs"]}
    checks.append(("fx-biome-exempt", by2["r1004"]["verdict"] == "NORM-COMPUTED"
                   and by2["r1004"]["biomes_exempt_applied"]))
    ok = sum(1 for _, c in checks if c)
    for tag, c in checks:
        print(f"[absorbv2-selftest] {tag}: {'PASS' if c else 'FAIL'}", file=sys.stderr)
    print(json.dumps({"absorbv2_selftest": f"{ok}/{len(checks)}",
                      "normtool_health": normtool_health()}, ensure_ascii=False))
    return ok == len(checks)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dir", help="папка с рана-директориями (run-env.txt+server-stdout.log)")
    ap.add_argument("--out", help="файл JSON-сводки (по умолчанию stdout)")
    ap.add_argument("--biomes-exempt", action="store_true")       # [482-C03.2]
    ap.add_argument("--compare-absorb", help="absorb_482.json для сверки классов")
    ap.add_argument("--selftest", action="store_true")
    a = ap.parse_args()
    if a.selftest:
        raise SystemExit(0 if selftest() else 1)
    if not a.dir or not os.path.isdir(a.dir):
        ap.error("--dir required (existing directory, or --selftest)")
    s = scan_dir(a.dir, biomes_exempt=a.biomes_exempt, compare_absorb=a.compare_absorb)
    js = json.dumps(s, ensure_ascii=False, indent=1)
    if a.out:
        open(a.out, "w").write(js)
        print(json.dumps({"out": a.out, "n_runs": s["n_runs"],
                          "n_skipped": s["n_skipped"], "gates": s["gates"]},
                         ensure_ascii=False))
    else:
        print(js)

if __name__ == "__main__":
    main()
