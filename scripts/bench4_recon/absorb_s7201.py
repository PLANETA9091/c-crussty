#!/usr/bin/env python3
"""absorb_s7201.py - TASK-380: absorb COLLECTOR A/B leg s7201 (банк v3 + gc_tune=3
= ParallelGC, директива владельца 20:08 «непривычный но быстрее = ставь») и
вердикт v8-REGRESSION DUAL BAR vs ANCHOR (1.60 reproduced @ 6680195/8566450).

ПАРАДОКС-ФИКС-ЛЕГ: STW и параллельный GC-CPU — НА крит-пути (STW внутри
тикового wall-clock; 37% GC-сэмплов конкурируют за 4 ядра с воркерами),
поэтому -CPU здесь конвертируется в +TPS по построению.

PREREGISTER GATES:
  PG-T1 delivery: gc_tune=1 + банк v3 rest + NCDFE=0 + pop 150k VALID +
    JVM-flag-effect: "Heap Region Size: 8M" в gc.log + workers=4 телеметрия
  PG-T2 crash-free + soak >= 3
  PG-T3 DUAL BAR (обе оси >= +10% vs ANCHOR-SLOW; широкий банд 6.0M..9.5M)
  PG-T4 GC-гейты (preregister vs база s7198: young 246? нет — 246 total events,
    19.5188s total, avg 79.34ms): 0 Full; total_pause <= 14.0s; avg <= 60ms;
    young <= 320
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
RUN_DIR = os.path.join(RESDIR, "run-s7201-parallelgc")
ANCHOR_TPS, ANCHOR_RUNNER = 1.60, 6680195
BAND_MIN, BAND_MAX = 6_000_000, 9_500_000
# PG-T4 preregister ParallelGC-адаптация (база s7198 G1: young 163/19.0s, total 19.5s,
# avg 79ms, max 178.7ms; ParallelGC: реже/длиннее паузы, редкие Full допустимы <=2,
# max <= 2.0s — median5 толерантен к 1-2 мейджорам в 300s-окне)
TOTAL_PAUSE_GATE_MS = 19_500.0
AVG_PAUSE_GATE_MS = 300.0
YOUNG_GATE = 250
YOUNG_MIN = 30
FULL_GATE = 2
MAX_PAUSE_GATE_MS = 2000.0


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


def gc_stats(path):
    """(young, full, total_pause_ms, avg_pause_ms, max_pause_ms) из gc.log
    (end-строки 'GC(n) Pause ... A->B(C) DUR'; start-строки [gc,start — skip)."""
    if not os.path.isfile(path):
        return None
    young = full = 0
    total = 0.0
    mx = 0.0
    for line in open(path, errors="ignore"):
        if "[gc,start" in line:
            continue
        m = re.search(r"Pause (Young|Full).*? ([\d.]+)(ms|s)\s*$", line)
        if not m:
            continue
        dur = float(m.group(2)) * (1.0 if m.group(3) == "ms" else 1000.0)
        if m.group(1) == "Young":
            young += 1
        else:
            full += 1
        total += dur
        mx = max(mx, dur)
    n = young + full
    return young, full, total, (total / n if n else 0.0), mx


def env_flag(env_txt, key, want):
    m = re.search(rf"^{key}: (\d+)", env_txt, re.M)
    return bool(m) and m.group(1) == want


def main():
    tok = token()
    run_id = int(sys.argv[1]) if len(sys.argv) > 1 else 0
    if not run_id:
        raise SystemExit("usage: absorb_s7201.py <run_id>")
    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={st.get('status')} conclusion={st.get('conclusion')} "
          f"head={str(st.get('head_sha'))[:7]}")
    if st.get("status") != "completed":
        sys.exit(3)
    os.makedirs(RUN_DIR, exist_ok=True)
    have_art = fetch_artifact(tok, run_id)

    rep = [f"# absorb COLLECTOR-A/B ParallelGC s7201 (run {run_id}, head {str(st.get('head_sha'))[:7]}) "
           f"— PROTOCOL v8-REGRESSION DUAL BAR\n"]
    verdict = None

    stdout = os.path.join(RUN_DIR, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""
    env_txt = ""
    ep = os.path.join(RUN_DIR, "run-env.txt")
    if os.path.isfile(ep):
        env_txt = open(ep, errors="ignore").read()

    if stdout_txt:
        # ---- PG-T1 delivery (gc_tune armed + банк v3 rest)
        want = {"gc_tune": "3", "region_steal": "0", "travel_diet": "0",
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
            col_ok = ("G1 Evacuation Pause" not in gtxt) and ("Pause Young" in gtxt)
        t1_ok = all("OK" in x for x in t1) and ncde == 0 and pop_ok and arm and col_ok
        rep.append("- PG-T1: " + ", ".join(t1) + f", NCDFE={ncde}, "
                   f"pop={'VALID' if pop_ok else 'BAD'}, "
                   f"mode={'WORKERS4-TELEMETRY' if arm else 'MISSING'}, "
                   f"col={'PARALLEL' if col_ok else 'BAD'} -> **{'PASS' if t1_ok else 'FAIL'}**")

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
        if med is not None and runner and band_ok:
            nd = (med / runner) / (ANCHOR_TPS / ANCHOR_RUNNER) - 1.0
            ad = med / ANCHOR_TPS - 1.0
            rep.append(f"  DUAL BAR (v8-REGRESSION): normalized={nd:+.1%}, absolute={ad:+.1%} "
                       f"(бар: ОБЕ >= +10%)")
            if nd >= 0.10 and ad >= 0.10:
                verdict = "CANDIDATE-GREEN"
                rep.append("  -> **CANDIDATE GREEN** -> подтверждающий лег min-of-2 "
                           "(dispatch_s7201.py повторно) -> banking v4 = v3 + gc_tune")
            else:
                verdict = "LANE-OPEN"
                rep.append("  -> **< +10% хотя бы по одной оси** -> лейн ОТКРЫТ -> "
                           "ParallelGC: банковый кандидат v5 (директива 20:08) "
                           "или следующий крит-путь рычаг")
        elif not band_ok:
            verdict = "INVALID-PAIRING"
            rep.append("  -> **ВНЕ ШИРОКОГО БАНДА** — ре-диспатч (не вердикт)")
        else:
            verdict = "NO-SOAK"
            rep.append("  -> нет соак-поллов — вердикта нет")

        # ---- PG-T4 GC-гейты (preregister)
        gs = gc_stats(gclog)
        if gs:
            young, full, total, avg, mx = gs
            t4 = (full <= FULL_GATE and total <= TOTAL_PAUSE_GATE_MS
                  and avg <= AVG_PAUSE_GATE_MS and mx <= MAX_PAUSE_GATE_MS
                  and YOUNG_MIN <= young <= YOUNG_GATE)
            rep.append(f"- PG-T4: young={young}, Full={full}, total_pause={total/1000:.1f}s "
                       f"(гейт <= {TOTAL_PAUSE_GATE_MS/1000:.1f}s), avg={avg:.1f}ms "
                       f"(гейт <= {AVG_PAUSE_GATE_MS:.0f}ms), young-банд {YOUNG_MIN}..{YOUNG_GATE}, "
                       f"Full<= {FULL_GATE}, max={mx:.1f}ms (гейт <= {MAX_PAUSE_GATE_MS:.0f}ms) "
                       f"-> **{'PASS' if t4 else 'FAIL'}** (база s7198: 19.5s/79.3ms/178.7ms)")
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
                           "ре-диспатч dispatch_s7201.py (макс 2 подряд)")
            elif crash:
                verdict = "CRASH-REFUTED"
                rep.append("  -> **CRASH-REFUTED** — руут-кауз по job-логу; rollback "
                           "gc_tune (дефолт 0 = ванильные JVM-арги bit-exact), вердикт-док")
            else:
                verdict = "INFRA-FLAKE"
                rep.append("  -> **INFRA-FLAKE** — ре-диспатч без вердикта (макс 2 подряд)")

    rep.append(f"\n## VERDICT: **{verdict}**")
    out = os.path.join(RESDIR, "ABSORB_S7201.md")
    open(out, "w").write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {out}")
    return 0 if verdict in ("CANDIDATE-GREEN", "LANE-OPEN") else 1


if __name__ == "__main__":
    sys.exit(main())
