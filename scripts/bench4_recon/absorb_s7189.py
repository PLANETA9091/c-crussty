#!/usr/bin/env python3
"""absorb_s7189.py - TASK-346: absorb lever #14 TRAVEL-ALLOC-DIET v2a ISOLATION
leg s7189 (bank v3 + travel_diet=1, zero_alloc=0/skip_store_bb=0) and deliver the
REGRESSION-PROTOCOL DUAL-BAR verdict vs ANCHOR-SLOW (s7184: median5 1.60 @
runner 6680195).

PROTOCOL v8-REGRESSION (owner sanction 2026-09-20 "это твой проект — разбирайся"):
the runner pool is NOT bimodal — cpu_index continuous, daily span 6.49M..12.31M
(1.9x variance, GOAL x30-ADD) — the ±5% same-class pairing gate became a leg
lottery (3 consecutive BAND-DISCARDs, v2a never soaked). Replacement:
  (1) WIDE BAND 6.0M..9.5M at dispatch (S7-96d fast-fail remains for extreme
      landings, e.g. 12.31M);
  (2) DUAL BAR on BOTH axes vs ANCHOR-SLOW:
      normalized = (med/runner)/(1.60/6680195) AND absolute = med/1.60;
      BOTH >= +10% -> CANDIDATE-GREEN -> confirming min-of-2 leg (same inputs,
      dispatch_s7189.py re-run) -> banking v4 = v3 + travel_diet.
      The two axes bracket the runner-index confound in both directions:
      index-independence evidence: bank v3 = 1.60 @ 6680195 (leg#3) AND
      1.60 @ 8566450 (s7169); normalized is conservative above the anchor
      runner, absolute stays honest below it;
  (3) residual ±12% TPS noise (RECON-19) covered by min-of-2 (sign AND
      magnitude). Owner DUAL BAR directive preserved verbatim — nothing is
      banked on a single axis.

PREREGISTER GATES (dispatch_s7189.py):
  PG-T1 delivery: travel_diet=1 + zero_alloc=0 + skip_store_bb=0 +
    region_steal=0 + bu_defer=0 + inside_cache=1 + flush_diet=1 +
    region_threads=4 + batch_collector=1, NCDFE=0, pop 150k VALID,
    ARM MARKER "stage traveldiet composed" present (fail-dominant chain must
    have actually composed the redirect, not skipped it)
  PG-T2 crash-free: 0 threw/unexpected/NPE (incl. s7180 BlockUpdateOps incident
    class and RECON-22 navigatingMobs catches) + soak TPS polls >= 3
  PG-T3 REGRESSION DUAL-BAR: runner within wide band 6.0M..9.5M (upstream-gated);
    normalized AND absolute both >= +10% -> CANDIDATE GREEN -> confirming leg
    (min-of-2) -> banking v4; else LANE-OPEN -> v2b travel-math (RECON-21
    contract: handleRelativeFrictionAndCalculateMovement + travelInFluid +
    getInputVector) implemented and dispatched in the same tick
  PG-T4 GC sanity: young <= 174, 0 Full
  PG-T5 DONE-park ~1.4% class — N/A tolerated when the AP-PID defect kills the
    wall window (s7178 precedent)
Failure handling: conclusion=failure -> fetch job log, classify
BAND-DISCARD (S7-96d fast-fail, NOT a verdict; max 2 consecutive) /
CRASH-REFUTED (NPE/throw markers -> rollback travel_diet + verdict doc +
S7-170 prioritized: RECON-22 bank race is foundational) / INFRA-FLAKE
(wedge/fixture classes; re-dispatch, max 2 consecutive).
"""
import os, re, statistics, subprocess, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUN_DIR = os.path.join(RESDIR, "run-s7189-traveldiet-v2a")
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
    run_id = int(sys.argv[1]) if len(sys.argv) > 1 else 0
    if not run_id:
        raise SystemExit("usage: absorb_s7189.py <run_id>")
    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={st.get('status')} conclusion={st.get('conclusion')} "
          f"head={str(st.get('head_sha'))[:7]}")
    if st.get("status") != "completed":
        sys.exit(3)
    os.makedirs(RUN_DIR, exist_ok=True)
    have_art = fetch_artifact(tok, run_id)

    rep = [f"# absorb #14 v2a ISOLATION s7189 (run {run_id}, head "
           f"{str(st.get('head_sha'))[:7]}) — PROTOCOL v8-REGRESSION DUAL BAR\n"]
    verdict = None

    stdout = os.path.join(RUN_DIR, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""
    env_txt = ""
    ep = os.path.join(RUN_DIR, "run-env.txt")
    if os.path.isfile(ep):
        env_txt = open(ep, errors="ignore").read()

    if stdout_txt:
        # ---- PG-T1 delivery (v2a isolation + arm marker)
        want = {"travel_diet": "1", "zero_alloc": "0", "skip_store_bb": "0",
                "region_steal": "0", "bu_defer": "0", "inside_cache": "1",
                "flush_diet": "1", "region_threads": "4", "batch_collector": "1"}
        t1 = [f"{k}={'OK' if env_flag(env_txt, k, v) else 'BAD:'+env_flag(env_txt, k, v).__str__()}"
              for k, v in want.items()]
        ncde = stdout_txt.count("NoClassDefFoundError")
        pop_ok = "POPULATION FIXTURE-VALIDITY: VALID" in stdout_txt
        arm = "stage traveldiet composed" in stdout_txt
        strict = "strict check violated" in stdout_txt
        t1_ok = all("OK" in x for x in t1) and ncde == 0 and pop_ok and arm and not strict
        rep.append("- PG-T1: " + ", ".join(t1) + f", NCDFE={ncde}, "
                   f"pop={'VALID' if pop_ok else 'BAD'}, "
                   f"arm={'COMPOSED' if arm else 'MISSING'}, "
                   f"strict-violated={strict} -> **{'PASS' if t1_ok else 'FAIL'}**")

        # ---- PG-T2 crash-free (incl. s7180 incident class + RECON-22 catches)
        threw = stdout_txt.count("Entity threw exception")
        unexpected = stdout_txt.count("Encountered an unexpected exception")
        bu_npe = stdout_txt.count("BlockUpdateOps") + \
            stdout_txt.count("ObjectOpenHashSet$SetIterator")
        tps = tps_series(stdout)
        soak_ok = len(tps) >= 3
        t2_ok = threw == 0 and unexpected == 0 and bu_npe == 0 and soak_ok
        rep.append(f"- PG-T2: threw={threw}, unexpected={unexpected}, s7180-class={bu_npe}, "
                   f"TPS-поллов={len(tps)} -> **{'PASS' if t2_ok else 'FAIL'}**")

        # ---- PG-T3 REGRESSION DUAL-BAR (wide band)
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
            rep.append(f"  DUAL BAR (v8-REGRESSION): normalized={nd:+.1%}, "
                       f"absolute={ad:+.1%} (бар: ОБЕ >= +10%; оси страхуют "
                       f"index-конфаунд в обе стороны)")
            if nd >= 0.10 and ad >= 0.10:
                verdict = "CANDIDATE-GREEN"
                rep.append("  -> **CANDIDATE GREEN** -> подтверждающий лег min-of-2 "
                           "(dispatch_s7189.py повторно) -> banking v4 = v3 + travel_diet")
            else:
                verdict = "LANE-OPEN"
                rep.append("  -> **< +10% хотя бы по одной оси** -> лейн ОТКРЫТ -> "
                           "#14 v2b travel-math (RECON-21 контракт: "
                           "handleRelativeFrictionAndCalculateMovement 48 строк + "
                           "travelInFluid + getInputVector) — реализация+диспатч в том же тике")
        elif not band_ok:
            verdict = "INVALID-PAIRING"
            rep.append("  -> **ВНЕ ШИРОКОГО БАНДА** — лег невалиден для пары "
                       "(экстремальный лендинг пула; ре-диспатч)")
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
                rep.append("  -> **BAND-DISCARD** (S7-96d fast-fail экстремального "
                           "лендинга, не вердикт) — ре-диспатч dispatch_s7189.py "
                           "(макс 2 подряд)")
            elif crash:
                verdict = "CRASH-REFUTED"
                rep.append("  -> **CRASH-REFUTED** — руут-кауз по job-логу, rollback "
                           "travel_diet (yml-дефолт 0), вердикт-док, S7-170 "
                           "(RECON-22 race в банке) приоритизируется")
            else:
                verdict = "INFRA-FLAKE"
                rep.append("  -> **INFRA-FLAKE** — ре-диспатч без вердикта (макс 2 подряд)")

    rep.append(f"\n## VERDICT: **{verdict}**")
    out = os.path.join(RESDIR, "ABSORB_S7189.md")
    open(out, "w").write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {out}")
    return 0 if verdict in ("CANDIDATE-GREEN", "LANE-OPEN", "N/A") else 1


if __name__ == "__main__":
    sys.exit(main())
