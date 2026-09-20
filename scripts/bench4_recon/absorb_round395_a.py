#!/usr/bin/env python3
"""absorb_round395_a.py — TASK-395 mega-round, agent-A lever `items_index`.

Absorb лега round-395-a-merge-index (world-bench-parallel.yml, lever_flag=
items_index, lever_arg=1) на БАНКЕ v4 (fp=4, pop 150k seed 42, ParallelGC,
band 6.0M..9.5M, radius 640, seconds 300).

II-A delivery: inputs = РОВНО dispatch_round395_a dict (бит-в-бит, урок
TASK-392) + pop VALID + NCDFE=0 + ParallelGC + band + 0 crash.
II-B crash-free + TPS-поллы >= 3.
II-C LEVER-ARMED: [crussty-plugin] items_index armed-маркеры в server-stdout
(define trio -> computed patch -> retransform rc=0; dormant = INFRA).
II-D ВЕРДИКТ: median5 @ runner_cpu_index vs TPS_exp-интерполяция якоря-банка
(2.6 @ 8551924 / 2.2 @ 6653417) — normalized дельта (GREEN >= +3%, зачёт
>= +5%, амбиция >= +15%) + absolute median5; RAM (gc.log: young/Full/total).
II-E ПРОФИЛЬ-КАРТА: лейны cpu-collapsed vs базлайн s7206#3 (35528326290,
115655 сэмплов) — items-лейн был -> стал (обязателен к отчёту).
"""
import os, re, statistics, subprocess, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/rounds/ROUND-395/agent-A"
RUN_DIR = os.path.join(RESDIR, "run-395a")
BANK_POINTS = ((2.6, 8_551_924), (2.2, 6_653_417))
BAND_MIN, BAND_MAX = 6_000_000, 9_500_000
BASELINE_TOTAL = 115_655
# items-лейн ДОЛЖЕН быть в отчёте был -> стал (лейн-мапа = базлайновая,
# НЕ трогается — урок lane_map валидации).
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
# РОВНО dispatch_round395_a.py (бит-в-бит).
WANT_INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4", "fluid_guard": "1",
    "gc_tune": "3", "inside_cache": "1", "flush_diet": "1", "fluid_dirty": "0",
    "fluid_bitmask": "0", "region_threads": "4", "batch_collector": "1",
    "inside_bitmask": "0", "skip_store_bb": "0", "region_steal": "0",
    "bu_defer": "0", "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G", "cpu_band_min": "6000000",
    "cpu_band_max": "9500000", "lever_flag": "items_index", "lever_arg": "1",
}


def token():
    with open("/tmp/gh_token") as f:
        return f.read().strip()


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


def median(xs):
    xs = sorted(xs)
    n = len(xs)
    if not n:
        return 0.0
    return xs[n // 2] if n % 2 else (xs[n // 2 - 1] + xs[n // 2]) / 2.0


def interp_tps(runner_idx):
    (t1, r1), (t2, r2) = BANK_POINTS
    if r2 == r1:
        return t1
    return t2 + (t1 - t2) * (runner_idx - r2) / (r1 - r2)


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


def env_val(env_txt, key):
    m = re.search(rf"^{re.escape(key)}: (\S+)", env_txt, re.M)
    return m.group(1) if m else None


def env_flag(env_txt, key, want):
    """String-aware: matches `key: value` prefix (numeric or id-string)."""
    v = env_val(env_txt, key)
    return v is not None and v.split(" ", 1)[0] == want


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
        raise SystemExit("usage: absorb_round395_a.py <run_id>")
    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={st.get('status')} conclusion={st.get('conclusion')} "
          f"head={str(st.get('head_sha'))[:7]} branch={st.get('head_branch')}")
    if st.get("status") != "completed":
        sys.exit(3)
    os.makedirs(RUN_DIR, exist_ok=True)
    have_art = fetch_artifact(tok, run_id)

    rep = [f"# absorb round-395-a items_index (run {run_id}, head {str(st.get('head_sha'))[:7]}) "
           "— MEGA-ROUND lever: spatial-hash item-merge index на БАНКЕ v4\n"]
    verdict = None

    stdout = os.path.join(RUN_DIR, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""
    envp = os.path.join(RUN_DIR, "run-env.txt")
    env_txt = open(envp, errors="ignore").read() if os.path.isfile(envp) else ""
    logp = os.path.join(RUN_DIR, "world3-bench.log")
    log_txt = open(logp, errors="ignore").read() if os.path.isfile(logp) else ""

    # ---------- II-A delivery (inputs бит-в-бит + pop VALID +ParallelGC) ------
    runner = "0"
    if env_txt:
        checks = [f"{k}={'OK' if env_flag(env_txt, k, v) else 'BAD'}"
                  for k, v in WANT_INPUTS.items()]
        pop_ok = "POPULATION FIXTURE-VALIDITY: VALID" in stdout_txt or \
                 "POPULATION FIXTURE-VALIDITY: VALID" in log_txt
        ncde = stdout_txt.count("NoClassDefFoundError") + log_txt.count("NoClassDefFoundError")
        col_ok = "Using Parallel" in (stdout_txt + log_txt)
        runner = (env_val(env_txt, "runner_cpu_index") or "0")
        band_ok = BAND_MIN <= int(runner) <= BAND_MAX
        t1 = all("BAD" not in x for x in checks) and pop_ok and ncde == 0 and col_ok
        rep.append("- II-A: inputs=" + ("OK" if t1 else "MISMATCH") +
                   " [" + " ".join(checks) + "]" +
                   ", pop=" + ("VALID" if pop_ok else "INVALID") +
                   f", NCDFE={ncde}, col=" + ("PARALLEL" if col_ok else "BAD") +
                   f", runner={runner} (band {'OK' if band_ok else 'OUT'})" +
                   f" -> **{'PASS' if t1 else 'FAIL'}**")
        if not t1:
            verdict = "DELIVERY-FAIL"

    # ---------- II-B crash-free --------------------------------------------
    if stdout_txt:
        threw = len(re.findall(r"Encountered an unexpected exception|ReportedException",
                               stdout_txt))
        polls = len(re.findall(r"TPS from last 5s", log_txt))
        t2 = threw == 0 and polls >= 3
        rep.append(f"- II-B: threw={threw}, TPS-поллов={polls} -> **{'PASS' if t2 else 'FAIL'}**")
        if not t2 and verdict is None:
            verdict = "CRASH-REFUTED"

    # ---------- II-C lever armed -------------------------------------------
    if stdout_txt:
        marker = "[crussty-plugin] items_index:"
        lines = [ln for ln in stdout_txt.splitlines() if marker in ln]
        armed = any("armed, retransform rc=0" in ln for ln in lines) and \
            any("computed patch" in ln for ln in lines) and \
            any("defined" in ln and "in kernel loader" in ln for ln in lines)
        dormant = any("dormant" in ln or "stays dormant" in ln for ln in lines)
        patch = next((ln.split("computed patch ")[-1] for ln in lines
                      if "computed patch" in ln), "-")
        rep.append(f"- II-C: lever lines={len(lines)}, armed=" +
                   ("YES" if armed else "NO") + (", DORMANT-MARKERS" if dormant and not armed else "") +
                   f", patch=({patch})")
        for ln in lines:
            rep.append(f"    > {ln.strip()}")
        if not armed and verdict is None:
            verdict = "LEVER-DORMANT"

    # ---------- II-D TPS вердикт -------------------------------------------
    d_norm = 0.0
    med = 0.0
    tps_exp = 0.0
    if log_txt:
        ser = tps_series(log_txt)
        if ser:
            med = median(ser)
            r_idx = int(runner) if runner.isdigit() else 0
            tps_exp = interp_tps(r_idx)
            d_norm = (med / tps_exp - 1.0) if tps_exp else 0.0
            gcs = gc_stats_parallel(os.path.join(RUN_DIR, "gc.log"))
            gc_txt = ""
            if gcs:
                pauses, full, total, avg, mx = gcs
                gc_txt = (f"; GC young={pauses}, Full={full}, STW total={total/1000:.1f}s, "
                          f"avg={avg:.0f}ms, max={mx:.0f}ms")
            rep.append(f"- II-D: median5={med:.2f} @ {runner}; банк v4 якорь-интерполяция "
                       f"TPS_exp={tps_exp:.2f}; delta_norm={d_norm:+.1%} "
                       f"(abs {med:.2f} vs {tps_exp:.2f}){gc_txt}")
            rep.append(f"  поллы: n={len(ser)}, min={min(ser):.2f}, max={max(ser):.2f}")
            if verdict is None:
                verdict = "GREEN" if d_norm >= 0.03 else "RED"
        else:
            rep.append("- II-D: TPS-поллов нет в артефакте")
            if verdict is None:
                verdict = "NO-TPS"

    # ---------- II-E профиль-карта (items был -> стал) ----------------------
    cpu = os.path.join(RUN_DIR, "cpu-collapsed.txt")
    if have_art and os.path.isfile(cpu):
        tot, lanes = lane_map(cpu)
        rep.append(f"- II-E ПРОФИЛЬ-КАРТА: total={tot} сэмплов (базлайн {BASELINE_TOTAL})")
        rows = []
        for k, (base, _) in BASELINE_LANES.items():
            cur = lanes[k]
            cur_p = cur / tot if tot else 0.0
            base_p = base / BASELINE_TOTAL
            rows.append((cur_p - base_p, k, cur, cur_p, base, base_p))
        for d, k, cur, cur_p, base, base_p in sorted(rows, reverse=True):
            arrow = "РОСТ" if d > 0.01 else ("спад" if d < -0.01 else "флэт")
            rep.append(f"  - {k}: базлайн {base_p:.2%} -> ран {cur_p:.2%} ({d:+.2%}) "
                       f"[{base}->{cur} сэмплов] {arrow}")
        items = next(r for r in rows if r[1] == "items")
        rep.append(f"  -> ITEMS-ЛЕЙН (обязателен): 31.17% -> {items[3]:.2%} "
                   f"({items[0]:+.2%}) [{items[4]}->{items[2]} сэмплов]")
    else:
        rep.append("- II-E: cpu-collapsed отсутствует")

    if verdict is None:
        verdict = "DELIVERY-FAIL"

    # ---------- FAILURE-рулетка (нет артефакта/stdout — диагностика job.log) --
    if not have_art or not stdout_txt:
        jl = fetch_joblog(tok, run_id)
        crash = sum(jl.count(x) for x in
                    ("Encountered an unexpected exception", "ReportedException",
                     "NullPointerException"))
        band = bool(re.search(r"::notice::runner_cpu_index=\d+ OUTSIDE band", jl))
        unbound = bool(re.search(r"unbound variable", jl))
        delivery503 = bool(re.search(r"503|Failed to download|curl: \(\d+\)", jl))
        build = bool(re.search(r"error\[|error:|failed to select|linker", jl))
        rep.append(f"- FAILURE-рулетка: crash={crash}, band={band}, unbound={unbound}, "
                   f"delivery-503={delivery503}, build-err={build}")
        if unbound and not band:
            verdict = "INFRA-SCRIPT-FAIL"
        elif build and crash == 0:
            verdict = "INFRA-BUILD-FAIL"
        elif band and crash == 0:
            verdict = "BAND-DISCARD"
        elif delivery503 and crash == 0:
            verdict = "INFRA-DELIVERY-FAIL"
        elif crash:
            verdict = "CRASH-REFUTED"

    rep.append(f"\n## VERDICT: **{verdict}**")
    outp = os.path.join(RESDIR, "ABSORB_ROUND395_A.md")
    open(outp, "w", encoding="utf-8").write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {outp}")


if __name__ == "__main__":
    main()
