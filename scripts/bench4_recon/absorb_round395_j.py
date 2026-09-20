#!/usr/bin/env python3
"""absorb_round395_j.py — agent J (TASK-395, vector items_manager) absorb.

Хот-словарь = ТОЧНО диспатч dispatch_round395_j.py (урок TASK-392: полная
матрица инпутов, не частичная). Workflow: world-bench-parallel.yml
(per-ref concurrency), ref=round-395-j-manager, lever_flag=items_manager.
Якоря/TPS_exp/lane_map НЕ тронуты (BANK_POINTS/BASELINE_LANES из absorb_s7207);
items-лейн ДОЛЖЕН упасть (механизм выводит ItemEntity из общего dispatch).

Запуск: python3 scripts/bench4_recon/absorb_round395_j.py <run_id>
"""
import os, re, statistics, subprocess, sys, urllib.request, zipfile

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/rounds/ROUND-395/agent-J/run-j"
BANK_POINTS = ((2.6, 8_551_924), (2.2, 6_653_417))
BAND_MIN, BAND_MAX = 6_000_000, 9_500_000
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

# ХОТ-СЛОВАРЬ = ТОЧНО диспатч dispatch_round395_j.py (урок TASK-392)
WANT_INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0", "population_target": "150000",
    "population_seed": "42", "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
}


def token():
    try:
        tok = open("/tmp/gh_token").read().strip()
        if tok:
            return tok
    except OSError:
        pass
    url = subprocess.run(["git", "-C", "/home/z/rounds/ROUND-395/agent-J/repo",
                          "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token")
    return m.group(1)


def api(tok, url):
    import json
    req = urllib.request.Request(f"{API}{url}", headers={
        "Authorization": f"token {tok}", "Accept": "application/vnd.github+json"})
    try:
        with urllib.request.urlopen(req, timeout=60) as r:
            return json.loads(r.read())
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:200]}", file=sys.stderr)
        return {}


def _fetch_redirected(tok, url, dest):
    req = urllib.request.Request(url, headers={"Authorization": f"token {tok}"})

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
    arts = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/artifacts").get("artifacts", [])
    bench = [a for a in arts if a["name"] == "world3-bench"]
    if not bench:
        print(f"no world3-bench artifact (available: {[a['name'] for a in arts]})")
        return False
    a = bench[0]
    dest = os.path.join(RESDIR, a["name"] + ".zip")
    _fetch_redirected(tok, f"{API}/repos/{REPO}/actions/artifacts/{a['id']}/zip", dest)
    print(f"downloaded {dest} ({os.path.getsize(dest)} bytes)")
    with zipfile.ZipFile(dest) as z:
        z.extractall(RESDIR)
    return True


def fetch_joblog(tok, run_id):
    jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs").get("jobs", [])
    if not jobs:
        return ""
    dest = os.path.join(RESDIR, "job.log")
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
    m = re.search(rf"^{key}: (\S+)", env_txt, re.M)
    return bool(m) and m.group(1) == want


def env_val(env_txt, key):
    m = re.search(rf"^{key}: (\S+)", env_txt, re.M)
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


def interp_tps(runner_idx):
    (t1, r1), (t2, r2) = BANK_POINTS
    if r2 == r1:
        return t1
    return t2 + (t1 - t2) * (runner_idx - r2) / (r1 - r2)


def heap_mb_from_logs(gc_txt, env_txt):
    # best-effort: ParallelGC init line "Heap Size: ..." / xmx fallback
    for pat in (r"Heap Size: \d+K\)?,?.*?(\d+)M", r"Heap:\s*(\d+)M"):
        m = re.search(pat, gc_txt)
        if m:
            return int(m.group(1))
    m = re.search(r"^SERVER_XMX: (\d+)G", env_txt, re.M)
    if m:
        return int(m.group(1)) * 1024
    return 0


def main():
    tok = token()
    run_id = int(sys.argv[1]) if len(sys.argv) > 1 else 0
    if not run_id:
        raise SystemExit("usage: absorb_round395_j.py <run_id>")
    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={st.get('status')} conclusion={st.get('conclusion')} "
          f"head={str(st.get('head_sha'))[:7]} branch={st.get('head_branch')}")
    if st.get("status") != "completed":
        sys.exit(3)
    os.makedirs(RESDIR, exist_ok=True)
    have_art = fetch_artifact(tok, run_id)

    rep = [f"# absorb round-395-j (run {run_id}, head {str(st.get('head_sha'))[:7]}, "
           f"conclusion {st.get('conclusion')}) — vector items_manager на БАНКЕ v4\n"]
    verdict = None

    stdout = os.path.join(RESDIR, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""
    envp = os.path.join(RESDIR, "run-env.txt")
    env_txt = open(envp, errors="ignore").read() if os.path.isfile(envp) else ""
    logp = os.path.join(RESDIR, "world3-bench.log")
    log_txt = open(logp, errors="ignore").read() if os.path.isfile(logp) else ""

    # ---------- J-T1 delivery: хот-словарь = ТОЧНО диспатч + lever arm ----------
    if env_txt:
        checks = []
        t1 = True
        for k, v in WANT_INPUTS.items():
            ok = env_flag(env_txt, k, v)
            checks.append(f"{k}={'OK' if ok else 'BAD'}")
            t1 = t1 and ok
        lf = env_val(env_txt, "lever_flag") or ""
        lf_ok = lf == "items_manager"
        t1 = t1 and lf_ok
        pop_ok = "POPULATION FIXTURE-VALIDITY: VALID" in stdout_txt or \
                 "POPULATION FIXTURE-VALIDITY: VALID" in log_txt
        ncde = stdout_txt.count("NoClassDefFoundError") + log_txt.count("NoClassDefFoundError")
        col_ok = "Using Parallel" in (stdout_txt + log_txt)
        runner = env_val(env_txt, "runner_cpu_index") or "0"
        band_ok = BAND_MIN <= int(runner) <= BAND_MAX
        arm_bridge = "[crussty-plugin] items_manager: defined" in (stdout_txt + log_txt)
        arm_tele = "items_manager: telemetry" in (stdout_txt + log_txt)
        t1 = t1 and pop_ok and ncde == 0 and col_ok and band_ok
        rep.append(f"- J-T1: input-матрица {'OK' if all('BAD' not in c for c in checks) else 'BAD'}"
                   f", lever_flag={'OK' if lf_ok else 'BAD(' + lf + ')'}"
                   + ("" if not checks else " | " + " ".join(checks)))
        rep.append(f"- J-T1b: pop={'VALID' if pop_ok else 'INVALID'}, NCDFE={ncde}, "
                   f"col={'PARALLEL' if col_ok else 'BAD'}, "
                   f"runner={runner} (band {'OK' if band_ok else 'OUT'}), "
                   f"bridge-defined={'YES' if arm_bridge else 'NO'}, "
                   f"telemetry={'YES' if arm_tele else 'NO'} "
                   f"-> **{'PASS' if t1 else 'FAIL'}**")
        if not t1:
            verdict = "DELIVERY-FAIL"

    # ---------- J-T2 crash-free ----------
    if stdout_txt:
        threw = len(re.findall(r"Encountered an unexpected exception|ReportedException",
                               stdout_txt))
        polls = len(re.findall(r"TPS from last 5s", log_txt))
        t2 = threw == 0 and polls >= 3
        rep.append(f"- J-T2: threw={threw}, TPS-поллов={polls} -> **{'PASS' if t2 else 'FAIL'}**")
        if not t2 and verdict is None:
            verdict = "CRASH-REFUTED"

    # ---------- J-T3: median5 @ runner vs банк-якорь (delta TPS) ----------
    med = 0.0
    d_norm = 0.0
    tps_exp = 0.0
    if log_txt:
        ser = tps_series(log_txt)
        if ser:
            med = statistics_median(ser)
            r_idx = int(runner) if runner.isdigit() else 0
            tps_exp = interp_tps(r_idx)
            d_norm = (med / tps_exp - 1.0) if tps_exp else 0.0
            rep.append(f"- J-T3: median5={med:.2f} @ {runner}; банк-якорь (fp=4) "
                       f"TPS_exp={tps_exp:.2f}; дельта={d_norm:+.1%} "
                       f"(БЫЛО {tps_exp:.2f} -> СТАЛО {med:.2f} @ тот же runner)")
        else:
            rep.append("- J-T3: TPS-поллов нет в артефакте")
            if verdict is None:
                verdict = "NO-TPS"

    # ---------- J-T4 GC ----------
    gcs = gc_stats_parallel(os.path.join(RESDIR, "gc.log"))
    heap_mb = 0
    if gcs:
        pauses, full, total, avg, mx = gcs
        gc_txt = open(os.path.join(RESDIR, "gc.log"), errors="ignore").read() \
            if os.path.isfile(os.path.join(RESDIR, "gc.log")) else ""
        heap_mb = heap_mb_from_logs(gc_txt, env_txt)
        rep.append(f"- J-T4 GC (банк fp=4 справка 18.8s/162ms/2400ms/Full=7): "
                   f"young={pauses}, Full={full}, total={total/1000:.1f}s, avg={avg:.0f}ms, "
                   f"max={mx:.0f}ms, heap≈{heap_mb}M")
        stw_total_s = total / 1000.0
    else:
        stw_total_s = -1.0
        full = -1

    # ---------- J-T5 ПРОФИЛЬ-КАРТА: items-лейн был -> стал ----------
    items_after_pct = -1.0
    cpu = os.path.join(RESDIR, "cpu-collapsed.txt")
    if have_art and os.path.isfile(cpu):
        tot, lanes = lane_map(cpu)
        rep.append(f"- J-T5 ПРОФИЛЬ-КАРТА: total={tot} сэмплов (базлайн {BASELINE_TOTAL})")
        rows = []
        for k, (base, _) in BASELINE_LANES.items():
            cur = lanes[k]
            cur_p = cur / tot if tot else 0.0
            base_p = base / BASELINE_TOTAL
            rows.append((cur_p - base_p, k, cur, cur_p, base, base_p))
        for d, k, cur, cur_p, base, base_p in sorted(rows, reverse=True):
            arrow = "РОСТ" if d > 0.01 else ("спад" if d < -0.01 else "флэт")
            rep.append(f"  - {k}: банк {base_p:.2%} -> run {cur_p:.2%} ({d:+.2%}) "
                       f"[{base}->{cur} сэмплов] {arrow}")
        items_row = [r for r in rows if r[1] == "items"][0]
        items_after_pct = items_row[3] * 100.0
        rep.append(f"  -> ITEMS-LANE: 31.17% -> {items_after_pct:.2%} "
                   f"(36051 -> {items_row[2]} сэмплов) — {'УПАЛ' if items_row[0] < -0.01 else 'НЕ УПАЛ'}")
        if verdict is None:
            verdict = "PROFILE-MAP READY"
    else:
        rep.append("- J-T5: cpu-collapsed отсутствует")
        if verdict is None:
            verdict = "NO-PROFILE"

    if verdict is None:
        verdict = "DELIVERY-FAIL"

    # ---------- FAILURE-рулетка v2 (уроки s7206#2/s7207#1/393) ----------
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
        if verdict in ("NO-PROFILE", "DELIVERY-FAIL", "NO-TPS") or not stdout_txt:
            if unbound and not band:
                verdict = "INFRA-SCRIPT-FAIL"
                rep.append("  -> **INFRA-SCRIPT-FAIL** (root-cause fix, потом ре-диспатч)")
            elif band and crash == 0:
                verdict = "BAND-DISCARD"
                rep.append("  -> **BAND-DISCARD** (S7-96d fast-fail) — ре-диспатч (макс 2)")
            elif delivery503 and crash == 0:
                verdict = "INFRA-DELIVERY-FAIL"
                rep.append("  -> **INFRA-DELIVERY-FAIL** — ре-ролл после root-cause")
            elif crash:
                verdict = "CRASH-REFUTED"
                rep.append("  -> **CRASH-REFUTED** — руут-кауз по job-логу")

    rep.append(f"\n## VERDICT: **{verdict}**")
    outp = os.path.join(RESDIR, "ABSORB_ROUND395_J.md")
    open(outp, "w", encoding="utf-8").write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {outp}")
    print(f"\nJSON: tps_med={med:.2f} runner={runner} delta={d_norm:+.4f} "
          f"items_after_pct={items_after_pct:.2f} stw_total_s={stw_total_s:.1f} "
          f"full={full} heap_mb={heap_mb}")


if __name__ == "__main__":
    main()
