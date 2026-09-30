#!/usr/bin/env python3
"""dispatch_512b.py — RE-GRAIN-2 волна ×511 (закон 15a): p31ib-c5b1-3 @ab4e48e
(C5b-патч ветка, МЕРЖ №24 leg-2 хвост) + fen7-9/unf7-9 INDUCE @adbb4d42 (CLM-G3 ч.2,
lever_arg mc312a-канон) + w8feed1-3 forensics @PIN (WILDA спека, 0 код-дельт) +
stz93v2/stz94v2 (v2-ассеты 107a119, CLM-STZ лестница-барьер-2)."""
import json, subprocess, time, sys

REPO = "PLANETA9091/c-crussty"
PIN = "09ca39617d2e9f4fc94d4c462a72129ce34f6ff6"
C5B_SHA = None  # verify remote
INDUCE_SHA = None
WF = "world-bench-parallel.yml"
ASSET_REF = "round-511-stz-assets"
RAW = f"https://raw.githubusercontent.com/{REPO}/{ASSET_REF}/dp_assets"
INDUCE_ARG = "stz59_probe=1,stz59_profile=INDUCE,stz59_stretch_ns=4000,stz59_inorder=1,stz59_sections=4096,stz59_wpt=4096"


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
        BRANCH_SHA[branch] = "c5b"
    if kw.pop("induce", False):
        BRANCH_SHA[branch] = "induce"
    base = {"world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
            "radius": "640", "seconds": "300", "fake_players": "4", "fluid_guard": "1",
            "gc_tune": "3", "inside_cache": "1", "flush_diet": "1", "fluid_dirty": "0",
            "fluid_bitmask": "0", "region_threads": "4", "batch_collector": "1",
            "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
            "population_target": "150000", "server_xmx": "10G", "server_xms": "4G",
            "lever_flag": "", "lever_arg": "", "datapack_url": ""}
    base.update(kw)
    LEG.append((branch, base))


# p31ib-c5b ×3 (seeds 1534-1536) — МЕРЖ №24 путь leg-2
for i, seed in enumerate((1534, 1535, 1536)):
    add(f"round-512-p31cbc{i+1}", population_seed=str(seed),
        lever_flag="cmp456_chunkmono_p31snap", p31=True)
# fen7-9 INDUCE (seeds 1537-1539, FENCED via cmp466_stz59fence)
for i, seed in enumerate((1537, 1538, 1539)):
    add(f"round-512-fen{i+7}", population_seed=str(seed),
        lever_flag="cmp466_stz59fence", lever_arg=INDUCE_ARG, induce=True)
# unf7-9 INDUCE (seeds 1540-1542, UNFENCED, lever_flag empty)
for i, seed in enumerate((1540, 1541, 1542)):
    add(f"round-512-unf{i+7}", population_seed=str(seed),
        lever_flag="", lever_arg=INDUCE_ARG, induce=True)
# w8feed1-3 forensics (seeds 1529-1531, 0 код-дельт, WILDA CLM-WILDA)
for i, seed in enumerate((1529, 1530, 1531)):
    add(f"round-512-w8feed{i+1}", population_seed=str(seed))
# stz93v2 (600s) + stz94v2 (300s) — лестница-барьер-2
add("round-512-stz93v2", seconds="600", datapack_url=f"{RAW}/stz93v2.zip", population_seed="1543")
add("round-512-stz94v2", seconds="300", datapack_url=f"{RAW}/stz94v2.zip", population_seed="1544")


def main():
    global C5B_SHA, INDUCE_SHA
    # анти-пласебо G0: верификация head_sha код-веток
    g = gh("GET", "git/ref/heads/round-511-p31ib-c5b")
    C5B_SHA = g.get("object", {}).get("sha", "")
    g2 = gh("GET", "git/ref/heads/round-511-stz59-induce")
    INDUCE_SHA = g2.get("object", {}).get("sha", "")
    print(f"c5b={C5B_SHA[:12]} induce={INDUCE_SHA[:12]}", flush=True)
    if not C5B_SHA.startswith("ab4e48e") or not INDUCE_SHA.startswith("adbb4d4"):
        print("MISMATCH-ABORT", flush=True)
        sys.exit(2)
    t0 = time.time()
    ok = 0
    for branch, inputs in LEG:
        kind = BRANCH_SHA.get(branch)
        want_sha = {"c5b": C5B_SHA, "induce": INDUCE_SHA}.get(kind, kind or PIN)
        if make_branch(branch, want_sha):
            if dispatch(branch, inputs):
                rid, created = verify(branch)
                ok += 1
                print(f"  {branch} → run {rid} @{created}", flush=True)
        else:
            print(f"  {branch} branch-FAIL", flush=True)
    json.dump({"dispatched": ok, "legs": [{"branch": b, "inputs": i} for b, i in LEG]},
              open("/home/z/rounds/ROUND-511/dispatch_512b.json", "w"), indent=1)
    print(f"DONE: {ok} диспатчей ({time.time()-t0:.0f}s)", flush=True)


if __name__ == "__main__":
    main()
