#!/usr/bin/env python3
"""dispatch_479_a9_bn.py — [479-A9] F9 пост-мортем + BN 6.9.8 retry на №17-мастере.

Пост-мортем 36371358886 (галочка (1), log-строка):
  [world3 02:53:18Z] FATAL: level.dat parent has no region/:
    /home/runner/work/c-crussty/c-crussty/world3-run/worldx/world
  + find: Permission denied ×4 (world/{DIM1,datapacks,DIM-1,region}).
  Причина: v3-зип world466-bn-stress-v3.zip хранит ВСЕ каталог-энтрии с mode
  0o600 (нет x-бита) → Info-ZIP unzip восстанавливает непроходимые dirs →
  preflight [ -d "$WORLD_SRC/region" ] ложен → die ДО бута (exit 1, ~0s в POP).
  HB1 «долго на POP-фазе» ОПРОВЕРГНУТ логом: рана сгорела на доставке
  fixture (02:50:30 → 02:53:18, boot не начат). Класс: G3.0 FIXTURE-VALIDITY
  (не G3.1 quiesce/emap game-logic — тот резидентен в мастере через МЕРЖ №17
  f66feb1b, что закрывает game-logic-плечо класса на retry).

Retry (галочка (2)): BN 6.9.8 на новом master @9dd45aad (содержит f66feb1b
  МЕРЖ №17 = G3.1 quiesce-arm-all-paths + emap-superset resident) с v4-фикстурой
  world478-bn-stress-v4.zip (re-pack v3, dirs 0o755, file-байты бит-идентичны,
  7/7 content parity, zip CRC clean; sha256 510c8be3b5e645560244aeb9f2e60f5cb6f42097b3d771aa37c02ac83c6162c1,
  7073243 B, релиз v478-f9-bn-world — download-верифицирован 479-A9). Гейты:
  boot Done, AIOOBE=0, POP VALID. H-A9 (prereg, закон 18-iii): fresh-gen BN
  gen-шторм класса C63 → wall >15 мин → DISPATCHED run-id; SUCCESS ≤15 мин →
  гейты + dp-ось чтение (C74, пар vs ваниль-якоря не ждать). Канон x466-C98
  ЯВНЫМ JSON, band [6.0,9.5]M fast-fail, 1 реф=1 диспатч Л188a/b.

Usage: dispatch_479_a9_bn.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-479-a9-bn"
PIN_SHA = "9dd45aad29c7543367fe087155d43a3e7b211fd4"  # master tip: f66feb1b МЕРЖ №17 (G3.1 quiesce + emap resident) + board 479

BN_V4_URL = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
             "v478-f9-bn-world/world478-bn-stress-v4.zip")

# canon x466-C98 ЯВНЫМ JSON (дефолты yml = merge-поверхность), lever=''/'' —
# стенд меряет dp-мир (BN 6.9.8), не lever.
INPUTS = {
    "world_url": BN_V4_URL,
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
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    return m.group(1) if m else open("/tmp/gh_token").read().strip()


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


def ensure_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    cur = r.get("object", {}).get("sha")
    if cur == sha:
        print(f"{name}: GET-proof exists @ {sha[:12]} (Л188a)")
        return
    if cur:
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        print(f"{name}: moved -> {sha[:12]}")
        return
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})
    print(f"{name}: created FULL-sha @ {sha[:12]}")


def dispatch(tok, ref):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": INPUTS})
    print(f"dispatch {ref} -> {'204 OK' if r == {} else r}")
    return r == {}


def latest_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"]}
    return None


def main():
    dry = "--dry-run" in sys.argv
    tok = token()
    # новый master жив на origin? f66feb1b (МЕРЖ №17) должен быть предком PIN_SHA
    r = api(tok, f"/repos/{REPO}/commits/{PIN_SHA}")
    if "sha" not in r:
        print("PIN_SHA не найден на origin — STOP")
        sys.exit(3)
    print(f"PIN {PIN_SHA[:12]} live on origin (parents {r.get('parents',[{}])[0].get('sha','')[:12]})")
    if dry:
        print(json.dumps({"branch": BRANCH, "pin": PIN_SHA, "inputs": INPUTS}, indent=1))
        return
    ensure_alias(tok, BRANCH, PIN_SHA)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    ok = dispatch(tok, BRANCH)
    if not ok:
        sys.exit(2)
    time.sleep(30)
    run = latest_run(tok, BRANCH, mc)
    print(json.dumps(run, indent=1))


if __name__ == "__main__":
    main()
