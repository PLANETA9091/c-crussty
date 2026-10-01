#!/usr/bin/env python3
"""dispatch_519_ag246.py — AG-246 verification leg: canary RED x2 root-cause fix.

Root cause (НЕ CDN): run_benchv2.sh L57 — sha512sum -c строка tectonic имела ТРИ пробела
("$TECTONIC_SHA512   tectonic.zip") -> GNU sha512sum парсит filename как " tectonic.zip"
(leading space) -> "FAILED open or read" -> FAIL=1 -> exit 42 ДО boot. Детерминировано,
re-dispatch бессмысленен пока фикс не в master. Скачивание tectonic.zip УСПЕШНО (нет
G-DL retry строк в логах run-36788080912/36788083370). Локальный репли-тест: 2sp=OK,
3sp=byte-identical error signature. Fix на моей ветке: swarm-519-246 d8a437bd (1 char).
Seed 519002: seed-gate PASS (волна-519 диапазон, без коллизий). 1 диспатч из ≤2.
"""
import json, urllib.request

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "bench-v2.yml"
REF = "swarm-519-246"  # d8a437bd — 1-char fix sha512sum tectonic

inputs = {
    "seed": "519003",
    "run_seconds": "300",
    "radius_blocks": "1136",
    "server_xmx": "10G",
    "bench_dims": "minecraft:overworld,minecraft:the_nether,minecraft:the_end",
    "cpu_band_min": "",
    "cpu_band_max": "",
}
payload = json.dumps({"ref": REF, "inputs": inputs}).encode()
req = urllib.request.Request(
    f"{API}/actions/workflows/{WF}/dispatches", data=payload, method="POST",
    headers={"Authorization": f"Bearer {TOK}", "Accept": "application/vnd.github+json",
             "Content-Type": "application/json", "User-Agent": "swarm-519-ag246"})
try:
    with urllib.request.urlopen(req) as r:
        print("POST", r.status, "seed=519002 ref=" + REF,
              "rl-remaining:", r.headers.get("X-RateLimit-Remaining"))
except urllib.error.HTTPError as e:
    print("HTTP", e.code, e.read()[:300])
