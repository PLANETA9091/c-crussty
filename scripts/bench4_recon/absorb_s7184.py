#!/usr/bin/env python3
"""absorb_s7184.py - TASK-341: absorb SLOW-CLASS BANK re-cal leg s7184
(bank v3 config, cpu_band 6.5M..7.2M) and record the SLOW-CLASS ANCHOR.

Context (RECON-20 addendum): the CI runner pool turned bimodal-heavy — 5
consecutive slow-class landings (6.72M/6.87M/6.99M/6.73M/…) vs the fast anchor
class 8.49M/8.57M. Fast-band legs s7182/s7183 = BAND-DISCARD (S7-96d fast-fail,
11s, not a verdict). Fallback pairing plan: build a slow-class bank anchor,
then pair #14 TRAVEL-ALLOC-DIET legs INSIDE the slow class (DUAL BAR preserved:
both normalized AND absolute deltas vs the SAME-CLASS anchor, min-of-2).

GATES (RC-class):
  PG-R1 delivery: zero_alloc=0 + skip_store_bb=0 + region_steal=0 + bu_defer=0 +
    inside_cache=1 + flush_diet=1 + region_threads=4 + batch_collector=1,
    NCDFE=0, pop 150k VALID
  PG-R2 crash-free: 0 threw/unexpected/NPE + soak TPS polls >= 3
  PG-R3 ANCHOR MEASUREMENT (not a gate): median5 TPS + runner_cpu_index ->
    ANCHOR-SLOW; valid only when runner lands in 6.5M..7.2M band
  PG-R4 GC sanity: young <= 174, 0 Full
  PG-R5 park N/A tolerated (AP-PID defect precedent)
Failure roulette: BAND-DISCARD / CRASH-REFUTED / INFRA-FLAKE (job-log markers).
"""
import os, re, statistics, subprocess, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUN_DIR = os.path.join(RESDIR, "run-s7184-bank-slowclass")
BAND_LO, BAND_HI = 6500000, 7200000


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
        raise SystemExit("usage: absorb_s7184.py <run_id>")
    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={st.get('status')} conclusion={st.get('conclusion')} "
          f"head={str(st.get('head_sha'))[:7]}")
    if st.get("status") != "completed":
        sys.exit(3)
    os.makedirs(RUN_DIR, exist_ok=True)
    have_art = fetch_artifact(tok, run_id)

    rep = [f"# absorb SLOW-CLASS BANK s7184 (run {run_id}, head "
           f"{str(st.get('head_sha'))[:7]})\n"]
    verdict = None
    anchor_line = None

    stdout = os.path.join(RUN_DIR, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""
    env_txt = ""
    ep = os.path.join(RUN_DIR, "run-env.txt")
    if os.path.isfile(ep):
        env_txt = open(ep, errors="ignore").read()

    if stdout_txt:
        want = {"zero_alloc": "0", "skip_store_bb": "0", "region_steal": "0",
                "bu_defer": "0", "inside_cache": "1", "flush_diet": "1",
                "region_threads": "4", "batch_collector": "1"}
        r1 = [f"{k}={'OK' if env_flag(env_txt, k, v) else 'BAD'}" for k, v in want.items()]
        ncde = stdout_txt.count("NoClassDefFoundError")
        pop_ok = "POPULATION FIXTURE-VALIDITY: VALID" in stdout_txt
        r1_ok = all(x.endswith("OK") for x in r1) and ncde == 0 and pop_ok
        rep.append("- PG-R1: " + ", ".join(r1) + f", NCDFE={ncde}, "
                   f"pop={'VALID' if pop_ok else 'BAD'} -> **{'PASS' if r1_ok else 'FAIL'}**")

        threw = stdout_txt.count("Entity threw exception")
        unexpected = stdout_txt.count("Encountered an unexpected exception")
        npe = stdout_txt.count("ObjectOpenHashSet$SetIterator") + \
            stdout_txt.count("BlockUpdateOps")
        tps = tps_series(stdout)
        r2_ok = threw == 0 and unexpected == 0 and npe == 0 and len(tps) >= 3
        rep.append(f"- PG-R2: threw={threw}, unexpected={unexpected}, NPE={npe}, "
                   f"TPS-поллов={len(tps)} -> **{'PASS' if r2_ok else 'FAIL'}**")

        m = re.search(r"runner_cpu_index: (\d+)", env_txt)
        runner = int(m.group(1)) if m else None
        med = None
        if len(tps) >= 5:
            med = statistics.median(sorted(tps[-5:]))
        elif tps:
            med = statistics.median(tps)
        in_band = runner is not None and BAND_LO <= runner <= BAND_HI
        rep.append(f"- PG-R3: runner={runner} (банд {BAND_LO}..{BAND_HI}: "
                   f"{'В БАНДЕ' if in_band else 'ВНЕ БАНДЫ'}), median5={med}")
        if med is not None and in_band:
            verdict = "ANCHOR-RECORDED"
            anchor_line = f"ANCHOR-SLOW: median5={med} @ runner={runner}"
            rep.append(f"  -> **ANCHOR-SLOW ЗАПИСАН: median5={med} @ {runner}** — "
                       f"пары #14 дальше в медленном классе (ДВОЙНОЙ БАР vs этот якорь)")
        elif not in_band:
            verdict = "INVALID-BAND"
            rep.append("  -> вне банды — якорь не записывается, ре-диспатч "
                       "dispatch_s7184.py")
        else:
            verdict = "NO-SOAK"
            rep.append("  -> нет соак-поллов")

        young, full = gc_stats(os.path.join(RUN_DIR, "gc.log"))
        r4 = young is None or (young <= 174 and full == 0)
        rep.append(f"- PG-R4: young={young}, Full={full} -> **{'PASS' if r4 else 'FAIL'}**")
        rep.append("- PG-R5: park N/A tolerated (AP-PID дефект)")

    if not have_art or not stdout_txt:
        jl = fetch_joblog(tok, run_id)
        crash = sum(jl.count(x) for x in
                    ("Encountered an unexpected exception", "ReportedException",
                     "ObjectOpenHashSet$SetIterator", "NullPointerException"))
        band = "OUTSIDE band" in jl
        fixture = jl.count("FIXTURE-VALIDITY: INVALID")
        rep.append(f"- FAILURE-рулетка: crash={crash}, band-discard={band}, "
                   f"fixture-INVALID={fixture}")
        if verdict is None:
            if band and crash == 0:
                verdict = "BAND-DISCARD"
                rep.append("  -> **BAND-DISCARD** — ре-диспатч без вердикта")
            elif crash:
                verdict = "CRASH-REFUTED"
            else:
                verdict = "INFRA-FLAKE"

    rep.append(f"\n## VERDICT: **{verdict}**")
    if anchor_line:
        rep.append(f"## {anchor_line}")
    out = os.path.join(RESDIR, "RECON20_ABSORB_S7184.md")
    open(out, "w").write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {out}")
    return 0 if verdict in ("ANCHOR-RECORDED",) else 1


if __name__ == "__main__":
    sys.exit(main())
