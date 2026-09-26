#!/usr/bin/env python3
"""dispatch_464_climb5a.py — TASK-464-29 (тик-464, закон 13) — chkclimb-5 нога-1, climb5-p32-1.

BATCH-ОПИСАНИЕ (нога-1 из батча TASK-464-29 chkclimb-5⊕P32):
  ветка  : round-464-climb5-p32-1 = origin/round-460-chkclimb-5 @ 4bcabb2f
           ("reroll-460 climb-5 wave-2", дерево == chkclimb-1 2d6c7ed6: P31
           INSIDE-BATCH + P32 snapreg sidecar на носителе cmp456_chunkmono)
  lever  : cmp456_chunkmono_p31snap — СОСТАВНОЙ (не соло):
           * P31 carrier src/inside_batch.rs enabled(): STRICT eq
             `cmp456_chunkmono_p31snap` (или env CRUSSTY_INSIDE_BATCH) —
             ПРИНИМАЕТ составной флаг;
           * P32 carrier src/inside_snap_registry.rs enabled(): STRICT eq
             `cmp459_snapreg` ИЛИ `cmp456_chunkmono_p31snap` (или env
             CRUSSTY_SNAPREG=1) — ПРИНИМАЕТ составной флаг.
           ⇒ соло-фолбэк cmp459_snapreg НЕ нужен (гипотеза-дельта соло
           +1.5-2.5пп не активируется — стреляем полной композицией).
  Армы   : blob InsideSnapRegistryOps.class md5 7b0e8e305fdc / 6072B /
           13 полей / 0 native (md5-контракт worklog:7712, javap -p -c
           подтверждён: гейт-строк в классе нет — флаг-гейт живёт в
           rust-бридже, класс определяется ТОЛЬКО при прохождении гейта).
  База   : chkclimb-5 нога +15.6 v4 / +18.40 v5; бар E-окна
           [6427199,6527199] norm_v5 <= -1.60 — не хватает -1.6пп.

ГИПОТЕЗА-ДЕЛЬТА (почему НЕ слепой ре-ролл +15.6):
  Л63: sidecar dDelta/dh = 0.011пп/пп на p31snap-носителе; композиция
  P31+P32 прогноз ноги +20..+21 => shallow-пары (>=20-бар хит в пару к
  a127 +23.8). Маркер-коммит разносит ре-роллы по хешам (INFRA-DUP-канон,
  law-15 wave): пустой коммит 4bcabb2f уже был таким маркером wave-2; этот
  — маркер wave-464 с javap-аудитом арм-пути P32.

ПРЕФЛАЙТ-ФАКТЫ (проверено перед диспатчем):
  - inside_batch::enabled() STRICT eq составной флаг: OK (src/inside_batch.rs:57-73)
  - inside_snap_registry::enabled() принимает cmp456_chunkmono_p31snap: OK
    (src/inside_snap_registry.rs:85-99)
  - javap -p -c InsideSnapRegistryOps: 13 полей, 0 native, ldc-строк
    lever-флагов 0 (гейт в rust, класс define-gated) — fail-closed.

УРОК ×447: argv-guard ПЕРЕД любым действием. --dry-run = только префлайт.
"""
import json, subprocess, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
WF = "world-bench-parallel.yml"
BRANCH = "round-464-climb5-p32-1"
BASE_SHA = "4bcabb2f"  # origin/round-460-chkclimb-5 (chkclimb-5 canon, нога +15.6@6477199)

LEVER = "cmp456_chunkmono_p31snap"  # СОСТАВНОЙ P31+P32 (оба гейта принимают — см. докстринг)

INPUTS = {
    "lever_flag": LEVER, "lever_arg": "1",
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "region_threads": "4", "batch_collector": "1",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
}


def main() -> int:
    dry = "--dry-run" in sys.argv
    if not dry and len(sys.argv) != 2:
        print("usage: dispatch_464_climb5a.py [--dry-run]  (argv-guard, урок x447)")
        return 2
    token = open("/tmp/gh_token").read().strip()
    payload = {"ref": BRANCH, "inputs": INPUTS}
    print(f"preflight: branch={BRANCH} base={BASE_SHA} lever={LEVER}")
    print(f"inputs={json.dumps(INPUTS, sort_keys=True)}")
    if dry:
        print("dry-run: no dispatch")
        return 0
    req = urllib.request.Request(
        f"https://api.github.com/repos/{REPO}/actions/workflows/{WF}/dispatches",
        data=json.dumps(payload).encode(),
        headers={
            "Authorization": f"token {token}",
            "Accept": "application/vnd.github+json",
        },
        method="POST",
    )
    with urllib.request.urlopen(req) as resp:
        print(f"dispatch: HTTP {resp.status} (204 = OK)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
