#!/usr/bin/env python3
"""check_blob_sync.py — аудит desync компаньонов (S7-170 урок, TASK-349).

entityinside/**/X.java НОВЕЕ committed entityinside/build/**/X.class =
CI-доставка едет со СТАРЫМ байткодом (include_bytes! вшивает committed
blob, javac в CI не запускается). Exit 1 при любом desync.

Nested-классы ($Mut, $GuardedNavigatingMobs) маппятся на файл-владелец
явной картой. Запуск: python3 scripts/check_blob_sync.py
"""
import subprocess
import sys

OWNER = {
    "entityinside/build/net/minecraft/world/entity/RegionTickOps$Mut.class":
        "entityinside/net/minecraft/world/entity/RegionTickOps.java",
    "entityinside/build/net/minecraft/world/entity/RegionTickOps$GuardedNavigatingMobs.class":
        "entityinside/net/minecraft/world/entity/RegionTickOps.java",
}


def git(*args):
    return subprocess.run(["git", *args], capture_output=True, text=True).stdout.strip()


def main():
    srcs = git("ls-files", "entityinside/*.java").splitlines()
    blobs = git("ls-files", "entityinside/build/*.class").splitlines()
    blob_by_owner = {}
    for b in blobs:
        owner = OWNER.get(b)
        if owner is None:
            # прямой маппинг build/<path>.class <- <path>.java
            cand = "entityinside/" + b[len("entityinside/build/"):]
            if cand.endswith(".class"):
                cand = cand[:-len(".class")] + ".java"
            owner = cand
        blob_by_owner.setdefault(owner, []).append(b)

    fail = 0
    for f in srcs:
        st = int(git("log", "-1", "--format=%ct", "--", f) or 0)
        owned = blob_by_owner.get(f, [])
        if not owned:
            continue
        for b in owned:
            cb = int(git("log", "-1", "--format=%ct", "--", b) or 0)
            if st > cb:
                age = (st - cb) // 60
                print(f"DESYNC: {f} newer than {b} ({age} min) — "
                      f"ПЕРЕСОБЕРИ: scripts/build_region_tick_ops.sh + commit blob")
                fail = 1
    if not fail:
        print("BLOB-SYNC OK: все committed .class не старше своих исходников")
    sys.exit(fail)


if __name__ == "__main__":
    main()
