#!/usr/bin/env python3
"""recon12_store_firehose.py — TASK-317 absorb: attribute the old->young
store-firehose from the INSTRUMENTED RECON leg artifacts (run with
recon_diag=1 on the exact v3 bank).

Inputs (artifact dir):
  - recon.jfr     : JFR profile recording (jdk.OldObjectSample = objects
                    reaching the old gen WITH allocation stack = producer
                    attribution; jdk.ObjectAllocationSample = control alloc
                    profile; both ACTIVE in Temurin 21 profile.jfc,
                    memory-leaks default=stack-traces)
  - remset.log    : -Xlog:gc+remset=debug (dirty-card intensity per GC)
  - refine.log    : -Xlog:gc+refine=debug

Usage:  python3 recon12_store_firehose.py <artifact_dir>

Decision rule (preregistered CLAIMS TASK-316 NEXT):
  entity-field share of old->young producers >= 40% AND model skip ceiling
  >= 2-3% wall  ->  GO lever #13 SKIP-STORE-DIET (preregister + lockstep
  oracle >= 1M + identity-use grep)   else  ->  paper-REFUTED #13.

C2-inlined setters (lesson #8 mirror) mean producer FRAMES above the
store are the attribution key: family = nearest recognizable ancestor in
the allocation stack. Percentages are object-count-based.
"""
import collections, re, subprocess, sys, os, shutil

def resolve_jfr_bin():
    """jfr binary: env JFR_BIN -> PATH -> next to java -> common JDK homes."""
    cand = os.environ.get("JFR_BIN")
    if cand and os.path.exists(cand):
        return cand
    p = shutil.which("jfr")
    if p:
        return p
    jp = shutil.which("java")
    if jp:
        nxt = os.path.join(os.path.dirname(os.path.dirname(jp)), "bin", "jfr")
        if os.path.exists(nxt):
            return nxt
    for home in ("/tmp/jdk21", os.path.expanduser("~/.jdk"),
                 "/usr/lib/jvm"):
        if home and os.path.exists(os.path.join(home, "bin", "jfr")):
            return os.path.join(home, "bin", "jfr")
    return "jfr"

JFR_BIN = resolve_jfr_bin()

# producer families: (family_name, [frame substrings]) — first match by
# deepest frame wins; families mirror the TASK-316 store-firehose census:
# ~110-150k setDeltaMovement + ~50-100k setBoundingBox + sync + lists / tick
FAMILIES = [
    ("deltaMovement/travel", [
        "setDeltaMovement", "getInputVector", "handleRelativeFriction",
        "travel", "aiStep", "serverAiStep", "moveRelative", " LivingEntity.",
        " Mob.", " PathfinderMob.",
    ]),
    ("boundingBox/move", [
        "setBoundingBox", "makeBoundingBox", "absMoveTo", "snapTo",
        "setPos", "Entity.move", "collide(", "performCollisions",
        "refreshDimensions", "checkInsideBlocks", "move(Difficulty",
    ]),
    ("sync/SynchedEntityData", [
        "SynchedEntityData", "DataItem", "markDirty", "set(",
    ]),
    ("chunk entity-lists", [
        "addIgnoring", "EntitySectionStorage", "SectionedEntityCache",
        "PersistentEntitySectionManager", "addEntity", "EntityCallbacks",
        "ServerLevel.add", "entitySection",
    ]),
    ("block-change path", [
        "setBlock", "sendBlockUpdated", "blockUpdated", "BlockState",
        "ChunkHolder", "markAndNotifyBlock", "setBlockUpdated",
    ]),
]
OTHER_ENTITY_MARKERS = ["Entity", "entity"]

def jfr_print(jfr_path, event):
    out = subprocess.run(
        [JFR_BIN, "print", "--events", event, jfr_path],
        capture_output=True, text=True, errors="replace")
    if out.returncode != 0:
        print(f"WARN: jfr print {event} failed rc={out.returncode}: {out.stderr[:300]}",
              file=sys.stderr)
        return ""
    return out.stdout

def parse_old_objects(txt):
    """Yield (class_name, object_age_ms, [stack frames]) from jfr print output."""
    events = txt.split("jdk.OldObjectSample")
    for ev in events[1:]:
        cm = re.search(r"object = (\S+)", ev)
        am = re.search(r"objectAge = ([\d.]+) ms", ev)
        stack = []
        sm = re.search(r"stackTrace = \[\n(.*?)\n\s*\]", ev, re.S)
        if sm:
            for line in sm.group(1).splitlines():
                fm = re.search(r"([\w.$]+\.\w+\(.*?\))", line)
                if fm:
                    stack.append(fm.group(1))
        yield (cm.group(1) if cm else "?",
               float(am.group(1)) if am else -1.0,
               stack)

def classify(stack):
    for frame in stack:  # deepest first in jfr print
        for fam, marks in FAMILIES:
            if any(m in frame for m in marks):
                return fam
    joined = " ".join(stack)
    if any(m in joined for m in OTHER_ENTITY_MARKERS):
        return "other-entity"
    return "non-entity/other"

def parse_alloc_samples(txt):
    """(class, weight_bytes, stack) for jdk.ObjectAllocationSample."""
    rows = []
    events = txt.split("jdk.ObjectAllocationSample")
    for ev in events[1:]:
        cm = re.search(r"objectClass = (\S+)", ev)
        wm = re.search(r"weight = (\d+)", ev)
        sm = re.search(r"stackTrace = \[\n(.*?)\n\s*\]", ev, re.S)
        stack = []
        if sm:
            for line in sm.group(1).splitlines():
                fm = re.search(r"([\w.$]+\.\w+\(.*?\))", line)
                if fm:
                    stack.append(fm.group(1))
        rows.append((cm.group(1) if cm else "?",
                     int(wm.group(1)) if wm else 0, stack))
    return rows

def remset_stats(path):
    per_gc = []
    pat = re.compile(r"GC\((\d+)\).*Visited cards (\d+) Total dirty (\d+) \(([\d.]+)%\)")
    with open(path, errors="replace") as f:
        for line in f:
            m = pat.search(line)
            if m:
                per_gc.append((int(m.group(1)), int(m.group(2)),
                               int(m.group(3)), float(m.group(4))))
    return per_gc

def main():
    adir = sys.argv[1] if len(sys.argv) > 1 else "."
    jfr = os.path.join(adir, "recon.jfr")
    remset = os.path.join(adir, "remset.log")
    refine = os.path.join(adir, "refine.log")

    print("== RECON-12 store-firehose attribution ==")
    if os.path.exists(jfr):
        old_txt = jfr_print(jfr, "jdk.OldObjectSample")
        fam_count, class_count, total = collections.Counter(), collections.Counter(), 0
        ages = []
        for cls, age, stack in parse_old_objects(old_txt):
            total += 1
            fam_count[classify(stack)] += 1
            class_count[cls] += 1
            if age >= 0:
                ages.append(age)
        print(f"\n-- OldObjectSample: {total} objects reached old gen")
        if total:
            print("-- share by PRODUCER family (object counts):")
            for fam, n in fam_count.most_common():
                print(f"   {fam:28s} {n:7d}  {100.0*n/total:6.2f}%")
            print("-- top classes:")
            for cls, n in class_count.most_common(15):
                print(f"   {cls:42s} {n:7d}  {100.0*n/total:6.2f}%")
            if ages:
                ages.sort()
                print(f"-- objectAge ms p50={ages[len(ages)//2]:.1f} p90={ages[int(len(ages)*0.9)]:.1f}")
            ent = sum(n for f_, n in fam_count.items()
                      if f_ not in ("non-entity/other",))
            print(f"\nENTITY-FIELD SHARE (all non-'non-entity/other'): {100.0*ent/total:.2f}%"
                  f"  (gate: >= 40%)")
        else:
            print("   NO OldObjectSample events — attribution fell back to "
                  "ObjectAllocationSample control profile (mark NO-GO-INSUFFICIENT-TOOL)")
        a_txt = jfr_print(jfr, "jdk.ObjectAllocationSample")
        rows = parse_alloc_samples(a_txt)
        if rows:
            w_total = sum(w for _, w, _ in rows)
            fam_w = collections.Counter()
            cls_w = collections.Counter()
            for cls, w, stack in rows:
                fam_w[classify(stack)] += w
                cls_w[cls] += w
            print(f"\n-- ObjectAllocationSample control: {len(rows)} samples, "
                  f"{w_total/1e6:.1f} MB weighted")
            for fam, w in fam_w.most_common():
                print(f"   {fam:28s} {w/1e6:9.1f} MB  {100.0*w/w_total:6.2f}%")
            print("   top classes:")
            for cls, w in cls_w.most_common(10):
                print(f"   {cls:42s} {w/1e6:9.1f} MB  {100.0*w/w_total:6.2f}%")
    else:
        print(f"MISSING {jfr}")

    if os.path.exists(remset):
        per_gc = remset_stats(remset)
        if per_gc:
            dirties = [d for _, _, d, _ in per_gc]
            visited = [v for _, v, _, _ in per_gc]
            pcts = [p for _, _, _, p in per_gc]
            print(f"\n-- remset debug: {len(per_gc)} GC cycles")
            dirties_s = sorted(dirties); pcts_s = sorted(pcts)
            print(f"   dirty cards/cycle p50={dirties_s[len(dirties_s)//2]:,} "
                  f"max={max(dirties):,}; dirty% p50={pcts_s[len(pcts_s)//2]:.2f}% "
                  f"max={max(pcts):.2f}%; sum visited={sum(visited):,}")
        else:
            print("\n-- remset debug: no 'Visited cards' lines found")
    else:
        print(f"MISSING {remset}")
    if os.path.exists(refine):
        n = sum(1 for _ in open(refine, errors="replace"))
        print(f"-- refine.log: {n} lines (inspect manually for thread-run stats)")

if __name__ == "__main__":
    main()
