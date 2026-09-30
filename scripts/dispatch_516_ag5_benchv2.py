#!/usr/bin/env python3
"""dispatch_516_ag5_benchv2.py — BENCH-V2 canon bring-up leg (AG-5, волна-516).

Диспатч canon-ветки swarm-516-5 (commit 4ca405e) на bench-v2.yml:
  canon = AG-433/104 линия (yml entry bench/worldv2/run_benchv2.sh)
        + AG-234 async wall-clock heartbeat драйвер (FAKE-GREEN класс F фикс)
        + plugin nether/end forceload (setChunkForceLoaded, vanilla=overworld-only)
        + 6 fake players (0 игроков = 0 natural spawn фикс)
        + Tectonic 3.0.25 канон-пин (3.0.29 = FATAL-алиас, sha512 7b3c5dee...)
Seed-gate: сверка с SEED-LEDGER волны-515 (AG-173, seed_ledger_515.json) до POST.
Band-гейт отключён (пустые cpu_band_min/max): bring-up лега, не pair-нога
(новая генерация раннеров ~11.2M вне канона 6.0-9.5M = BAND-DISCARD x515 урок 10).
"""
import json, os, subprocess, urllib.request

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "bench-v2.yml"
REF = "swarm-516-5"
SEED = "1549"  # next-free по SEED-LEDGER x515 (блок волны-516)

# --- seed-gate (канон ×516: коллизии ×19/8 групп) -----------------------------
LEDGER = "/home/z/rounds/ROUND-515/work/AG-173/seed_ledger_515.json"
if os.path.exists(LEDGER):
    data = json.load(open(LEDGER))
    allocated = set(data.get("seeds_allocated", [])) if isinstance(data, dict) else set(data)
    collisions = set()
    if isinstance(data, dict):
        for v in data.get("collisions", {}).values() if isinstance(data.get("collisions"), dict) else []:
            pass
    assert int(SEED) not in allocated, f"SEED-GATE FAIL: seed {SEED} занят в {LEDGER}"
    print(f"seed-gate PASS: seed {SEED} свободен ({len(allocated)} занято в леджере)")

inputs = {
    "seed": SEED,
    "run_seconds": "300",
    "radius_blocks": "1136",
    "server_xmx": "10G",
    "bench_dims": "minecraft:overworld,minecraft:the_nether,minecraft:the_end",
    "cpu_band_min": "",   # band-gate OFF: bring-up leg, не pair-нога
    "cpu_band_max": "",
}
payload = json.dumps({"ref": REF, "inputs": inputs}).encode()
req = urllib.request.Request(
    f"{API}/actions/workflows/{WF}/dispatches", data=payload, method="POST",
    headers={"Authorization": f"Bearer {TOK}", "Accept": "application/vnd.github+json",
             "Content-Type": "application/json", "User-Agent": "swarm-516-ag5"})
try:
    with urllib.request.urlopen(req) as r:
        print("POST", r.status, r.headers.get("X-RateLimit-Remaining"))
    print("DISPATCHED wf=bench-v2.yml ref=" + REF, "inputs:", json.dumps(inputs))
except urllib.error.HTTPError as e:
    print("HTTP", e.code, e.read()[:400])
    open("/home/z/rounds/ROUND-516/work/AG-5/disp_intent_516_ag5.json", "w").write(
        json.dumps({"wf": WF, "ref": REF, "inputs": inputs}, indent=2))
    raise SystemExit("DISP-INTENT saved")
