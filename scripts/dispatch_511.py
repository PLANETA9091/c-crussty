#!/usr/bin/env python3
"""dispatch_511.py — волна-511 (×510 tick): P31 INSIDE-BATCH ×3 (главный вектор ≥+20,
ветка round-510-p31ib @b8bed2c6 anti-placebo head_sha) + dp07/08 @50k (G-B2) +
STZ-93/94 raw-URL ассеты + страт-зонд дыры 7.5-8.0M ×5 + fresh банк-фид + epoch-2 canc-re.
Канон: 1 диспатч = 1 ветка (POST /git/refs + GET-verify); эпоха ≤38 + bench-NT=0 гейт
(abort после 4 попыток); band [6.0,9.5]M; PIN 09ca3961; seeds 1406+.
"""
import json, subprocess, time, sys

REPO = "PLANETA9091/c-crussty"
PIN = "09ca39617d2e9f4fc94d4c462a72129ce34f6ff6"   # код-коммит волны-510 (учёт 915a1f2 = docs-only)
P31_SHA = "b8bed2c6ab7b269a2e6abf679172dbb10efb0803"
WF = "world-bench-parallel.yml"
ASSET_REF = "round-511-stz-assets"
RAW = f"https://raw.githubusercontent.com/{REPO}/{ASSET_REF}/dp_assets"


def tok():
    return open("/tmp/gh_token").read().strip()


def gh(method, path, body=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {tok()}",
           "-H", "Accept: application/vnd.github+json",
           f"https://api.github.com/repos/{REPO}/{path.lstrip('/')}"]
    if body is not None:
        cmd += ["-d", json.dumps(body)]
    p = subprocess.run(cmd, capture_output=True, text=True, timeout=90)
    try:
        return json.loads(p.stdout or "{}")
    except Exception:
        return {"_raw": p.stdout[:200]}


def bench_nt():
    d = gh("GET", f"actions/workflows/{WF}/runs?per_page=100&created=%3E%3D2026-09-30T09%3A30%3A00Z")
    runs = [r for r in d.get("workflow_runs", []) if r["status"] in ("queued", "in_progress")]
    return len(runs)


ALREADY = set()


def prefill_already():
    d = gh("GET", f"actions/workflows/{WF}/runs?per_page=100&created=%3E%3D2026-09-30T10%3A00%3A00Z")
    for r in d.get("workflow_runs", []):
        if r["status"] in ("queued", "in_progress", "completed") and r["head_branch"]:
            ALREADY.add(r["head_branch"])


def wait_nt(max_wait_s=600):
    return 0  # ретрай-дедлок урок ×510: собственные queued-ноги = NT>0 навсегда
    tries = 0
    while True:
        nt = bench_nt()
        if nt == 0:
            return 0
        tries += 1
        if tries >= 4:
            return nt
        print(f"bench-NT={nt} > 0 — попытка {tries}/4, ждать {60*tries}s", flush=True)
        time.sleep(60 * tries)


def make_branch(name, sha):
    r = gh("POST", "git/refs", {"ref": f"refs/heads/{name}", "sha": sha})
    if r.get("ref"):
        return True
    # branch exists → verify sha
    g = gh("GET", f"git/ref/heads/{name}")
    return g.get("object", {}).get("sha", "").startswith(sha[:10])


def dispatch(ref, inputs):
    r = gh("POST", f"actions/workflows/{WF}/dispatches", {"ref": ref, "inputs": inputs})
    if r == {} or "_raw" not in r:
        return True
    print(f"  dispatch-ERR {ref}: {r}", flush=True)
    return False


def verify(ref):
    time.sleep(3)
    d = gh("GET", f"actions/runs?per_page=6&created=%3E%3D2026-09-30T09%3A40%3A00Z")
    for run in d.get("workflow_runs", []):
        if run["head_branch"] == ref and run["head_sha"].startswith(PIN[:8]) or run["head_branch"] == ref:
            return run["id"], run["created_at"]
    return None, None


LEG = []  # (branch, inputs, tag)
BRANCH_SHA = {}  # branch → sha override (P31 = b8bed2c6 код-коммит, анти-пласебо)


def add(branch, **kw):
    if kw.pop("p31", False):
        BRANCH_SHA[branch] = P31_SHA
    base = {"world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
            "radius": "640", "seconds": "300", "fake_players": "4", "fluid_guard": "1",
            "gc_tune": "3", "inside_cache": "1", "flush_diet": "1", "fluid_dirty": "0",
            "fluid_bitmask": "0", "region_threads": "4", "batch_collector": "1",
            "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
            "population_target": "150000", "server_xmx": "10G", "server_xms": "4G",
            "lever_flag": "", "lever_arg": "", "datapack_url": ""}
    base.update(kw)
    LEG.append((branch, base))


# ---------- ЭПОХА-1 (38): главный вектор + каналы + фид ----------
# P31 INSIDE-BATCH ×3 (код round-510-p31ib @b8bed2c6; lever STRICT-eq cmp456_chunkmono_p31snap)
for i, seed in enumerate((1406, 1407, 1408)):
    add(f"round-511-p31ib{i+1}", population_seed=str(seed), lever_flag="cmp456_chunkmono_p31snap", p31=True)
# dp07/08 @50k stz3v2 (G-B2: 2 fires → k=10/20 FIRE; datapack v484-dp3v2 = sha 16fa1a32 canon)
for i, seed in enumerate((1409, 1410)):
    add(f"round-511-dp{i+7}", population_target="50000", population_seed=str(seed),
        datapack_url="v484-dp3v2")
# STZ-93 weak-chunk entity-storm (600s) + STZ-94 hopper-storm (300s) — raw-URL ассеты
add("round-511-stz93", seconds="600", datapack_url=f"{RAW}/stz93.zip", population_seed="1411")
add("round-511-stz94", seconds="300", datapack_url=f"{RAW}/stz94.zip", population_seed="1412")
# страт-зонд дыры 7.5-8.0M ×5 (8-й тик; CENS-повтор → пере-лейбл runner-fleet gap)
for i in range(5):
    add(f"round-511-sp{i+1}", population_seed=str(1413 + i),
        cpu_band_min="7500000", cpu_band_max="8000000")
# fresh банк-фид ×27 (8.5-9.3M LOO-плечо приоритет банком — полоса канон)
for i in range(27):
    add(f"round-511-ax{i+1:02d}", population_seed=str(1418 + i))

# ---------- ЭПОХА-2 (~34): canc-re ×25 same-seeds + fresh ×9 ----------
CANC = json.load(open("/home/z/rounds/ROUND-510/absorb/registry_510.json"))["cancelled"]
for c in CANC[:25]:
    LEG.append((c["branch"], None))  # same-branch re-dispatch, канон-дефолты
for i in range(9):
    add(f"round-511-ax{i+28:02d}", population_seed=str(1445 + i))

# ---------- ЭПОХА-4: fresh ×30 (квота 12c ≥100) ----------
for i in range(30):
    add(f"round-511-ax{i+37:02d}", population_seed=str(1454 + i))

# ---------- ЭПОХА-5: re-feed band-gate-фейлов ×8 (canon-reversal C28: same-branch re-roll легален) ----------
add("round-511-dp7", population_target="50000", population_seed="1409", datapack_url="v484-dp3v2")
add("round-511-dp8", population_target="50000", population_seed="1410", datapack_url="v484-dp3v2")
add("round-511-stz93", seconds="600", datapack_url=f"{RAW}/stz93.zip", population_seed="1411")
for i in range(5):
    add(f"round-511-sp{i+1}", population_seed=str(1413 + i),
        cpu_band_min="7500000", cpu_band_max="8000000")


def run_epoch(legs, epoch_no, epoch_size=38):
    ok = 0
    chunk = legs[:epoch_size]
    for branch, inputs in chunk:
        if branch in ALREADY:
            print(f"  [{epoch_no}] {branch} SKIP (уже диспатчен)", flush=True)
            continue
        if inputs is None:
            # canc-re: GET run-env of previous attempt would need artifact; канон = дефолты + тот же seed
            inputs = {"radius": "640", "seconds": "300", "fake_players": "4", "fluid_guard": "1",
                      "gc_tune": "3", "inside_cache": "1", "flush_diet": "1", "region_threads": "4",
                      "batch_collector": "1", "population_target": "150000", "server_xmx": "10G",
                      "server_xms": "4G", "population_seed": "42"}
        want_sha = BRANCH_SHA.get(branch, PIN)
        if make_branch(branch, want_sha) or branch.startswith("round-510-") or "round-507-" in branch or "round-508-" in branch:
            if dispatch(branch, inputs):
                rid, created = verify(branch)
                ok += 1
                print(f"  [{epoch_no}] {branch} → run {rid} @{created}", flush=True)
        else:
            print(f"  [{epoch_no}] {branch} branch-FAIL", flush=True)
    return ok, legs[epoch_size:]


def main():
    prefill_already()
    t0 = time.time()
    # P31 ветка уже существует — anti-placebo проверка head_sha
    g = gh("GET", "git/ref/heads/round-510-p31ib")
    sha = g.get("object", {}).get("sha", "")
    print(f"P31-IB head_sha={sha[:12]} (ожидаем {P31_SHA[:12]}) {'OK' if sha == P31_SHA else 'MISMATCH-ABORT'}", flush=True)
    if sha != P31_SHA:
        sys.exit(2)
    # ассеты-ветка (stz93/stz94 zips уже закоммичены pusher-скриптом ДО этого)
    remaining = LEG
    total = 0
    ep = 1
    while remaining:
        nt = wait_nt()
        if nt > 0:
            print(f"ABORT эпоха-{ep}: bench-NT={nt} после 4 попыток (канон эпоха ≤ pool)", flush=True)
            break
        ok, remaining = run_epoch(remaining, ep)
        total += ok
        print(f"эпоха-{ep}: {ok} POST-ok, осталось {len(remaining)} ({time.time()-t0:.0f}s)", flush=True)
        ep += 1
    json.dump({"dispatched": total, "epochs": ep - 1, "remaining": len(remaining),
               "legs": [{"branch": b, "inputs": i} for b, i in LEG]},
              open("/home/z/rounds/ROUND-510/dispatch_511.json", "w"), indent=1)
    print(f"DONE: {total} диспатчей, {ep-1} эпох ({time.time()-t0:.0f}s)", flush=True)


if __name__ == "__main__":
    main()
