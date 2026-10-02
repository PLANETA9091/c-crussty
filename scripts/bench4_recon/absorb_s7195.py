#!/usr/bin/env python3
"""absorb_s7195.py - lever #15 INSIDE-BITMASK ACTIVATION leg (s7195) gates.

Одна команда: `python3 scripts/bench4_recon/absorb_s7195.py <run_id> [--protocol B]`
exit 3 = ещё летит (офлайн-ветка тика).

PREREGISTER GATES:
  PG-T1 delivery: банк v3 флаги + inside_bitmask=1 в run-env.txt; arm markers:
    "bridge owner armed" ПРИСУТСТВУЕТ, dormant-маркер ОТСУТСТВУЕТ (иначе env
    не дошёл до ядра = армирование не состоялось), "stage inside_bitmask
    composed" ПРИСУТСТВУЕТ, strict-violated/patch-rejected/missed-window
    ОТСУТСТВУЮТ (fail-dominant chain обязан реально скомпоновать гейт);
    NCDFE=0, POPULATION VALID
  PG-T2 crash-free: 0 threw/unexpected/s7180-class; RECON-22 navigatingMobs
    race = стохастический класс (фиксируется, НЕ новый баг #15)
  PG-T3 verdict protocol:
    v8 (default, действующий закон): DUAL BAR — normalized AND absolute
        both >= +10% vs ANCHOR-SLOW (s7184: median5 1.60 @ 6680195)
    B (owner option B): min-MSPT — normalized >= +10% AND median5 >= anchor
        (не-регресс TPS >= якорь-0); GREEN -> min-of-2 подтверждающий лег
        (dispatch_s7195.py --sanctioned повторно) -> banking v4 = v3+inside_bitmask
  PG-T4 GC sanity: young <= 174, 0 Full
  PG-T5 DONE-park N/A tolerated (AP-PID defect s7178)
Failure handling: BAND-DISCARD (не вердикт, ре-диспатч макс 2 подряд) /
CRASH-REFUTED (rollback inside_bitmask: env-дефолт 0 + вердикт-док; S7-170
приоритизируется) / INFRA-FLAKE (ре-диспатч).
"""
import os, re, statistics, subprocess, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUN_DIR = os.path.join(RESDIR, "run-s7195-insidebitmask-v1")
ANCHOR_TPS, ANCHOR_RUNNER = 1.60, 6680195
BAND_MIN, BAND_MAX = 6_000_000, 9_500_000
YOUNG_GATE = 174


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
    young = full = 0
    if not os.path.isfile(path):
        return None, None
    for line in open(path, errors="ignore"):
        if "[gc,start" in line:
            continue
        m = re.search(r"Pause (Young|Full).*?([\d.]+)(ms|s)\s*$", line)
        if m:
            if m.group(1) == "Young":
                young += 1
            else:
                full += 1
    return young, full


def env_flag(env_txt, key, want):
    m = re.search(rf"^{key}: (\d+)", env_txt, re.M)
    return bool(m) and m.group(1) == want


def main():
    tok = token()
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    protocol = "v8"
    if "--protocol" in sys.argv:
        idx = sys.argv.index("--protocol")
        if idx + 1 < len(sys.argv):
            protocol = sys.argv[idx + 1]
    if protocol not in ("v8", "B"):
        raise SystemExit("--protocol должен быть v8 или B")
    run_id = int(args[0]) if args else 0
    if not run_id:
        raise SystemExit("usage: absorb_s7195.py <run_id> [--protocol v8|B]")
    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={st.get('status')} conclusion={st.get('conclusion')} "
          f"head={str(st.get('head_sha'))[:7]}")
    if st.get("status") != "completed":
        sys.exit(3)
    os.makedirs(RUN_DIR, exist_ok=True)
    have_art = fetch_artifact(tok, run_id)

    rep = [f"# absorb #15 INSIDE-BITMASK s7195 (run {run_id}, head "
           f"{str(st.get('head_sha'))[:7]}) — protocol {protocol}"
           + (" (option B min-MSPT)" if protocol == "B" else " (v8-REGRESSION DUAL BAR)")
           + "\n"]
    verdict = None

    stdout = os.path.join(RUN_DIR, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""
    env_txt = ""
    ep = os.path.join(RUN_DIR, "run-env.txt")
    if os.path.isfile(ep):
        env_txt = open(ep, errors="ignore").read()

    if stdout_txt:
        # ---- PG-T1 delivery (bank v3 + inside_bitmask isolation + arm markers)
        want = {"inside_bitmask": "1", "travel_diet": "0", "skip_store_bb": "0",
                "region_steal": "0", "bu_defer": "0", "inside_cache": "1",
                "flush_diet": "1", "region_threads": "4", "batch_collector": "1"}
        t1 = [f"{k}={'OK' if env_flag(env_txt, k, v) else 'BAD'}" for k, v in want.items()]
        ncde = stdout_txt.count("NoClassDefFoundError")
        pop_ok = "POPULATION FIXTURE-VALIDITY: VALID" in stdout_txt
        armed_owner = "[crussty-plugin] inside_bitmask: bridge owner armed" in stdout_txt
        dormant = "[crussty-plugin] inside_bitmask: dormant" in stdout_txt
        arm = "stage inside_bitmask composed" in stdout_txt
        strict = "inside_bitmask strict check violated" in stdout_txt
        rejected = "inside_bitmask patch rejected" in stdout_txt
        missed = "inside_bitmask bridge missed its window" in stdout_txt
        s7170 = stdout_txt.count("[S7-170] nav-mobs guarded")
        s7170_note = ("guarded-marker=" + ("OK" if s7170 > 0 else "MISSING")) \
            if "nav-mobs" in stdout_txt or "[S7-170]" in stdout_txt else "guarded-marker=N/A (pre-S7-170 head)"
        t1_ok = all("OK" in x for x in t1) and ncde == 0 and pop_ok and arm \
            and armed_owner and not dormant and not strict and not rejected and not missed
        rep.append("- PG-T1: " + ", ".join(t1) + f", NCDFE={ncde}, "
                   f"pop={'VALID' if pop_ok else 'BAD'}, "
                   f"owner={'ARMED' if armed_owner else 'MISSING'}, "
                   f"dormant={'BAD-ENV!' if dormant else 'absent-OK'}, "
                   f"arm={'COMPOSED' if arm else 'MISSING'}, "
                   f"strict/rejected/missed={int(strict) + int(rejected) + int(missed)}, "
                   f"{s7170_note} -> **{'PASS' if t1_ok else 'FAIL'}**")

        # ---- PG-T2 crash-free (s7180 incident class + RECON-22 race catches)
        threw = stdout_txt.count("Entity threw exception")
        unexpected = stdout_txt.count("Encountered an unexpected exception")
        bu_npe = stdout_txt.count("BlockUpdateOps") + \
            stdout_txt.count("ObjectOpenHashSet$SetIterator")
        tps = tps_series(stdout)
        soak_ok = len(tps) >= 3
        t2_ok = threw == 0 and unexpected == 0 and bu_npe == 0 and soak_ok
        rep.append(f"- PG-T2: threw={threw}, unexpected={unexpected}, s7180-class={bu_npe}, "
                   f"TPS-поллов={len(tps)} -> **{'PASS' if t2_ok else 'FAIL'}**")

        # ---- PG-T3 verdict protocol (wide band)
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
            rep.append(f"  normalized={nd:+.1%}, absolute={ad:+.1%} "
                       f"(якорь 1.60 @ {ANCHOR_RUNNER})")
            if protocol == "B":
                # option B min-MSPT: скорость-ось >= +10% И TPS не-регресс (>= якорь-0)
                rep.append("  B-BAR (min-MSPT): normalized >= +10% И median5 >= 1.60 "
                           "(TPS не-регресс >= якорь-0)")
                if nd >= 0.10 and med >= ANCHOR_TPS:
                    verdict = "CANDIDATE-GREEN"
                    rep.append("  -> **CANDIDATE GREEN (B)** -> подтверждающий лег "
                               "min-of-2 (dispatch_s7195.py --sanctioned повторно) "
                               "-> banking v4 = v3 + inside_bitmask")
                else:
                    verdict = "NOT-GREEN"
                    rep.append("  -> **< бар по одной из осей** -> вердикт-док; "
                               "флагман не даёт min-MSPT-эффекта на живой сцене")
            else:
                rep.append("  DUAL BAR (v8-REGRESSION): normalized AND absolute "
                           "обе >= +10%")
                if nd >= 0.10 and ad >= 0.10:
                    verdict = "CANDIDATE-GREEN"
                    rep.append("  -> **CANDIDATE GREEN (v8)** — неожиданно (флагман "
                               "не банкингуется под v8 по RECON-32/33): перепроверить "
                               "артефакты перед min-of-2")
                else:
                    verdict = "EXPECTED-NO-DUAL-BAR"
                    rep.append("  -> **ожидаемо НЕ GREEN под v8** (TPS-конверсия ~0, "
                               "вердикт RECON-32/33) — вердикт по B-протоколу = "
                               "absorb_s7195.py --protocol B при санкции владельца")
        elif not band_ok:
            verdict = "INVALID-PAIRING"
            rep.append("  -> **ВНЕ ШИРОКОГО БАНДА** — экстремальный лендинг пула; "
                       "ре-диспатч")
        else:
            verdict = "NO-SOAK"
            rep.append("  -> нет соак-поллов — вердикта нет")

        # ---- PG-T4 GC
        young, full = gc_stats(os.path.join(RUN_DIR, "gc.log"))
        t4 = young is None or (young <= YOUNG_GATE and full == 0)
        rep.append(f"- PG-T4: young={young}, Full={full} -> **{'PASS' if t4 else 'FAIL'}**")
        if verdict is None and not t4:
            verdict = "GC-FAIL"

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
                           "ре-диспатч dispatch_s7195.py --sanctioned (макс 2 подряд)")
            elif crash:
                verdict = "CRASH-REFUTED"
                rep.append("  -> **CRASH-REFUTED** — руут-кауз по job-логу, rollback "
                           "inside_bitmask (env-дефолт 0), вердикт-док, S7-170 "
                           "(RECON-22 race в банке) приоритизируется")
            else:
                verdict = "INFRA-FLAKE"
                rep.append("  -> **INFRA-FLAKE** — ре-диспатч без вердикта (макс 2 подряд)")

    rep.append(f"\n## VERDICT: **{verdict}**")
    out = os.path.join(RESDIR, "ABSORB_S7195.md")
    open(out, "w").write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {out}")
    return 0 if verdict in ("CANDIDATE-GREEN", "EXPECTED-NO-DUAL-BAR", "NOT-GREEN") else 1


if __name__ == "__main__":
    sys.exit(main())
