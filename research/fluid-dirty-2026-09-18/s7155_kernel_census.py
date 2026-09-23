#!/usr/bin/env python3
"""s7155_kernel_census.py — RECON-3 / S7-155 feasibility-гейт region-threaded
entity ticking. Часть 1: структурный census kernel e2992d63 через javap
(Temurin JDK 21, scripts/ensure_javap.sh).

  A. Entity-цикл: дизасм ServerLevel.tick/tickNonPassenger/tickPassenger +
     EntityTickList (поля/методы/forEach) — упорядоченные вызовы.
  B. RNG-census: ВСЕ классы jar — ссылки Field Level.random / ServerLevel.random
     (общий, порядко-зависимый хазард) vs Field Entity.random (локальный).
  C. Cross-entity census: invokes broadphase-семейств (getEntities/noCollision/
     collide/getHardCollidingEntities/getNearestPlayer/pushEntities) по классам.

Выход: S7155_KERNEL_CENSUS.md + s7155_kernel_census.json
"""
import subprocess, re, collections, json, os, sys

JAVAP = "/tmp/jdk21/bin/javap"
KERNEL = ("/home/z/c-crussty/research/inside-cache-2026-09-18/"
          "run-s7149b-cumulative/patched-kernel.jar")
OUT_MD = "/home/z/c-crussty/research/fluid-dirty-2026-09-18/S7155_KERNEL_CENSUS.md"
OUT_JSON = "/home/z/c-crussty/research/fluid-dirty-2026-09-18/s7155_kernel_census.json"

def javap(args, timeout=300):
    r = subprocess.run([JAVAP, "-p", "-c", "-classpath", KERNEL] + args,
                       capture_output=True, text=True, timeout=timeout)
    if r.returncode != 0:
        sys.stderr.write(r.stderr[:400])
    return r.stdout

# ---------- список классов ----------
listing = subprocess.run(["unzip", "-l", KERNEL], capture_output=True, text=True).stdout
all_cls = []
for ln in listing.splitlines():
    parts = ln.split()
    if parts and parts[-1].endswith(".class"):
        all_cls.append(parts[-1][:-6].replace("/", "."))
print("classes in jar:", len(all_cls))

# ---------- A. структура entity-цикла ----------
STRUCT_CLASSES = [
    "net.minecraft.server.level.ServerLevel",
    "net.minecraft.world.level.entity.EntityTickList",
    "net.minecraft.world.level.Level",
    "net.minecraft.world.entity.Entity",
]
struct_out = {}
for c in STRUCT_CLASSES:
    txt = javap([c])
    struct_out[c] = txt
    open(OUT_JSON + ".struct." + c.rsplit(".", 1)[-1] + ".txt", "w").write(txt)
    print("struct:", c, len(txt), "chars")

# ---------- B+C. batch census ----------
FIELD_RE = re.compile(r"(?:Field|Method|InterfaceMethod)\s+(\S+?\.)([^\s:;()]+)(?::([L\[\(][^\s]*)?)?\s*$")
BATCH = 400
rng_shared = collections.defaultdict(set)   # owner.random -> classes
rng_entity = collections.defaultdict(set)   # Entity.random -> classes
xent = collections.defaultdict(collections.Counter)  # class -> fam -> n
CLASS_RE = re.compile(r"^(?:[a-z_$][\w$]*\.)+[A-Z_$][\w$]*;$")

XENT_FAMS = {
    "getEntities": "getEntities",
    "getEntitiesOfClass": "getEntitiesOfClass",
    "noCollision": "noCollision",
    "collide": "collide",
    "getHardCollidingEntities": "getHardColliding",
    "getNearestPlayer": "getNearestPlayer",
    "pushEntities": "pushEntities",
    "getEntityCollisions": "getEntityCollisions",
}

cur = None
n_ok = 0
for i in range(0, len(all_cls), BATCH):
    batch = all_cls[i:i+BATCH]
    txt = javap(batch, timeout=600)
    for ln in txt.splitlines():
        cls_m = re.match(r"^(?:public\s+|final\s+|abstract\s+|class\s+|interface\s+|"
                         r"(?:public|final|abstract)\s+)+class\s+(\S+)", ln)
        if cls_m and not ln.startswith(" "):
            cur = cls_m.group(1)
            n_ok += 1
            continue
        if cur is None:
            continue
        is_entity_pkg = cur.startswith("net.minecraft.world.entity.")
        m = FIELD_RE.search(ln)
        if not m:
            continue
        owner = m.group(1)[:-1].replace("/", ".")
        name = m.group(2)
        kind_ln = "Field" in ln.split("//")[0] or "Field" in ln
        op = ln.strip().split(":")[-1].strip().split()[0] if ":" in ln else ""
        if "Field" in ln and ln.strip().split()[1] in ("getfield", "putfield", "getstatic", "putstatic"):
            if name == "random":
                key = owner + "." + name
                if owner in ("net.minecraft.world.level.Level",
                             "net.minecraft.server.level.ServerLevel"):
                    rng_shared[key].add(cur)
                elif owner == "net.minecraft.world.entity.Entity":
                    rng_entity[key].add(cur)
        elif is_entity_pkg and ("Method" in ln or "InterfaceMethod" in ln):
            for k, fam in XENT_FAMS.items():
                if name == k:
                    xent[cur][fam] += 1
                    break
    if (i // BATCH) % 5 == 0:
        print("batch", i // BATCH, "done", flush=True)

print("census done; classes seen:", n_ok)
print("shared rng:", {k: len(v) for k, v in rng_shared.items()})
print("entity rng classes:", len(rng_entity.get("net.minecraft.world.entity.Entity.random", set())))

shared_entity_users = sorted(set().union(*rng_shared.values())) if rng_shared else []
shared_entity_users = [c for c in shared_entity_users
                       if c.startswith("net.minecraft.world.entity.")]
local_entity_users = sorted(rng_entity.get("net.minecraft.world.entity.Entity.random", set()))

json.dump({
    "rng_shared": {k: sorted(v) for k, v in rng_shared.items()},
    "rng_entity": {k: sorted(v) for k, v in rng_entity.items()},
    "shared_entity_users": shared_entity_users,
    "local_entity_users": local_entity_users,
    "xent": {c: dict(v) for c, v in xent.items()},
    "n_classes": len(all_cls),
    "n_classes_seen": n_ok,
}, open(OUT_JSON, "w"), indent=1)

with open(OUT_MD, "w") as fh:
    fh.write("# S7-155 RECON-3 часть 1 — структурный census kernel e2992d63 "
             "через javap (%d классов, %d распознано)\n\n"
             % (len(all_cls), n_ok))
    fh.write("## B. RNG-census: общий Level/ServerLevel.random\n\n")
    fh.write("| поле | классов-референсеров (все jar) | entity-классов |\n|---|---|---|\n")
    for k, v in rng_shared.items():
        ent_n = len([c for c in v if c.startswith("net.minecraft.world.entity.")])
        fh.write("| `%s` | %d | %d |\n" % (k, len(v), ent_n))
    fh.write("\n| поле | классов |\n|---|---|\n")
    for k, v in rng_entity.items():
        fh.write("| `%s` (локальный) | %d |\n" % (k, len(v)))
    fh.write("\n### ОБЩИЙ random: entity-классы-пользователи (порядко-зависимый "
             "хазард планировщика): %d\n\n" % len(shared_entity_users))
    for c in shared_entity_users:
        fh.write("- `%s`\n" % c)
    fh.write("\n### ЛОКАЛЬНЫЙ Entity.random: %d классов (полный список в json)\n\n" %
             len(local_entity_users))
    fh.write("## C. Cross-entity invokes по entity-классам (топ-25)\n\n")
    fh.write("| класс | сайтов | семейства |\n|---|---|---|\n")
    for cls, cnt in sorted(xent.items(), key=lambda x: -sum(x[1].values()))[:25]:
        fh.write("| `%s` | %d | %s |\n"
                 % (cls, sum(cnt.values()),
                    ", ".join("%s=%d" % kv for kv in cnt.most_common())))
print("MD written:", OUT_MD)
