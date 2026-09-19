#!/usr/bin/env python3
"""absorb_s7176.py — absorb REGION-STEAL lever #13 v1 gate leg s7176 (TASK-334).

Pregister (TASK-333 / dispatch_s7176.py):
  PG-S2 delivery: run-env "region_steal: 1" + pop VALID + NCDFE=0 + ARMED markers
  PG-S3a DONE-park kill: main-thread wall samples on CyclicBarrier <= 40/900 (base 121/901)
  PG-S3b balance: worker-vs-main spread — N/A-tolerant (collapsed does not split threads)
  PG-S4 TPS: last-5 median >= 1.98 (bank 1.80 + 10% GREEN bar, v7)
  PG-S5 GC sanity: young <= 174, Full = 0
  CRASH-FREE: <= 5 exception lines in server stdout
Verdict: PASS -> CUMULATIVE v4 = v3 + region_steal=1 (banking in-tick);
         TPS FAIL -> REFUTED-by-TPS -> rollback region_steal=0 (infrastructure bridge).
"""
import json, os, re, subprocess, sys, urllib.request, urllib.error, zipfile

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RUN_DIR = "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7176-region-steal"
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
    if not arts:
        raise SystemExit("no artifacts")
    a = max(arts, key=lambda x: x["size"])
    print(f"artifact: {a['name']} size={a['size']} id={a['id']}")
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
    """main thread (MinecraftServer.spin root) total + barrier samples."""
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
    run_id = int(sys.argv[1]) if len(sys.argv) > 1 else 35452028284
    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={st.get('status')} conclusion={st.get('conclusion')} head={str(st.get('head_sha'))[:7]}")
    if st.get("status") != "completed":
        sys.exit(3)
    if st.get("conclusion") != "success":
        sys.exit(4)
    os.makedirs(RUN_DIR, exist_ok=True)
    fetch_artifact(tok, run_id)

    rep = [f"# RECON-16 — absorb REGION-STEAL gate лега s7176 (run {run_id}, head {str(st.get('head_sha'))[:7]})\n"]

    stdout = os.path.join(RUN_DIR, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""

    # ---- PG-S2 delivery
    env_txt = ""
    ep = os.path.join(RUN_DIR, "run-env.txt")
    if os.path.isfile(ep):
        env_txt = open(ep, errors="ignore").read()
    s2 = []
    s2_ok = True
    if "region_steal: 1" in env_txt:
        s2.append("region_steal=1 ARMED")
    else:
        s2.append("region_steal NOT ARMED in run-env")
        s2_ok = False
    ncde = stdout_txt.count("NoClassDefFoundError")
    s2.append(f"NCDFE={ncde}")
    if ncde:
        s2_ok = False
    pop_ok = re.search(r"pop[^\n]*VALID|VALID[^\n]*pop", stdout_txt, re.I) or "150000" in stdout_txt
    s2.append("pop VALID" if pop_ok else "pop marker NOT found")
    if not pop_ok:
        s2_ok = False
    rep.append("- PG-S2: " + ", ".join(s2) + " -> **" + ("PASS" if s2_ok else "FAIL") + "**")

    # ---- PG-S3a DONE-park
    tot, park = wall_park(RUN_DIR)
    if tot is None:
        rep.append("- PG-S3a: **N/A** — wall-collapsed.txt отсутствует")
        s3a_ok = None
    elif tot == 0:
        rep.append("- PG-S3a: **N/A** — main-thread wall-сэмплов нет")
        s3a_ok = None
    else:
        share = 100.0 * park / tot
        s3a_ok = park <= PARK_GATE
        rep.append(f"- PG-S3a: DONE-park {park}/{tot} = {share:.1f}% (база {PARK_BASE}/{PARK_MAIN_BASE} = "
                   f"{100.0*PARK_BASE/PARK_MAIN_BASE:.1f}%, гейт ≤{PARK_GATE}) -> "
                   f"**{'PASS' if s3a_ok else 'FAIL'}**")

    # ---- PG-S3b balance (N/A-tolerant: collapsed stacks do not split worker threads)
    rep.append("- PG-S3b: **N/A** — collapsed-стеки не разделяют треды-воркеры; "
               "баланс проверяется косвенно через PG-S3a+PG-S4")

    # ---- PG-S4 TPS
    tps = tps_series(stdout) if os.path.isfile(stdout) else []
    if len(tps) < 5:
        rep.append("- PG-S4: **FAIL** — TPS-линий нет (n=" + str(len(tps)) + ")")
        s4_ok = False
        med = None
    else:
        last5 = sorted(tps[-5:])
        med = last5[len(last5) // 2]
        s4_ok = med >= BANK_TPS
        rep.append(f"- PG-S4: TPS линий={len(tps)} median5={med:.2f} (гейт ≥{BANK_TPS}, банк 1.80+10%) -> "
                   f"**{'PASS' if s4_ok else 'FAIL'}**")
        rep.append(f"  - серия: {tps}")

    # ---- PG-S5 GC
    young, full = gc_stats(os.path.join(RUN_DIR, "gc.log"))
    if young is None:
        rep.append("- PG-S5: **N/A** — gc.log отсутствует")
        s5_ok = None
    else:
        s5_ok = (young <= YOUNG_GATE) and (full == 0)
        rep.append(f"- PG-S5: young={young} (≤{YOUNG_GATE}) Full={full} -> **{'PASS' if s5_ok else 'FAIL'}**")

    # ---- CRASH-FREE
    exc = len(re.findall(r"^(Exception|java\.lang\.\w+Exception|\w+Exception)", stdout_txt, re.M))
    crash_ok = exc <= 5
    rep.append(f"- CRASH-FREE: исключений={exc} (≤5) -> **{'PASS' if crash_ok else 'FAIL'}**")

    # ---- Verdict
    checks = [x for x in [s2_ok, s3a_ok, s4_ok, s5_ok, crash_ok] if x is not None]
    all_pass = all(checks)
    if all_pass and s4_ok:
        rep.append(f"- ВЕРДИКТ: **GREEN — lever #13 REGION-STEAL v1 подтверждён** (TPS {med:.2f} ≥ {BANK_TPS}, "
                   f"DONE-park убит {park} vs {PARK_BASE}); CUMULATIVE v4 = v3 + region_steal=1 (банкинг в тике)")
    elif s2_ok and (s4_ok is False):
        rep.append(f"- ВЕРДИКТ: **REFUTED-by-TPS — lever #13 v1 не дал ≥10%** (median5={med:.2f} < {BANK_TPS}); "
                   f"rollback region_steal=0 (bridge=infrastructure), лейн НЕ закрыт (v7)")
    else:
        rep.append("- ВЕРДИКТ: **ЕСТЬ ПРОБЛЕМЫ ДОСТАВКИ/САНИТИ** — руут-кауз + фикс + ре-диспатч в тике")

    rep_path = os.path.join(os.path.dirname(RUN_DIR), "RECON16_REGION_STEAL_ABSORB.md")
    with open(rep_path, "w") as f:
        f.write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {rep_path}")


if __name__ == "__main__":
    main()
