#!/usr/bin/env python3
"""absorb_s7174.py — absorb INSIDE-DIET lever #12 v1 gate leg s7174 (TASK-332).

Run: set at dispatch (dispatch_s7174.py). Lane base (RECON14C, s7173):
inside-blocks = 31.18% of alloc window / 9.20% CPU.

PREREGISTER GATES (dispatch_s7174.py docstring):
  PG-ID2 delivery: pop VALID, NCDFE=0,
    "inside_diet: defined ... InsideDietOps + ... InsideDietVisitor in kernel loader",
    "entity_compose: stage inside_diet composed (Retargeted { sites: 1 })",
    "entity_compose: ARMED chain [...inside_diet...]", region_threads ARMED,
    batch_collector defined, OOM=нет
  PG-ID3 TPS: median5 >= 1.60 (harm floor); delta vs bank 1.80 printed
    (LEVER-BOOST; буст может быть <10% — тогда лейн НЕ закрыт, v7)
  PG-ID4a inside-blocks alloc lane <= -8% vs 31.18% (expect -10..-25%)
  PG-ID4b young <= 154, Full = 0
  PG-ID4c inside-CPU lane delta vs 9.20% (gate >= -3%)
  CRASH-FREE: "Entity threw exception" <= 5
Banking: full PASS -> CUMULATIVE v4 = v3 + inside_diet=1 (zero_cursor=0);
         FAIL -> REFUTED + rollback inside_diet=0.
Exit: 0 absorbed, 3 in flight, 4 failure/delivery, 2 technical.
"""
import json, os, re, subprocess, sys, urllib.request
from collections import Counter

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
DEFAULT_RUN = "35444398986"
EXPECTED_SHA = "SET_AT_DISPATCH"

BASE_LANE_ALLOC = 31.18
BASE_LANE_CPU = 9.20
BANK_TPS_MED5 = 1.80
BANK_YOUNG = 154
INSIDE_RX = re.compile(r"checkInsideBlocks|InsideBlockEffectApplier|StepBasedCollector|InsideBlockOps")


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


def run_status(tok, run_id):
    r = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    return r.get("status", "?"), r.get("conclusion"), r.get("head_sha", "?")[:12]


def collapsed_share(path, rx):
    agg = Counter()
    total = 0
    if not os.path.isfile(path):
        return None, 0
    with open(path, errors="ignore") as f:
        for line in f:
            p = line.rstrip("\n").rpartition(" ")
            if not p[0]:
                continue
            try:
                n = int(p[2])
            except ValueError:
                continue
            total += n
            if rx.search(p[0]):
                agg[p[0].split(";")[-1].split("$Lambda")[0]] += n
    share = (sum(agg.values()) / total * 100.0) if total else None
    return share, total


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


def main():
    run_id = sys.argv[1] if len(sys.argv) > 1 and sys.argv[1].isdigit() else DEFAULT_RUN
    tok = token()
    st, cc, sha = run_status(tok, run_id)
    print(f"run {run_id}: status={st} conclusion={cc} head={sha}")
    if st in ("in_progress", "queued", "waiting"):
        return 3
    dest = f"{RESDIR}/run-s7174-inside-diet"
    subprocess.run([sys.executable, "/home/z/c-crussty/scripts/bench4_recon/fetch_artifact.py",
                    run_id, dest], check=False)
    d = f"{dest}/world3-bench" if os.path.isdir(f"{dest}/world3-bench") else dest

    rep = [f"# RECON-14 — absorb INSIDE-DIET gate лега s7174 (run {run_id}, head {sha})\n"]
    if EXPECTED_SHA != "SET_AT_DISPATCH" and not sha.startswith(EXPECTED_SHA):
        rep.append(f"- ВНИМАНИЕ: head {sha} != ожидаемого {EXPECTED_SHA}")

    sso_p = f"{d}/server-stdout.log"
    if not os.path.isfile(sso_p):
        rep.append("- **FAIL**: server-stdout.log отсутствует — дефект доставки")
        print("\n".join(rep))
        return 4
    sso = open(sso_p, errors="ignore").read()

    # ---- PG-ID2 delivery
    z2 = []
    z2.append("pop " + ("VALID" if "FIXTURE-VALIDITY: VALID" in sso else "INVALID"))
    z2.append(f"NCDFE={sso.count('NoClassDefFoundError')}")
    z2.append("bridges=" + ("есть" if re.search(r"inside_diet: defined .*InsideDietOps \+ .*InsideDietVisitor", sso) else "НЕТ"))
    z2.append("stage=" + ("composed" if re.search(r"entity_compose: stage inside_diet composed \(Retargeted \{ sites: 1 \}\)", sso) else "НЕТ"))
    z2.append("chain=" + ("ARMED" if re.search(r"entity_compose: ARMED chain \[.*inside_diet.*\]", sso) else "НЕТ"))
    z2.append("region_threads=" + ("ARMED" if "region_threads: ARMED" in sso else "НЕТ"))
    z2.append("batch_collector=" + ("defined" if "batch_collector: defined" in sso else "НЕТ"))
    z2.append("OOM=" + ("ДА" if "OutOfMemoryError" in sso else "нет"))
    pg2_ok = ("VALID" in z2[0] and z2[1] == "NCDFE=0" and "НЕТ" not in " ".join(z2[2:7])
              and z2[7] == "OOM=нет")
    rep.append("- PG-ID2: " + ", ".join(z2) + " -> **" + ("PASS" if pg2_ok else "FAIL") + "**")

    # ---- PG-ID3 TPS
    tps = tps_series(sso_p)
    med = sorted(tps[-5:])[2] if len(tps) >= 5 else (sorted(tps)[len(tps) // 2] if tps else None)
    if med is None:
        rep.append("- PG-ID3: **FAIL** — TPS-линий нет")
        pg3_ok = False
        boost = None
    else:
        boost = (med - BANK_TPS_MED5) / BANK_TPS_MED5 * 100.0
        pg3_ok = med >= 1.60
        rep.append(f"- PG-ID3: TPS lines={len(tps)} median5={med:.2f} "
                   f"(гейт ≥1.60 -> {'PASS' if pg3_ok else 'FAIL'}); дельта vs банка "
                   f"{BANK_TPS_MED5:.2f} = {boost:+.1f}% (LEVER-BOOST: "
                   f"{'GREEN ≥10%' if boost >= 10.0 else '<10% — лейн НЕ закрыт (v7)'})")

    # ---- PG-ID4a inside-alloc lane
    a_share, a_total = collapsed_share(f"{d}/alloc-collapsed.txt", INSIDE_RX)
    if a_share is None:
        rep.append("- PG-ID4a: **N/A** — alloc-collapsed.txt отсутствует")
        pg4a_ok = False
    else:
        drop = (BASE_LANE_ALLOC - a_share) / BASE_LANE_ALLOC * 100.0
        pg4a_ok = drop >= 8.0
        rep.append(f"- PG-ID4a: inside-alloc лейн {a_share:.2f}% (база {BASE_LANE_ALLOC:.2f}%, "
                   f"samples={a_total}) дроп={drop:+.1f}% (гейт ≥8% -> "
                   f"{'PASS' if pg4a_ok else 'FAIL'}, ожидание 10..25%)")

    # ---- PG-ID4b GC
    young, full = gc_stats(f"{d}/gc.log")
    if young is None:
        rep.append("- PG-ID4b: **N/A** — gc.log отсутствует")
        pg4b_ok = False
    else:
        pg4b_ok = young <= BANK_YOUNG and full == 0
        rep.append(f"- PG-ID4b: young={young} (гейт ≤{BANK_YOUNG}), Full={full} -> "
                   f"**{'PASS' if pg4b_ok else 'FAIL'}**")

    # ---- PG-ID4c inside-CPU lane
    c_share, _ = collapsed_share(f"{d}/cpu-collapsed.txt", INSIDE_RX)
    if c_share is None:
        rep.append("- PG-ID4c: **N/A** — cpu-collapsed.txt отсутствует")
        pg4c_ok = False
    else:
        drop = (BASE_LANE_CPU - c_share) / BASE_LANE_CPU * 100.0
        pg4c_ok = drop >= 3.0
        rep.append(f"- PG-ID4c: inside-CPU лейн {c_share:.2f}% (база {BASE_LANE_CPU:.2f}%) "
                   f"дроп={drop:+.1f}% (гейт ≥3% -> {'PASS' if pg4c_ok else 'FAIL'})")

    ent = len(re.findall(r"Entity threw exception", sso))
    crash_ok = ent <= 5
    rep.append(f"- CRASH-FREE: Entity threw exception={ent} (≤5) -> "
               f"**{'PASS' if crash_ok else 'FAIL'}**")

    gates = [("PG-ID2", pg2_ok), ("PG-ID3", pg3_ok), ("PG-ID4a", pg4a_ok),
             ("PG-ID4b", pg4b_ok), ("PG-ID4c", pg4c_ok), ("CRASH-FREE", crash_ok)]
    all_pass = all(v for _, v in gates)
    rep.append("- ИТОГ гейтов: " + ", ".join(f"{k}={'PASS' if v else 'FAIL'}" for k, v in gates))
    if all_pass:
        if boost is not None and boost >= 10.0:
            rep.append("- **ВЕРДИКТ: GREEN — банк CUMULATIVE v4 = v3 + inside_diet=1; "
                       "внутри-лейн ТОП-1 упал (буст ≥10%)**")
        else:
            rep.append("- **ВЕРДИКТ: гейты PASS — банкинг v4 = v3 + inside_diet=1 "
                       "(harm-floor OK); НО буст <10% → по v7 лейн НЕ закрыт: "
                       "v2 (walk-транскрипция) — следующий рычаг**")
    else:
        rep.append("- **ВЕРДИКТ: FAIL → REFUTED + rollback inside_diet=0 (мосты остаются "
                   "инфраструктурой); по v7 лейн НЕ закрыт — v2/следующий рычаг**")

    out = f"{RESDIR}/RECON14_S7174_ABSORB.md"
    with open(out, "w") as f:
        f.write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nverdict written: {out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
