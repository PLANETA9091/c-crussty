#!/usr/bin/env python3
"""absorb_s7168.py — one-command absorb of the RECON-13d chunk-parse census leg
s7168 (TASK-327 preregister; run 35435848509; CRUSSTY_PARSE_DIAG).

Usage:
  python3 scripts/bench4_recon/absorb_s7168.py [run_id]   # run_id defaults 35435848509
  python3 scripts/bench4_recon/absorb_s7168.py --selftest # offline parser/verdict check

Exit codes:
  0  absorbed -> verdict doc written (research/gc-recon-2026-09-19/RECON13_PARSE_DIAG_ABSORB.md)
  3  leg still in flight -> caller must NOT absorb yet (tick charter 4c)
  4  leg finished with failure / delivery-gate FAIL -> absorb evidence (still writes doc)
  2  technical error

Preregister TASK-327 (diag leg, pure observability, bank v3 + parse_diag=1):
  PG-D0 census: chunk-parse-diag.txt present, SUMMARY non-empty, total_loads>0
  PG-D1 delivery: FIXTURE-VALIDITY VALID + NCDFE=0 + no OOM + parse_diag patch
    marker (Retargeted/AlreadyPatched) + no "hook stays dormant"/"patch rejected"
  PG-D2 TPS: last-5 median >= 2.0 (not worse than diag base s7165 median5=2.0)
  PG-D3 remset: dirty p50 within +/-1% of base s7165 6,324,224
  Verdict rule (pre-generated, lever #12 decode-cache by revision):
    repeat_share >= 30% -> GO decode-cache (parity-gated implementation next)
    repeat_share < 10%  -> cache REFUTED (lane = ticket-churn; next TOP-1 = TOP-2
                           entity-tick-core 21.39% CPU)
    10..30%             -> extended RECON-13e (decompose reload callers)
Census file format (ChunkParseDiagOps.dumpToFile):
  SUMMARY total_loads=N unique_chunks=N repeat_loads=N repeat_share_pct=F span_ms=N
  chunk <x:int> <z:int> <count>            (sorted by count desc, top 20000)
"""
import json, re, statistics, subprocess, sys, os, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
DEFAULT_RUN = "35435848509"
EXPECTED_SHA = "1fc46c3"
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
BASE_10G = f"{RESDIR}/run-s7165-recon-diag"   # s7165: TPS median5=2.0, remset p50=6,324,224
BASE_REMSET_P50 = 6324224
BASE_TPS_MED5 = 2.0


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    return re.match(r"^https://[^:]+:([^@]+)@github\.com/", url).group(1)


def api(tok, path):
    req = urllib.request.Request(f"{API}{path}", headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.loads(r.read() or b"{}")


def run_status(tok, run_id):
    r = api(tok, f"/repos/{REPO}/actions/runs/{run_id}")
    return r.get("status"), r.get("conclusion"), r.get("head_sha", "")[:7]


# ---------------------------------------------------------------- census parse
def parse_census(path):
    """-> dict or None. Parses SUMMARY + chunk rows, bucket histogram, top-10,
    consistency cross-check (rows must sum to total unless top-20000 cap hit)."""
    try:
        txt = open(path, errors="ignore").read()
    except FileNotFoundError:
        return None
    m = re.search(r"^SUMMARY total_loads=(\d+) unique_chunks=(\d+) repeat_loads=(\d+) "
                  r"repeat_share_pct=([\d.]+) span_ms=(\d+)$", txt, re.M)
    if not m:
        return None
    rows = []  # (x, z, count)
    for line in txt.splitlines():
        p = line.split(" ")
        if len(p) == 4 and p[0] == "chunk":
            try:
                rows.append((int(p[1]), int(p[2]), int(p[3])))
            except ValueError:
                pass
    buckets = {"1": 0, "2": 0, "3-5": 0, "6-10": 0, "11-50": 0, ">50": 0}
    for _, _, c in rows:
        k = "1" if c == 1 else "2" if c == 2 else "3-5" if c <= 5 else \
            "6-10" if c <= 10 else "11-50" if c <= 50 else ">50"
        buckets[k] += 1
    row_sum = sum(c for _, _, c in rows)
    row_reps = sum(c - 1 for _, _, c in rows if c > 1)
    return {
        "total_loads": int(m.group(1)), "unique_chunks": int(m.group(2)),
        "repeat_loads": int(m.group(3)), "repeat_share_pct": float(m.group(4)),
        "span_ms": int(m.group(5)),
        "rows": len(rows), "row_sum": row_sum, "row_reloads": row_reps,
        "rows_capped": len(rows) >= 20000,
        "dist": buckets,
        "top": sorted(rows, key=lambda r: -r[2])[:10],
        "consistent": (row_sum == int(m.group(1))) or (len(rows) >= 20000 and row_sum <= int(m.group(1))),
    }


def verdict_rule(share):
    if share >= 30.0:
        return ("GO lever #12: decode-cache по ревизии",
                f"repeat_share {share:.1f}% >= 30% -> реализация decode-cache в следующем тике "
                "ДО КОНЦА (парити-гейты: median-exact, remset/TPS не хуже базы)")
    if share < 10.0:
        return ("REFUTED: кэш декодированных chunk-data",
                f"repeat_share {share:.1f}% < 10% -> лейн = first-loads/ticket-churn, кэш не окупается; "
                "следующий ТОП-1 = entity-tick-core 21.39% CPU (travel-physics AABB/Vec3)")
    return ("Расширенный RECON-13e", f"repeat_share {share:.1f}% в серой зоне 10-30% -> "
            "декомпозиция вызывателей reload перед выбором рычага")


# ------------------------------------------------------------- CI-log helpers
def tps_series(path):
    out = []
    for line in open(path, errors="ignore"):
        m = re.search(r"TPS from last 5s, 1m, 5m, 15m: ([\d.]+)", line)
        if m and float(m.group(1)) < 20:  # exclude pre-inject ~21.9 line
            out.append(float(m.group(1)))
    return out


def remset_p50(path):
    dirty = []
    for line in open(path, errors="ignore"):
        m = re.search(r"Total dirty (\d+) \(", line)
        if m:
            dirty.append(int(m.group(1)))
    return (statistics.median(dirty), len(dirty), max(dirty)) if dirty else (0, 0, 0)


def gc_stats(path):
    young = full = 0
    tot = mx = 0.0
    for line in open(path, errors="ignore"):
        m = re.search(r"Pause (Young|Full).*?([\d.]+)(ms|s)\s*$", line)
        if not m or "[gc,start" in line:
            continue
        ms = float(m.group(2)) * (1 if m.group(3) == "ms" else 1000)
        tot += ms
        mx = max(mx, ms)
        if m.group(1) == "Young":
            young += 1
        else:
            full += 1
    return young, full, tot / 1000.0, mx


# -------------------------------------------------------------------- selftest
def selftest():
    import tempfile
    # fixture: 10 chunks; counts 100,10,5,3,3,2,1,1,1,1 -> total=127 uniq=10 reps=117
    lines = ["SUMMARY total_loads=127 unique_chunks=10 repeat_loads=117 "
             "repeat_share_pct=92.1 span_ms=299000"]
    counts = [100, 10, 5, 3, 3, 2, 1, 1, 1, 1]
    for i, c in enumerate(counts):
        lines.append(f"chunk {i} {-i} {c}")
    d = tempfile.mkdtemp()
    p = f"{d}/chunk-parse-diag.txt"
    open(p, "w").write("\n".join(lines) + "\n")
    c = parse_census(p)
    assert c and c["total_loads"] == 127 and c["unique_chunks"] == 10, c
    assert c["repeat_share_pct"] == 92.1 and c["span_ms"] == 299000
    assert c["rows"] == 10 and c["row_sum"] == 127 and c["row_reloads"] == 117, c
    assert c["consistent"] is True, c
    assert c["dist"] == {"1": 4, "2": 1, "3-5": 3, "6-10": 1, "11-50": 0, ">50": 1}, c["dist"]
    assert c["top"][0] == (0, 0, 100), c["top"]
    # verdict branches
    assert verdict_rule(60.0)[0].startswith("GO"), verdict_rule(60.0)
    assert verdict_rule(5.0)[0].startswith("REFUTED"), verdict_rule(5.0)
    assert verdict_rule(15.0)[0].startswith("Расширенный"), verdict_rule(15.0)
    # границы по прегистеру: <10 REFUTED (строго), 10-30 серая, >=30 GO
    assert verdict_rule(30.0)[0].startswith("GO") and verdict_rule(9.99)[0].startswith("REFUTED")
    assert verdict_rule(10.0)[0].startswith("Расширенный") and verdict_rule(29.99)[0].startswith("Расширенный")
    # missing file -> None (PG-D0 FAIL path)
    assert parse_census("/nonexistent/x.txt") is None
    # truncated/garbage -> None
    open(f"{d}/bad.txt", "w").write("no summary here\n")
    assert parse_census(f"{d}/bad.txt") is None
    print("selftest PASS: parser + buckets + consistency + verdict branches")
    return 0


# ----------------------------------------------------------------------- main
def main():
    run_id = sys.argv[1] if len(sys.argv) > 1 and sys.argv[1].isdigit() else DEFAULT_RUN
    tok = token()
    st, cc, sha = run_status(tok, run_id)
    print(f"run {run_id}: status={st} conclusion={cc} head={sha}")
    if st in ("in_progress", "queued", "waiting"):
        return 3
    dest = f"{RESDIR}/run-s7168-parse-diag"
    subprocess.run([sys.executable, "/home/z/c-crussty/scripts/bench4_recon/fetch_artifact.py",
                    run_id, dest], check=False)
    d = f"{dest}/world3-bench" if os.path.isdir(f"{dest}/world3-bench") else dest

    rep = [f"# RECON-13d — absorb переписи chunk-parse лега s7168 (run {run_id}, head {sha})\n"]
    if not sha.startswith(EXPECTED_SHA):
        rep.append(f"- ВНИМАНИЕ: head {sha} != ожидаемого {EXPECTED_SHA} (чужой head — учесть)")

    # ---- PG-D0 census
    c = parse_census(f"{d}/chunk-parse-diag.txt")
    if c is None:
        rep.append("- PG-D0: **FAIL** — chunk-parse-diag.txt отсутствует/без SUMMARY "
                   "(доставка диага дефектна; вердикт по кэшу НЕВОЗМОЖЕН)")
        share = None
    else:
        span_s = c["span_ms"] / 1000.0
        rate = c["total_loads"] / span_s if span_s > 0 else 0
        rep += [
            f"- PG-D0: census OK — total_loads={c['total_loads']:,} unique={c['unique_chunks']:,} "
            f"repeats={c['repeat_loads']:,} repeat_share={c['repeat_share_pct']}% "
            f"span={span_s:.0f}s rate={rate:.1f}/s",
            f"- консистентность: rows={c['rows']:,} row_sum={c['row_sum']:,} "
            f"(capped={c['rows_capped']}) -> {'PASS' if c['consistent'] else 'MISMATCH'}",
            f"- гистограмма нагрузок на чанк: {c['dist']}",
            "- топ-10 перезагружаемых чанков (x z count): " + "; ".join(
                f"({x},{z})={n}" for x, z, n in c["top"]),
        ]
        share = c["repeat_share_pct"]

    # ---- PG-D1 delivery
    sso = open(f"{d}/server-stdout.log", errors="ignore").read()
    stderr_txt = ""
    for cand in ("server-stderr.log", "stderr.log"):
        p = f"{d}/{cand}"
        if os.path.isfile(p):
            stderr_txt += open(p, errors="ignore").read()
    pgd1 = []
    pgd1.append("pop " + ("VALID" if "FIXTURE-VALIDITY: VALID" in sso else "INVALID"))
    pgd1.append(f"NCDFE={sso.count('NoClassDefFoundError')}")
    pgd1.append("OOM=" + ("ДА" if "OutOfMemoryError" in sso else "нет"))
    patch_log = sso + stderr_txt
    if re.search(r"parse_diag: computed patch for \S+ \(.+(Retargeted|AlreadyPatched)", patch_log):
        pgd1.append("parse_diag patch=ARMED")
    elif "parse_diag" in patch_log:
        pgd1.append("parse_diag patch=СБОЙ (см. маркеры dormant/rejected)")
    else:
        pgd1.append("parse_diag маркеров не найдено (проверить лог-канал)")
    pgd1_ok = ("VALID" in pgd1[0] and "NCDFE=0" in pgd1[1] and pgd1[2].endswith("нет")
               and "ARMED" in pgd1[3])
    rep.append(f"- PG-D1 доставка: {'; '.join(pgd1)} -> {'PASS' if pgd1_ok else 'FAIL'}")

    # ---- PG-D2 TPS
    tps = tps_series(f"{d}/server-stdout.log")
    med = statistics.median(tps[-5:]) if len(tps) >= 5 else (tps[-1] if tps else 0)
    pgd2_ok = med >= BASE_TPS_MED5
    rep.append(f"- PG-D2: TPS last-5 медиана={med} (база s7165={BASE_TPS_MED5}, "
               f"серия n={len(tps)}: {tps}) -> {'PASS' if pgd2_ok else 'FAIL'}")

    # ---- PG-D3 remset (N/A-tolerant: remset-диаг мог быть не включён в диспатч)
    rp_path = f"{d}/remset.log"
    if os.path.isfile(rp_path):
        rp = remset_p50(rp_path)
        dev = (abs(rp[0] - BASE_REMSET_P50) / BASE_REMSET_P50 * 100) if rp[0] else float("inf")
        pgd3_ok = dev <= 1.0
        rep.append(f"- PG-D3: remset dirty p50={rp[0]:,.0f} циклов={rp[1]} max={rp[2]:,} "
                   f"(база s7165={BASE_REMSET_P50:,}, отклонение={dev:.2f}%) "
                   f"-> {'PASS' if pgd3_ok else 'FAIL'}")
    else:
        pgd3_ok = None
        rep.append("- PG-D3: N/A — remset.log в артефакте отсутствует (remset-диаг не включён "
                   "в диспатч s7168; дефект пегистера TASK-327, не сервера)")

    # ---- verdict
    gates = [pgd1_ok, pgd2_ok, pgd3_ok] + ([True] if c is not None else [False])
    if share is None:
        rep.append("- ВЕРДИКТ: **ДЕЛИВЕРИ-ДЕФЕКТ** — перепись не доставлена; чинить доставку "
                   "(CRUSSTY_PARSE_DIAG_FILE/upload), повторный диаг-лег; кэш-решение отложено")
    else:
        v, why = verdict_rule(share)
        rep.append(f"- ВЕРДИКТ: **{v}** — {why}")
        rep.append(f"- Гейты доставки: PG-D1={pgd1_ok} PG-D2={pgd2_ok} PG-D3={pgd3_ok} "
                   f"-> {'ЧИСТО' if all(gates) else 'ЕСТЬ ПРОБЛЕМЫ (см. выше)'}")
    young, full, tot_s, mx_ms = gc_stats(f"{d}/gc.log")
    rep.append(f"- GC-фон: young={young} full={full} суммарно={tot_s:.1f}s max={mx_ms:.0f}ms "
               f"(база s7165: young=161 full=0 20.3s max=189ms)")

    out = f"{RESDIR}/RECON13_PARSE_DIAG_ABSORB.md"
    open(out, "w").write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {out}")
    return 0 if cc == "success" else 4


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--selftest":
        sys.exit(selftest())
    try:
        sys.exit(main())
    except FileNotFoundError as e:
        print(f"technical error: {e}", file=sys.stderr)
        sys.exit(2)
