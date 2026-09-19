#!/usr/bin/env python3
"""absorb_s7180.py - TASK-340: absorb STEAL v2 PAIRED leg s7180
(run 35460026013 @ 0c65771, region_steal=1 + bu_defer=1) and deliver the
PAIRED VERDICT vs the s7178 anchor (RECON-19: median5=1.40 @ runner 8493973).

PREREGISTER GATES (dispatch_s7179/7180.py):
  PG-P1 delivery: region_steal=1 + bu_defer=1 + "BU-DEFER composed" marker
    + NCDFE=0 + pop VALID
  PG-P2 crash-free: 0 threw/unexpected/NPE + soak evidence (TPS polls >= 3
    + alloc/cpu collapsed non-empty when present)
  PG-P3 PAIRED TPS: runner_cpu_index within ±5% of ANCHOR_RUNNER (else the
    leg is INVALID FOR PAIRING -> re-dispatch, no verdict); verdict metric =
    speed-normalized delta vs anchor (1.40 @ 8493973):
      >= +10% -> CANDIDATE GREEN -> confirming leg s7181 (min-of-2) required
        before CUMULATIVE v4 banking;
      < +10%  -> lane stays OPEN -> RECON-20 with lever #14 TRAVEL-ALLOC-DIET
  PG-P4 GC sanity: young <= 174, 0 Full
  PG-P5 DONE-park ~1.4% class (s7177 evidence 16/1174) — N/A tolerated when
    the AP-PID defect kills the wall window (s7178 precedent).
INFRA-FLAKE rule (s7179 precedent): fixture-INVALID / wedge markers -> verdict
INFRA-FLAKE, re-dispatch without lever verdict (max 2 consecutive attempts).
"""
import os, re, statistics, subprocess, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUN_DIR = os.path.join(RESDIR, "run-s7180-steal-pair")
ANCHOR_TPS, ANCHOR_RUNNER = 1.40, 8493973
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


def fetch_artifact(tok, run_id):
    import zipfile
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
    with urllib.request.urlopen(urllib.request.Request(loc), timeout=600) as r, \
            open(dest, "wb") as f:
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
    run_id = int(sys.argv[1]) if len(sys.argv) > 1 else 35460026013
    st = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={st.get('status')} conclusion={st.get('conclusion')} "
          f"head={str(st.get('head_sha'))[:7]}")
    if st.get("status") != "completed":
        sys.exit(3)
    os.makedirs(RUN_DIR, exist_ok=True)
    if st.get("conclusion") != "success":
        # infra-flake class (s7179): try to pull the job log conclusion markers
        print("conclusion != success -> потенциальный INFRA-FLAKE; "
              "разобрать job-лог до вердикта (рулетка s7179)")
        sys.exit(4)
    fetch_artifact(tok, run_id)

    rep = [f"# RECON-20 - absorb STEAL v2 paired lega s7180 (run {run_id}, head "
           f"{str(st.get('head_sha'))[:7]})\n"]

    stdout = os.path.join(RUN_DIR, "server-stdout.log")
    stdout_txt = open(stdout, errors="ignore").read() if os.path.isfile(stdout) else ""
    env_txt = ""
    ep = os.path.join(RUN_DIR, "run-env.txt")
    if os.path.isfile(ep):
        env_txt = open(ep, errors="ignore").read()

    # ---- PG-P1 delivery
    p1 = []
    p1_ok = True
    for key, want in (("region_steal", "1"), ("bu_defer", "1")):
        m = re.search(rf"^{key}: (\d+)", env_txt, re.M)
        ok = bool(m) and m.group(1) == want
        p1.append(f"{key}={'✓' if ok else 'NOT ' + want}")
        p1_ok = p1_ok and ok
    composed = "BU-DEFER composed" in stdout_txt
    ncde = stdout_txt.count("NoClassDefFoundError")
    pop_ok = "POPULATION FIXTURE-VALIDITY: VALID" in stdout_txt
    p1.append(f"composed={'✓' if composed else '✗'}, NCDFE={ncde}, "
              f"pop={'VALID' if pop_ok else '✗'}")
    p1_ok = p1_ok and composed and ncde == 0 and pop_ok
    rep.append("- PG-P1: " + ", ".join(p1) + " -> **" + ("PASS" if p1_ok else "FAIL") + "**")

    # ---- PG-P2 crash-free + soak evidence
    threw = stdout_txt.count("Entity threw exception")
    unexpected = stdout_txt.count("Encountered an unexpected exception")
    npe_mark = stdout_txt.count("wrapped\" is null") + \
        stdout_txt.count("ObjectOpenHashSet$SetIterator")
    tps = tps_series(stdout) if os.path.isfile(stdout) else []
    soak_ok = len(tps) >= 3
    p2_ok = threw == 0 and unexpected == 0 and npe_mark == 0 and soak_ok
    rep.append(f"- PG-P2: threw={threw}, unexpected={unexpected}, NPE={npe_mark}, "
               f"TPS-поллов={len(tps)} -> **{'PASS' if p2_ok else 'FAIL'}**")

    # ---- PG-P3 paired TPS verdict
    m = re.search(r"runner_cpu_index: (\d+)", env_txt)
    runner = int(m.group(1)) if m else None
    med = None
    if len(tps) >= 5:
        med = statistics.median(sorted(tps[-5:]))
    elif tps:
        med = statistics.median(tps)
    pair_ok = runner is not None and abs(runner - ANCHOR_RUNNER) / ANCHOR_RUNNER <= 0.05
    rep.append(f"- PG-P3: runner={runner} (якорь {ANCHOR_RUNNER}, пара ±5%: "
               f"{'В КЛАССЕ' if pair_ok else 'ВНЕ КЛАССА'}), median5={med}, "
               f"серия: {tps}")
    delta = None
    if pair_ok and med:
        delta = 100.0 * ((med / runner) / (ANCHOR_TPS / ANCHOR_RUNNER) - 1)
        rep.append(f"  - нормализованная парная дельта vs якоря: **{delta:+.1f}%** "
                   f"(бар ≥ +10%)")

    # ---- PG-P4 GC
    young, full = gc_stats(os.path.join(RUN_DIR, "gc.log"))
    p4_ok = (young is not None) and (young <= YOUNG_GATE) and (full == 0)
    rep.append(f"- PG-P4: young={young} (≤{YOUNG_GATE}) Full={full} -> "
               f"**{'PASS' if p4_ok else 'FAIL'}**")

    # ---- PG-P5 park (N/A tolerated)
    tot, park = wall_park(RUN_DIR)
    if tot:
        rep.append(f"- PG-P5: DONE-park {park}/{tot} = {100.0*park/tot:.1f}% "
                   f"(класс s7177: 1.4%)")
    else:
        rep.append("- PG-P5: **N/A** (AP-PID дефект класса s7178 — толерантен)")

    # ---- Verdict
    if not pair_ok:
        rep.append("- ВЕРДИКТ: **ЛЕГ НЕВАЛИДЕН ДЛЯ ПАРЫ** (раннер вне ±5% класса якоря) "
                   "→ ре-диспатч той же ноги без вердикта по рычагу")
    elif p1_ok and p2_ok and p4_ok and delta is not None and delta >= 10.0:
        rep.append("- ВЕРДИКТ: **CANDIDATE GREEN** — парная дельта ≥ +10%; по min-of-2 "
                   "нужен подтверждающий лег s7181 (dispatch_s7181.py) → оба ≥ +10% "
                   "= CUMULATIVE v4 = v3 + region_steal=1 + bu_defer=1")
    elif p1_ok and p2_ok and p4_ok and delta is not None:
        rep.append(f"- ВЕРДИКТ: **< +10% ({delta:+.1f}%)** — лейн НЕ закрыт (v7) → "
                   "RECON-20: рычаг #14 TRAVEL-ALLOC-DIET (travel-листья 45.73% alloc, "
                   "вызов-сайты recon14_travel.py); мосты = инфраструктура")
    else:
        rep.append("- ВЕРДИКТ: **ДЕФЕКТ ДОСТАВКИ/СРЕДЫ** — руут-кауз + ре-диспатч")

    rep_path = os.path.join(RESDIR, "RECON20_STEAL_PAIRED_ABSORB.md")
    with open(rep_path, "w") as f:
        f.write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {rep_path}")


if __name__ == "__main__":
    main()
