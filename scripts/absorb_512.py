#!/usr/bin/env python3
"""absorb_512.py — масс-абсорб волны-512 (тик ×512, 20:43+08).
normtool P=8 параллельно (канон скорости ×498) + env-парсер из кэш-zip.
Выход: registry_512.json (все ноги) + bank_feed_512.json (§3-класс)."""
import json, re, subprocess, zipfile, os
from concurrent.futures import ThreadPoolExecutor

LEGS = json.load(open("/tmp/legs_512.json"))
ART = "/home/z/rounds/ROUND-512/art"
NT = "/home/z/c-crussty/scripts/normtool_478.py"

RE_LEV = re.compile(r"lever_flag=([^\s(]*)")
RE_POP = re.compile(r"population_target[:=]\s*\"?(\d+)")
RE_SEED = re.compile(r"population_seed[:=]\s*\"?(\d+)")
RE_DPURL = re.compile(r"datapack_url[:=]\s*\"?([^\s\"]*)")
RE_RAD = re.compile(r"radius[:=]\s*\"?(\d+)")

def env_of(rid):
    p = f"{ART}/art_{rid}.zip"
    if not os.path.exists(p):
        return {}
    try:
        with zipfile.ZipFile(p) as z:
            env = z.read("run-env.txt").decode(errors="replace")
    except Exception:
        return {}
    lev = RE_LEV.search(env)
    return {
        "lever": (lev.group(1) if lev else "") or "",
        "pop": int(RE_POP.search(env).group(1)) if RE_POP.search(env) else None,
        "seed": int(RE_SEED.search(env).group(1)) if RE_SEED.search(env) else None,
        "datapack": (RE_DPURL.search(env).group(1) if RE_DPURL.search(env) else "") or "",
        "radius": int(RE_RAD.search(env).group(1)) if RE_RAD.search(env) else None,
    }

def norm_one(rid, exempt=False):
    args = ["python3", NT, "--run-id", str(rid), "--workdir", ART]
    if exempt:
        args.append("--biomes-exempt")
    r = subprocess.run(args, capture_output=True, text=True)
    try:
        return json.loads(r.stdout.strip().splitlines()[-1])
    except Exception:
        return {"run_id": rid, "verdict": "PARSE-ERR", "err": (r.stderr or r.stdout)[-160:]}

# целевые ноги: все succ round-512-* + sp6/w8feed3 уже normtool'нуты — но добор из кэша мгновенен
targets = []
for br, lst in LEGS.items():
    rid, st, con, ts = lst[0]
    if st == "completed" and con == "success" and br.startswith("round-512-"):
        targets.append((br, rid))
print(f"targets: {len(targets)}", flush=True)

def work(t):
    br, rid = t
    d = norm_one(rid)
    d["branch"] = br
    return d

with ThreadPoolExecutor(max_workers=8) as ex:
    results = list(ex.map(work, targets))

# biomes-exempt повторный проход для AIOOBE-проб (cmp420_chunk2 прецедент Л-474-C88.2)
for d in results:
    if d.get("verdict") == "FIXTURE-INVALID" and d.get("aioobe") and not d.get("aioobe_other"):
        d2 = norm_one(d["run_id"], exempt=True)
        if d2.get("verdict") == "NORM-COMPUTED":
            d2["branch"] = d["branch"]
            d2["exempt_applied"] = True
            results[results.index(d)] = d2

# env-обогащение
for d in results:
    d.update(env_of(d["run_id"]))

reg = {"tool": "absorb_512", "tick": "x512", "n": len(results), "results": results}
json.dump(reg, open("/home/z/rounds/ROUND-512/absorb/registry_512.json", "w"), indent=1, ensure_ascii=False)

# §3-фид: vanilla (lever=='') ∧ CLEAN ∧ band ∧ pop==150000 ∧ datapack=='' ∧ norm ≤15
feed = []
for d in results:
    if (d.get("verdict") == "NORM-COMPUTED" and d.get("m1_state") == "CLEAN"
            and d.get("lever", "x") == "" and d.get("pop") == 150000
            and d.get("datapack", "x") == "" and (d.get("norm_v5") or 99) <= 15.0):
        feed.append({"branch": d["branch"], "run_id": d["run_id"], "cpu": d["cpu_index"],
                     "norm": d["norm_v5"], "seed": d.get("seed"),
                     "strict": d.get("norm_c42_robust")})
feed.sort(key=lambda x: x["cpu"])
bf = {"tool": "absorb_512", "tick": "x512", "n_feed": len(feed), "points": feed}
json.dump(bf, open("/home/z/rounds/ROUND-512/absorb/bank_feed_512.json", "w"), indent=1)

from collections import Counter
cls = Counter(d.get("verdict", "ERR") for d in results)
print("classes:", dict(cls))
print(f"§3 feed: {len(feed)}")
print("cpu range feed:", feed[0]["cpu"] if feed else None, "-", feed[-1]["cpu"] if feed else None)
gap = [f for f in feed if 7_500_000 <= f["cpu"] <= 8_000_000]
print(f"fleet-gap [7.5,8.0]M новых точек: {len(gap)}")
nc = sum(1 for d in results if d.get("ncdfe"))
print(f"NCDFE: {nc}/{len(results)}")
