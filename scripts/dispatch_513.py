#!/usr/bin/env python3
"""dispatch_513.py — волна-513 (RE-GRAIN ×512, закон 15a): 104 ноги.
4 ib @b8bed2c6 + 3 occ @3b89c10 (gc6 re-roll) + 4 dp @7a62df9 (50k) +
4 sp [7.5,8.0]M + 89 ax @7a62df9. Эпохи 38/38/28 (анти-кансел «эпоха ≤ pool»).
Каноны: POST /git/refs FULL-sha + GET-верификация (Л188a); 1 диспатч = 1 ветка (Л188b)."""
import json, subprocess, time, sys

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "world-bench-parallel.yml"
WORLD = "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip"

SH_MASTER = "7a62df9bde65877fa2ef066dda93b39092849ed7"
SH_IB = "b8bed2c6ab7b269a2e6abf679172dbb10efb0803"
SH_OCC = "3b89c10298a27653abbc1207e17ef2410aedd79c"

def canon_inputs(seed, lever="", lever_arg="", pop="150000", dp="", gc="3",
                 band_min="", band_max=""):
    return {
        "world_url": WORLD, "radius": "640", "seconds": "300", "fake_players": "4",
        "fluid_guard": "1", "gc_tune": gc, "inside_cache": "1", "flush_diet": "1",
        "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
        "batch_collector": "1", "skip_store_bb": "0", "region_steal": "0",
        "bu_defer": "0", "population_target": pop, "server_xmx": "10G",
        "server_xms": "4G", "lever_flag": lever, "lever_arg": lever_arg,
        "datapack_url": dp, "population_seed": str(seed),
        **({"cpu_band_min": band_min, "cpu_band_max": band_max} if band_min else {}),
    }

legs = []
# ib8-11 @b8bed2c6 (P31-IB carrier, CLIMB k=6-10)
for i, seed in enumerate(range(1552, 1556)):
    legs.append({"branch": f"round-513-ib{8+i}", "sha": SH_IB,
                 "inputs": canon_inputs(seed, lever="cmp456_chunkmono_p31snap")})
# occ4-6 @3b89c10, gc_tune=6 re-roll (CLM-OCC-REROLL), seeds 1556-1558
for i, seed in enumerate(range(1556, 1559)):
    legs.append({"branch": f"round-513-occ{4+i}", "sha": SH_OCC,
                 "inputs": canon_inputs(seed, lever="cmp511_occ", gc="6")})
# dp13-16 @7a62df9 (G-B2 4-слот, seeds 1545-1548, CLM-DP-GB2)
for i, seed in enumerate(range(1545, 1549)):
    legs.append({"branch": f"round-513-dp{13+i}", "sha": SH_MASTER,
                 "inputs": canon_inputs(seed, pop="50000", dp="v484-dp3v2")})
# sp11-14 fleet-gap добивка [7.5,8.0]M (CLM-BANK-AUDIT: зона 1/5 клеток)
for i, seed in enumerate(range(1648, 1652)):
    legs.append({"branch": f"round-513-sp{11+i}", "sha": SH_MASTER,
                 "inputs": canon_inputs(seed, band_min="7500000", band_max="8000000")})
# ax-фид ×89 vanilla @7a62df9, seeds 1559-1647
for i, seed in enumerate(range(1559, 1648)):
    legs.append({"branch": f"round-513-ax{i+1:02d}", "sha": SH_MASTER,
                 "inputs": canon_inputs(seed)})

def gh(method, url, payload=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {TOK}",
           "-H", "Accept: application/vnd.github+json", url]
    if payload:
        cmd += ["-d", json.dumps(payload)]
    return subprocess.run(cmd, capture_output=True, text=True).stdout

# 1) создать ветки (POST refs) + GET-верификация
ok_refs, fail_refs = 0, []
for l in legs:
    r = gh("POST", f"{API}/git/refs",
           {"ref": f"refs/heads/{l['branch']}", "sha": l["sha"]})
    if '"ref"' in r:
        ok_refs += 1
    else:
        # уже существует? GET-верификация sha
        g = json.loads(gh("GET", f"{API}/git/ref/heads/{l['branch']}") or "{}")
        if g.get("object", {}).get("sha", "").startswith(l["sha"]):
            ok_refs += 1
        else:
            fail_refs.append((l["branch"], r[:120]))
print(f"refs ok={ok_refs}/{len(legs)} fail={len(fail_refs)}", flush=True)
if fail_refs[:3]:
    print("fail sample:", fail_refs[:3])

# 2) диспатч эпохами 38/38/28
EPOCHS = [38, 38, 28]
dispatched, fails = 0, []
start = 0
for ei, n in enumerate(EPOCHS):
    chunk = legs[start:start + n]
    for l in chunk:
        r = gh("POST", f"{API}/actions/workflows/{WF}/dispatches",
               {"ref": l["branch"], "inputs": l["inputs"]})
        if r.strip() == "":
            dispatched += 1
        else:
            fails.append((l["branch"], r[:160]))
    print(f"epoch {ei+1}: +{len(chunk)} (dispatched={dispatched})", flush=True)
    if ei < len(EPOCHS) - 1:
        time.sleep(90)   # анти-кансел: эпоха ≤ pool, пауза между залпами
    start += n

json.dump({"tool": "dispatch_513", "dispatched": dispatched, "refs_ok": ok_refs,
           "fails": fails, "legs": legs},
          open("/home/z/rounds/ROUND-512/dispatch_513.json", "w"), indent=1, ensure_ascii=False)
print(f"DONE dispatched={dispatched}/{len(legs)} fails={len(fails)}")
