"""absorb_e2.py — TASK-397-E2 absorb (copy of bench4_recon/absorb_round.py):
WANT1 carries lever_flag=items_sweep2/lever_arg=1 + ARM-proof grep (loud markers).
Usage: python3 absorb_round.py <run_id> <tag>
Харнесс/якоря/TPS_exp/lane_map — идентичны absorb_s7207.py (2 точки банка v4 fp=4).
Want-словарь = диспатч world-bench-parallel.yml (банк v4 + lever_flag НЕпустой).
"""
import os, re, sys, json, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
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
# world-bench-parallel.yml диспатч-хребет (общий для всех раунд-легов)
WANT1 = {"inside_cache": "1", "flush_diet": "1", "region_threads": "4",
         "batch_collector": "1", "fluid_guard": "1", "gc_tune": "3",
         "population_target": "150000", "fake_players": "4",
         "lever_flag": "items_sweep2", "lever_arg": "1"}
WANT0 = {"fluid_bitmask": "0", "fluid_dirty": "0", "inside_bitmask": "0",
         "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0"}


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL")
    return m.group(1)


import subprocess  # noqa: E402


def api(tok, url):
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
    for attempt in range(5):
        try:
            with urllib.request.urlopen(urllib.request.Request(loc), timeout=600) as r, \
                    open(dest, "wb") as f:
                while True:
                    b = r.read(1 << 20)
                    if not b:
                        break
                    f.write(b)
            return
        except Exception as e:  # noqa: BLE001
            wait = min(80, 10 * (2 ** attempt))
            print(f"fetch retry {attempt + 1}: {e} — sleep {wait}s", file=sys.stderr)
            import time
            time.sleep(wait)
    raise SystemExit("artifact download failed after 5 retries")


def fetch_artifact(tok, run_id, run_dir):
    import zipfile
    arts = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/artifacts").get("artifacts", [])
    bench = [a for a in arts if a["name"] == "world3-bench"]
    if not bench:
        print(f"no world3-bench artifact (available: {[a['name'] for a in arts]})")
        return False
    dest = os.path.join(run_dir, "world3-bench.zip")
    _fetch_redirected(tok, f"{API}/repos/{REPO}/actions/artifacts/{bench[0]['id']}/zip", dest)
    print(f"downloaded {dest} ({os.path.getsize(dest)} bytes)")
    with zipfile.ZipFile(dest) as z:
        z.extractall(run_dir)
    return True


def fetch_joblog(tok, run_id, run_dir):
    jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs").get("jobs", [])
    if not jobs:
        return ""
    dest = os.path.join(run_dir, "job.log")
    try:
        _fetch_redirected(tok, f"{API}/repos/{REPO}/actions/jobs/{jobs[0]['id']}/logs", dest)
        return open(dest, errors="ignore").read()
    except Exception as e:  # noqa: BLE001
        print(f"job-log fetch failed: {e}", file=sys.stderr)
        return ""


def tps_series(txt):
    out = []
    for line in txt.splitlines():
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


def main():
    tok = token()
    run_id = int(sys.argv[1])
    tag = sys.argv[2] if len(sys.argv) > 2 else f"run{run_id}"
    run_dir = os.path.join(RESDIR, f"round-{tag}")
    os.makedirs(run_dir, exist_ok=True)

    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    branch = st.get("head_branch", "?")
    print(f"run {run_id}: branch={branch} status={st.get('status')} "
          f"conclusion={st.get('conclusion')} head={str(st.get('head_sha'))[:7]}")
    if st.get("status") != "completed":
        sys.exit(3)
    # cache: переиспользуем уже скачанный артефакт
    if os.path.isfile(os.path.join(run_dir, "server-stdout.log")) and \
            os.path.isfile(os.path.join(run_dir, "cpu-collapsed.txt")):
        have_art = True
        print("artifact cached in", run_dir)
    else:
        have_art = fetch_artifact(tok, run_id, run_dir)

    rep = [f"# absorb ROUND ({tag}, run {run_id}, branch {branch}, head {str(st.get('head_sha'))[:7]})\n"]
    verdict = None

    stdout = os.path.join(run_dir, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""
    envp = os.path.join(run_dir, "run-env.txt")
    env_txt = open(envp, errors="ignore").read() if os.path.isfile(envp) else ""
    # TPS-поллы живут в server-stdout.log (артефакт не содержит world3-bench.log)
    log_txt = stdout_txt
    gclp = os.path.join(run_dir, "gc.log")
    gcl_txt = open(gclp, errors="ignore").read() if os.path.isfile(gclp) else ""

    # ---------- T1 delivery (банк v4 fp=4 + lever ARMED) ----------
    if env_txt:
        t1a = [f"{k}={'OK' if env_val(env_txt, k) == v else 'BAD(' + str(env_val(env_txt, k)) + ')'}"
               for k, v in WANT1.items()]
        t1b = [f"{k}={'OK' if env_val(env_txt, k) == v else 'BAD(' + str(env_val(env_txt, k)) + ')'}"
               for k, v in WANT0.items()]
        lever = env_val(env_txt, "lever_flag") or env_val(env_txt, "LEVER_FLAG")
        pop_ok = "POPULATION FIXTURE-VALIDITY: VALID" in stdout_txt or \
                 "POPULATION FIXTURE-VALIDITY: VALID" in log_txt
        ncde = stdout_txt.count("NoClassDefFoundError") + log_txt.count("NoClassDefFoundError")
        col_ok = "Using Parallel" in (stdout_txt + gcl_txt)
        runner = env_val(env_txt, "runner_cpu_index") or "0"
        band_ok = BAND_MIN <= int(runner) <= BAND_MAX
        t1 = all("BAD" not in x for x in t1a + t1b) and pop_ok and ncde == 0 and col_ok
        rep.append(f"- T1: lever={lever or '(n/a в run-env)'}, pop=" +
                   ("VALID" if pop_ok else "INVALID") +
                   f", NCDFE={ncde}, col=" + ("PARALLEL" if col_ok else "BAD") +
                   f", runner={runner} (band {'OK' if band_ok else 'OUT'}) | " +
                   " ".join(t1a + t1b) + f" -> **{'PASS' if t1 else 'FAIL'}**")
        if not t1:
            verdict = "DELIVERY-FAIL"

    # ---------- T1b ARM proof (TASK-397-E2 loud markers) ----------
    m_arm = re.search(r"items_sweep2: ARMED (\S+) (\d+) -> (\d+) bytes", stdout_txt)
    m_fail = re.findall(r"items_sweep2: PATCH-FAIL (.*)$", stdout_txt, re.M)
    m_sight = re.search(r"items_sweep2: pristine sighting (\S+) (\d+) bytes \(major (\d+)\)", stdout_txt)
    m_sweep = re.search(r"items_sweep2: hook serve (\S+) (\d+) bytes", stdout_txt)
    armed = bool(m_arm) and bool(m_sweep)
    rep.append(f"- T1b ARM: sighting={m_sight.groups() if m_sight else None}, "
               f"serve={m_sweep.groups() if m_sweep else None}, "
               f"ARMED={m_arm.groups() if m_arm else None}, "
               f"PATCH-FAIL={m_fail if m_fail else 'none'} -> **{'ARMED' if armed else 'NOT-ARMED'}**")
    if not armed and verdict is None:
        verdict = "NOT-ARMED"

    # ---------- T2 crash-free ----------
    if stdout_txt:
        threw = len(re.findall(r"Encountered an unexpected exception|ReportedException", stdout_txt))
        polls = len(re.findall(r"TPS from last 5s", log_txt))
        t2 = threw == 0 and polls >= 3
        rep.append(f"- T2: threw={threw}, TPS-поллов={polls} -> **{'PASS' if t2 else 'FAIL'}**")
        if not t2 and verdict is None:
            verdict = "CRASH-REFUTED"

    # ---------- T3 dual-bar legs ----------
    med = tps_exp = d_norm = 0.0
    runner_i = 0
    if stdout_txt:
        ser = tps_series(stdout_txt)
        if ser:
            med = statistics_median(ser)
            runner = env_val(env_txt, "runner_cpu_index") or "0"
            runner_i = int(runner) if runner.isdigit() else 0
            tps_exp = interp_tps(runner_i)
            d_norm = (med / tps_exp - 1.0) if tps_exp else 0.0
            rep.append(f"- T3: median={med:.2f} @ {runner_i} (поллов={len(ser)}); "
                       f"TPS_exp={tps_exp:.2f}; normalized={d_norm:+.1%}")
    gcs = gc_stats_parallel(os.path.join(run_dir, "gc.log"))
    if gcs:
        pauses, full, total, avg, mx = gcs
        rep.append(f"- GC: young={pauses}, Full={full}, total={total / 1000:.1f}s, "
                   f"avg={avg:.0f}ms, max={mx:.0f}ms (банк-справка 18.8s/162ms/2400ms/Full=7)")

    # ---------- T5 lane map ----------
    items_pct = None
    cpu = os.path.join(run_dir, "cpu-collapsed.txt")
    if have_art and os.path.isfile(cpu):
        tot, lanes = lane_map(cpu)
        rep.append(f"- T5 ЛЕЙНЫ: total={tot} сэмплов (базлайн {BASELINE_TOTAL})")
        for k, (base, _) in BASELINE_LANES.items():
            cur = lanes[k]
            cur_p = cur / tot if tot else 0.0
            base_p = base / BASELINE_TOTAL
            arrow = "РОСТ" if cur_p - base_p > 0.01 else ("спад" if cur_p - base_p < -0.01 else "флэт")
            rep.append(f"  - {k}: {base_p:.2%} -> {cur_p:.2%} ({cur_p - base_p:+.2%}) {arrow}")
        items_pct = lanes["items"] / tot * 100 if tot else 0.0
    else:
        rep.append("- T5: cpu-collapsed отсутствует")

    if verdict is None:
        if med and tps_exp:
            verdict = ("GREEN-CANDIDATE" if d_norm >= 0.05 else
                       "PARITY/LOW" if d_norm >= -0.01 else "RED")
        else:
            verdict = "NO-TPS"

    if not have_art or not stdout_txt:
        jl = fetch_joblog(tok, run_id, run_dir)
        crash = sum(jl.count(x) for x in ("Encountered an unexpected exception",
                                          "ReportedException", "NullPointerException"))
        band = bool(re.search(r"::notice::runner_cpu_index=\d+ OUTSIDE band", jl))
        unbound = bool(re.search(r"unbound variable", jl))
        d503 = bool(re.search(r"503|Failed to download|curl: \(\d+\)", jl))
        rep.append(f"- FAILURE-рулетка: crash={crash}, band={band}, unbound={unbound}, delivery-503={d503}")
        if unbound and not band:
            verdict = "INFRA-SCRIPT-FAIL"
        elif band and crash == 0:
            verdict = "BAND-DISCARD"
        elif d503 and crash == 0:
            verdict = "INFRA-DELIVERY-FAIL"
        elif crash:
            verdict = "CRASH-REFUTED"

    rep.append(f"\n## VERDICT: **{verdict}**")
    print("\n".join(rep))
    outp = os.path.join(run_dir, "ABSORB.md")
    open(outp, "w", encoding="utf-8").write("\n".join(rep) + "\n")
    print(f"\nwritten: {outp}")
    print(json.dumps({"tag": tag, "run_id": run_id, "branch": branch,
                      "verdict": verdict, "tps_med": round(med, 2),
                      "tps_exp": round(tps_exp, 2), "delta_norm_pct": round(d_norm * 100, 1),
                      "items_pct": round(items_pct, 2) if items_pct else None,
                      "armed": armed, "armed_marker": (m_arm.group(0) if m_arm else None),
                      "gc_total_s": round(gcs[2] / 1000, 1) if gcs else None,
                      "full": gcs[1] if gcs else None}, ensure_ascii=False))


if __name__ == "__main__":
    main()
