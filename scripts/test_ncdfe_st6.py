#!/usr/bin/env python3
"""test_ncdfe_st6.py — ROUND-474-C53 (NEXT-474-3): ST-6 фолт-инъекция юнит-тест.

Остаток Л268/Л270 (NCDFE-страж v2: R2-фикс в мастере, R1 fail-open был
задокументирован, но жил): проверяет, что
  (1) ncdfe_guard.sh --st6 (R3): искусственные NCDFE-носители ОБЯЗАНЫ ловиться
      (страж обязан FAIL на каждом injected-носителя);
  (2) изолированные фолт-инъекции: искусственный NCDFE-.java (static-init
      cross-Ops и touch-без-handler) страж обязан FAIL (exit 1) с точным
      детектор-маркером, а не SKIP/OK;
  (3) DELIVERY-FAIL-семантика exit 2 (R3 FO-4): SKIP при NCDFE_STRICT=1 =
      exit 1; не-строгий SKIP печатает loud-маркер DELIVERY-FAIL;
  (4) ST-6 log-gate контракт (S13) на R1-рантайме: src/mobs_ai.rs содержит
      0x forbidden fail-open маркера "arming anyway" и >=1x fail-closed
      маркера "arm ABORT (fail-closed" (probe-then-arm closure);
  (5) канон-блобы мастера (guard --selftest) и R2-векторы (--vectors, при
      доступном javap) остаются зелёными.

Офлайн: сеть не трогается. Usage:
  python3 scripts/test_ncdfe_st6.py [--repo /path/to/checkout]
Exit: 0 = ALL PASS, 1 = FAIL.
"""
import argparse
import os
import re
import shutil
import subprocess
import sys
import tempfile

FORBIDDEN = "arming" + " anyway"      # собирается конкатенацией — не ловить сам себя в этом файле
ABORT_MARKER = "arm ABORT (fail-closed"


def run(cmd, cwd=None, env=None):
    e = dict(os.environ)
    if env:
        e.update(env)
    p = subprocess.run(cmd, cwd=cwd, env=e, capture_output=True, text=True, timeout=300)
    return p.returncode, (p.stdout or "") + (p.stderr or "")


def check(fails, name, cond, detail=""):
    print(f'  [{"PASS" if cond else "FAIL"}] {name}' + (f" — {detail}" if detail and not cond else ""))
    if not cond:
        fails.append(name)


def main():
    ap = argparse.ArgumentParser(description="ST-6 NCDFE fault-injection unit test (C53)")
    ap.add_argument("--repo", default=os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    args = ap.parse_args()
    repo = os.path.abspath(args.repo)
    guard = os.path.join(repo, "scripts", "ncdfe_guard.sh")
    mobs_ai = os.path.join(repo, "src", "mobs_ai.rs")
    fails = []
    print(f"selftest test_ncdfe_st6.py @ {repo}:")

    # --- T1: guard --st6 (искусственные NCDFE-носители -> страж обязан FAIL) ---
    rc, out = run(["bash", guard, "--st6"], cwd=repo)
    check(fails, "T1 guard --st6: все injected NCDFE-носители пойманы (exit 0)",
          rc == 0, f"rc={rc} tail={out.strip().splitlines()[-1:]}")

    with tempfile.TemporaryDirectory(prefix="st6_py_") as td:
        # --- T2: искусственный NCDFE-.java №1 (static-init cross-Ops) ---
        f1 = os.path.join(td, "St6PyClinitOps.java")
        with open(f1, "w") as f:
            f.write("public class St6PyClinitOps { static { int x = St6PyOtherOps.idCount; } }\n")
        rc, out = run(["bash", guard, f1], cwd=repo)
        check(fails, "T2 artificial static-init cross-Ops .java -> exit 1 + 'define-порядок'",
              rc == 1 and "define-порядок" in out, f"rc={rc}")

        # --- T3: искусственный NCDFE-.java №2 (touch без catch(Throwable)) ---
        # статическая форма Ops.<member> — именно её ловит J2; голый
        # параметр-тип не резолвится жадно и NCDFE-хазой не является
        f2 = os.path.join(td, "St6PyNoHandlerOps.java")
        with open(f2, "w") as f:
            f.write("public class St6PyNoHandlerOps { public int f() { return St6PyOtherOps.idCount; } }\n")
        rc, out = run(["bash", guard, f2], cwd=repo)
        check(fails, "T3 artificial touch-no-handler .java -> exit 1 + 'throwable-маркеры отсутствуют'",
              rc == 1 and "throwable-маркеры отсутствуют" in out, f"rc={rc}")

        # --- T4: DELIVERY-FAIL-семантика (R3 FO-4): STRICT=1 + носитель-призрак -> exit 1 ---
        ghost = os.path.join(td, "St6PyGhostOps.class")
        rc, out = run(["bash", guard, ghost], cwd=repo, env={"NCDFE_STRICT": "1"})
        check(fails, "T4 STRICT=1 + отсутствующий носитель -> exit 1 (fail-closed delivery)",
              rc == 1, f"rc={rc}")
        # не-строгий SKIP обязан быть LOUD (exit 2 + маркер DELIVERY-FAIL)
        rc, out = run(["bash", guard, ghost], cwd=repo)
        check(fails, "T5 не-строгий SKIP -> exit 2 + loud-маркер DELIVERY-FAIL",
              rc == 2 and "DELIVERY-FAIL" in out, f"rc={rc}")

        # --- T6 (опц., javac): компилированный artificial NCDFE-.class обязан FAIL ---
        javac = os.environ.get("NCDFE_JAVAC") or ""
        if not javac:
            for c in ("/tmp/jdk21/bin/javac", "/home/z/tools/jdk-21.0.12.1+1/bin/javac"):
                if os.path.exists(c):
                    javac = c
                    break
        if javac and os.path.exists(javac):
            other = os.path.join(td, "St6PyOtherOps.java")
            with open(other, "w") as f:
                f.write("public class St6PyOtherOps { public static int idCount = 7; }\n")
            rc_c, _ = run([javac, "-d", td, other, f1, f2], cwd=repo)
            if rc_c == 0 and os.path.exists(os.path.join(td, "St6PyClinitOps.class")):
                rc, out = run(["bash", guard, os.path.join(td, "St6PyClinitOps.class")], cwd=repo)
                check(fails, "T6 artificial compiled clinit-touch .class -> exit 1 (C2-детектор жив)",
                      rc == 1 and "define-порядок" in out, f"rc={rc}")
            else:
                print("  [SKIP] T6 javac не смог скомпилировать фикс-fitures")
        else:
            print("  [SKIP] T6 javac недоступен (задай NCDFE_JAVAC) — .java-фолты держат контракт")

    # --- T7: ST-6 log-gate контракт R1 (S13): forbidden-маркер вымер, ABORT жив ---
    if os.path.exists(mobs_ai):
        src = open(mobs_ai, encoding="utf-8").read()
        n_forbidden = src.count(FORBIDDEN)
        check(fails, "T7a mobs_ai.rs: 0x forbidden fail-open маркера 'arming anyway'",
              n_forbidden == 0, f"найдено {n_forbidden}")
        check(fails, "T7b mobs_ai.rs: >=1x fail-closed маркер 'arm ABORT (fail-closed'",
              src.count(ABORT_MARKER) >= 1)
        check(fails, "T7c mobs_ai.rs: probe-then-arm гейт (r1_order_gate + UPSTREAM_PUSH_CLASS)",
              "fn r1_order_gate" in src and "UPSTREAM_PUSH_CLASS" in src)
    else:
        check(fails, "T7 mobs_ai.rs найден", False, mobs_ai)

    # --- T8: канон-блобы мастера остаются зелёными (selftest 0 FAIL) ---
    rc, out = run(["bash", guard, "--selftest"], cwd=repo)
    check(fails, "T8 guard --selftest (канон T1 NCDFE=0 на блобах мастера)", rc == 0, f"rc={rc}")

    # --- T9 (опц.): R2-векторы 5/5 при доступном javap ---
    javap = os.environ.get("NCDFE_JAVAP") or ""
    if not javap:
        for c in ("/tmp/jdk21/bin/javap", "/home/z/tools/jdk-21.0.12.1+1/bin/javap"):
            if os.path.exists(c):
                javap = c
                break
    if javap:
        rc, out = run(["bash", guard, "--vectors"], cwd=repo,
                      env={"NCDFE_JAVAP": javap, "NCDFE_JAVAC": javap.replace("javap", "javac")})
        check(fails, "T9 guard --vectors 5/5 (R2 fail-open векторы живы)", rc == 0, f"rc={rc}")
    else:
        print("  [SKIP] T9 javap недоступен")

    print(f'st6: {"ALL PASS" if not fails else "FAIL: " + ", ".join(fails)}')
    return 0 if not fails else 1


if __name__ == "__main__":
    sys.exit(main())
