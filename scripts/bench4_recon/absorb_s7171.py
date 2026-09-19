#!/usr/bin/env python3
"""absorb_s7171.py — absorb ZERO-CURSOR lever #11 v1 gate leg s7171 (TASK-331).

Run: 35441020234 @ 399e2c2 (dispatch_s7171.py, 19:46:21 +08). s7170 was the
delivery-DEFECT twin (run 35440054574): run_world3.sh line 161 literal
'BlockPos$6' inside a double-quoted echo expanded as unbound $6 under set -u,
exit 1 BEFORE the server started — fixed 5686c5b, no data from s7170.

PREREGISTER GATES (dispatch_s7171.py docstring, A/B vs fresh same-bank
profile s7169 = 35437243128 and banked v3 legs):
  PG-Z2 delivery: 0 NCDFE + pop 150k VALID
    + "[crussty-plugin] zero_cursor: pristine sighting net/minecraft/core/BlockPos"
    + "zero_cursor: defined net/minecraft/core/ZeroCursorIter + net/minecraft/core/ZeroCursorOps in kernel loader"
    + "zero_cursor: computed redirect for net/minecraft/core/BlockPos (... Retargeted { sites: 1 })"
    + "zero_cursor: net/minecraft/core/BlockPos armed, retransform rc=0"
    + "region_threads: ARMED" + "batch_collector: defined"
  PG-Z3 TPS: last-5 median >= 1.60 (bank harm floor); delta vs banked v3
    median 1.80 — the LEVER BOOST verdict comes from this delta.
  PG-Z4a cursor-alloc lane: needle family (BlockPos$6 | betweenCornersInDirection |
    forEachBlockIntersectedBetween) share of alloc-collapsed samples >= -50%
    vs s7169 base 29.18% (expect -80..-95%; LongSet/Vec3 remain in the lane)
  PG-Z4b young GC <= 154 (bank v3); 0 Full GC
  PG-Z4c cursor-CPU lane: 6.42% baseline, gate >= -30% (report only family)
  CRASH-FREE: "Entity threw exception" <= 5/run
Banking: full PASS -> CUMULATIVE v4 = v3 + zero_cursor=1 (boost verdict
printed separately per v7 "ТОП-1 ОБЯЗАН УПАСТЬ": буст >=10% = лейн-под-лейн
упал; <10% = банчить можно, лейн НЕ закрыт);
         FAIL -> REFUTED + rollback zero_cursor=0 (bridges remain infra).

Exit: 0 absorbed (verdict printed), 3 in flight, 4 run failure, 2 technical.
"""
import json, os, re, subprocess, sys, urllib.request
from collections import Counter

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
DEFAULT_RUN = "35444005075"
EXPECTED_SHA = "27ef945"

BASE_CURSOR_ALLOC = 29.18   # s7169 alloc-collapsed needle share, %
BASE_CURSOR_CPU = 6.42      # s7169 cpu-collapsed needle share, %
BANK_TPS_MED5 = 1.80        # banked CUMULATIVE v3 median5
BANK_YOUNG = 154            # bank v3 young GC count
CURSOR_RX = re.compile(r"BlockPos\$6|betweenCornersInDirection|forEachBlockIntersectedBetween")
ZCURSOR_RX = re.compile(r"ZeroCursorIter|ZeroCursorOps")


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
    """Needle-family share of a collapsed profile file (frames<TAB>count)."""
    agg = Counter()
    total = 0
    if not os.path.isfile(path):
        return None, 0, agg
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
                agg[p[0].split(";")[-1]] += n
    share = (sum(agg.values()) / total * 100.0) if total else None
    return share, total, agg


def tps_series(path):
    out = []
    for line in open(path, errors="ignore"):
        m = re.search(r"TPS from last 5s, 1m, 5m, 15m: ([\d.]+)", line)
        if m and float(m.group(1)) < 20:  # exclude pre-inject ~21.9 line
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
    dest = f"{RESDIR}/run-s7171-zero-cursor"
    subprocess.run([sys.executable, "/home/z/c-crussty/scripts/bench4_recon/fetch_artifact.py",
                    run_id, dest], check=False)
    d = f"{dest}/world3-bench" if os.path.isdir(f"{dest}/world3-bench") else dest

    rep = [f"# RECON-14 — absorb ZERO-CURSOR gate лега s7171 (run {run_id}, head {sha})\n"]
    if not sha.startswith(EXPECTED_SHA):
        rep.append(f"- ВНИМАНИЕ: head {sha} != ожидаемого {EXPECTED_SHA} (чужой head — учесть)")

    sso_p = f"{d}/server-stdout.log"
    if not os.path.isfile(sso_p):
        rep.append("- **FAIL**: server-stdout.log отсутствует — артефакт пуст "
                   "(повтор дефекта доставки?)")
        print("\n".join(rep))
        return 4
    sso = open(sso_p, errors="ignore").read()

    # ---- PG-Z2 delivery
    z2 = []
    z2.append("pop " + ("VALID" if "FIXTURE-VALIDITY: VALID" in sso else "INVALID"))
    z2.append(f"NCDFE={sso.count('NoClassDefFoundError')}")
    z2.append("pristine=" + ("есть" if "zero_cursor: pristine sighting net/minecraft/core/BlockPos" in sso else "НЕТ"))
    z2.append("bridges=" + ("есть" if re.search(r"zero_cursor: defined .*ZeroCursorIter \+ .*ZeroCursorOps", sso) else "НЕТ"))
    redir = re.search(r"zero_cursor: computed redirect .*BlockPos[^\n]*Retargeted \{ sites: 1 \}", sso)
    z2.append("redirect=" + ("Retargeted{sites:1}" if redir else ("НЕТ" if "zero_cursor: computed redirect" in sso else "маркеров нет")))
    armed = re.search(r"zero_cursor: net/minecraft/core/BlockPos armed, retransform rc=0", sso)
    z2.append("armed=" + ("rc=0" if armed else "НЕТ"))
    z2.append("region_threads=" + ("ARMED" if "region_threads: ARMED" in sso else "НЕТ"))
    z2.append("batch_collector=" + ("defined" if "batch_collector: defined" in sso else "НЕТ"))
    z2.append("OOM=" + ("ДА" if "OutOfMemoryError" in sso else "нет"))
    pgz2_ok = ("VALID" in z2[0] and z2[1] == "NCDFE=0" and "НЕТ" not in " ".join(z2[2:7])
               and z2[8] == "OOM=нет")
    rep.append("- PG-Z2: " + ", ".join(z2) + " -> **" + ("PASS" if pgz2_ok else "FAIL") + "**")

    # ---- PG-Z3 TPS
    tps = tps_series(sso_p)
    if len(tps) >= 5:
        med = sorted(tps[-5:])[2]
    elif tps:
        med = sorted(tps)[len(tps) // 2]
    else:
        med = None
    if med is None:
        rep.append("- PG-Z3: **FAIL** — TPS-линий нет в stdout")
        pgz3_ok = False
        boost = None
    else:
        boost = (med - BANK_TPS_MED5) / BANK_TPS_MED5 * 100.0
        pgz3_ok = med >= 1.60
        rep.append(f"- PG-Z3: TPS lines={len(tps)} median5={med:.2f} "
                   f"(гейт ≥1.60 -> {'PASS' if pgz3_ok else 'FAIL'}); "
                   f"дельта vs банка {BANK_TPS_MED5:.2f} = {boost:+.1f}% "
                   f"(LEVER-BOOST: {'GREEN ≥10%' if boost >= 10.0 else '<10% — лейн НЕ закрыт (v7)'})")

    # ---- PG-Z4a cursor-alloc lane
    a_share, a_total, a_leaf = collapsed_share(f"{d}/alloc-collapsed.txt", CURSOR_RX)
    if a_share is None:
        rep.append("- PG-Z4a: **N/A** — alloc-collapsed.txt отсутствует")
        pgz4a_ok = False
    else:
        drop = (BASE_CURSOR_ALLOC - a_share) / BASE_CURSOR_ALLOC * 100.0
        pgz4a_ok = drop >= 50.0
        top = "; ".join(f"{k.rsplit('/',1)[-1]}={v}" for k, v in a_leaf.most_common(5))
        rep.append(f"- PG-Z4a: cursor-alloc лейн {a_share:.2f}% (база {BASE_CURSOR_ALLOC:.2f}%, "
                   f"samples={a_total}) дроп={drop:+.1f}% (гейт ≥−50% -> "
                   f"{'PASS' if pgz4a_ok else 'FAIL'}, ожидание −80..−95%); топ: {top or '—'}")

    # ---- PG-Z4b GC
    young, full = gc_stats(f"{d}/gc.log")
    if young is None:
        rep.append("- PG-Z4b: **N/A** — gc.log отсутствует")
        pgz4b_ok = False
    else:
        pgz4b_ok = young <= BANK_YOUNG and full == 0
        rep.append(f"- PG-Z4b: young={young} (гейт ≤{BANK_YOUNG}), Full={full} -> "
                   f"**{'PASS' if pgz4b_ok else 'FAIL'}**")

    # ---- PG-Z4c cursor-CPU lane
    c_share, c_total, c_leaf = collapsed_share(f"{d}/cpu-collapsed.txt", CURSOR_RX)
    z_share, _, _ = collapsed_share(f"{d}/cpu-collapsed.txt", ZCURSOR_RX)
    if c_share is None:
        rep.append("- PG-Z4c: **N/A** — cpu-collapsed.txt отсутствует")
        pgz4c_ok = False
    else:
        drop = (BASE_CURSOR_CPU - c_share) / BASE_CURSOR_CPU * 100.0
        pgz4c_ok = drop >= 30.0
        rep.append(f"- PG-Z4c: cursor-CPU лейн {c_share:.2f}% (база {BASE_CURSOR_CPU:.2f}%) "
                   f"дроп={drop:+.1f}% (гейт ≥−30% -> {'PASS' if pgz4c_ok else 'FAIL'}); "
                   f"ZeroCursor-фреймы CPU: {z_share if z_share is not None else 0:.2f}%")

    # ---- CRASH-FREE
    ent = len(re.findall(r"Entity threw exception", sso))
    crash_ok = ent <= 5
    rep.append(f"- CRASH-FREE: Entity threw exception={ent} (≤5) -> "
               f"**{'PASS' if crash_ok else 'FAIL'}**")

    gates = [("PG-Z2", pgz2_ok), ("PG-Z3", pgz3_ok), ("PG-Z4a", pgz4a_ok),
             ("PG-Z4b", pgz4b_ok), ("PG-Z4c", pgz4c_ok), ("CRASH-FREE", crash_ok)]
    all_pass = all(v for _, v in gates)
    rep.append("- ИТОГ гейтов: " + ", ".join(f"{k}={'PASS' if v else 'FAIL'}" for k, v in gates))
    if all_pass:
        if boost is not None and boost >= 10.0:
            rep.append("- **ВЕРДИКТ: GREEN — банк CUMULATIVE v4 = v3 + zero_cursor=1; "
                       "cursor-под-лейн ТОП-1 упал (буст ≥10%); свежий ТОП → следующий круг**")
        else:
            rep.append("- **ВЕРДИКТ: гейты PASS — банкинг v4 = v3 + zero_cursor=1 "
                       "(harm-floor OK); НО буст <10% → по v7 лейн НЕ закрыт: "
                       "свежий RECON cursor-лейна / следующий под-лейн ТОП-1**")
    else:
        rep.append("- **ВЕРДИКТ: FAIL → REFUTED + rollback zero_cursor=0 (мосты остаются "
                   "инфраструктурой); следующий под-лейн ТОП-1 (Vec3/AABB travel-чейн "
                   "или remset-драйвер)**")

    out = f"{RESDIR}/RECON14_S7173_ABSORB.md"
    with open(out, "w") as f:
        f.write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nverdict written: {out}")
    return 0 if all_pass or cc == "success" else 4


if __name__ == "__main__":
    sys.exit(main())
