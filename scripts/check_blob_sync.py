#!/usr/bin/env python3
"""check_blob_sync.py — аудит desync компаньонов (S7-170 урок, TASK-349).

entityinside/**/X.java НОВЕЕ committed entityinside/build/**/X.class =
CI-доставка едет со СТАРЫМ байткодом (include_bytes! вшивает committed
blob, javac в CI не запускается). Exit 1 при любом desync.

Nested-классы ($Mut, $GuardedNavigatingMobs) маппятся на файл-владелец
явной картой. Запуск: python3 scripts/check_blob_sync.py
"""
import re
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
    blob_set = set(blobs)
    blob_by_owner = {}
    for b in blobs:
        owner = OWNER.get(b)
        if owner is None:
            # прямой маппинг build/<path>.class <- <path>.java
            cand = "entityinside/" + b[len("entityinside/build/"):]
            if cand.endswith(".class"):
                cand = cand[:-len(".class")] + ".java"
            # TASK-462-61 (LEDGER-24 ×461): вложенный компаньон X$Y.class
            # принадлежит файлу-владельцу X.java (как явная карта OWNER для
            # $Mut/$GuardedNavigatingMobs) — иначе свежесть вложенных блобов
            # вообще не проверяется.
            if "$" in cand:
                cand = cand.split("$", 1)[0] + ".java"
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
                # S7-174 (TASK-373): bit-identical rebuild exemption. If the
                # blob content in the worktree equals the committed blob
                # (git hash-object == HEAD:<path>), the recompile of the
                # newer source reproduced the blob BYTE-EXACTLY — the blob
                # IS fresh; git simply has no new object to record, so the
                # commit-timestamp proxy goes stale. Real desyncs (blob
                # actually older than source) keep flagging: their worktree
                # content differs from HEAD or the hash differs.
                wt = git("hash-object", b).strip()
                head = git("rev-parse", f"HEAD:{b}").strip()
                if wt == head:
                    continue
                age = (st - cb) // 60
                print(f"DESYNC: {f} newer than {b} ({age} min) — "
                      f"ПЕРЕСОБЕРИ: scripts/build_region_tick_ops.sh + commit blob")
                fail = 1

    # TASK-462-61 (LEDGER-24 ×461, recipe item 5): для каждого tracked .java с
    # именованным вложенным классом (class/enum/interface/record с отступом)
    # наличие tracked $-вложенного .class ОБЯЗАТЕЛЬНО — корень INVALID run
    # 36174354289: InsideBatchOps.java объявлял QuantumVisitor, но байта
    # класса в репо не было (0 файлов) => NCDFE ×38 на первом SERVE.
    nested_pat = re.compile(
        r"^[ \t]+(?:private\s+|public\s+|protected\s+)?(?:static\s+)?"
        r"(?:final\s+|abstract\s+)?(?:class|enum|interface|record)\s+(\w+)", re.M)
    for f in srcs:
        top_blob = f.replace("entityinside/net/minecraft/",
                             "entityinside/build/net/minecraft/", 1)[:-len(".java")] + ".class"
        if top_blob not in blob_set:
            continue
        try:
            txt = open(f, encoding="utf-8", errors="ignore").read()
        except OSError:
            continue
        for m in nested_pat.finditer(txt):
            nb = top_blob[:-len(".class")] + "$" + m.group(1) + ".class"
            if nb not in blob_set:
                print(f"NESTED-HOLE: {f} declares {m.group(1)} but {nb} is not "
                      f"tracked — cert-канон: javac-вывод обязан коммититься "
                      f"целиком (top + $nested, flat==nested)")
                fail = 1
    if not fail:
        print("BLOB-SYNC OK: все committed .class не старше своих исходников, "
              "вложенные компаньоны на месте")
    sys.exit(fail)


if __name__ == "__main__":
    main()
