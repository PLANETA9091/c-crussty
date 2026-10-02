#!/usr/bin/env python3
# RECON-10 (TASK-315): метрики байткода лестницы collide для рычага #12 NATIVE-COLLIDE.
# Вход: javap -p -c дампы (CollisionUtil, Entity). Выход: per-method размер кода,
# внешние вызовы (не-арифметика), dcmp/epsilon-пороги, классификация вербатим-портабельности.
import re, sys, json, os

CU = "/tmp/recon10/cu_full.txt"
EN = "/tmp/recon10/entity_full.txt"

def parse_methods(path):
    txt = open(path, encoding="utf-8", errors="replace").read().splitlines()
    methods = []
    cur = None
    sig_re = re.compile(r"^  (?:public|private|protected|static|final|\s)*[a-zA-Z].*\(.*\);$")
    for i, line in enumerate(txt):
        if sig_re.match(line) and "Code:" not in line:
            cur = {"sig": line.strip().rstrip(";"), "code": []}
            methods.append(cur)
        elif cur is not None and line.startswith("    "):
            cur["code"].append(line)
        elif line.startswith("  ") and not line.startswith("    ") and cur is not None and not sig_re.match(line):
            cur = None
    return methods

def method_name(sig):
    m = re.search(r"\s([a-zA-Z_$][a-zA-Z0-9_$]*)\(", sig)
    return m.group(1) if m else sig

def analyze(methods, label):
    out = {}
    for m in methods:
        name = method_name(m["sig"])
        body = "\n".join(m["code"])
        nlines = len(re.findall(r"^\s+\d+:", body, re.M))
        calls = re.findall(r"(?:invokevirtual|invokestatic|invokespecial|invokeinterface)\s+#\d+\s+//\s+Method\s+(\S+)", body)
        ext = {}
        for tgt in calls:
            m2 = re.match(r"([a-zA-Z0-9_/$]+)\.([a-zA-Z0-9_$<>]+)", tgt)
            key = m2.group(1) if m2 else tgt
            ext[key] = ext.get(key, 0) + 1
        out[(name, m["sig"])] = {
            "label": label, "bytecode_units": nlines,
            "calls": ext,
            "eps7": body.count("1.0E-7"),
            "dcmp": len(re.findall(r"dcmpg|dcmpl", body)),
            "new": len(re.findall(r"\bnew\b", body)),
        }
    return out

def report(cls, methods, names):
    for name in names:
        sub = {k: v for k, v in methods.items() if k[0] == name}
        if not sub:
            print(f"[{cls}] {name:36s} ABSENT")
            continue
        tot = sum(v["bytecode_units"] for v in sub.values())
        calls_all = {}
        for v in sub.values():
            for c, n in v["calls"].items():
                calls_all[c] = calls_all.get(c, 0) + n
        eps = sum(v["eps7"] for v in sub.values())
        dcmp = sum(v["dcmp"] for v in sub.values())
        new = sum(v["new"] for v in sub.values())
        print(f"[{cls}] {name:36s} units={tot:5d} dcmp={dcmp:3d} eps7={eps:2d} new={new:2d} methods={len(sub)}")
        for c, n in sorted(calls_all.items(), key=lambda x: -x[1])[:12]:
            print(f"      call {c} x{n}")

def main():
    cu = parse_methods(CU) if os.path.exists(CU) else []
    en = parse_methods(EN) if os.path.exists(EN) else []
    cu_a, en_a = analyze(cu, "cu"), analyze(en, "en")
    json.dump({f"{cls}::{k[1]}": v for cls, md in (("CU", cu_a), ("EN", en_a)) for k, v in md.items()},
              open("/tmp/recon10/recon10_methods.json", "w"), indent=1)
    report("CU", cu_a, ["performCollisions", "performVoxelCollisions", "performAABBCollisions",
                        "performVoxelCollisionsX", "performVoxelCollisionsY", "performVoxelCollisionsZ",
                        "performAABBCollisionsX", "performAABBCollisionsY", "performAABBCollisionsZ",
                        "collideX", "collideY", "collideZ",
                        "getCollisionsForBlocksOrWorldBorder", "getEntityHardCollisions", "getCollisions",
                        "isCollidingWithBorder", "voxelShapeIntersectNoEmpty",
                        "voxelShapeIntersect", "isEmpty", "collidedWithFluid" ])
    print()
    report("EN", en_a, ["move", "collide", "collideBoundingBox", "moveRelative", "travel", "handleRelativeFrictionAndCalculateMovement",
                        "applyEffectsFromBlocks", "checkInsideBlocks"])

if __name__ == "__main__":
    main()
