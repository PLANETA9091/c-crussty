#!/usr/bin/env python3
"""TASK-438-C retag: add cmp437_chunk4 (R7 carrier, chunk-send snapshot
widening) to every STRICT-OR gate site that already carries cmp435_chunk3.
Mechanical law-7 composition, byte-exact per site (19 rust sites across 19
files; java sites edited separately). Idempotent: skips already-retagged
sites."""
import pathlib
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent

# (relative path, old, new, expected_count)
EDITS = [
    ("src/collide_batch.rs",
     '|| v == "cmp434_chunkpl" || v == "cmp435_chunk3"',
     '|| v == "cmp434_chunkpl" || v == "cmp435_chunk3" || v == "cmp437_chunk4"',
     2),
    ("src/colpush.rs",
     't == "cmp434_chunkpl" || t == "cmp435_chunk3"',
     't == "cmp434_chunkpl" || t == "cmp435_chunk3" || t == "cmp437_chunk4"',
     1),
    ("src/entity_query.rs",
     '| Ok("cmp434_chunkpl") | Ok("cmp435_chunk3")',
     '| Ok("cmp434_chunkpl") | Ok("cmp435_chunk3") | Ok("cmp437_chunk4")',
     2),
    ("src/entity_query.rs",
     '|| s == "cmp434_chunkpl" || s == "cmp435_chunk3"',
     '|| s == "cmp434_chunkpl" || s == "cmp435_chunk3" || s == "cmp437_chunk4"',
     1),
    ("src/goal_selector.rs",
     '|| v == "cmp434_chunkpl" || v == "cmp435_chunk3"',
     '|| v == "cmp434_chunkpl" || v == "cmp435_chunk3" || v == "cmp437_chunk4"',
     1),
    ("src/inside_bitmask.rs",
     'v.trim() == "cmp434_chunkpl" || v.trim() == "cmp435_chunk3")',
     'v.trim() == "cmp434_chunkpl" || v.trim() == "cmp435_chunk3" || v.trim() == "cmp437_chunk4")',
     1),
    ("src/inside_snap.rs",
     'v.trim() == "cmp434_chunkpl" || v.trim() == "cmp435_chunk3")',
     'v.trim() == "cmp434_chunkpl" || v.trim() == "cmp435_chunk3" || v.trim() == "cmp437_chunk4")',
     1),
    ("src/items_index.rs",
     '|| v == "cmp434_chunkpl" || v == "cmp435_chunk3"',
     '|| v == "cmp434_chunkpl" || v == "cmp435_chunk3" || v == "cmp437_chunk4"',
     1),
    ("src/items_manager.rs",
     '|| f == "cmp434_chunkpl" || f == "cmp435_chunk3"',
     '|| f == "cmp434_chunkpl" || f == "cmp435_chunk3" || f == "cmp437_chunk4"',
     1),
    ("src/items_manager.rs",
     '|| flag == "cmp434_chunkpl" || flag == "cmp435_chunk3";',
     '|| flag == "cmp434_chunkpl" || flag == "cmp435_chunk3" || flag == "cmp437_chunk4";',
     1),
    ("src/mobs_ai.rs",
     '| Ok("cmp434_chunkpl") | Ok("cmp435_chunk3")',
     '| Ok("cmp434_chunkpl") | Ok("cmp435_chunk3") | Ok("cmp437_chunk4")',
     1),
    ("src/mobs_grid.rs",
     '|| v == "cmp434_chunkpl" || v == "cmp435_chunk3"',
     '|| v == "cmp434_chunkpl" || v == "cmp435_chunk3" || v == "cmp437_chunk4"',
     1),
    ("src/mobs_manager.rs",
     '|| f == "cmp434_chunkpl" || f == "cmp435_chunk3"',
     '|| f == "cmp434_chunkpl" || f == "cmp435_chunk3" || f == "cmp437_chunk4"',
     3),
    ("src/mobs_soa.rs",
     '|| f == "cmp434_chunkpl" || f == "cmp435_chunk3"',
     '|| f == "cmp434_chunkpl" || f == "cmp435_chunk3" || f == "cmp437_chunk4"',
     2),
    ("src/mobs_sscan.rs",
     '| Ok("cmp434_chunkpl") | Ok("cmp435_chunk3")',
     '| Ok("cmp434_chunkpl") | Ok("cmp435_chunk3") | Ok("cmp437_chunk4")',
     1),
    ("src/nav_plane.rs",
     '|| v == "cmp434_chunkpl" || v == "cmp435_chunk3"',
     '|| v == "cmp434_chunkpl" || v == "cmp435_chunk3" || v == "cmp437_chunk4"',
     1),
    ("src/noise_fill.rs",
     '|| v == "cmp434_chunkpl" || v == "cmp435_chunk3"',
     '|| v == "cmp434_chunkpl" || v == "cmp435_chunk3" || v == "cmp437_chunk4"',
     1),
    ("src/queryplane.rs",
     '|| v.trim() == "cmp434_chunkpl" || v.trim() == "cmp435_chunk3"',
     '|| v.trim() == "cmp434_chunkpl" || v.trim() == "cmp435_chunk3" || v.trim() == "cmp437_chunk4"',
     1),
    ("src/queryplane.rs",
     'Ok("cmp435_chunk3") => "cmp435_chunk3", // TASK-435-C: R6 carrier marker id',
     'Ok("cmp435_chunk3") => "cmp435_chunk3", // TASK-435-C: R6 carrier marker id\n'
     '        Ok("cmp437_chunk4") => "cmp437_chunk4", // TASK-438-C: R7 carrier marker id (chunk-send snapshot widening)',
     1),
    ("src/stagger.rs",
     '| Ok("cmp434_chunkpl") | Ok("cmp435_chunk3")',
     '| Ok("cmp434_chunkpl") | Ok("cmp435_chunk3") | Ok("cmp437_chunk4")',
     1),
    ("src/tickplane.rs",
     '|| v == "cmp434_chunkpl" || v == "cmp435_chunk3"',
     '|| v == "cmp434_chunkpl" || v == "cmp435_chunk3" || v == "cmp437_chunk4"',
     1),
    ("src/chunk_parse.rs",
     '                || v == "cmp435_chunk3"\n',
     '                || v == "cmp435_chunk3"\n                || v == "cmp437_chunk4"\n',
     1),
]


def main() -> int:
    failures = 0
    for rel, old, new, expected in EDITS:
        path = REPO / rel
        text = path.read_text()
        n = text.count(old)
        if n != expected:
            print(f"FAIL {rel}: expected {expected} occurrence(s) of {old[:60]!r}, found {n}")
            failures += 1
            continue
        if new in text and old not in text:
            print(f"SKIP {rel}: already retagged")
            continue
        path.write_text(text.replace(old, new))
        print(f"OK   {rel}: {expected} site(s) retagged")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
