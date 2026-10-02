"""absorb_s7207.py - ЦЕЛЬ-ДЕПЛОЙ профиль-лег: players-32 (TASK-386-аменд/394) — absorb лега s7207
ДИАГНОСТИЧЕСКИЙ лег (НЕ гейт, без банкинга): карта скейлинга конкарренси
игроков на БАНКЕ v4 под ParallelGC. Цель = найти под-лейн, растущий с
игроками (МЕГА-БУСТ МАНДАТ: следующий архитектурный рычаг с потолком >=+10%
TPS-эквивалента, обоснование потолок/конверсия ДО диспатча).

PG-T1 delivery: банк v4 в ЧИСТОМ виде (fluid_bitmask=0 — REFUTED #16,
fluid_dirty_ledger=0, inside_bitmask=0, travel_diet/skip_store_bb/
region_steal/bu_defer=0) + fake_players=32 + pop 150k VALID + ParallelGC +
band 6.0M..9.5M + 0 NCDFE.
PG-T2 crash-free + TPS-поллы >= 3.
PG-T3 ИНФОРМАЦИОННЫЙ: median5 @ runner vs TPS_exp-интерполяция якоря-банка
(fp=4: 2.6 @ 8551924 / 2.2 @ 6653417) = ЦЕНА КОНКАРРЕНСИ fp=32 (БЫЛО -> СТАЛО
для отчёта владельцу). НЕ dual bar: популяция другая, банкинга нет.
PG-T4 страховочные ParallelGC — ЧИСЛА без gate-вердикта (fp=32 сам по себе
стресс GC; банк-гейты калиброваны на fp=4).
PG-T5 ПРОФИЛЬ-КАРТА (главное): лейны cpu-collapsed при fp=32 vs БАЗЛАЙН
банка v4 при fp=4 (s7206#3 35528326290, 115655 сэмплов, константы ниже) ->
дельты по лейнам -> топ-растущие = кандидаты следующего архитектурного
рычага. Оверлап регексов допустим (каждый лейн меряется независимо).
"""
import os, re, subprocess, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUN_DIR = os.path.join(RESDIR, "run-s7207-players32")
BANK_POINTS = ((2.6, 8_551_924), (2.2, 6_653_417))
BAND_MIN, BAND_MAX = 6_000_000, 9_500_000
# PG-T4 справочные числа банка v4 leg#2 (fp=4): young 109, Full 7, 18.8s/162ms/2400ms
# БАЗЛАЙН-КАРТА ЛЕЙНОВ (банк v4, fp=4, cpu-collapsed s7206#3, 115655 сэмплов):
# регексы-оверлапы, каждый лейн независим; веса = сэмплы.
BASELINE_TOTAL = 115_655
BASELINE_LANES = {
    "items": (36051, r"ItemEntity\.tick"),
    "fluid": (19332, r"updateFluidHeightAndDoFluidPushing|FluidPushGuardHook|collidedWithFluid|getFluidState|FluidState\.|FlowingFluid|Fluid\.getAABB"),
    "broadphase": (18108, r"CollisionUtil|getEntities|EntityLookup|getCollisionsForBlocksOrWorldBorder|performCollisions|EntitySectionStorage"),
    "nav_ai": (16380, r"PathNavigation|GoalSelector|\.Brain|behavior|PathFinder"),
    "inside_volatile": (13885, r"checkInsideBlocks|collidedWithShapeMovingFrom"),
    "fastutil": (9874, r"it/unimi"),
    "java_util": (8110, r"java/util/"),
    "paletted": (7408, r"PalettedContainer|SimpleBitStorage"),
    "players_packets": (12, r"ServerGamePacketListener|Clientbound|PlayerChunkSender|packet"),
}


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL")
    return m.group(1)


def api(tok, url):
    import json
    req = urllib.request.Request(f"{API}{url}", headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    try:
        with urllib.request.urlopen(req, timeout=60) as r:
            return json.loads(r.read())
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:200]}", file=sys.stderr)
        return {}


def _fetch_redirected(tok, url, dest):
    req = urllib.request.Request(url, headers={"Authorization": f"Bearer {tok}"})

    class NoRedirect(urllib.request.HTTPRedirectHandler):
        def redirect_request(self, req, fp, code, msg, headers, newurl):
            return None

    opener = urllib.request.build_opener(NoRedirect)
    try:
        opener.open(req, timeout=300)
        raise SystemExit("expected redirect")
    except urllib.error.HTTPError as e:
        if e.code not in (301, 302, 303, 307):
            raise
        loc = e.headers["Location"]
    with urllib.request.urlopen(urllib.request.Request(loc), timeout=600) as r, \
            open(dest, "wb") as f:
        while True:
            b = r.read(1 << 20)
            if not b:
                break
            f.write(b)


def fetch_artifact(tok, run_id):
    import zipfile
    arts = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/artifacts").get("artifacts", [])
    bench = [a for a in arts if a["name"] == "world3-bench"]
    if not bench:
        print(f"no world3-bench artifact (available: {[a['name'] for a in arts]})")
        return False
    a = bench[0]
    dest = os.path.join(RUN_DIR, a["name"] + ".zip")
    _fetch_redirected(tok, f"{API}/repos/{REPO}/actions/artifacts/{a['id']}/zip", dest)
    print(f"downloaded {dest} ({os.path.getsize(dest)} bytes)")
    with zipfile.ZipFile(dest) as z:
        z.extractall(RUN_DIR)
    return True


def fetch_joblog(tok, run_id):
    jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs").get("jobs", [])
    if not jobs:
        return ""
    dest = os.path.join(RUN_DIR, "job.log")
    try:
        _fetch_redirected(tok, f"{API}/repos/{REPO}/actions/jobs/{jobs[0]['id']}/logs", dest)
        return open(dest, errors="ignore").read()
    except Exception as e:  # noqa: BLE001
        print(f"job-log fetch failed: {e}", file=sys.stderr)
        return ""


def tps_series(path):
    out = []
    for line in open(path, errors="ignore"):
        m = re.search(r"TPS from last 5s, 1m, 5m, 15m: ([\d.]+)", line)
        if m and float(m.group(1)) < 20:
            out.append(float(m.group(1)))
    return out


def gc_stats_parallel(path):
    if not os.path.isfile(path):
        return None
    pauses = full = 0
    total = 0.0
    mx = 0.0
    for line in open(path, errors="ignore"):
        if "[gc,start" in line:
            continue
        m = re.search(r"Pause ([\w ]*?)[A-Za-z ]*.*? ([\d.]+)(ms|s)\s*$", line)
        if not m:
            m = re.search(r"Pause\b.*? ([\d.]+)(ms|s)\s*$", line)
            if not m:
                continue
        if "Full" in line:
            full += 1
        else:
            pauses += 1
        dur = float(m.group(2)) * (1.0 if m.group(3) == "ms" else 1000.0)
        total += dur
        mx = max(mx, dur)
    n = pauses + full
    return pauses, full, total, (total / n if n else 0.0), mx


def env_flag(env_txt, key, want):
    m = re.search(rf"^{key}: (\d+)", env_txt, re.M)
    return bool(m) and m.group(1) == want


def env_val(env_txt, key):
    m = re.search(rf"^{key}: (\d+)", env_txt, re.M)
    return m.group(1) if m else None


def lane_map(path):
    tot = 0
    lanes = {k: 0 for k in BASELINE_LANES}
    for ln in open(path, errors="ignore"):
        parts = ln.rstrip("\n").rsplit(" ", 1)
        if len(parts) != 2 or not parts[1].isdigit():
            continue
        w = int(parts[1])
        tot += w
        for k, (_, rx) in BASELINE_LANES.items():
            if re.search(rx, parts[0]):
                lanes[k] += w
    return tot, lanes


def main():
    tok = token()
    run_id = int(sys.argv[1]) if len(sys.argv) > 1 else 0
    if not run_id:
        raise SystemExit("usage: absorb_s7207.py <run_id>")
    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={st.get('status')} conclusion={st.get('conclusion')} "
          f"head={str(st.get('head_sha'))[:7]}")
    if st.get("status") != "completed":
        sys.exit(3)
    os.makedirs(RUN_DIR, exist_ok=True)
    have_art = fetch_artifact(tok, run_id)

    rep = [f"# absorb s7207 players-32 (run {run_id}, head {str(st.get('head_sha'))[:7]}) "
           f"— ЦЕЛЬ-ДЕПЛОЙ диагностика конкарренси на БАНКЕ v4 (не гейт-лег, без банкинга)\n"]
    verdict = None

    stdout = os.path.join(RUN_DIR, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""
    envp = os.path.join(RUN_DIR, "run-env.txt")
    env_txt = open(envp, errors="ignore").read() if os.path.isfile(envp) else ""
    logp = os.path.join(RUN_DIR, "world3-bench.log")
    log_txt = open(logp, errors="ignore").read() if os.path.isfile(logp) else ""

    # ---------- PG-T1 delivery (банк v4 чистый + fake_players=32) ----------
    if env_txt:
        want0 = {"fluid_bitmask": "0", "fluid_dirty_ledger": "0", "fluid_dirty": "0",
                 "inside_bitmask": "0", "travel_diet": "0", "skip_store_bb": "0",
                 "region_steal": "0", "bu_defer": "0"}
        want1 = {"inside_cache": "1", "flush_diet": "1", "region_threads": "4",
                 "batch_collector": "1", "fluid_guard": "1", "gc_tune": "3",
                 "population_target": "150000"}
        t1a = [f"{k}=0:{'OK' if env_flag(env_txt, k, '0') else 'BAD'}" for k in want0]
        t1b = [f"{k}={'OK' if env_flag(env_txt, k, v) else 'BAD'}" for k, v in want1.items()]
        fp = env_val(env_txt, "fake_players")
        fp_ok = fp == "32"
        pop_ok = "POPULATION FIXTURE-VALIDITY: VALID" in stdout_txt or \
                 "POPULATION FIXTURE-VALIDITY: VALID" in log_txt
        ncde = stdout_txt.count("NoClassDefFoundError") + log_txt.count("NoClassDefFoundError")
        col_ok = "Using Parallel" in (stdout_txt + log_txt)
        runner = env_val(env_txt, "runner_cpu_index") or "0"
        band_ok = BAND_MIN <= int(runner) <= BAND_MAX
        t1 = all("BAD" not in x for x in t1a + t1b) and fp_ok and pop_ok and ncde == 0 and col_ok
        rep.append("- PG-T1: fp=32:" + ("OK" if fp_ok else f"BAD({fp})") +
                   ", pop=" + ("VALID" if pop_ok else "INVALID") +
                   f", NCDFE={ncde}, col=" + ("PARALLEL" if col_ok else "BAD") +
                   f", runner={runner} (band {'OK' if band_ok else 'OUT'}) | " +
                   " ".join(t1a + t1b) + f" -> **{'PASS' if t1 else 'FAIL'}**")
        if not t1:
            verdict = "DELIVERY-FAIL"

    # ---------- PG-T2 crash-free ----------
    if stdout_txt:
        threw = len(re.findall(r"Encountered an unexpected exception|ReportedException",
                               stdout_txt))
        polls = len(re.findall(r"TPS from last 5s", log_txt))
        t2 = threw == 0 and polls >= 3
        rep.append(f"- PG-T2: threw={threw}, TPS-поллов={polls} -> **{'PASS' if t2 else 'FAIL'}**")
        if not t2 and verdict is None:
            verdict = "CRASH-REFUTED"

    # ---------- PG-T3 ИНФОРМАЦИОННЫЙ: цена конкарренси fp=32 ----------
    if log_txt:
        ser = tps_series_p(log_txt)
        if ser:
            med = statistics_median(ser)
            r_idx = int(runner) if runner.isdigit() else 0
            tps_exp = interp_tps(r_idx)
            d_norm = (med / tps_exp - 1.0) if tps_exp else 0.0
            rep.append(f"- PG-T3 (ИНФО): median5={med:.2f} @ {runner}; банк fp=4 интерполяция "
                       f"TPS_exp={tps_exp:.2f}; ЦЕНА КОНКАРРЕНСИ fp=32 vs fp=4: "
                       f"{d_norm:+.1%} (БЫЛО {tps_exp:.2f} -> СТАЛО {med:.2f} @ тот же runner)")
        else:
            rep.append("- PG-T3 (ИНФО): TPS-поллов нет в артефакте")

    # ---------- PG-T4 GC справочно ----------
    gcs = gc_stats_parallel(os.path.join(RUN_DIR, "gc.log"))
    if gcs:
        pauses, full, total, avg, mx = gcs
        rep.append(f"- PG-T4 (ИНФО, банк fp=4 справка 18.8s/162ms/2400ms/Full=7): "
                   f"young={pauses}, Full={full}, total={total/1000:.1f}s, avg={avg:.0f}ms, "
                   f"max={mx:.0f}ms")

    # ---------- PG-T5 ПРОФИЛЬ-КАРТА fp=32 vs банк fp=4 ----------
    cpu = os.path.join(RUN_DIR, "cpu-collapsed.txt")
    if have_art and os.path.isfile(cpu):
        tot, lanes = lane_map(cpu)
        rep.append(f"- PG-T5 ПРОФИЛЬ-КАРТА: total={tot} сэмплов (базлайн {BASELINE_TOTAL})")
        rows = []
        for k, (base, _) in BASELINE_LANES.items():
            cur = lanes[k]
            cur_p = cur / tot if tot else 0.0
            base_p = base / BASELINE_TOTAL
            rows.append((cur_p - base_p, k, cur, cur_p, base, base_p))
        for d, k, cur, cur_p, base, base_p in sorted(rows, reverse=True):
            arrow = "РОСТ" if d > 0.01 else ("спад" if d < -0.01 else "флэт")
            rep.append(f"  - {k}: fp4 {base_p:.2%} -> fp32 {cur_p:.2%} ({d:+.2%}) "
                       f"[{base}->{cur} сэмплов] {arrow}")
        top = max(rows)
        rep.append(f"  -> ТОП-РОСТ: **{top[1]}** {top[4]/BASELINE_TOTAL:.2%} -> "
                   f"{top[2]/tot:.2%} ({top[0]:+.2%}) — кандидат следующего "
                   f"архитектурного рычага (МЕГА-БУСТ мандат: потолок-обоснование "
                   f"в RECON следующего тика)")
        if verdict is None:
            verdict = "PROFILE-MAP READY"
    else:
        rep.append("- PG-T5: cpu-collapsed отсутствует")
        if verdict is None:
            verdict = "NO-PROFILE"

    if verdict is None:
        verdict = "DELIVERY-FAIL"

    # ---------- FAILURE-рулетка v2 (нет артефакта/stdout — диагностика по job.log;
    # уроки s7206#2 unbound + s7207#1 503 + TASK-393 cc0c3f7 рулетка) ----------
    if not have_art or not stdout_txt:
        jl = fetch_joblog(tok, run_id)
        crash = sum(jl.count(x) for x in
                    ("Encountered an unexpected exception", "ReportedException",
                     "NullPointerException"))
        band = bool(re.search(r"::notice::runner_cpu_index=\d+ OUTSIDE band", jl))
        unbound = bool(re.search(r"unbound variable", jl))
        delivery503 = bool(re.search(r"503|Failed to download|curl: \(\d+\)", jl))
        rep.append(f"- FAILURE-рулетка: crash={crash}, band={band}, unbound={unbound}, "
                   f"delivery-503={delivery503}")
        if verdict in ("NO-PROFILE", "DELIVERY-FAIL") or not stdout_txt:
            if unbound and not band:
                verdict = "INFRA-SCRIPT-FAIL"
                rep.append("  -> **INFRA-SCRIPT-FAIL** (set -u умер на unbound variable — "
                           "root-cause fix в bench-скрипте, потом ре-диспатч §5)")
            elif band and crash == 0:
                verdict = "BAND-DISCARD"
                rep.append("  -> **BAND-DISCARD** (S7-96d fast-fail, не вердикт) — "
                           "ре-диспатч dispatch_s7207.py (макс 2 подряд)")
            elif delivery503 and crash == 0:
                verdict = "INFRA-DELIVERY-FAIL"
                rep.append("  -> **INFRA-DELIVERY-FAIL** (503/сеть внешних загрузок — "
                           "ре-ролл ТОЛЬКО после root-cause §5; хардинг 785b0a4 стоит)")
            elif crash:
                verdict = "CRASH-REFUTED"
                rep.append("  -> **CRASH-REFUTED** — руут-кауз по job-логу")

    rep.append(f"\n## VERDICT: **{verdict}**")
    outp = os.path.join(RESDIR, "ABSORB_S7207.md")
    open(outp, "w", encoding="utf-8").write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {outp}")


def tps_series_p(path):
    out = []
    for line in open(path, errors="ignore"):
        m = re.search(r"TPS from last 5s, 1m, 5m, 15m: ([\d.]+)", line)
        if m and float(m.group(1)) < 20:
            out.append(float(m.group(1)))
    return out


def statistics_median(xs):
    xs = sorted(xs)
    n = len(xs)
    if not n:
        return 0.0
    return xs[n // 2] if n % 2 else (xs[n // 2 - 1] + xs[n // 2]) / 2.0


def interp_tps(runner_idx):
    """Линейная интерполяция TPS_exp по 2 точкам банка v4 (fp=4)."""
    (t1, r1), (t2, r2) = BANK_POINTS
    if r2 == r1:
        return t1
    return t2 + (t1 - t2) * (runner_idx - r2) / (r1 - r2)


if __name__ == "__main__":
    main()
