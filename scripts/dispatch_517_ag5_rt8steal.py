#!/usr/bin/env python3
"""dispatch_517_ag5_rt8steal.py — пара base-vs-patch rt8+steal=1 (AG-5, волна-517).

Гипотеза preregistered ДО диспатча: work/AG-5/GATE_517_AG5.md (закон 14a/16).
Прецедент Л-482-C43: rt8+steal=1 norm +20.49 (1 реплика, min-of-3 не добит).
Ноги: swarm-517-5a (base, ваниль-якорь x466-C98) vs swarm-517-5b (patch,
единственная дельта region_threads=8+region_steal=1), ОДИНАКОВЫЙ sha c7f6eb6
(master head; bench/worldv2+src бит-идентичны канону 8bab7a6, оба concurrency
per-LEG фикса в yml). 1 нога = 1 ветка-алиас (Л188b/Л407r), refs = POST /git/refs
FULL-sha + GET-верификация (Л188a). Код-дельт нет → ноль worktree (диск 98%,
OFFLINE-канон). Бюджет 2/2 POST. Seed-gate: population_seed 42 = канон x466-C98,
SEED-LEDGER-коллизий нет (canon-вектор, не новый seed).
"""
import json, subprocess, time

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "world-bench-parallel.yml"
SHA = "c7f6eb6a55d8a9086b71d7ee6ddc9ba2d5f7d7e3"  # будет заменён на FULL из git ниже

full = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "master"],
                      capture_output=True, text=True).stdout.strip()
assert full.startswith("c7f6eb6"), f"master moved? {full}"
SHA = full

BASE = {"region_threads": "4", "region_steal": "0"}
PATCH = {"region_threads": "8", "region_steal": "1"}

def vec(delta):
    v = {
        "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
        "radius": "640", "seconds": "300", "fake_players": "4",
        "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
        "fluid_dirty": "0", "fluid_bitmask": "0", "batch_collector": "1",
        "skip_store_bb": "0", "bu_defer": "0",
        "population_target": "150000", "population_seed": "42",
        "server_xmx": "10G", "server_xms": "4G",
        "cpu_band_min": "6000000", "cpu_band_max": "9500000",
        "lever_flag": "", "lever_arg": "",
    }
    v.update(delta)
    return v

legs = [
    {"alias": "swarm-517-5a", "delta": BASE},
    {"alias": "swarm-517-5b", "delta": PATCH},
]

def gh(method, url, payload=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {TOK}",
           "-H", "Accept: application/vnd.github+json", url]
    if payload is not None:
        cmd += ["-d", json.dumps(payload)]
    return subprocess.run(cmd, capture_output=True, text=True).stdout

ok = True
for l in legs:
    r = gh("POST", f"{API}/git/refs", {"ref": f"refs/heads/{l['alias']}", "sha": SHA})
    g = json.loads(gh("GET", f"{API}/git/ref/heads/{l['alias']}") or "{}")
    got = (g.get("object") or {}).get("sha", "")
    m = got == SHA
    ok &= m
    print(f"ref {l['alias']}: post={'ok' if m else r[:120]} get={got[:12]} match={m}", flush=True)
if not ok:
    raise SystemExit("REF-VERIFY FAIL — DISP-INTENT: payload сохранён, POST не делаю")

dispatched = []
for l in legs:
    r = gh("POST", f"{API}/actions/workflows/{WF}/dispatches",
           {"ref": l["alias"], "inputs": vec(l["delta"])})
    if r.strip() == "":
        dispatched.append(l["alias"])
        print(f"POST 204 {l['alias']}", flush=True)
    else:
        print(f"POST FAIL {l['alias']}: {r[:200]}", flush=True)

# run-id capture: воркер-полл по head_sha (event=workflow_dispatch)
run_ids = {}
t0 = time.time()
while time.time() - t0 < 90 and len(run_ids) < len(dispatched):
    time.sleep(15)
    out = gh("GET", f"{API}/actions/runs?event=workflow_dispatch&head_sha={SHA}&per_page=20")
    try:
        for w in json.loads(out).get("workflow_runs", []):
            b = w.get("head_branch", "")
            if b in dispatched and b not in run_ids:
                run_ids[b] = w["id"]
    except Exception:
        pass

res = {"tool": "dispatch_517_ag5_rt8steal", "sha": SHA, "gate": "work/AG-5/GATE_517_AG5.md",
       "legs": legs, "dispatched": dispatched, "run_ids": run_ids,
       "ts_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}
json.dump(res, open("/home/z/rounds/ROUND-517/work/AG-5/disp_517_ag5.json", "w"),
          indent=1, ensure_ascii=False)
print(json.dumps(res, ensure_ascii=False))
if not dispatched:
    raise SystemExit("DISP-INTENT: payload в work/AG-5/disp_517_ag5.json")
