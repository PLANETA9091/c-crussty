"""absorb_boot424.py — TASK-425-C boot-Done metric (player-visible chunk axis).

Usage: python3 absorb_boot424.py <run_id> <tag>
Parses the ALREADY-ABSORBED run dir (research/gc-recon-2026-09-19/round-<tag>/)
and extracts from server-stdout.log:
  * boot_done_s  — "Done (X.XXXs)!" (vanilla line, no harness change);
  * ramp_polls   — first 3 soak TPS polls (the post-Done ramp window);
  * tps_med      — median of all sub-20 polls (pairs with absorb_round.py).
Appends a BOOT-METRIC section to the run dir ABSORB.md and prints JSON.
"""
import json, os, re, statistics, sys

RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"


def main():
    run_id = int(sys.argv[1])
    tag = sys.argv[2] if len(sys.argv) > 2 else f"run{run_id}"
    run_dir = os.path.join(RESDIR, f"round-{tag}")
    out = os.path.join(run_dir, "server-stdout.log")
    if not os.path.isfile(out):
        print(json.dumps({"tag": tag, "run_id": run_id, "error": "no server-stdout.log (absorb first)"}))
        return 1
    txt = open(out, errors="ignore").read()
    m = re.search(r"Done \(([\d.]+)s\)!", txt)
    boot_done_s = float(m.group(1)) if m else None
    polls = [float(x) for x in re.findall(r"TPS from last 5s, 1m, 5m, 15m: ([\d.]+)", txt)
             if float(x) < 20]
    ramp = polls[:3]
    med = statistics.median(polls) if polls else None
    rep = [
        "",
        "## BOOT-METRIC (TASK-425-C, player-visible chunk axis)",
        f"- boot_done_s: **{boot_done_s:.3f}s**" if boot_done_s is not None
        else "- boot_done_s: NOT FOUND (no 'Done (X.XXXs)!' line)",
        f"- ramp_polls (first 3 soak): {ramp}",
        f"- tps_med (all soak polls): {med}",
    ]
    with open(os.path.join(run_dir, "ABSORB.md"), "a", encoding="utf-8") as f:
        f.write("\n".join(rep) + "\n")
    print(json.dumps({"tag": tag, "run_id": run_id, "boot_done_s": boot_done_s,
                      "ramp_polls": ramp, "tps_med": med, "polls_n": len(polls)}))
    return 0


if __name__ == "__main__":
    sys.exit(main())
