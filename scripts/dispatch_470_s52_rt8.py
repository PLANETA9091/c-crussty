#!/usr/bin/env python3
"""dispatch_470_s52_rt8.py — ROUND-470 S52 (КЛИМБ-chunk) — W8@r480 reopen: rt8-band ре-роллы ×3.

Л199: rt8@r480 Amdahl-потолок +23.7пп ≥ БАР(+20) — W8-окно reopen стратумом
(φ 0.567→0.713); W8@r640 REFUTED ×3 слоя (Л190: Amdahl +14.05пп, N_regions≤4 шапка).
Прежние rt8-ноги мертвы бесплатно (BOTTLENECK §5): u1 36264125995 / u3 36264125630
band-dead (median null), rt8p u2 −33.49 = G1-дыра (Л-470-S06.2 — числом не считать)
→ ре-ролл легален ТОЛЬКО на живом gc6 (gc_tune=6 = ParallelGC+MD256M+RCC512M,
Л-470-S06.5 «rt8p/S52 ре-роллы легализованы на gc6»).

НОГА env-only (0 Java/0 Rust дельт): ветки round-470-s52-rt8a/b/c = алиасы ОДНОГО
sha 40068dbe (b3853246 = МЕРЖ №11 + board-docs; код бит-идентичен, = sha якоря-твина
r480c/d) + region_threads=8 + radius=480 + gc_tune=6. Л188b: 1 диспатч = 1 ветка
(concurrency cancel-in-progress per-ref); Л188a: POST /git/refs FULL-sha + GET-вериф.

H-S52 PREREGISTER (гейты, вердикт тик-471):
- Amdahl-прогноз: Δpair(rt8−rt4) ≤ +23.7пп (φ=0.713, 8 тредов: 1/(0.287+0.713/8)=2.66×);
  точка leg-norm = стратум-класс r480a (−3.00@6749854) + 23.7 = +20.7 ≥ БАР,
  честный band ±MAE 5.91 → [+14.8, +26.6] — бар берётся только при полном capture.
- PAIR-vs-ANCHOR: решающая пара vs r480c run 36270314846 (rt4@gc6, ТОТ ЖЕ sha) —
  Δcpu ≤50k (Л195 run-env), min-of-3, HOST-excl; norm-бар = банк v5 (Л201-узел [6.9,7.2]M).
- Вердикт-дерево: (a) min-of-3 norm ≥+20 ∧ Δpair∈(0,+23.7] → W8@r480 VERIFIED
  (компо-голова: L211-RE-ARM rt8⊕noise-fill); (b) Δpair>0, norm<+20 → REFUTED_CENS
  потолком Amdahl; (c) Δpair≤0 → φ-модель мертва на gc6, W8-окно закрыть окончательно.
- Гигиена: band fast-fail [6.0,9.5]M = бесплатный дискард → ре-диспатч (канон S06
  36269776506); lever_flag/arg пустые (1 нога = 1 дельта: только region_threads 8-vs-4).

Usage: dispatch_470_s52_rt8.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN_SHA = "40068dbe5accbbb3d0ac68aba8062bdd746a9405"  # master b3853246 + board-docs (sha r480c/d)
LEGS = ["round-470-s52-rt8a", "round-470-s52-rt8b", "round-470-s52-rt8c"]

# Канон-вектор S06-стратума (Л-470-S06.5: r480/300s/fp4/150k/seed42/10G/4G/bc1/ic1/fd1,
# band 6.0-9.5M) c ДВУМЯ дельтами ноги: region_threads 4→8 (rt8), gc_tune 3→6
# (живой ParallelGC-пресет — обязательное условие reopen, Л-470-S06.2). lever пустой.
INPUTS = {
    "radius": "480", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "8",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    if m:
        return m.group(1)
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}")
        raise
    return json.loads(body) if body else {}


def main():
    if any(a != "--dry-run" for a in sys.argv[1:]):
        raise SystemExit(f"argv-guard: unknown args {sys.argv[1:]}")
    dry = "--dry-run" in sys.argv[1:]
    tok = token()

    # Л188a-канон: создать ref (POST /git/refs FULL-sha — S20-урок 422 на short),
    # затем GET-верификация object.sha ДО диспатча.
    for br in LEGS:
        try:
            live = api(tok, f"/repos/{REPO}/git/ref/heads/{br}")["object"]["sha"]
        except urllib.error.HTTPError:
            live = None
        if live == PIN_SHA:
            print(f"ref exists OK: {br} @ {live[:8]}", flush=True)
        else:
            if live:
                raise SystemExit(f"REF MISMATCH: {br} @ {live} != {PIN_SHA[:8]}")
            if dry:
                print(f"(dry) would POST /git/refs {br} @ {PIN_SHA[:8]}", flush=True)
                continue
            api(tok, f"/repos/{REPO}/git/refs", method="POST",
                data={"ref": f"refs/heads/{br}", "sha": PIN_SHA})
            time.sleep(3)
            live = api(tok, f"/repos/{REPO}/git/ref/heads/{br}")["object"]["sha"]
            if live != PIN_SHA:
                raise SystemExit(f"POST-REF VERIFY FAIL: {br} @ {live}")
            print(f"ref created+verified: {br} @ {live[:8]} (= b3853246-линия, r480c-twin)", flush=True)

    print(f"inputs: {json.dumps(INPUTS, ensure_ascii=False)}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatch", flush=True)
        return

    for br in LEGS:
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": INPUTS})
        print(f"dispatched {br}: rt8@r480 gc6 band[6.0,9.5]M (HTTP 204)", flush=True)
        time.sleep(4)

    # Захват run-id: полл свежих запусков WF и мэтч по head_branch.
    ids = {}
    for _ in range(6):
        time.sleep(20)
        runs = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?per_page=30")
        for r in runs.get("workflow_runs", []):
            if r["head_branch"] in LEGS and r["event"] == "workflow_dispatch":
                ids.setdefault(r["head_branch"], r["id"])
        if len(ids) == len(LEGS):
            break
    for br in LEGS:
        print(f"RUN-ID {br} = {ids.get(br, 'POLL-MISS (re-poll manually)')}", flush=True)
    print("=== ROUND-470 S52 BATCH COMPLETE ===", flush=True)


if __name__ == "__main__":
    main()
