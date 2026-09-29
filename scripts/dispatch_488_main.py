#!/usr/bin/env python3
"""dispatch_488_main.py — волны диспатчей ×488 (тик 11:08+08).
A: CI-доба 4 dormant wiring-веток (арбитр base-rep пришёл: dp-база 0.4 = плато канона C24).
B: рестарты 5 failed-ног ×487.
C: 91 ролл-дроу (st-окно / dp-якоря / gc6-классы / canary / страта-зонды) — норма 12c ≥100.
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
MASTER = None  # заполняем из API

WORLD = "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip"
DP_URL = "https://github.com/PLANETA9091/c-crussty/releases/download/v484-dp3v2/stz3v2-fixture.zip"
TOWERS = "https://raw.githubusercontent.com/PLANETA9091/c-crussty/round-478-a5-tow-re/stress/world-stress-towers.zip"

def tok(): return open("/tmp/gh_token").read().strip()

def api(url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok()}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload: req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            b = r.read()
        return json.loads(b) if b else {}
    except urllib.error.HTTPError as e:
        return {"_http_error": e.code, "_err": e.read().decode("utf-8","replace")[:200]}

def canon(world=WORLD, **over):
    d = {
        "world_url": world, "radius": "640", "seconds": "300", "fake_players": "4",
        "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
        "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4", "batch_collector": "1",
        "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
        "datapack_url": "", "population_target": "150000", "population_seed": "42",
        "server_xmx": "10G", "server_xms": "4G",
        "cpu_band_min": "6000000", "cpu_band_max": "9500000",
        "lever_flag": "", "lever_arg": "",
    }
    d.update(over); return d

def mkbranch(name, sha):
    r = api(f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{name}", "sha": sha})
    return "_http_error" not in r or r["_http_error"] == 422  # 422 = уже существует

def dispatch(branch, inputs):
    r = api(f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": branch, "inputs": inputs})
    return "_http_error" not in r

def main():
    m = api(f"/repos/{REPO}/git/ref/heads/master")
    master = m["object"]["sha"]
    print(f"master pin = {master[:8]}")

    results = []
    # ---------- A: CI-доба wiring (dp-стенд, STRICT-флаги) ----------
    wiring = [
        ("round-487-c02-bitset", "round-487-c02-bitset"),
        ("round-487-c65-sbulk1", "cmp487_sbulk1"),
        ("round-487-c66-biroar", "cmp487_bir1"),
        ("round-487-c67-roar1",  "cmp487_roar1"),
    ]
    for br, flag in wiring:
        inp = canon(datapack_url=DP_URL, lever_flag=flag)
        ok = dispatch(br, inp)
        results.append(("A-wiring", br, flag, "DISPATCHED" if ok else f"FAIL{inp.get('_http_error','')}"))
        print(results[-1]); time.sleep(1)

    # ---------- B: рестарты failed ×487 ----------
    restarts = [
        ("round-487-c19-st7",     canon()),                    # STRICT st7 roll
        ("round-487-xs1",         canon()),                    # st7 roll
        ("round-487-xs2",         canon()),                    # st7 roll
        ("round-487-c36-sensn16", canon()),                    # sensn16 canon draw
        ("round-487-c52-dp03",    canon(datapack_url=DP_URL)), # dp-0.3 rep
        ("round-487-c58-towr4",   canon(world=TOWERS)),        # towers rt4 rep
    ]
    for br, inp in restarts:
        ok = dispatch(br, inp)
        results.append(("B-restart", br, "-", "DISPATCHED" if ok else "FAIL"))
        print(results[-1]); time.sleep(1)

    # ---------- C: ролл-дроу 91 ----------
    rolls = []
    for i in range(1, 51):  # 50 st-канон draws
        rolls.append((f"round-488-an{i}", canon()))
    for i in range(1, 13):  # 12 dp-0.3 якоря
        rolls.append((f"round-488-dp{i}", canon(datapack_url=DP_URL)))
    g6 = [("g6a","165000"),("g6b","165000"),("g6c","205000"),("g6d","205000"),("g6e","210000"),("g6f","210000")]
    for tag, pop in g6:  # 6 gc6-классов (xmx12G)
        rolls.append((f"round-488-{tag}", canon(gc_tune="6", server_xmx="12G", population_target=pop)))
    for i in range(1, 9):  # 8 canary corridor
        rolls.append((f"round-488-cnr{i}", canon()))
    strata = (["6.3"]*5, ["8.5"]*5, ["9.2"]*5)  # страта-зонды (квоты band — как x487 C21/C22)
    si = 1
    for grp in strata:
        for _ in grp:
            rolls.append((f"round-488-str{si}", canon())); si += 1

    ok_n = 0
    for br, inp in rolls:
        if mkbranch(br, master):
            ok = dispatch(br, inp)
            ok_n += ok
            results.append(("C-roll", br, "-", "DISPATCHED" if ok else "FAIL"))
        else:
            results.append(("C-roll", br, "-", "MKBRANCH-FAIL"))
        time.sleep(0.4)
    print(f"rolls dispatched: {ok_n}/{len(rolls)}")

    # итог
    with open("/tmp/abs488/dispatch_488_results.json", "w") as f:
        json.dump({"master": master, "results": [list(r) for r in results]}, f, ensure_ascii=False, indent=1)
    total = sum(1 for r in results if r[3] == "DISPATCHED")
    print(f"TOTAL DISPATCHED ×488: {total}/{len(results)}")

if __name__ == "__main__":
    main()
