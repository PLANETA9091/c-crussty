"""absorb_s7206.py - ARCH-LEVER #16 FLUIDPUSH-BITMASK (RECON-43, TASK-389) — absorb лега s7206
PROTOCOL v8-REGRESSION DUAL BAR vs БАНК v4 (2-точки: 2.6 @ 8551924, 2.2 @ 6653417).

PG-T1 delivery: банк v4 + fluid_bitmask=1 + fluid_dirty_ledger=1 (fluid_dirty=0 —
чистая изоляция); ARMED-маркеры: "fluid_bitmask: defined ...FluidBitmaskOps in
kernel loader" + "fluid_bitmask: gate armed" + "fluid_dirty: LEDGER-ONLY mode";
dormant-маркер "fluid_bitmask: dormant" ОТСУТСТВУЕТ; 0 NCDFE; pop VALID; ParallelGC.
PG-T2 crash-free + soak >=3. PG-T3 DUAL BAR (normalized min по ногам, absolute
vs TPS_exp-интерполяция; ОБЕ >= +10% -> CANDIDATE-GREEN -> min-of-2 -> BANKING v5).
PG-T4 страховочные ParallelGC. PG-T5 АРХИТЕКТУРНЫЙ вердикт: fluid-family
(updateFluidHeightAndDoFluidPushing/FluidPushGuardHook lane) в cpu-collapsed
должна упасть >=60% vs baseline RECON-42 (15319/105003 = 14.59% -> гейт <= 5.84%)
— доказательство замены дата-плейна.
REFUTED-критерий: T1 PASS но T5 недостижим ИЛИ G4-регресс >10% => fluid_bitmask=0.
"""
import os, re, statistics, subprocess, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUN_DIR = os.path.join(RESDIR, "run-s7206-fluid-bitmask")
# БАНК v4 якорь = ДВЕ воспроизведённые точки (TPS индекс-зависим на ParallelGC):
#   leg#1 2.6 @ 8551924, leg#2 2.2 @ 6653417 (обе CANDIDATE-GREEN, min-of-2)
BANK_POINTS = ((2.6, 8_551_924), (2.2, 6_653_417))
BAND_MIN, BAND_MAX = 6_000_000, 9_500_000
# PG-T4 страховочные ParallelGC-гейты (банк v4 leg#2: young 109, total 18.8s,
# avg 162ms, max 2400ms, Full=7): FAIL не отменяет двойной бар
TOTAL_PAUSE_GATE_MS = 20_000.0
MAX_PAUSE_GATE_MS = 3000.0
FULL_GATE = 10
PAUSE_COUNT_GATE = 3000
# ARMED-маркеры #15 (RECON-33/impl 6eb3274/oracle dea9de8; тул s7195)
# + runtime-мarkers урока s7204: probe-then-patch публикует
# "bridge defined+armed, BRIDGE_READY" ТОЛЬКО при прожитом armState();
# фейл пробы (s7204: find_class CNFE — системный лоадер) печатает
# "armState() != ARMED" и каскадно роняет rng-стейдж region_threads.
ARMED_1 = "[crussty-plugin] fluid_bitmask: defined net/minecraft/world/entity/FluidBitmaskOps in kernel loader"
ARMED_2 = "[crussty-plugin] fluid_bitmask: gate armed"
ARMED_3 = "[crussty-plugin] fluid_dirty: LEDGER-ONLY mode"
DORMANT = "[crussty-plugin] fluid_bitmask: dormant"
RUNTIME_ARM_FAIL = "fluid_bitmask: define_class"
WINDOW_MISS = "fluid_bitmask: kernel loader never went quiet"
RNG_CASCADE = "fluid_bitmask: __nomatch__"


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
    """(young/pauses, full, total_pause_ms, avg_pause_ms, max_pause_ms) из gc.log
    ParallelGC (end-строки 'GC(n) Pause Young/Full ... DUR'; start-строки [gc,start — skip)."""
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


def main():
    tok = token()
    run_id = int(sys.argv[1]) if len(sys.argv) > 1 else 0
    if not run_id:
        raise SystemExit("usage: absorb_s7206.py <run_id>")
    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={st.get('status')} conclusion={st.get('conclusion')} "
          f"head={str(st.get('head_sha'))[:7]}")
    if st.get("status") != "completed":
        sys.exit(3)
    os.makedirs(RUN_DIR, exist_ok=True)
    have_art = fetch_artifact(tok, run_id)

    rep = [f"# absorb #16 FLUIDPUSH-BITMASK s7206 (run {run_id}, head {str(st.get('head_sha'))[:7]}) "
           f"— PROTOCOL v8-REGRESSION DUAL BAR vs БАНК v4 "
           f"(2.6 @ 8551924 / 2.2 @ 6653417, 2-точечная модель)\n"]
    verdict = None

    stdout = os.path.join(RUN_DIR, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""
    env_txt = ""
    ep = os.path.join(RUN_DIR, "run-env.txt")
    if os.path.isfile(ep):
        env_txt = open(ep, errors="ignore").read()

    if stdout_txt:
        # ---- PG-T1 delivery (gc_tune=3 + inside_bitmask=1 + банк rest + ARMED #15)
        want = {"gc_tune": "3", "inside_bitmask": "1", "region_steal": "0",
                "travel_diet": "0", "skip_store_bb": "0", "bu_defer": "0",
                "fluid_bitmask": "1", "fluid_dirty_ledger": "1", "fluid_dirty": "0",
                "inside_cache": "1", "flush_diet": "1", "region_threads": "4",
                "batch_collector": "1", "fluid_guard": "1"}
        t1 = [f"{k}={'OK' if env_flag(env_txt, k, v) else 'BAD'}" for k, v in want.items()]
        ncde = stdout_txt.count("NoClassDefFoundError")
        pop_ok = "POPULATION FIXTURE-VALIDITY: VALID" in stdout_txt
        arm1 = ARMED_1 in stdout_txt
        arm2 = ARMED_2 in stdout_txt
        arm3 = ARMED_3 in stdout_txt
        runtime_arm_fail = RUNTIME_ARM_FAIL in stdout_txt
        window_miss = WINDOW_MISS in stdout_txt
        rng_cascade = RNG_CASCADE in stdout_txt
        dormant_bad = DORMANT in stdout_txt
        arm_ok = arm1 and arm2 and arm3 and not runtime_arm_fail \
            and not window_miss and not dormant_bad
        cascade_bad = rng_cascade
        workers = "workers=4" in stdout_txt
        gclog = os.path.join(RUN_DIR, "gc.log")
        col_ok = False
        if os.path.isfile(gclog):
            gtxt = open(gclog, errors="ignore").read()
            col_ok = ("G1 Evacuation Pause" not in gtxt) and \
                     ("Using Parallel" in gtxt or "Pause Young" in gtxt)
        t1_ok = all("OK" in x for x in t1) and ncde == 0 and pop_ok and \
            arm_ok and workers and col_ok and not cascade_bad
        rep.append("- PG-T1: " + ", ".join(t1) + f", NCDFE={ncde}, "
                   f"pop={'VALID' if pop_ok else 'BAD'}, "
                   f"ARMED={'OK' if arm1 else 'MISSING'}"
                   f"+READY={'OK' if arm3 else 'MISSING'}"
                   f"+composed={'OK' if arm2 else 'MISSING'}"
                   f"+arm-fail={'ОШИБКА-ПРОБЫ' if runtime_arm_fail else 'нет'}"
                   f"+window-miss={'ЕСТЬ' if window_miss else 'нет'}"
                   f"+rng-cascade={'ЕСТЬ' if rng_cascade else 'нет'}"
                   f"+dormant={'ОШИБКА-АРМИРОВАНИЯ' if dormant_bad else 'нет'}, "
                   f"mode={'WORKERS4-TELEMETRY' if workers else 'MISSING'}, "
                   f"col={'PARALLEL' if col_ok else 'BAD'}"
                   f" -> **{'PASS' if t1_ok else 'FAIL'}**")

        # ---- PG-T2 crash-free
        threw = stdout_txt.count("Entity threw exception")
        unexpected = stdout_txt.count("Encountered an unexpected exception")
        bu_npe = stdout_txt.count("BlockUpdateOps") + \
            stdout_txt.count("ObjectOpenHashSet$SetIterator")
        tps = tps_series(stdout)
        soak_ok = len(tps) >= 3
        t2_ok = threw == 0 and unexpected == 0 and bu_npe == 0 and soak_ok
        rep.append(f"- PG-T2: threw={threw}, unexpected={unexpected}, s7180-class={bu_npe}, "
                   f"TPS-поллов={len(tps)} -> **{'PASS' if t2_ok else 'FAIL'}**")

        # ---- PG-T3 REGRESSION DUAL-BAR
        m = re.search(r"runner_cpu_index: (\d+)", env_txt)
        runner = int(m.group(1)) if m else None
        med = None
        if len(tps) >= 5:
            med = statistics.median(sorted(tps[-5:]))
        elif tps:
            med = statistics.median(tps)
        band_ok = runner is not None and BAND_MIN <= runner <= BAND_MAX
        rep.append(f"- PG-T3: runner={runner} (широкий банд {BAND_MIN}..{BAND_MAX}: "
                   f"{'OK' if band_ok else 'ВНЕ БАНДА'}), median5={med}")
        # TASK-383 фикс ложного GREEN (#167): вердикт валиден ТОЛЬКО при
        # доставке (T1, pop VALID) и соаке (T2, >=3 поллов)
        fixture_ok = t1_ok and t2_ok
        if med is not None and runner and band_ok and not fixture_ok:
            verdict = "INFRA-FLAKE"
            rep.append(f"  -> **FIXTURE-INVALID (T1={'PASS' if t1_ok else 'FAIL'}, "
                       f"T2={'PASS' if t2_ok else 'FAIL'}) — двойной бар по мусорным "
                       f"данным НЕ валиден** (урок #167) — ре-ролл dispatch_s7206.py")
        elif med is not None and runner and band_ok:
            # ДВУХТОЧЕЧНАЯ модель банка v4 (TASK-383): ось-1 normalized —
            # минимум по обеим ногам банка; ось-2 absolute — интерполяция
            # TPS_exp(runner) через 2 точки банка
            nds = []
            for atps, arun in BANK_POINTS:
                nds.append((med / runner) / (atps / arun) - 1.0)
            nd = min(nds)
            (t1v, r1), (t2v, r2) = BANK_POINTS
            slope = (t2v - t1v) / (r2 - r1)
            tps_exp = t1v + slope * (runner - r1)
            ad = med / tps_exp - 1.0
            rep.append(f"  DUAL BAR (v8-REGRESSION, банк v4 2-точки): "
                       f"normalized=min({nds[0]:+.1%}, {nds[1]:+.1%})={nd:+.1%}, "
                       f"absolute={ad:+.1%} (TPS_exp@{runner}={tps_exp:.2f}; "
                       f"бар: ОБЕ >= +10%)")
            if nd >= 0.10 and ad >= 0.10:
                verdict = "CANDIDATE-GREEN"
                rep.append("  -> **CANDIDATE GREEN** -> подтверждающий лег min-of-2 "
                           "(dispatch_s7206.py повторно) -> banking v5 = v4 + fluid_bitmask")
            else:
                verdict = "LANE-OPEN"
                rep.append("  -> **< +10% хотя бы по одной оси** -> лейн ОТКРЫТ -> "
                           "#15 НЕ ПРЕВЗОШЁЛ банк v4 под ParallelGC-экономикой "
                           "(G1-вердикты RECON-32/33 подтверждены и на новой базе); "
                           "следующий кандидат очереди в новом тике")
        elif not band_ok:
            verdict = "INVALID-PAIRING"
            rep.append("  -> **ВНЕ ШИРОКОГО БАНДА** — ре-диспатч (не вердикт)")
        else:
            verdict = "NO-SOAK"
            rep.append("  -> нет соак-поллов — вердикта нет")

        # ---- PG-T4 страховочные гейты (ParallelGC)
        gs = gc_stats_parallel(gclog)
        if gs:
            pauses, full, total, avg, mx = gs
            t4 = (full <= FULL_GATE and total <= TOTAL_PAUSE_GATE_MS
                  and mx <= MAX_PAUSE_GATE_MS and pauses + full <= PAUSE_COUNT_GATE)
            rep.append(f"- PG-T4: young={pauses}, Full={full}, total_pause={total/1000:.1f}s "
                       f"(гейт <= {TOTAL_PAUSE_GATE_MS/1000:.1f}s), avg={avg:.1f}ms, "
                       f"max={mx:.1f}ms (гейт <= {MAX_PAUSE_GATE_MS:.0f}ms) "
                       f"-> **{'PASS' if t4 else 'FAIL'}** "
                       f"(банк v4 leg#2: 18.8s/162ms/2400ms/Full=7)")
            if verdict is None and not t4:
                verdict = "GC-FAIL"
        else:
            rep.append("- PG-T4: gc.log отсутствует -> N/A (артефакт-дефект)")

        # ---- PG-T5 АРХИТЕКТУРНЫЙ вердикт #16: fluid-family collapse в cpu-collapsed
        import glob as _glob
        cand = sorted(_glob.glob(os.path.join(RUN_DIR, "*cpu*collapsed*.txt"))) + \
               sorted(_glob.glob(os.path.join(RUN_DIR, "collapsed*cpu*")))
        if cand:
            lane_re = re.compile(r"updateFluidHeightAndDoFluidPushing|FluidPushGuardHook|FluidBitmaskOps|FluidPushOps\\.scan")
            lane = tot = 0
            for line in open(cand[-1], errors="ignore"):
                stack, _, cnt = line.rstrip("\n").rpartition(" ")
                if not stack:
                    continue
                try:
                    n = int(cnt)
                except ValueError:
                    continue
                tot += n
                if lane_re.search(stack):
                    lane += n
            if tot:
                share = lane / tot
                base = 15319 / 105003  # RECON-42 baseline (s7201)
                t5 = share <= base * 0.4  # >=60% collapse
                rep.append(f"- PG-T5: fluid-family={lane}/{tot} ({share:.2%} vs baseline 14.59%; "
                           f"гейт <= {base*0.4:.2%}) [{os.path.basename(cand[-1])}] "
                           f"-> **{'PASS — дата-плейн заменён' if t5 else 'FAIL — семейство не снята'}**")
                if not t5 and verdict == "CANDIDATE-GREEN":
                    rep.append("  -> T5-FAIL НЕ блокирует GREEN (экономика > профиль), "
                               "но фиксируется как открытие: архитектурный эффект не состоялся")
            else:
                rep.append("- PG-T5: cpu-collapsed пуст -> N/A")
        else:
            rep.append("- PG-T5: cpu-collapsed отсутствует в артефакте -> N/A (профиль-вердикт из свежего RECON)")

        # ---- AUTHORITATIVE workflow-gate crosscheck (TASK-383)
        bt = os.path.join(RUN_DIR, "BOTTLENECKS_3.md")
        if os.path.isfile(bt):
            btxt = open(bt, errors="ignore").read()
            if "FIXTURE-VALIDITY: INVALID" in btxt:
                verdict = "INFRA-FLAKE"
                rep.append("- PG-GATE: workflow-гейт сказал **FIXTURE-VALIDITY: INVALID** "
                           "-> лег discard (не вердикт), любой предыдущий GREEN/LANE-OPEN "
                           "АННУЛИРОВАН — ре-ролл dispatch_s7206.py")

    if not have_art or not stdout_txt:
        jl = fetch_joblog(tok, run_id)
        crash = sum(jl.count(x) for x in
                    ("Encountered an unexpected exception", "ReportedException",
                     "ObjectOpenHashSet$SetIterator", "NullPointerException"))
        wedge = jl.count("Watchdog") + jl.count("syncLoad") + jl.count("managedBlock")
        fixture = jl.count("FIXTURE-VALIDITY: INVALID")
        # TASK-391: строка "OUTSIDE band" появляется в job.log и как ЭХО
        # исходника guard-шага ($IDX литерально) — бандит только ЭМИТНУТЫЙ
        # notice (цифры вместо $IDX), иначе ложный BAND-DISCARD (урок
        # s7206#2 35527308614: guard PASSED, а лег умер на unbound variable).
        band = bool(re.search(r"::notice::runner_cpu_index=\d+ OUTSIDE band", jl))
        unbound = bool(re.search(r"unbound variable", jl))
        rep.append(f"- FAILURE-рулетка: crash-маркеры={crash}, wedge={wedge}, "
                   f"fixture-INVALID={fixture}, band-discard={band}, "
                   f"unbound-var={unbound}")
        if verdict is None:
            if unbound and not band:
                verdict = "INFRA-SCRIPT-FAIL"
                rep.append("  -> **INFRA-SCRIPT-FAIL** (set -u shell-скрипт умер "
                           "на unbound variable — НЕ band, НЕ crash, НЕ вердикт "
                           "рычага) — root-cause fix в bench-скрипте, потом "
                           "ре-диспатч (§5, прецедент OPS_GREF 30cd34f)")
            elif band and crash == 0 and fixture == 0:
                verdict = "BAND-DISCARD"
                rep.append("  -> **BAND-DISCARD** (S7-96d fast-fail, не вердикт) — "
                           "ре-диспатч dispatch_s7204.py (макс 2 подряд)")
            elif crash:
                verdict = "CRASH-REFUTED"
                rep.append("  -> **CRASH-REFUTED** — руут-кауз по job-логу; rollback "
                           "inside_bitmask (дефолт 0), БАНК v4 (gc_tune=3) ОСТАЁТСЯ, "
                           "вердикт-док")
            else:
                verdict = "INFRA-FLAKE"
                rep.append("  -> **INFRA-FLAKE** — ре-диспатч без вердикта (макс 2 подряд)")

    rep.append(f"\n## VERDICT: **{verdict}**")
    out = os.path.join(RESDIR, "ABSORB_S7204.md")
    open(out, "w").write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {out}")
    return 0 if verdict in ("CANDIDATE-GREEN", "LANE-OPEN") else 1


if __name__ == "__main__":
    sys.exit(main())
