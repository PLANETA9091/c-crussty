#!/usr/bin/env python3
"""act_526_37.py — AG-37 волна-526: primary sim21+sim27 миды sim-оси (зазор 20-28,
0-клейм; соседи sim19 AG-259 / sim24 AG-195 / sim29 AG-271), fallback sim44+sim56
(sim-верх зазор 40-64 между sim40 AG-279 и sim64 AG-267), fallback2 w7936+w5376
w-миды. Канон-носитель sim-оси PIN 2171d6da (sim-distance plumbing AG-138), fp4/
r1136/9000s/dcp900, zero-code. Tree FULL>=3200, yml-inputs assert, CAS-CLAIM с
regex-boundary sib-check, refs API + вериф sha==PIN, 2 POST (31s гэп, AG-338),
вериф queued + head_sha==PIN, payload JSON. --dry = чеки без записи."""
import base64, json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
DRY = "--dry" in sys.argv
PAIR = ("fallback2" if "--fallback2" in sys.argv
        else "fallback" if "--fallback" in sys.argv else "primary")
WF = {"primary": "bench-v2.yml", "fallback": "bench-v2.yml",
      "fallback2": "bench-v2.yml"}[PAIR]
PIN = "2171d6da775975c4bee94748f549ad16f02074e1"  # sim-канон AG-138 x525

CLAIMS = {
 "primary": ("CLAIM | AG-37 | sim21+sim27 миды sim-оси (зазор 20-28, 0-клейм): "
             "fp4/r1136/9000s/dcp900 @2171d6da | 2 POST"),
 "fallback": ("CLAIM | AG-37 | sim44+sim56 миды sim-верх (зазор 40-64, 0-клейм): "
              "fp4/r1136/9000s/dcp900 @2171d6da | 2 POST"),
 "fallback2": ("CLAIM | AG-37 | w7936+w5376 w-миды (7680-8192/4608-6912, "
               "0-клейм): 1d/9000s/dcp900 @2171d6da | 2 POST"),
}
CLAIM = CLAIMS[PAIR]
# regex-boundary dedup (урок доски: substring врёт, sim1 ловит sim10/100)
SIB_RX = {
 "primary": [r"sim21(?![0-9])", r"sim27(?![0-9])"],
 "fallback": [r"sim44(?![0-9])", r"sim56(?![0-9])"],
 "fallback2": [r"w7936(?![0-9])", r"w5376(?![0-9])"],
}[PAIR]
MINE_RX = r"CLAIM \| AG-37 \| sim" if PAIR != "fallback2" else \
          r"CLAIM \| AG-37 \| w7936"
SIB_KEYS_SAFE = [k.replace("(?![0-9])", "") for k in SIB_RX]

SIM_BASE = {"radius_blocks": "1136", "run_seconds": "9000",
            "server_xmx": "10G", "bench_dims": "minecraft:overworld",
            "dim_gen_window": "256", "drain_cap_polls": "900",
            "fake_players": "4"}
LEGS = {
 "primary": [
   ("swarm-526-37", "525037", dict(SIM_BASE, seed="525037",
                                   simulation_distance="21"), "sim21"),
   ("swarm-526-37b", "526037", dict(SIM_BASE, seed="526037",
                                    simulation_distance="27"), "sim27"),
 ],
 "fallback": [
   ("swarm-526-37", "525037", dict(SIM_BASE, seed="525037",
                                   simulation_distance="44"), "sim44"),
   ("swarm-526-37b", "526037", dict(SIM_BASE, seed="526037",
                                    simulation_distance="56"), "sim56"),
 ],
 "fallback2": [
   ("swarm-526-37", "525037", {"server_xmx": "10G", "radius_blocks": "1136",
    "run_seconds": "9000", "bench_dims": "minecraft:overworld",
    "dim_gen_window": "7936", "drain_cap_polls": "900",
    "seed": "525037"}, "w7936"),
   ("swarm-526-37b", "526037", {"server_xmx": "10G", "radius_blocks": "1136",
    "run_seconds": "9000", "bench_dims": "minecraft:overworld",
    "dim_gen_window": "5376", "drain_cap_polls": "900",
    "seed": "526037"}, "w5376"),
 ],
}[PAIR]
YML_NEED = ["radius_blocks", "run_seconds", "seed", "server_xmx", "bench_dims",
            "dim_gen_window", "drain_cap_polls", "fake_players",
            "simulation_distance"]
OUT = "/home/z/rounds/ROUND-526/work/AG-37/dispatch_526_37.json"


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote",
                          "get-url", "origin"], capture_output=True,
                         text=True).stdout
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url.strip())
    return m.group(1) if m else open("/tmp/gh_token").read().strip()


TOK = token()


def api(url, method="GET", data=None, tries=5):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {TOK}",
        "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    for a in range(tries):
        try:
            with urllib.request.urlopen(req, payload, timeout=90) as r:
                body = r.read().decode()
                return r.status, (json.loads(body) if body.strip() else {})
        except urllib.error.HTTPError as e:
            code = e.code
            raw = e.read().decode()[:160]
            if code in (403, 409, 422, 429) and a < tries - 1:
                print(f"HTTP {code} retry {a}: {raw[:80]}", flush=True)
                time.sleep(4 * (a + 1))
                continue
            return code, {"_raw": raw}
        except Exception as ex:
            if a < tries - 1:
                time.sleep(4 * (a + 1))
                continue
            return 0, {"_raw": str(ex)[:160]}
    return 0, {"_raw": "retries exhausted"}


print(f"PAIR={PAIR} WF={WF} PIN={PIN[:12]}", flush=True)
assert len(CLAIM) <= 120, f"claim {len(CLAIM)}>120"

# --- 0. PIN жив, tree FULL >=3200 (эра-канон 2026-10-01) ---
code, c = api(f"/repos/{REPO}/commits/{PIN}")
assert code == 200, f"pin resolve {code}"
tree_sha = c["commit"]["tree"]["sha"]
code, t = api(f"/repos/{REPO}/git/trees/{tree_sha}?recursive=1")
assert code == 200 and not t.get("truncated", True), f"tree {code} trunc"
n = len(t["tree"])
print(f"PIN msg={c['commit']['message'][:60]!r} tree={tree_sha[:8]} "
      f"files={n} {'FULL' if n >= 3200 else 'SPARSE-FATAL'}", flush=True)
assert n >= 3200, "sparse tree — откат"

# --- 0b. yml inputs на PIN (урок AG-216/276: PIN-проверку yml ДО выбора пары) ---
code, y = api(f"/repos/{REPO}/contents/.github/workflows/{WF}?ref={PIN}")
assert code == 200, f"yml {code}"
yml = base64.b64decode(y["content"]).decode()
missing = [k for k in YML_NEED if k not in yml]
assert not missing, f"yml missing inputs {missing}"
print(f"yml inputs OK ({len(YML_NEED)})", flush=True)
if DRY:
    print("DRY OK — claim:", CLAIM, f"({len(CLAIM)} ch)", flush=True)
    sys.exit(0)

# --- 1. CAS-append CLAIM (live GET + regex-boundary sib-check) ---
claimed = False
for a in range(8):
    code, d = api(f"/repos/{REPO}/contents/SHARED_BOARD.md?ref=master")
    if code != 200:
        print(f"board GET-err {code}", flush=True)
        time.sleep(4)
        continue
    sha, content = d["sha"], base64.b64decode(d["content"]).decode()
    if re.search(MINE_RX, content):
        print("CLAIM already mine — continue", flush=True)
        claimed = True
        break
    hit = [k for k in SIB_RX if re.search(k, content)]
    if hit:
        print(f"PIVOT: sib took {hit} — EXIT 3 (CAS-лаг канон)", flush=True)
        sys.exit(3)
    new = content + ("" if content.endswith("\n") else "\n") + CLAIM + "\n"
    code, r = api(f"/repos/{REPO}/contents/SHARED_BOARD.md", "PUT", {
        "message": "board: AG-37 wave-526 claim " + "+".join(
            k.replace("(?![0-9])", "") for k in SIB_KEYS_SAFE) +
        " (append-only)",
        "content": base64.b64encode(new.encode()).decode(),
        "sha": sha, "branch": "master"})
    if code == 200 and r.get("commit", {}).get("sha"):
        print(f"CLAIM APPEND OK commit={r['commit']['sha'][:8]}", flush=True)
        claimed = True
        break
    print(f"retry {a}: {code} {str(r.get('message'))[:60]}", flush=True)
    time.sleep(3)
assert claimed, "claim not appended"

# --- 2. снапшот ран-идов до диспатча (чистая атрибуция, AG-276) ---
code, pre = api(f"/repos/{REPO}/actions/runs?per_page=100")
pre_ids = {r["id"] for r in pre.get("workflow_runs", [])} if code == 200 else set()
print(f"pre-snapshot runs={len(pre_ids)}", flush=True)

# --- 3. refs + диспатчи (<=2/агента; POST-ы >=30с — урок AG-338) ---
runids, fails = {}, []
for i, (ref, seed, inputs, label) in enumerate(LEGS):
    code, _ = api(f"/repos/{REPO}/git/refs", "POST",
                  {"ref": f"refs/heads/{ref}", "sha": PIN})
    ok = code == 201
    if not ok:
        code2, g = api(f"/repos/{REPO}/git/ref/heads/{ref}")
        ok = code2 == 200 and g.get("object", {}).get("sha", "") == PIN
    code3, v = api(f"/repos/{REPO}/git/ref/heads/{ref}")
    ver = v.get("object", {}).get("sha", "") if code3 == 200 else ""
    print(f"branch {ref}: created={ok} sha={ver[:8]} match={ver == PIN}",
          flush=True)
    if ver != PIN:
        fails.append(f"ref-mismatch {ref}")
        sys.exit(f"FATAL sha mismatch {ref}")
    code, d2 = api(f"/repos/{REPO}/actions/workflows/{WF}/dispatches", "POST",
                   {"ref": ref, "inputs": inputs})
    okd = code in (200, 204)
    print(f"dispatch {ref} {label} seed={seed}: HTTP {code} "
          f"{'OK' if okd else d2}", flush=True)
    if not okd:
        fails.append(f"dispatch {ref} -> {code}")
        sys.exit(f"FATAL dispatch {ref} -> {code}")
    if i == 0:
        time.sleep(31)  # урок AG-338: 2-й POST <4с = 204 без run

# --- 4. вериф queued + head_sha==PIN ---
time.sleep(10)
code, d3 = api(f"/repos/{REPO}/actions/runs?per_page=60")
for run in d3.get("workflow_runs", []) if code == 200 else []:
    if run["head_branch"] in {b for b, _, _, _ in LEGS} and \
            run["id"] not in pre_ids:
        runids[run["head_branch"]] = run["id"]
        print(f"RUN {run['id']} {run['head_branch']} sha={run['head_sha'][:8]} "
              f"status={run['status']} "
              f"match={'OK' if run['head_sha'] == PIN else 'MISMATCH'}",
              flush=True)
if len(runids) != 2:
    print("WARN: not both runs visible — sha-атрибуция в вериф-скрипте",
          flush=True)

json.dump({"pin": PIN, "tree_files": n, "claim": CLAIM, "wf": WF,
           "runs": runids, "dispatch_fails": fails,
           "legs": [{"branch": b, "seed": s, "label": lb, "inputs": inp}
                    for b, s, inp, lb in LEGS]},
          open(OUT, "w"), indent=1, ensure_ascii=False)
print("PAYLOAD saved", flush=True)
