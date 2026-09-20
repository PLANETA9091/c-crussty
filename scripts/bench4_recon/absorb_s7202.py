#!/usr/bin/env python3
"""absorb_s7202.py - TASK-383: absorb COLLECTOR A/B #2 leg s7202 (банк v4 + gc_tune=4
= ZGC generational) и вердикт v8-REGRESSION DUAL BAR vs ANCHOR-БАНКА v4
(2.60 @ 8551924 leg#1; банк v4 = v3 + UseParallelGC, GOAL ×68).

ТЕЗИС A/B: ZGC-generational убирает STW-паузы (sub-ms) ценой concurrent-CPU
(~10-15%) на 4-ядерном раннере — прямой антипод ParallelGC (ядра свободны,
паузы длинные). Победа только если concurrent-CPU дёшев, а исчезновение STW
конвертируется. Директива 20:08: непривычный-но-быстрый = ставим.

PREREGISTER GATES:
  PG-T1 delivery: gc_tune=4 + банк rest + NCDFE=0 + pop 150k VALID +
    collector-proof: "Using The Z Garbage Collector" в gc.log (и НЕТ
    "G1 Evacuation Pause") + workers=4 телеметрия
  PG-T2 crash-free + soak >= 3
  PG-T3 DUAL BAR (обе оси >= +10% vs ANCHOR-БАНКА v4; широкий банд 6.0M..9.5M)
  PG-T4 ZGC-гейты (пререгистр TASK-381): Full <= 2; total_pause <= 10.0s;
    max_pause <= 50ms; pause-count <= 3000 (ZGC: Pause Mark Start/End +
    Relocate Start — 3 события/цикл, sub-ms)
  PG-T5 DONE-park N/A tolerated (AP-PID defect).
Failure handling: BAND-DISCARD / CRASH-REFUTED (rollback gc_tune default 0 =
vanilla JVM args bit-exact) / INFRA-FLAKE, max 2 подряд.
rc: 0 = CANDIDATE-GREEN/LANE-OPEN, 1 = fail/refuted, 2 = INFRA-FLAKE,
3 = run not completed, 5 = incomplete data.
"""
import os, re, statistics, subprocess, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUN_DIR = os.path.join(RESDIR, "run-s7202-zgcgen")
# БАНК v4 якорь = ДВЕ воспроизведённые точки (TPS индекс-зависим на ParallelGC):
#   leg#1 2.6 @ 8551924, leg#2 2.2 @ 6653417 (обе CANDIDATE-GREEN, min-of-2)
BANK_POINTS = ((2.6, 8_551_924), (2.2, 6_653_417))
BAND_MIN, BAND_MAX = 6_000_000, 9_500_000
# PG-T4 ZGC preregister (TASK-381): sub-ms pause-модель; база ParallelGC v4 leg#1:
# total 24.6s, max 2954ms, Full=9 — ZGC должен убрать длинные паузы ПОЛНОСТЬЮ
TOTAL_PAUSE_GATE_MS = 10_000.0
MAX_PAUSE_GATE_MS = 50.0
FULL_GATE = 2
PAUSE_COUNT_GATE = 3000


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


def gc_stats_zgc(path):
    """(pauses, full, total_pause_ms, avg_pause_ms, max_pause_ms) из gc.log ZGC.
    События: GC(n) Pause Mark Start/Mark End/Relocate Start DUR; 'Full' отдельно
    (allocation stall / OOM-fallback). Начальные [gc,start-строки skip."""
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
            # fallback: любой Pause с длительностью в конце
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
        raise SystemExit("usage: absorb_s7202.py <run_id>")
    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={st.get('status')} conclusion={st.get('conclusion')} "
          f"head={str(st.get('head_sha'))[:7]}")
    if st.get("status") != "completed":
        sys.exit(3)
    os.makedirs(RUN_DIR, exist_ok=True)
    have_art = fetch_artifact(tok, run_id)

    rep = [f"# absorb COLLECTOR-A/B #2 ZGC-gen s7202 (run {run_id}, head {str(st.get('head_sha'))[:7]}) "
           f"— PROTOCOL v8-REGRESSION DUAL BAR vs БАНК v4 (2.60 @ {ANCHOR_RUNNER})\n"]
    verdict = None

    stdout = os.path.join(RUN_DIR, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""
    env_txt = ""
    ep = os.path.join(RUN_DIR, "run-env.txt")
    if os.path.isfile(ep):
        env_txt = open(ep, errors="ignore").read()

    if stdout_txt:
        # ---- PG-T1 delivery (gc_tune armed + банк rest)
        want = {"gc_tune": "4", "region_steal": "0", "travel_diet": "0",
                "skip_store_bb": "0", "bu_defer": "0", "inside_cache": "1",
                "flush_diet": "1", "region_threads": "4", "batch_collector": "1",
                "fluid_guard": "1"}
        t1 = [f"{k}={'OK' if env_flag(env_txt, k, v) else 'BAD'}" for k, v in want.items()]
        ncde = stdout_txt.count("NoClassDefFoundError")
        pop_ok = "POPULATION FIXTURE-VALIDITY: VALID" in stdout_txt
        arm = "workers=4" in stdout_txt
        gclog = os.path.join(RUN_DIR, "gc.log")
        col_ok = False
        if os.path.isfile(gclog):
            gtxt = open(gclog, errors="ignore").read()
            col_ok = ("G1 Evacuation Pause" not in gtxt) and \
                     ("Using The Z Garbage Collector" in gtxt or "ZGC" in gtxt)
        t1_ok = all("OK" in x for x in t1) and ncde == 0 and pop_ok and arm and col_ok
        rep.append("- PG-T1: " + ", ".join(t1) + f", NCDFE={ncde}, "
                   f"pop={'VALID' if pop_ok else 'BAD'}, "
                   f"mode={'WORKERS4-TELEMETRY' if arm else 'MISSING'}, "
                   f"col={'ZGC' if col_ok else 'BAD'} -> **{'PASS' if t1_ok else 'FAIL'}**")

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
                       f"данным НЕ валиден** (урок #167) — ре-ролл dispatch_s7202.py")
        elif med is not None and runner and band_ok:
            # ДВУХТОЧЕЧНАЯ модель банка v4 (TASK-383): ось-1 normalized —
            # минимум по обеим ногам банка (строгий min-of-2 якорь);
            # ось-2 absolute — интерполяция TPS_exp(runner) через 2 точки банка
            nds = []
            for atps, arun in BANK_POINTS:
                nds.append((med / runner) / (atps / arun) - 1.0)
            nd = min(nds)
            (t1, r1), (t2, r2) = BANK_POINTS
            slope = (t2 - t1) / (r2 - r1)
            tps_exp = t1 + slope * (runner - r1)
            ad = med / tps_exp - 1.0
            rep.append(f"  DUAL BAR (v8-REGRESSION, банк v4 2-точки): "
                       f"normalized=min({nds[0]:+.1%}, {nds[1]:+.1%})={nd:+.1%}, "
                       f"absolute={ad:+.1%} (TPS_exp@{runner}={tps_exp:.2f}; "
                       f"бар: ОБЕ >= +10%)")
            if nd >= 0.10 and ad >= 0.10:
                verdict = "CANDIDATE-GREEN"
                rep.append("  -> **CANDIDATE GREEN** -> подтверждающий лег min-of-2 "
                           "(dispatch_s7202.py повторно) -> banking v5 = v4 + ZGC")
            else:
                verdict = "LANE-OPEN"
                rep.append("  -> **< +10% хотя бы по одной оси** -> лейн ОТКРЫТ -> "
                           "ZGC НЕ ПРЕВЗОШЁЛ ParallelGC-банк; следующий кандидат "
                           "очереди (ParallelGCThreads-tuning / THP) в новом тике")
        elif not band_ok:
            verdict = "INVALID-PAIRING"
            rep.append("  -> **ВНЕ ШИРОКОГО БАНДА** — ре-диспатч (не вердикт)")
        else:
            verdict = "NO-SOAK"
            rep.append("  -> нет соак-поллов — вердикта нет")

        # ---- PG-T4 ZGC-гейты (preregister)
        gs = gc_stats_zgc(gclog)
        if gs:
            pauses, full, total, avg, mx = gs
            t4 = (full <= FULL_GATE and total <= TOTAL_PAUSE_GATE_MS
                  and mx <= MAX_PAUSE_GATE_MS and pauses + full <= PAUSE_COUNT_GATE)
            rep.append(f"- PG-T4: pauses={pauses}, Full={full}, total_pause={total/1000:.1f}s "
                       f"(гейт <= {TOTAL_PAUSE_GATE_MS/1000:.1f}s), avg={avg:.3f}ms, "
                       f"max={mx:.3f}ms (гейт <= {MAX_PAUSE_GATE_MS:.0f}ms) "
                       f"-> **{'PASS' if t4 else 'FAIL'}** "
                       f"(банк v4 leg#1 ParallelGC: 24.6s/max 2954ms/Full=9)")
            if verdict is None and not t4:
                verdict = "GC-FAIL"
        else:
            rep.append("- PG-T4: gc.log отсутствует -> N/A (артефакт-дефект)")

        # ---- PG-T5 park (N/A tolerated)
        spin = "MinecraftServer.lambda$spin$2"
        p = os.path.join(RUN_DIR, "wall-collapsed.txt")
        if os.path.isfile(p):
            tot = park = 0
            for line in open(p, errors="ignore"):
                stack, _, cnt = line.rstrip("\n").rpartition(" ")
                if spin in stack:
                    try:
                        n = int(cnt)
                    except ValueError:
                        continue
                    tot += n
                    if "CyclicBarrier" in stack:
                        park += n
            share = park / tot if tot else None
            rep.append(f"- PG-T5: park={park}/{tot}"
                       + (f" ({share:.1%}; класс ~1.4%)" if share is not None else "")
                       + " -> PASS/N/A")
        else:
            rep.append("- PG-T5: wall-окно отсутствует (AP-PID дефект) -> **N/A**")

        # ---- AUTHORITATIVE workflow-gate crosscheck (TASK-383)
        bt = os.path.join(RUN_DIR, "BOTTLENECKS_3.md")
        if os.path.isfile(bt):
            btxt = open(bt, errors="ignore").read()
            if "FIXTURE-VALIDITY: INVALID" in btxt:
                verdict = "INFRA-FLAKE"
                rep.append("- PG-GATE: workflow-гейт сказал **FIXTURE-VALIDITY: INVALID** "
                           "-> лег discard (не вердикт), любой предыдущий GREEN/LANE-OPEN "
                           "АННУЛИРОВАН — ре-ролл dispatch_s7202.py")

    if not have_art or not stdout_txt:
        jl = fetch_joblog(tok, run_id)
        crash = sum(jl.count(x) for x in
                    ("Encountered an unexpected exception", "ReportedException",
                     "ObjectOpenHashSet$SetIterator", "NullPointerException"))
        wedge = jl.count("Watchdog") + jl.count("syncLoad") + jl.count("managedBlock")
        fixture = jl.count("FIXTURE-VALIDITY: INVALID")
        band = "OUTSIDE band" in jl
        rep.append(f"- FAILURE-рулетка: crash-маркеры={crash}, wedge={wedge}, "
                   f"fixture-INVALID={fixture}, band-discard={band}")
        if verdict is None:
            if band and crash == 0 and fixture == 0:
                verdict = "BAND-DISCARD"
                rep.append("  -> **BAND-DISCARD** (S7-96d fast-fail, не вердикт) — "
                           "ре-диспатч dispatch_s7202.py (макс 2 подряд)")
            elif crash:
                verdict = "CRASH-REFUTED"
                rep.append("  -> **CRASH-REFUTED** — руут-кауз по job-логу; rollback "
                           "gc_tune (дефолт 0 = ванильные JVM-арги bit-exact), вердикт-док")
            else:
                verdict = "INFRA-FLAKE"
                rep.append("  -> **INFRA-FLAKE** — ре-диспатч без вердикта (макс 2 подряд)")

    rep.append(f"\n## VERDICT: **{verdict}**")
    out = os.path.join(RESDIR, "ABSORB_S7202.md")
    open(out, "w").write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {out}")
    return 0 if verdict in ("CANDIDATE-GREEN", "LANE-OPEN") else 1


if __name__ == "__main__":
    sys.exit(main())
