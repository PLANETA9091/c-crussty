#!/usr/bin/env python3
"""dispatch_470_s74_totem.py — ROUND-470 S74 (СТРЕСС-инет) — totem-jungle heavy-function stand.
Moar Totems V2.1.2 + WASD Libraries V8.2.0 (per-player held-totem function-ladder класс,
«каждый игрок держит функцию-лесенку»): world470-s74-totem-v1.zip (сервер-рождённый level.dat
DataVersion 4556, пустой region, datapacks = оригиналы + bukkit dual-mcmeta) как release-ассет.
Канон диспатчера tick-466/470: argv-guard, sha-pin, canonical anchor inputs, band 6.0-9.5M,
ref-push = git push + GET-верификация (Л188a), 1 диспатч = 1 ветка (Л188b).
Usage: dispatch_470_s74_totem.py [--dry-run] [--legs N]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

WORLD_URL = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
             "v470-s74-totem/world470-s74-totem-v1.zip")
BRANCHES = ["round-470-s74-totem", "round-470-s74-totem2"]
BASE_SHA = "b005e9ab1d8b76691bb585ce017c5ff786a8aa27"  # remote master tip (S52 board-repair); docs-only поверх (0 код-дельты)

# Банк-вектор x466-C98 (канон), world_url = totem-стенд; ваниль-кернел (lever пуст).
INPUTS = {
    "world_url": WORLD_URL,
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
    if m:
        return m.group(1)
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None, headers=None):
    h = {"Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"}
    if headers:
        h.update(headers)
    req = urllib.request.Request(url if url.startswith("http") else API + url,
                                 method=method, headers=h)
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=120) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        if e.code != 404:
            print(f"HTTP {e.code}: {e.read()[:300]}")
        raise
    return json.loads(body) if body else {}


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def ensure_release_asset(tok):
    tag = "v470-s74-totem"
    rel_url = f"{API}/repos/{REPO}/releases/tags/{tag}"
    try:
        rel = api(tok, rel_url)
        print(f"release {tag} exists: id={rel['id']}")
    except urllib.error.HTTPError:
        rel = api(tok, f"{API}/repos/{REPO}/releases", method="POST", data={
            "tag_name": tag, "target_commitish": "master",
            "name": "v470-s74-totem totem-stand (S74 x470)",
            "body": ("totem-jungle heavy-function stand (S74 x470): Moar Totems V2.1.2 "
                     "(sha1 a67da4be) + WASD Libraries V8.2.0 (sha1 7d8fa1ae); "
                     "world v2 scaffold (level.dat DataVersion 4556, empty region); "
                     "sha256 world zip 3885699df5281153f7104c86bfb26f909c49f6077767be3098efdae76f618a56"),
            "draft": False, "prerelease": False})
        print(f"release {tag} created: id={rel['id']}")
    assets = {a["name"]: a for a in rel.get("assets", [])}
    name = "world470-s74-totem-v1.zip"
    if name in assets:
        print(f"asset present: {name} size={assets[name]['size']}")
        return assets[name]["browser_download_url"]
    local = "/home/z/rounds/ROUND-470/S74/world470-s74-totem-v1.zip"
    upload = rel["upload_url"].split("{")[0] + f"?name={name}"
    data = open(local, "rb").read()
    req = urllib.request.Request(upload, data=data, method="POST", headers={
        "Authorization": f"Bearer {tok}",
        "Content-Type": "application/zip", "Content-Length": str(len(data))})
    with urllib.request.urlopen(req, timeout=300) as r:
        out = json.loads(r.read())
    print(f"asset uploaded: {out['name']} size={out['size']}")
    return out["browser_download_url"]


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()

    url = ensure_release_asset(tok)
    # anon-URL канон: browser_download_url (без токена)
    print(f"world_url: {url}")
    if INPUTS["world_url"] != url:
        raise SystemExit(f"URL MISMATCH: script={INPUTS['world_url']} live={url}")

    for br in BRANCHES:
        # sha-pin канон Л188a: ветка = docs-only цепочка ПОВЕРХ base — compare-гейт:
        # дифф branch..base обязан содержать ТОЛЬКО docs-файлы стенда (0 код-дельты)
        cmp = api(tok, f"/repos/{REPO}/compare/{BASE_SHA[:12]}...{br}")
        files = {f["filename"] for f in cmp.get("files", [])}
        allowed = {"research/RESEARCH-S74-TOTEM.md", "scripts/dispatch_470_s74_totem.py"}
        if not files or not files.issubset(allowed):
            raise SystemExit(f"SHA MISMATCH: {br} diff-files={sorted(files)} allowed={sorted(allowed)}")
        print(f"preflight OK: {br} @ {cmp['commits'][-1]['sha'][:8]} "
              f"(docs-only diff vs {BASE_SHA[:8]}: {len(files)} files)", flush=True)
    print(f"inputs: {json.dumps(INPUTS, ensure_ascii=False)}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return
    run_ids = []
    for br in BRANCHES:
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": INPUTS})
        print(f"dispatched: {br} totem-stand band=[6.0,9.5]M gc3 pop150k fp4", flush=True)
        time.sleep(4)
    print(f"=== ROUND-470 S74 BATCH COMPLETE: {len(BRANCHES)} диспатчей ===", flush=True)


if __name__ == "__main__":
    main()
