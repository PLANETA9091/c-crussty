#!/usr/bin/env python3
"""dispatch_512.py — волна-512 (×511 tick): GATE-3 закон-18 цикл P31-IB (ре-ветки ×4 new seeds
на @b8bed2c6: min-of-3 20.93 < 22.9 → добор legs) + pair-окна якорей (ib1 [8.95,9.06]M /
ib2 [6.60,6.71]M / ib3 [6.69,6.80]M ±50k) + банк-фид + dp9/10 (G-B2 2 fires → k=10/20) +
stz93/94 re-feed + sp6-10 fleet-gap зонд + dp900 r12. Канон: эпоха ≤38, bench-NT=0 (retray-дедлок
урок ×510), PIN 09ca3961 (код master без изменений с волны-510), seeds 1417-1528 fresh."""
import json, subprocess, time, sys

REPO = "PLANETA9091/c-crussty"
PIN = "09ca39617d2e9f4fc94d4c462a72129ce34f6ff6"
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


ALREADY = set()


def prefill_already():
    d = gh("GET", f"actions/workflows/{WF}/runs?per_page=100&created=%3E%3D2026-09-30T10%3A00:00Z")
    for r in d.get("workflow_runs", []):
        if r["head_branch"]:
            ALREADY.add(r["head_branch"])


def make_branch(name, sha):
    r = gh("POST", "git/refs", {"ref": f"refs/heads/{name}", "sha": sha})
    if r.get("ref"):
        return True
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
    d = gh("GET", "actions/runs?per_page=8")
    for run in d.get("workflow_runs", []):
        if run["head_branch"] == ref:
            return run["id"], run["created_at"]
    return None, None


LEG = []
BRANCH_SHA = {}


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


# ---------- ЭПОХА-1 (38): GATE-3 P31-IB добор + каналы ----------
for i, seed in enumerate((1417, 1418, 1419, 1420)):
    add(f"round-512-p31ib{i+4}", population_seed=str(seed), lever_flag="cmp456_chunkmono_p31snap", p31=True)
# dp9/10 @50k (G-B2: 2 fires → k=10/20 q05 0.3242 FIRE)
for i, seed in enumerate((1430, 1431)):
    add(f"round-512-dp{i+9}", population_target="50000", population_seed=str(seed),
        datapack_url="v484-dp3v2")
# stz93-re (failure no-artifact) + stz94-re (CENS M1) new seeds
add("round-512-stz93", seconds="600", datapack_url=f"{RAW}/stz93.zip", population_seed="1432")
add("round-512-stz94", seconds="300", datapack_url=f"{RAW}/stz94.zip", population_seed="1433")
# dp900 r12 (мед-канон 0.3 n=15 → n=16)
add("round-512-dp900", seconds="900", population_seed="1434")
# sp6-10 fleet-gap зонд [7.5,8.0]M (9-й тик дыры)
for i in range(5):
    add(f"round-512-sp{i+6}", population_seed=str(1435 + i),
        cpu_band_min="7500000", cpu_band_max="8000000")
# fresh якоря/фид ×24 (pair-окна p31ib + банк §3)
for i in range(24):
    add(f"round-512-ax{i+1:02d}", population_seed=str(1440 + i))

# ---------- ЭПОХА-2 (38): фид ----------
for i in range(38):
    add(f"round-512-ax{i+25:02d}", population_seed=str(1464 + i))

# ---------- ЭПОХА-3 (27): фид ----------
for i in range(27):
    add(f"round-512-ax{i+63:02d}", population_seed=str(1502 + i))


def run_epoch(legs, epoch_no, epoch_size=38):
    ok = 0
    chunk = legs[:epoch_size]
    for branch, inputs in chunk:
        if branch in ALREADY:
            print(f"  [{epoch_no}] {branch} SKIP (уже существует)", flush=True)
            continue
        want_sha = BRANCH_SHA.get(branch, PIN)
        if make_branch(branch, want_sha):
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
    g = gh("GET", "git/ref/heads/round-510-p31ib")
    sha = g.get("object", {}).get("sha", "")
    print(f"P31-IB head_sha={sha[:12]} (ожидаем {P31_SHA[:12]}) {'OK' if sha == P31_SHA else 'MISMATCH-ABORT'}", flush=True)
    if sha != P31_SHA:
        sys.exit(2)
    remaining = LEG
    total = 0
    ep = 1
    while remaining:
        ok, remaining = run_epoch(remaining, ep)
        total += ok
        print(f"эпоха-{ep}: {ok} POST-ok, осталось {len(remaining)} ({time.time()-t0:.0f}s)", flush=True)
        ep += 1
    json.dump({"dispatched": total, "epochs": ep - 1,
               "legs": [{"branch": b, "inputs": i} for b, i in LEG]},
              open("/home/z/rounds/ROUND-511/dispatch_512.json", "w"), indent=1)
    print(f"DONE: {total} диспатчей, {ep-1} эпох ({time.time()-t0:.0f}s)", flush=True)


if __name__ == "__main__":
    main()
