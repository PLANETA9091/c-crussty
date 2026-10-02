#!/usr/bin/env python3
"""absorb_s7178.py - TASK-338: absorb BANK RE-CALIBRATION leg s7178
(run 35457901949 @ df6b3c4, bank v3 config, region_steal=0 + bu_defer=0).

PREREGISTER GATES (dispatch_s7178.py):
  PG-RC1 delivery: 0 NCDFE + pop 150k VALID + bank v3 markers
    (inside_cache=1, flush_diet=1, region_threads=4, batch_collector=1)
    + region_steal=0 + bu_defer=0 (rollback verified)
  PG-RC2 runner probe: runner_cpu_index recorded (pairing validity for the
    STEAL v2 leg = future leg within +-5%)
  PG-RC3 TPS: console last-5 median recorded as v3-RECAL anchor (measurement,
    not a pass/fail gate); normalized deltas vs s7169 (1.60 @ 8566450) and
    s7177 (1.40 @ 6874223) reported
  PG-RC4 GC sanity: young <= 174, 0 Full
  PG-RC5 profile-axis sanity: main DONE-park share reproduces the 13.4% class
    (bank v3 has no park-kill lever armed)
Verdict: anchor v3-RECAL recorded -> dispatch s7179 (region_steal=1+
bu_defer=1) next tick -> paired verdict id 339.
"""
import os, re, statistics, subprocess, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUN_DIR = os.path.join(RESDIR, "run-s7178-recal")
YOUNG_GATE = 174
PARK_BASE, PARK_MAIN_BASE, PARK_CLASS = 121, 901, 20  # 13.4% class +-20 samples
S7169_TPS, S7169_RUNNER = 1.60, 8566450
S7177_TPS, S7177_RUNNER = 1.40, 6874223


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL")
    return m.group(1)


def api(tok, url):
    req = urllib.request.Request(f"{API}{url}", headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    try:
        with urllib.request.urlopen(req, timeout=60) as r:
            return json_loads(r.read())
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:200]}", file=sys.stderr)
        return {}


def json_loads(b):
    import json
    return json.loads(b)


def fetch_artifact(tok, run_id):
    arts = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/artifacts").get("artifacts", [])
    bench = [a for a in arts if a["name"] == "world3-bench"]
    if not bench:
        raise SystemExit(f"no world3-bench artifact (available: {[a['name'] for a in arts]})")
    a = bench[0]
    print(f"artifact: {a['name']} size={a['size_in_bytes']} id={a['id']}")
    dest = os.path.join(RUN_DIR, a["name"] + ".zip")
    req = urllib.request.Request(
        f"{API}/repos/{REPO}/actions/artifacts/{a['id']}/zip",
        headers={"Authorization": f"Bearer {tok}"})

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
    breq = urllib.request.Request(loc)  # no auth on the blob host
    with urllib.request.urlopen(breq, timeout=600) as r, open(dest, "wb") as f:
        while True:
            b = r.read(1 << 20)
            if not b:
                break
            f.write(b)
    print(f"downloaded {dest} ({os.path.getsize(dest)} bytes)")
    import zipfile
    with zipfile.ZipFile(dest) as z:
        z.extractall(RUN_DIR)
    print(f"extracted to {RUN_DIR}")


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


def wall_park(run_dir):
    spin = "MinecraftServer.lambda$spin$2"
    tot = park = 0
    p = os.path.join(run_dir, "wall-collapsed.txt")
    if not os.path.isfile(p):
        return None, None
    for line in open(p, errors="ignore"):
        stack, _, cnt = line.rstrip("\n").rpartition(" ")
        if not stack or spin not in stack:
            continue
        try:
            n = int(cnt)
        except ValueError:
            continue
        tot += n
        if "CyclicBarrier" in stack:
            park += n
    return tot, park


def env_val(env_txt, key):
    m = re.search(rf"^{key}: (\d+)", env_txt, re.M)
    return m.group(1) if m else None


def main():
    tok = token()
    run_id = int(sys.argv[1]) if len(sys.argv) > 1 else 35457901949
    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={st.get('status')} conclusion={st.get('conclusion')} "
          f"head={str(st.get('head_sha'))[:7]}")
    if st.get("status") != "completed":
        sys.exit(3)
    if st.get("conclusion") != "success":
        sys.exit(4)
    os.makedirs(RUN_DIR, exist_ok=True)
    fetch_artifact(tok, run_id)

    rep = [f"# RECON-19 - absorb BANK RE-CAL lega s7178 (run {run_id}, head "
           f"{str(st.get('head_sha'))[:7]})\n"]

    stdout = os.path.join(RUN_DIR, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""
    env_txt = ""
    ep = os.path.join(RUN_DIR, "run-env.txt")
    if os.path.isfile(ep):
        env_txt = open(ep, errors="ignore").read()

    # ---- PG-RC1 delivery (rollback + bank v3 markers)
    rc1 = []
    rc1_ok = True
    for key, want in (("region_steal", "0"), ("bu_defer", "0"), ("inside_cache", "1"),
                      ("flush_diet", "1"), ("region_threads", "4"),
                      ("batch_collector", "1")):
        got = env_val(env_txt, key)
        ok = got == want
        rc1.append(f"{key}={got}{'✓' if ok else ' (want ' + want + ')'}")
        rc1_ok = rc1_ok and ok
    ncde = stdout_txt.count("NoClassDefFoundError")
    pop_ok = "POPULATION FIXTURE-VALIDITY: VALID" in stdout_txt
    armed = "ARMED" in stdout_txt
    rc1.append(f"NCDFE={ncde}, pop={'VALID' if pop_ok else 'NOT-VALID'}, "
               f"ARMED-маркер={'есть' if armed else 'НЕТ'}")
    rc1_ok = rc1_ok and ncde == 0 and pop_ok and armed
    rep.append("- PG-RC1: " + ", ".join(rc1) + " -> **" + ("PASS" if rc1_ok else "FAIL") + "**")

    # ---- PG-RC2 runner probe
    m = re.search(r"runner_cpu_index: (\d+)", env_txt)
    runner = int(m.group(1)) if m else None
    rep.append(f"- PG-RC2: runner_cpu_index = {runner} "
               f"(s7169={S7169_RUNNER}, s7177={S7177_RUNNER}; пара валидна при ±5%)")

    # ---- PG-RC3 TPS anchor (measurement)
    tps = tps_series(stdout) if os.path.isfile(stdout) else []
    med = None
    if len(tps) >= 5:
        med = statistics.median(sorted(tps[-5:]))
    rep.append(f"- PG-RC3: TPS линий={len(tps)} median5={med if med is None else round(med, 2)} "
               f"(якорь v3-RECAL; ожидание 1.4-1.8); серия: {tps}")
    if runner and med:
        n78 = med / runner
        d169 = 100.0 * (n78 / (S7169_TPS / S7169_RUNNER) - 1)
        d177 = 100.0 * (n78 / (S7177_TPS / S7177_RUNNER) - 1)
        rep.append(f"  - нормализованная дельта vs s7169 {d169:+.1f}%, vs s7177 {d177:+.1f}%")

    # ---- PG-RC4 GC
    young, full = gc_stats(os.path.join(RUN_DIR, "gc.log"))
    rc4_ok = (young is not None) and (young <= YOUNG_GATE) and (full == 0)
    rep.append(f"- PG-RC4: young={young} (≤{YOUNG_GATE}) Full={full} -> "
               f"**{'PASS' if rc4_ok else 'FAIL'}**")

    # ---- PG-RC5 park baseline reproduction
    tot, park = wall_park(RUN_DIR)
    if tot:
        share = 100.0 * park / tot
        rc5_ok = abs(park - PARK_BASE) <= PARK_CLASS
        rep.append(f"- PG-RC5: DONE-park {park}/{tot} = {share:.1f}% (база 121/901 = 13.4%, "
                   f"класс ±{PARK_CLASS}) -> **{'PASS' if rc5_ok else 'FAIL'}**")
    else:
        rc5_ok = None
        rep.append("- PG-RC5: **N/A** — wall-сэмплов нет")

    # ---- Verdict
    if rc1_ok and rc4_ok and rc5_ok is not False:
        rep.append(
            f"- ВЕРДИКТ: **ЯКОРЬ v3-RECAL ЗАПИСАН** (median5={med}, runner={runner}, "
            f"park {park}/{tot}); NEXT: DISPATCH s7179 (region_steal=1 + bu_defer=1) → "
            f"парный вердикт: нормализованная дельта ≥ +10% (обе ноги ±5% runner) = GREEN "
            f"banking v4, иначе лейн открыт с рычагом #14 (TRAVEL-ALLOC-DIET)")
    else:
        rep.append("- ВЕРДИКТ: **ДЕФЕКТ ДОСТАВКИ/СРЕДЫ** — руут-кауз + ре-диспатч re-cal лега")

    rep_path = os.path.join(RESDIR, "RECON19_BANK_RECAL_ABSORB.md")
    with open(rep_path, "w") as f:
        f.write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {rep_path}")


if __name__ == "__main__":
    main()
