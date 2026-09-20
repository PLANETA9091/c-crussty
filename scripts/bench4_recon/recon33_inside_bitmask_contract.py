#!/usr/bin/env python3
# RECON-33: javap contract for inside-bitmask flagman (option-B candidate).
# Goal: pin the exact entry gates of checkInsideBlocks/applyEffectsFromBlocks,
# the per-block predicate semantics (all-air short-circuit legality), and the
# section state-carrier (LevelChunkSection nonEmptyBlockCount) for an
# event-driven section solid-bitmask prefilter. OFFLINE, no dispatch.
import subprocess, re, os, sys

JAR = "research/gc-recon-2026-09-19/run-s7194-zeroalloc-v1/patched-kernel.jar"
JAVAP = "/tmp/jdk21/bin/javap"
OUT = "research/gc-recon-2026-09-19/contract-inside-bitmask-s7194"
os.makedirs(OUT, exist_ok=True)

CLASSES = [
    "net.minecraft.world.entity.Entity",
    "net.minecraft.world.level.chunk.LevelChunkSection",
    "net.minecraft.world.level.chunk.LevelChunk",
    "net.minecraft.world.level.chunk.PalettedContainer",
    "net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase",
    "net.minecraft.world.level.block.state.BlockBehaviour",
    "net.minecraft.world.level.block.Block",
    "net.minecraft.world.level.Level",
]

def javap(cls, extra=None):
    cmd = [JAVAP, "-p", "-c", "-cp", JAR]
    if extra:
        cmd += extra
    cmd.append(cls)
    r = subprocess.run(cmd, capture_output=True, text=True)
    if r.returncode != 0:
        print(f"FAIL {cls}: {r.stderr[:200]}")
        return ""
    return r.stdout

def method_bodies(text):
    """split javap -c output into {signature: body}"""
    lines = text.splitlines()
    bodies, cur, sig = {}, [], None
    for ln in lines:
        m = re.match(r"^  [a-zA-Z].*\(.*\);?$", ln) or re.match(r"^  .*\b(\w+)\(.*\);\s*$", ln)
        is_decl = bool(re.match(r"^  (public|private|protected|static|final|synchronized|abstract|native|[a-zA-Z$][\w$<>.,\[\]]*\s).*\(.*", ln)) and "(" in ln and not ln.strip().startswith(("descriptor:", "Code:", "LineNumberTable", "stack=", "locals=", "args_size"))
        if is_decl and "(" in ln:
            if sig:
                bodies[sig] = "\n".join(cur)
            sig, cur = ln.strip(), []
        elif sig is not None:
            cur.append(ln)
    if sig:
        bodies[sig] = "\n".join(cur)
    return bodies

results = {}
for cls in CLASSES:
    t = javap(cls)
    fn = os.path.join(OUT, cls.replace("$", "_").split(".")[-1] + "_full.txt")
    with open(fn, "w") as f:
        f.write(t)
    results[cls] = t
    print(f"{cls}: {len(t)} bytes -> {fn}")

# ---- Drill 1: Entity call sites of checkInsideBlocks / applyEffectsFromBlocks
ent = results["net.minecraft.world.entity.Entity"]
bodies = method_bodies(ent)
print("\n=== ENTITY: methods touching checkInsideBlocks/applyEffectsFromBlocks ===")
hits = []
for sig, body in bodies.items():
    if re.search(r"checkInsideBlocks|applyEffectsFromBlocks", body):
        n_ci = len(re.findall(r"Method.*checkInsideBlocks", body))
        n_af = len(re.findall(r"Method.*applyEffectsFromBlocks", body))
        hits.append((sig, n_ci, n_af, len(body)))
        print(f"  CALLER {sig[:110]}  (checkInsideBlocks x{n_ci}, applyEffects x{n_af}, body {len(body)}B)")

# full bodies of the two gates themselves
for key in ("checkInsideBlocks", "applyEffectsFromBlocks", "lambda$checkInsideBlocks"):
    print(f"\n=== ENTITY: bodies named *{key}* ===")
    for sig, body in bodies.items():
        if key in sig:
            print(f"--- {sig}")
            print(body[:3000])

# ---- Drill 2: LevelChunkSection state carriers
lcs = results["net.minecraft.world.level.chunk.LevelChunkSection"]
print("\n=== LEVELCHUNKSECTION: fields ===")
for ln in lcs.splitlines():
    if re.match(r"^  (private|public|protected|static|final)", ln) and "(" not in ln:
        print(" ", ln.strip())
lb = method_bodies(lcs)
print("=== LEVELCHUNKSECTION: setBlockState / getBlockState / counters ===")
for sig, body in lb.items():
    if re.search(r"setBlockState|getBlockState|nonEmptyBlockCount|hasOnlyAir|recalcBlockCounts", sig):
        print(f"--- {sig}")
        print(body[:2200])

# ---- Drill 3: LevelChunk.getSection
lc = results["net.minecraft.world.level.chunk.LevelChunk"]
lb2 = method_bodies(lc)
print("\n=== LEVELCHUNK: getSection* ===")
for sig, body in lb2.items():
    if sig.startswith("getSection") or " getSections(" in sig:
        print(f"--- {sig}")
        print(body[:1200])

# ---- Drill 4: BlockStateBase predicates used by inside-gate (isAir etc.)
bsb = results["net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase"]
bb = method_bodies(bsb)
print("\n=== BLOCKSTATEBASE: isAir/entityInside-adjacent ===")
for sig, body in bb.items():
    if re.match(r"public (boolean|void) (isAir|entityInside|getBlock)", sig):
        print(f"--- {sig}")
        print(body[:800])

# ---- Drill 5: who calls checkInsideBlocks across likely callers (move/baseTick already in Entity). Also ServerLevel tick paths not needed offline.
print("\n=== ENTITY: move() body excerpt (inside-check position) ===")
for sig, body in bodies.items():
    if re.match(r"(public|private).*\bmove\(.*", sig) and "checkInsideBlocks" in body:
        idx = body.find("checkInsideBlocks")
        print(f"--- {sig}")
        print("...", body[max(0, idx-1200):idx+600])

print("\nDONE RECON-33 contract dump")
