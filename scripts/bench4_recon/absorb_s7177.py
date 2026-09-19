#!/usr/bin/env python3
"""absorb_s7177.py - absorb STEAL v2 defect-fix gate leg s7177 (TASK-336).

Pregister (TASK-335 / dispatch_s7177.py):
  PG-V2  delivery: run-env "bu_defer: 1" + "region_steal: 1" + stdout
         "BU-DEFER composed: sendBlockUpdated -> BlockUpdateOps.handle" +
         NCDFE=0 + pop 150k VALID
  PG-V2a crash-free vs the s7176 incident: 0 "Entity threw exception",
         0 "Encountered an unexpected exception", 0 sendBlockUpdated NPE
         markers ("wrapped" is null / ObjectOpenHashSet$SetIterator), and
         SOAK-COMPLETE evidence: alloc-collapsed.txt non-empty (s7176 crash
         left it EMPTY) + TPS polls >= 4 (s7176 crash had 2)
  PG-V2b DONE-park kill: main-thread wall samples on CyclicBarrier
         <= 40/901 (base 121/901 = 13.4%, RECON-15/16 methodology; wall
         collapsed with MinecraftServer.spin root, calibrated on s7169)
  PG-V2c TPS: last-5 median >= 1.98 (bank 1.80 + 10% GREEN bar). RECON-15
         arithmetic predicts ~+4-6% from park-kill alone; TPS < 1.98 ->
         REFUTED-by-TPS -> rollback region_steal=0 + bu_defer=0 - the lane
         is NOT closed (v7)
  PG-V2d GC sanity: young <= 174 (s7169 clean-base), Full = 0
Verdict: PASS  -> CUMULATIVE v4 = v3 + region_steal=1 + bu_defer=1 (banking
               in-tick) -> fresh TOP across the three axes;
         FAIL-by-TPS -> rollback both flags, RECON-17 on the same lane;
         delivery defect -> root-cause + fix + re-dispatch in-tick.
"""
import json, os, re, subprocess, sys, urllib.request, urllib.error, zipfile

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RUN_DIR = "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7177-steal-v2"
BANK_TPS = 1.98
PARK_BASE = 121
PARK_MAIN_BASE = 901
PARK_GATE = 40
YOUNG_GATE = 174


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
            return json.loads(r.read())
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:200]}", file=sys.stderr)
        return {}


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
    with urllib.request.urlopen(req, timeout=300) as r, open(dest, "wb") as f:
        while True:
            b = r.read(1 << 20)
            if not b:
                break
            f.write(b)
    print(f"downloaded {dest} ({os.path.getsize(dest)} bytes)")
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
    """main thread (MinecraftServer.spin root) total + CyclicBarrier samples."""
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


def main():
    tok = token()
    run_id = int(sys.argv[1]) if len(sys.argv) > 1 else 35455926384
    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={st.get('status')} conclusion={st.get('conclusion')} head={str(st.get('head_sha'))[:7]}")
    if st.get("status") != "completed":
        sys.exit(3)
    if st.get("conclusion") != "success":
        sys.exit(4)
    os.makedirs(RUN_DIR, exist_ok=True)
    fetch_artifact(tok, run_id)

    rep = [f"# RECON-18 - absorb STEAL v2 gate lega s7177 (run {run_id}, head {str(st.get('head_sha'))[:7]})\n"]

    stdout = os.path.join(RUN_DIR, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""

    # ---- PG-V2 delivery
    env_txt = ""
    ep = os.path.join(RUN_DIR, "run-env.txt")
    if os.path.isfile(ep):
        env_txt = open(ep, errors="ignore").read()
    s2 = []
    s2_ok = True
    if "bu_defer: 1" in env_txt:
        s2.append("bu_defer=1 ARMED")
    else:
        s2.append("bu_defer NOT ARMED in run-env")
        s2_ok = False
    if "region_steal: 1" in env_txt:
        s2.append("region_steal=1")
    else:
        s2.append("region_steal NOT 1")
        s2_ok = False
    if "BU-DEFER composed" in stdout_txt:
        s2.append("BU-DEFER composed marker")
    else:
        s2.append("BU-DEFER composed marker MISSING")
        s2_ok = False
    ncde = stdout_txt.count("NoClassDefFoundError")
    s2.append(f"NCDFE={ncde}")
    if ncde:
        s2_ok = False
    pop_ok = "POPULATION FIXTURE-VALIDITY: VALID" in stdout_txt
    s2.append("pop VALID" if pop_ok else "pop VALID marker NOT found")
    if not pop_ok:
        s2_ok = False
    rep.append("- PG-V2: " + ", ".join(s2) + " -> **" + ("PASS" if s2_ok else "FAIL") + "**")

    # ---- PG-V2a crash-free vs s7176 incident
    threw = stdout_txt.count("Entity threw exception")
    unexpected = stdout_txt.count("Encountered an unexpected exception")
    npe_mark = stdout_txt.count("wrapped\" is null") + stdout_txt.count("ObjectOpenHashSet$SetIterator")
    alloc_path = os.path.join(RUN_DIR, "alloc-collapsed.txt")
    alloc_size = os.path.getsize(alloc_path) if os.path.isfile(alloc_path) else 0
    tps = tps_series(stdout) if os.path.isfile(stdout) else []
    soak_ok = alloc_size > 1024 and len(tps) >= 4
    v2a = threw == 0 and unexpected == 0 and npe_mark == 0 and soak_ok
    rep.append(
        f"- PG-V2a: threw={threw}, unexpected={unexpected}, sendBlockUpdated-NPE={npe_mark}, "
        f"alloc-collapsed={alloc_size}B (crash-leg had EMPTY), TPS-поллов={len(tps)} (crash-leg 2) -> "
        f"**{'PASS' if v2a else 'FAIL'}**")

    # ---- PG-V2b DONE-park
    tot, park = wall_park(RUN_DIR)
    if tot is None or tot == 0:
        rep.append("- PG-V2b: **N/A** — main-thread wall-сэмплов нет")
        v2b_ok = None
    else:
        share = 100.0 * park / tot
        v2b_ok = park <= PARK_GATE
        rep.append(f"- PG-V2b: DONE-park {park}/{tot} = {share:.1f}% (база {PARK_BASE}/{PARK_MAIN_BASE} = "
                   f"{100.0*PARK_BASE/PARK_MAIN_BASE:.1f}%, гейт ≤{PARK_GATE}) -> "
                   f"**{'PASS' if v2b_ok else 'FAIL'}**")

    # ---- PG-V2c TPS
    if len(tps) < 5:
        rep.append(f"- PG-V2c: **FAIL** — TPS-линий нет (n={len(tps)})")
        v2c_ok = False
        med = None
    else:
        last5 = sorted(tps[-5:])
        med = last5[len(last5) // 2]
        v2c_ok = med >= BANK_TPS
        rep.append(f"- PG-V2c: TPS линий={len(tps)} median5={med:.2f} (гейт ≥{BANK_TPS}, банк 1.80+10%) -> "
                   f"**{'PASS' if v2c_ok else 'FAIL'}**")
        rep.append(f"  - серия: {tps}")

    # ---- PG-V2d GC
    young, full = gc_stats(os.path.join(RUN_DIR, "gc.log"))
    if young is None:
        rep.append("- PG-V2d: **N/A** — gc.log отсутствует")
        v2d_ok = None
    else:
        v2d_ok = (young <= YOUNG_GATE) and (full == 0)
        rep.append(f"- PG-V2d: young={young} (≤{YOUNG_GATE}) Full={full} -> **{'PASS' if v2d_ok else 'FAIL'}**")

    # ---- Verdict
    checks = [x for x in [s2_ok, v2a, v2b_ok, v2d_ok] if x is not None]
    infra_ok = all(checks)
    if v2a and infra_ok and v2c_ok:
        rep.append(
            f"- ВЕРДИКТ: **GREEN — STEAL v2 (region_steal=1 + bu_defer=1) подтверждён** "
            f"(TPS {med:.2f} ≥ {BANK_TPS}, DONE-park {park} vs {PARK_BASE}, crash-free); "
            f"CUMULATIVE v4 = v3 + region_steal=1 + bu_defer=1 (банкинг в тике) → свежий ТОП")
    elif v2a and infra_ok and not v2c_ok:
        rep.append(
            f"- ВЕРДИКТ: **REFUTED-by-TPS — буст <10%** (median5={med:.2f} < {BANK_TPS}); "
            f"rollback region_steal=0 + bu_defer=0, лейн НЕ закрыт (v7) → RECON-17 того же лейна")
    else:
        rep.append("- ВЕРДИКТ: **ЕСТЬ ПРОБЛЕМЫ ДОСТАВКИ/КРЭШ** — руут-кауз + фикс + ре-диспатч в тике")

    rep_path = os.path.join(os.path.dirname(RUN_DIR), "RECON18_STEAL_V2_ABSORB.md")
    with open(rep_path, "w") as f:
        f.write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {rep_path}")


if __name__ == "__main__":
    main()
