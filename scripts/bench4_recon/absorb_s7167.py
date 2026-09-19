#!/usr/bin/env python3
"""absorb_s7167.py — one-command absorb of the FREE-HOST 2.5GB RECON leg
(TASK-321 preregister; run 35430216073; S7-167).

Usage: python3 scripts/bench4_recon/absorb_s7167.py [run_id]
  run_id defaults to 35430216073.

Exit codes:
  0  absorbed -> verdict doc written (research/gc-recon-2026-09-19/RECON13_FREEHOST_ABSORB.md)
  3  leg still in flight -> caller must NOT absorb yet (tick charter 4c)
  4  leg finished with failure -> absorb crash-evidence (still writes doc)
  2  technical error

Preregister TASK-321 (RECON leg, NOT a banking gate):
  PG-A delivery: pop 150k VALID + ARMED [inside->rng->batch] (no sbb) +
    0 NCDFE + run-env echoes server_xms/server_xmx = 2G/2G
  Honest window (a): OOM / GC death spiral -> INFEASIBLE-BY-MEMORY
    (scene 150k cannot fit the 2.5GB container class; gc.log evidence:
    phase of death, Full-GC thrash, allocation-fail rate). VALID finding.
  Honest window (b): run completes -> fresh TOP re-sort under low-heap:
    young-GC count/duty vs 10G diag base 35425246662, card-set CPU lane,
    alloc family shares (AABB/Vec3/other-entity), TPS series
    -> next TOP-1 target under FREE-HOST profile.
"""
import json, re, subprocess, sys, statistics, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
DEFAULT_RUN = "35430216073"
BASE_10G = "35425246662"  # 10G diag base for cross-heap reference (NOT absolute A/B)
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"


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


def collapsed_total(path):
    t = 0
    for line in open(path, errors="ignore"):
        p = line.rsplit(" ", 1)
        if len(p) == 2:
            try:
                t += int(p[1])
            except ValueError:
                pass
    return t


def collapsed_match(path, rx):
    rx = re.compile(rx)
    tot = s = 0
    for line in open(path, errors="ignore"):
        p = line.rsplit(" ", 1)
        if len(p) == 2:
            try:
                n = int(p[1])
            except ValueError:
                continue
            tot += n
            if rx.search(p[0]):
                s += n
    return s, tot


def gc_stats(path):
    """young count, full count, total pause seconds, max single pause.
    Unified log: completion lines end with the duration ('4.707ms' / '1.234s');
    '[gc,start' marker lines have NO duration and must NOT be counted."""
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


def remset_p50(path):
    dirty = []
    for line in open(path, errors="ignore"):
        m = re.search(r"Total dirty (\d+) \(", line)
        if m:
            dirty.append(int(m.group(1)))
    return (statistics.median(dirty), len(dirty), max(dirty)) if dirty else (0, 0, 0)


def tps_series(path):
    out = []
    for line in open(path, errors="ignore"):
        m = re.search(r"TPS from last 5s, 1m, 5m, 15m: ([\d.]+)", line)
        if m and float(m.group(1)) < 20:  # exclude pre-inject ~21.9 line
            out.append(float(m.group(1)))
    return out


def oom_evidence(d):
    ev = []
    for fname, pats in (
        ("server-stdout.log", [r"OutOfMemoryError: (\w+ ?\w*)"]),
        ("gc.log", [r"OutOfMemoryError", r"Full GC .*Allocation Failure"]),
    ):
        try:
            txt = open(f"{d}/{fname}", errors="ignore").read()
        except FileNotFoundError:
            continue
        for pat in pats:
            hits = re.findall(pat, txt)
            if hits:
                from collections import Counter
                ev.append((fname, pat, Counter(hits).most_common(3)))
    return ev


def main():
    run_id = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_RUN
    tok = token()
    st, cc, sha = run_status(tok, run_id)
    print(f"run {run_id}: status={st} conclusion={cc} head={sha}")
    if st in ("in_progress", "queued", "waiting"):
        return 3
    dest = f"{RESDIR}/run-s7167-freehost"
    subprocess.run([sys.executable, "/home/z/c-crussty/scripts/bench4_recon/fetch_artifact.py",
                    run_id, dest], check=False)
    d = f"{dest}/world3-bench" if __import__("os").path.isdir(f"{dest}/world3-bench") else dest

    rep = [f"# RECON-13 — absorb FREE-HOST 2.5GB лега s7167 (run {run_id}, head {sha})\n"]
    env = open(f"{d}/run-env.txt", errors="ignore").read()
    xms = re.search(r"server_xms: (\S+)", env)
    xmx = re.search(r"server_xmx: (\S+)", env)
    rep.append(f"- heap: xms={xms.group(1) if xms else '?'} xmx={xmx.group(1) if xmx else '?'} "
               f"(пегистер: оба 2G)")
    pgA = []
    pgA.append("pop " + ("VALID" if "FIXTURE-VALIDITY: VALID" in open(f"{d}/server-stdout.log", errors="ignore").read() else "INVALID"))
    sso = open(f"{d}/server-stdout.log", errors="ignore").read()
    pgA.append("ARMED no-sbb: " + ("да" if re.search(r"ARMED chain \[inside->rng->batch\]", sso) else "НЕТ"))
    pgA.append(f"NCDFE: {sso.count('NoClassDefFoundError')}")
    rep.append(f"- PG-A доставка: {'; '.join(pgA)}")

    oom = oom_evidence(d)
    if cc == "failure" or oom:
        rep.append(f"- ВЕРДИКТ: **INFEASIBLE-BY-MEMORY** (окно (a)) — conclusion={cc}, OOM-маркеры: {oom}")
        rep.append("- Следствие: сцена 150k не помещается в класс контейнера 2.5GB; варианты владельцу: "
                   "сниженная популяция free-host трека ИЛИ 10G-класс хостинга. FREE-HOST A/B база = недоступна.")
    else:
        young, full, tot_s, mx_ms = gc_stats(f"{d}/gc.log")
        b_young, b_full, b_tot, b_mx = gc_stats(f"{RESDIR}/run-s7165-recon-diag/gc.log")
        cs, ct = collapsed_match(f"{d}/cpu-collapsed.txt",
                                 r"G1CardSet|G1RemSet|G1ScanCardClosure|refine_card_concurrently|G1ConcurrentRefine|G1UpdateBuffer|G1DirtyCardQueue|G1HotCardCache")
        aabb, at = collapsed_match(f"{d}/alloc-collapsed.txt", r"AABB")
        v3, vt = collapsed_match(f"{d}/alloc-collapsed.txt", r"Vec3")
        oe, ot = collapsed_match(f"{d}/alloc-collapsed.txt",
                                 r"LazyEntityCollisionContext|Entity\$+Lambda|PathFinder|GoalSelector|Brain")
        rp = remset_p50(f"{d}/remset.log")
        tps = tps_series(f"{d}/server-stdout.log")
        med = statistics.median(tps[-5:]) if len(tps) >= 5 else (tps[-1] if tps else 0)
        rep += [
            f"- ВЕРДИКТ: ран ЗАВЕРШЁН (окно (b)) — свежий ТОП под low-heap:",
            f"- GC: young={young} full={full} суммарно={tot_s:.1f}s max={mx_ms:.0f}ms "
            f"(10G-база: young={b_young} full={b_full} {b_tot:.1f}s max={b_mx:.0f}ms)",
            f"- card-set CPU-лейн: {cs:,}/{ct:,} = {100*cs/ct:.2f}% (10G-база 25.72%)",
            f"- аллок-семьи: AABB {100*aabb/at:.2f}% | Vec3 {100*v3/vt:.2f}% | other-entity {100*oe/ot:.2f}%",
            f"- remset dirty p50={rp[0]:,} циклов={rp[1]} (10G-база 6,324,224/165 — кросс-heap, не абсолют A/B)",
            f"- TPS last-5 медиана={med} (серия: {tps})",
            f"- СЛЕДУЮЩИЙ ТОП-1 (FREE-HOST профиль): максимум из GC-дuty и card-set lane → цель атаки",
        ]
    out = f"{RESDIR}/RECON13_FREEHOST_ABSORB.md"
    open(out, "w").write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {out}")
    return 0 if cc == "success" else 4


if __name__ == "__main__":
    sys.exit(main())
