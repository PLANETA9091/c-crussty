#!/usr/bin/env python3
"""dispatch_479_v3_wave.py — COMMANDER 479-V3 MEGA-SWARM v19.0 (тик ×479): ЯКОРНАЯ ВОЛНА — решётка якорей под все активные пары.

CLAIM: МАСС-ВОЛНА диспатчей (12c) — 10 ваниль-диспатчей @master 0 код-дельт
(алиасы round-479-v3-y01..y10, canon x466-C98 ЯВНЫМ JSON, урок C66-C73),
мишени cpu: 6.45M/6.5M/6.55M/6.9M/7.0M/7.1M/7.2M/8.8M/8.9M/9.0M — банк-фиды §3
(канон Л-470-S31.1: фид полным band, пост-хок idx-фильтр по run-env.txt) +
решётка для будущих pair-fresh (Δ≤50k). band GLOB [6000000,9500000] fast-fail
(канон A7/Y1/Y3) — band-miss → 1 free ре-ролл (закон W3, Л188c); ре-роллы free
(fast-fail пре-download). Финал: {run-id ×10+, DISPATCHED} (закон 18-iii), бенчи
НЕ ждать (12e). Pin FULL-sha origin/master e503160c (CLAIM-база == pin, 0 код-
дельт по построению; прегист scripts-only закон 14a/16 — этот файл, дрейф
PIN→live-мастер стражится hot-surface-guard'ом). Пороги НЕ двигать, закон-5
запреты. LEDGER Л-479-V3, board [479-V3].
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "e503160cdf7a4aef55ac282d13f719d4ca05500a"  # CLAIM-база == origin/master FULL sha
EVID = "/home/z/rounds/ROUND-479/V3"

# решётка мишеней (cpu(run-env) зоны для pair-fresh Δ≤50k, банк-фид §3)
TARGETS = [
    ("round-479-v3-y01", 6450000), ("round-479-v3-y02", 6500000),
    ("round-479-v3-y03", 6550000), ("round-479-v3-y04", 6900000),
    ("round-479-v3-y05", 7000000), ("round-479-v3-y06", 7100000),
    ("round-479-v3-y07", 7200000), ("round-479-v3-y08", 8800000),
    ("round-479-v3-y09", 8900000), ("round-479-v3-y10", 9000000),
]
ALIASES = [a for a, _ in TARGETS]
TMAP = dict(TARGETS)

# canon x466-C98 — ЯВНЫЙ JSON, yml-дефолты = merge-поверхность (урок C73);
# band GLOB [6000000,9500000] fast-fail (канон A7/Y1/Y3); lever_flag/arg ПУСТО
# (ваниль-руны, 0 код-дельт, master default = vanilla)
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def drift_guard(tok):
    """0-дельт-страж: PIN→live-мастер не должен трогать bench-поверхности
    (прегист scripts/board-only поверх CLAIM-пина — закон 14a/16 fixup-канон Y1/F2)."""
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    print(f"origin/master live = {live}")
    if live.startswith(PIN[:12]):
        print("live == PIN: 0 дельт по построению")
        return
    stat = subprocess.run(
        ["git", "-C", "/home/z/c-crussty", "diff", "--name-only", PIN[:12], live[:12]],
        capture_output=True, text=True).stdout
    hot = [ln for ln in stat.splitlines()
           if ("src/" in ln or ".github/" in ln or "Cargo" in ln or "pom" in ln
               or "native/" in ln)]
    if hot:
        print("DRIFT-HOT files (bench surface touched!):", *hot, sep="\n", file=sys.stderr)
        sys.exit(3)
    print(f"drift {PIN[:8]}->{live[:8]}: scripts/board-only OK "
          f"({len(stat.strip().splitlines()) if stat.strip() else 0} files)")


def ensure_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    cur = r.get("object", {}).get("sha")
    if cur == sha:
        print(f"{name}: GET-proof exists @ {sha[:8]} (Л188a)")
        return
    if cur:
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        print(f"{name}: moved -> {sha[:8]}")
        return
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})  # FULL sha (S20: short=422)
    v = api(tok, f"/repos/{REPO}/git/ref/heads/{name}").get("object", {}).get("sha")
    assert v == sha, f"alias verify failed: {v} != {sha}"
    print(f"{name}: created FULL-sha @ {sha[:8]}, GET-verified (Л188a)")


def dispatch(tok, ref, tag=""):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": INPUTS})
    ok = r == {}
    print(f"dispatch {ref} {tag}-> {'204 OK' if ok else r}")
    return ok


def collect_runs(tok, min_created):
    """Все волночные руны по алиасам, отсортированы по created (attempt-1/2...)."""
    res = {a: [] for a in ALIASES}
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=100")
    for run in runs.get("workflow_runs", []):
        hb = run["head_branch"]
        if hb in res and run["created_at"] > min_created:
            res[hb].append({"id": run["id"], "status": run["status"],
                            "conclusion": run["conclusion"], "sha": run["head_sha"][:8],
                            "created": run["created_at"]})
    for a in res:
        res[a].sort(key=lambda x: x["created"])
    return res


def main():
    tok = token()
    drift_guard(tok)
    for a in ALIASES:
        ensure_alias(tok, a, PIN)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    n_ok = 0
    for a in ALIASES:
        if dispatch(tok, a):  # 1 реф = 1 диспатч волны (Л188a/b)
            n_ok += 1
        time.sleep(3)
    print(f"wave-1 accepted {n_ok}/{len(ALIASES)}; waiting 150s for registrations + band-gates...")
    time.sleep(150)
    runs = collect_runs(tok, mc)
    # W3: band-miss fast-fail (completed/failure < ~5 мин после старта волны) →
    # 1 free ре-ролл (Л188c) — attempt-2 на том же реф
    rerolled = []
    for a in ALIASES:
        atts = runs[a]
        if atts and atts[-1]["status"] == "completed" and atts[-1]["conclusion"] == "failure":
            if dispatch(tok, a, "re-roll W3 "):
                rerolled.append(a)
            time.sleep(3)
    if rerolled:
        print(f"re-rolled {len(rerolled)}: {rerolled}; waiting 90s...")
        time.sleep(90)
    runs = collect_runs(tok, mc)
    # финальный опрос (12e: бенчи НЕ ждать — статус фиксем, не conclusion)
    verdict = {}
    dispatched = 0
    total_runs = 0
    for a in ALIASES:
        atts = runs[a]
        total_runs += len(atts)
        live = [r for r in atts if r["status"] in ("queued", "in_progress")]
        dispatched += 1 if live else 0
        if live:
            state = "DISPATCHED"
        elif atts and atts[-1]["conclusion"] == "failure":
            state = "BAND-MISS-FASTFAIL"
        else:
            state = "PENDING-POLL"
        verdict[a] = {"target_cpu": TMAP[a], "state": state, "attempts": atts}
    summary = {"pin": PIN[:8], "wave_accepted": n_ok, "re_rolls": rerolled,
               "total_runs": total_runs, "dispatched_aliases": dispatched,
               "run_ids": [r["id"] for a in ALIASES for r in runs[a]]}
    out = {"summary": summary, "aliases": verdict}
    print(json.dumps(out, indent=1))
    import os
    os.makedirs(EVID, exist_ok=True)
    with open(f"{EVID}/wave_479_v3.json", "w") as f:
        json.dump(out, f, indent=1)
    if total_runs < len(ALIASES):
        sys.exit(4)  # волна неполна — стюард добирает вручную


if __name__ == "__main__":
    main()
