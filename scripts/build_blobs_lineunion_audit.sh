#!/usr/bin/env bash
# build_blobs_lineunion_audit.sh — x468-S55: line-union safety audit of ALL
# build_*_blobs*.sh scripts (Л171 P0-канон: MERGE #9 line-union съел ';;'
# case-арм run_world3.sh @bee2b585:532; x463/LEDGER-46: union-glue
# "cmp457_paldelta|cmp457_eqsnap2" inside one equals() string slept 7/10
# java gate classes — substring-grep не ловит).
#
# Риск-классы (S55 audit, каждый с числом в отчёте):
#   A ';;'/case-армы    — bash -n + scripts/case_arm_scan.py (c77 e7087a96)
#   B склейка маркеров  — 2-токен pipe-glue литерал "cmpX|cmpY" (x463
#                         сигнатура; списки >=3 cmp-токенов легальны —
#                         BrainOps TICK2_FLAGS list-style, check_cp_exact canon)
#   C glue-примитивы    — echo -n / tr -d / paste -s / printf '%s' без \n
#                         (line-union-хрупкие эмиттеры) — WARN-инвентарь
#   D дубли-хвосты      — точные дубли командных хвостов install_*/gate_fe
#                         (432b-класс: gate_fe ... 'InsideBlockOps$Recorder' x2)
#   E heredoc-тела      — case_arm_scan их НЕ токенизирует -> WARN-инвентарь
#
# Usage: scripts/build_blobs_lineunion_audit.sh [--negative-test]
#   обычный режим: exit 0 = clean, 1 = FAIL найден
#   --negative-test: синтетика (съеденный ';;' + glue-пара) ДОЛЖНА ловиться
set -uo pipefail
cd "$(dirname "$0")/.."

NEG=0; [ "${1:-}" = "--negative-test" ] && NEG=1

FILES=$(ls scripts/build_*_blobs*.sh 2>/dev/null | sort -u)
[ -z "$FILES" ] && { echo "S55-audit: no build_*_blobs scripts found"; exit 2; }
N=$(echo "$FILES" | wc -l)
FAIL=0; WARN=0
echo "== S55 line-union audit: $N build_*_blobs script(s) =="

# ---------- A: bash -n + case_arm_scan ----------
for f in $FILES; do
  if ! bash -n "$f" 2>/tmp/s55_aud_bashn.$$; then
    echo "FAIL A bash-n: $f"; cat /tmp/s55_aud_bashn.$$; FAIL=$((FAIL+1))
  fi
done; rm -f /tmp/s55_aud_bashn.$$
if [ -f scripts/case_arm_scan.py ]; then
  OUT=$(python3 scripts/case_arm_scan.py $FILES 2>&1); RC=$?
  echo "$OUT" | grep 'FAIL' | grep -v 'case_arm_scan:' | sed 's/^/  /'
  echo "A: $(echo "$OUT" | tail -1)"
  [ "$RC" -ne 0 ] && FAIL=$((FAIL+1))
else
  echo "WARN A: scripts/case_arm_scan.py отсутствует — вендорь c77 blob e7087a96"; WARN=$((WARN+1))
fi

# ---------- B: 2-токен pipe-glue (x463 сигнатура) ----------
B_OUT=$(python3 - $FILES <<'PY'
import sys, re
run = re.compile(r'cmp[0-9]+_[A-Za-z0-9_]+(?:\|cmp[0-9]+_[A-Za-z0-9_]+)+')
bad = 0
for path in sys.argv[1:]:
    for i, line in enumerate(open(path, errors="replace"), 1):
        if line.lstrip().startswith("#"):
            continue  # комментарии описывают исторический glue — не литерал
        for seg in run.findall(line):
            if seg.count("|") == 1:  # ровно 2 токена = union-glue артефакт
                print("FAIL B pipe-glue pair: %s:%d: '%s'" % (path, i, seg))
                bad += 1
sys.exit(1 if bad else 0)
PY
); B_RC=$?
echo "$B_OUT" | sed 's/^/  /'
echo "B: 2-token pipe-glue literals: $(echo "$B_OUT" | grep -c 'FAIL B' || true)"
[ "$B_RC" -ne 0 ] && FAIL=$((FAIL+1))

# ---------- C: glue-примитивы (WARN-инвентарь) ----------
C=$(grep -nE "echo -n|tr -d|paste -s|printf '%s'" $FILES 2>/dev/null | wc -l)
if [ "$C" -gt 0 ]; then grep -nE "echo -n|tr -d|paste -s|printf '%s'" $FILES | sed 's/^/  WARN C: /'; fi
echo "C: glue-primitive lines: $C"; WARN=$((WARN+C))

# ---------- D: дубли-хвосты (install_*/gate_fe командные строки) ----------
D=0
for f in $FILES; do
  DUPS=$(grep -vE '^\s*(#|$)' "$f" | grep -E '^\s*(install_|gate_fe )' | sort | uniq -d)
  if [ -n "$DUPS" ]; then
    echo "$DUPS" | sed "s|^|  FAIL D dup-tail: $f: |"
    D=$((D + $(echo "$DUPS" | wc -l)))
  fi
done
echo "D: duplicate install/gate tail lines: $D"; FAIL=$((FAIL+D))

# ---------- E: heredoc-инвентарь (тела сканером не токенизируются) ----------
E=$(grep -c '<<' $FILES 2>/dev/null | awk -F: '{s+=$2} END{print s+0}')
echo "E: heredoc operators: $E"; [ "$E" -gt 0 ] && WARN=$((WARN+1))

echo "== S55 verdict: $N script(s), FAIL=$FAIL WARN=$WARN =="
if [ "$NEG" -eq 1 ]; then
  TMPN=$(mktemp -d)
  printf '#!/usr/bin/env bash\ncase x in\n  a) echo 1\n  b) echo 2;;\nesac\n' > "$TMPN/neg_arm.sh"
  printf '#!/usr/bin/env bash\nX="cmp457_paldelta|cmp457_eqsnap2"\n' > "$TMPN/neg_glue.sh"
  A_HIT=$(python3 scripts/case_arm_scan.py "$TMPN/neg_arm.sh" 2>&1 | grep -c ': FAIL ')
  G_HIT=$(python3 -c "import re;l='cmp457_paldelta|cmp457_eqsnap2';print(sum(1 for s in re.findall(r'cmp[0-9]+_[A-Za-z0-9_]+(?:\|cmp[0-9]+_[A-Za-z0-9_]+)+',l) if s.count('|')==1))")
  rm -rf "$TMPN"
  echo "negative-test: eaten-';;' caught=$A_HIT/1, glue-pair caught=$G_HIT/1"
  [ "$A_HIT" -ge 1 ] && [ "$G_HIT" -ge 1 ] && { echo "negative-test PASS (2/2)"; exit 0; }
  echo "negative-test FAIL"; exit 1
fi
[ "$FAIL" -eq 0 ] || exit 1
exit 0
