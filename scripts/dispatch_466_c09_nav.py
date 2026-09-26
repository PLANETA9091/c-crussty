#!/usr/bin/env python3
"""dispatch_466_c09_nav.py — TASK-466-C09 (КЛИМБ C63 navmath activate() вайринг).

x465 находка: move_plane::activate() мёртв в lib.rs (register без activate) —
все navmath-ноги эры = A/A. Вайринг f8502810 будит плоскость. Этот скрипт
диспатчит канон-банк ногу с lever cmp463_move и ВЕРИФИЦИРУЕТ lever ДО запуска
(navmath-1 урок: несобранный lever = гарантированный A/A).

РЕЖИМ: python3 scripts/dispatch_466_c09_nav.py [--dry]
"""
import json, subprocess, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
BENCH_WF = "world-bench-parallel.yml"

BRANCH = "round-466-c09-nav"
LEVER_FLAG = "cmp463_move"
LEVER_ARG = "c09nav"

INPUTS = {
    "lever_flag": LEVER_FLAG, "lever_arg": LEVER_ARG,
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1",
    "flush_diet": "1", "region_threads": "4", "batch_collector": "1",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
}


def tok():
    return open("/tmp/gh_token").read().strip()


def preflight():
    """navmath-1 lesson: verify the mechanism exists BEFORE dispatch."""
    src = open("src/lib.rs").read()
    assert "move_plane::activate();" in src, "lib.rs does not CALL move_plane::activate()"
    assert "move_plane::register();" in src, "lib.rs does not register the move byte hook"
    eq = open("src/entity_query.rs").read()
    assert 'Some("cmp463_move")' in eq, "entity_query define gate lost the move lever"
    assert "PURE-MOVE DELTA GUARD" in eq, "pure-move guard missing (contamination risk)"
    blob = open(
        "moveplane/build/net/minecraft/world/entity/ai/control/MovePlaneOps.class", "rb"
    ).read()
    assert b"cmp463_move" in blob, "MovePlaneOps blob lost the lever needle"
    assert b"moveDecide" in blob, "MovePlaneOps blob lost the native decl"
    mk = open("src/move_plane.rs").read()
    assert "pub fn activate" in mk and 'v.trim() == "cmp463_move"' in mk
    print("PREFLIGHT OK: activate wired + gates + blob needles live")


def main():
    preflight()
    payload = {"ref": BRANCH, "inputs": INPUTS}
    data = json.dumps(payload).encode()
    if "--dry" in sys.argv:
        print("DRY:", BRANCH, INPUTS)
        return 0
    req = urllib.request.Request(
        f"{API}/repos/{REPO}/actions/workflows/{BENCH_WF}/dispatches",
        data=data, method="POST",
        headers={"Authorization": f"token {tok()}",
                 "Accept": "application/vnd.github+json",
                 "Content-Type": "application/json",
                 "User-Agent": "task466-c09-nav"})
    try:
        with urllib.request.urlopen(req) as r:
            print(f"DISPATCH {BRANCH} lever={LEVER_FLAG}/{LEVER_ARG} HTTP {r.status}")
            return 0 if r.status == 204 else 1
    except urllib.error.HTTPError as e:
        print(f"DISPATCH FAIL HTTP {e.code}: {e.read()[:200]}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
