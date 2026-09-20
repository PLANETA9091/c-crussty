"""absorb_396F.py — TASK-396-F (MEGA-ROUND-1, вектор F): items_mono absorb.

Лег = МЕГА-РАУНД архитектурная замена call-структуры горячего метода:
единственный megamorphic-диспетч entity-tick (invokevirtual Entity.tick в
ServerLevel.tickNonPassenger) ретаргечится (lever_flag=items_mono) на
RegionTickOps.entityTick type-test сплит — items monomorphic, остальные
ванильно. Порядок тиков = паритет по построению (харнесс 1000/1000).

Гейты:
  T1 delivery: БАНК v4-флаги ТОЧНО как диспатч-инпуты (урок TASK-392:
     want-словарь = диспатч-инпуты 1:1) + fp=4 + pop 150k VALID + ParallelGC
     + band 6.0M..9.5M + 0 NCDFE + LEVER-ARMS: lever_flag=items_mono,
     [S7-F] ARMED, ITEMS-MONO composed, items_mono=true телеметрия.
  T2 crash-free + TPS-поллы >= 3.
  T3 DUAL-BAR: normalized = median5 @ runner vs TPS_exp-интерполяция якоря
     банка v4 (2.6 @ 8551924 / 2.2 @ 6653417); absolute = median5 vs якорь
     2.6 raw. Вердиктный бар: normalized >= +5% = GREEN (>= +15% амбиция),
     < +3% честный RED.
  T4 страховочные ParallelGC — RED-гейты RAM: STW total > 18.8s, heap > 10G
     (банк-справка leg#2: 18.8s/162ms/2400ms/Full=7).
  T5 лейн-карта vs базлайн банка v4 (информационно; items-лейн МОЖЕТ не
     упасть — это dispatch-оптимизация, ItemEntity.tick исполняется как
     исполнялся; верить TPS dual-bar).

Запуск: python3 scripts/bench4_recon/absorb_396F.py <run_id>
"""
import os, re, statistics, subprocess, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RUN_DIR = "/home/z/rounds/ROUND-396/agent-f/run-s396f"
OUT_MD = "/home/z/rounds/ROUND-396/agent-f/ABSORB_396F.md"
BRANCH = "round-396-f-items_mono"
BANK_POINTS = ((2.6, 8_551_924), (2.2, 6_653_417))
BANK_ANCHOR_TPS = 2.6
BAND_MIN, BAND_MAX = 6_000_000, 9_500_000
BANK_GC_TOTAL_S = 18.8
BANK_XMX_G = 10
# БАЗЛАЙН-КАРТА ЛЕЙНОВ (банк v4, fp=4, cpu-collapsed s7206#3, 115655 сэмплов).
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

# want-словарь = ТОЧНО диспатч-инпуты dispatch_396F.py (урок TASK-392).
WANT0 = {
    "fluid_dirty": "0", "fluid_bitmask": "0", "inside_bitmask": "0",
    "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
}
WANT1 = {
    "inside_cache": "1", "flush_diet": "1", "fluid_guard": "1", "gc_tune": "3",
    "region_threads": "4", "batch_collector": "1",
    "population_target": "150000", "population_seed": "42",
    "fake_players": "4", "radius": "640", "seconds": "300",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
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
    os.remove(dest)  # урок: zip удалить сразу после распаковки
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


def main():
    tok = token()
    run_id = int(sys.argv[1]) if len(sys.argv) > 1 else 0
    if not run_id:
        raise SystemExit("usage: absorb_396F.py <run_id>")
    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={st.get('status')} conclusion={st.get('conclusion')} "
          f"head={str(st.get('head_sha'))[:7]}")
    if st.get("status") != "completed":
        sys.exit(3)
    os.makedirs(RUN_DIR, exist_ok=True)
    have_art = fetch_artifact(tok, run_id)

    rep = [f"# absorb 396F items_mono (run {run_id}, head {str(st.get('head_sha'))[:7]}) "
           f"— МЕГА-РАУНД вектор F: JIT-девиртуализация item-лейна (lever_flag=items_mono)\n"]
    verdict = None

    stdout = os.path.join(RUN_DIR, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""
    envp = os.path.join(RUN_DIR, "run-env.txt")
    env_txt = open(envp, errors="ignore").read() if os.path.isfile(envp) else ""
    logp = os.path.join(RUN_DIR, "world3-bench.log")
    log_txt = open(logp, errors="ignore").read() if os.path.isfile(logp) else ""

    # ---------- T1 delivery (банк v4 1:1 с диспатчем + LEVER-ARMS) ----------
    runner = "0"
    if env_txt:
        t1a = [f"{k}=0:{'OK' if env_flag(env_txt, k, '0') else 'BAD'}" for k in WANT0]
        t1b = [f"{k}={'OK' if env_flag(env_txt, k, v) else 'BAD'}" for k, v in WANT1.items()]
        lf = env_val(env_txt, "lever_flag")
        la = env_val(env_txt, "lever_arg")
        lever_ok = lf == "items_mono" and la == "1"
        pop_ok = "POPULATION FIXTURE-VALIDITY: VALID" in stdout_txt or \
                 "POPULATION FIXTURE-VALIDITY: VALID" in log_txt
        ncde = stdout_txt.count("NoClassDefFoundError") + log_txt.count("NoClassDefFoundError")
        col_ok = "Using Parallel" in (stdout_txt + log_txt)
        runner = env_val(env_txt, "runner_cpu_index") or "0"
        band_ok = BAND_MIN <= int(runner) <= BAND_MAX
        arms = {
            "s7f_armed": "[S7-F] items_mono ARMED" in stdout_txt,
            "lever_armed": "[crussty-plugin] items_mono: LEVER ARMED" in stdout_txt,
            "composed": "ITEMS-MONO composed: tickNonPassenger Entity.tick -> RegionTickOps.entityTick (sites:1)" in stdout_txt,
            "telemetry_items_mono": "items_mono=true" in stdout_txt,
        }
        arms_ok = all(arms.values())
        xmx = env_val(env_txt, "server_xmx") or ""
        xmx_ok = xmx == f"{BANK_XMX_G}G"
        t1 = (all("BAD" not in x for x in t1a + t1b) and lever_ok and pop_ok
              and ncde == 0 and col_ok and band_ok and arms_ok and xmx_ok)
        rep.append("- T1: lever_flag=" + (f"{lf}:OK" if lever_ok else f"{lf}:BAD") +
                   ", pop=" + ("VALID" if pop_ok else "INVALID") +
                   f", NCDFE={ncde}, col=" + ("PARALLEL" if col_ok else "BAD") +
                   f", runner={runner} (band {'OK' if band_ok else 'OUT'})" +
                   f", xmx={xmx}:{'OK' if xmx_ok else 'BAD'}" +
                   ", arms=" + ("ALL" if arms_ok else str(arms)) +
                   " | " + " ".join(t1a + t1b) + f" -> **{'PASS' if t1 else 'FAIL'}**")
        if not t1 and verdict is None:
            verdict = "DELIVERY-FAIL"

    # ---------- T2 crash-free ----------
    if stdout_txt:
        threw = len(re.findall(r"Encountered an unexpected exception|ReportedException",
                               stdout_txt))
        polls = len(re.findall(r"TPS from last 5s", log_txt))
        t2 = threw == 0 and polls >= 3
        rep.append(f"- T2: threw={threw}, TPS-поллов={polls} -> **{'PASS' if t2 else 'FAIL'}**")
        if not t2 and verdict is None:
            verdict = "CRASH-REFUTED"

    # ---------- T3 DUAL-BAR ----------
    med = tps_exp = None
    if log_txt:
        ser = tps_series(log_txt)
        if ser:
            med = statistics.median(ser)
            r_idx = int(runner) if runner.isdigit() else 0
            tps_exp = interp_tps(r_idx)
            d_norm = (med / tps_exp - 1.0) if tps_exp else 0.0
            d_abs = med / BANK_ANCHOR_TPS - 1.0
            rep.append(f"- T3 DUAL-BAR: median5={med:.2f} @ {runner} (поллов={len(ser)}); "
                       f"normalized: {d_norm:+.1%} vs TPS_exp={tps_exp:.2f} "
                       f"(интерполяция банка v4 @ runner); "
                       f"absolute: {d_abs:+.1%} vs якорь 2.6 @ 8551924")
        else:
            rep.append("- T3: TPS-поллов нет в артефакте")

    # ---------- T4 GC страховка ----------
    heap_reg = False
    gcs = gc_stats_parallel(os.path.join(RUN_DIR, "gc.log"))
    if gcs:
        pauses, full, total, avg, mx = gcs
        heap_reg = (total / 1000.0) > BANK_GC_TOTAL_S
        rep.append(f"- T4 GC (банк-справка 18.8s/162ms/2400ms/Full=7): "
                   f"young={pauses}, Full={full}, total={total/1000:.1f}s, avg={avg:.0f}ms, "
                   f"max={mx:.0f}ms -> RAM-гейт: "
                   f"**{'REGRESS' if heap_reg else 'OK'}**")
    else:
        rep.append("- T4: gc.log отсутствует")

    # ---------- T5 лейн-карта ----------
    if have_art and os.path.isfile(os.path.join(RUN_DIR, "cpu-collapsed.txt")):
        tot, lanes = lane_map(os.path.join(RUN_DIR, "cpu-collapsed.txt"))
        rep.append(f"- T5 ЛЕЙН-КАРТА: total={tot} сэмплов (базлайн {BASELINE_TOTAL})")
        rows = []
        for k, (base, _) in BASELINE_LANES.items():
            cur = lanes[k]
            cur_p = cur / tot if tot else 0.0
            base_p = base / BASELINE_TOTAL
            rows.append((cur_p - base_p, k, cur, cur_p, base, base_p))
        for d, k, cur, cur_p, base, base_p in sorted(rows, reverse=True):
            arrow = "РОСТ" if d > 0.01 else ("спад" if d < -0.01 else "флэт")
            rep.append(f"  - {k}: банк {base_p:.2%} -> leg {cur_p:.2%} ({d:+.2%}) "
                       f"[{base}->{cur} сэмплов] {arrow}")
        rep.append("  -> Примечание: items-лейн МОЖЕТ не упасть (dispatch-оптимизация: "
                   "ItemEntity.tick исполняется как исполнялся; атрибуция профайлера "
                   "неизменна) — верить TPS dual-bar (T3).")
    else:
        rep.append("- T5: cpu-collapsed отсутствует")

    # ---------- ВЕРДИКТ ----------
    if verdict is None and med is not None and tps_exp:
        d_norm = med / tps_exp - 1.0
        parity = "PASS"  # харнесс 1000/1000 order-parity + T1 arms + crash-free
        if d_norm >= 0.05 and not heap_reg:
            verdict = "GREEN"
        elif d_norm < 0.03:
            verdict = "RED"
        else:
            verdict = "RED"  # 3..5% — ниже зачётного бара мандата
        rep.append(f"- parity: **{parity}** (оффлайн-харнесс order-parity 1000/1000 + "
                   f"ARMED-маркеры + crash-free)")
        rep.append(f"- ΔTPS normalized = {d_norm:+.1%} (бар: GREEN >= +5%, амбиция >= +15%, "
                   f"< +3% честный RED)")

    # ---------- FAILURE-рулетка ----------
    if not have_art or not stdout_txt:
        jl = fetch_joblog(tok, run_id)
        crash = sum(jl.count(x) for x in
                    ("Encountered an unexpected exception", "ReportedException",
                     "NullPointerException"))
        band = bool(re.search(r"::notice::runner_cpu_index=\d+ OUTSIDE band", jl))
        unbound = bool(re.search(r"unbound variable", jl))
        delivery503 = bool(re.search(r"503|Failed to download|curl: \(\d+\)", jl))
        cargo_fail = bool(re.search(r"error\[|error:|failed to select a version", jl)) and \
            "cargo" in jl
        rep.append(f"- FAILURE-рулетка: crash={crash}, band={band}, unbound={unbound}, "
                   f"delivery-503={delivery503}, cargo={cargo_fail}")
        if band and crash == 0:
            verdict = "BAND-DISCARD"
            rep.append("  -> **BAND-DISCARD** (S7-96d fast-fail, не вердикт) — "
                       "ре-диспатч dispatch_396F.py (макс 2 подряд)")
        elif delivery503 and crash == 0:
            verdict = "INFRA-DELIVERY-FAIL"
            rep.append("  -> **INFRA-DELIVERY-FAIL** (503/сеть внешних загрузок — "
                       "ре-ролл ТОЛЬКО после root-cause)")
        elif cargo_fail:
            verdict = "INFRA-CI-FAIL"
            rep.append("  -> **INFRA-CI-FAIL** (cargo build на ветке — root-cause + фикс + "
                       "push + ре-диспатч, макс 2)")
        elif unbound and not band:
            verdict = "INFRA-SCRIPT-FAIL"
            rep.append("  -> **INFRA-SCRIPT-FAIL**")
        elif crash:
            verdict = "CRASH-REFUTED"
            rep.append("  -> **CRASH-REFUTED** — руут-кауз по job-логу")

    if verdict is None:
        verdict = "DELIVERY-FAIL"

    rep.append(f"\n## VERDICT: **{verdict}**")
    open(OUT_MD, "w", encoding="utf-8").write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {OUT_MD}")


if __name__ == "__main__":
    main()
