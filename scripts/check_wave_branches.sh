#!/usr/bin/env bash
# check_wave_branches.sh — branch-integrity sweep (AG-373, wave-519)
# Guard против «plumbing-деревьев» (×518: ветки 9/29/41 собрали дерево 3.2M-deleted,
# мёрж целиком уничтожил бы master — спасение 0b3dee2). Сканает origin/wave-* ветки:
#   RED  = deletions-vs-master > 500 файлов  (оборванное дерево, мёржить НЕЛЬЗЯ)
#   RED  = committer != PLANETA9091          (канон коммитов от лица владельца)
#   INFO = branch == master head             (пушей сверх master ещё нет)
#   INFO = additions+modifications == 0      (пустой payload)
# Usage: check_wave_branches.sh <glob>   (default 'swarm-519-*')
set -u
GLOB="${1:-swarm-519-*}"
cd "$(git rev-parse --show-toplevel 2>/dev/null || echo .)" || exit 1
MASTER=$(git rev-parse origin/master 2>/dev/null || git rev-parse master) || exit 1
printf '%-24s %-10s %-22s %-7s %6s %6s %6s  %s\n' BRANCH HEAD COMMITTER FLAGS ADD DEL MOD NOTE
red_total=0
for ref in $(git for-each-ref --format='%(refname:short)' "refs/remotes/origin/${GLOB}"); do
  sha=$(git rev-parse --short "$ref")
  cn=$(git log -1 --format='%cn' "$ref")
  ce=$(git log -1 --format='%ce' "$ref")
  read -r add del mod <<<"$(git diff --name-status "$MASTER"..."$ref" 2>/dev/null | awk '{c[$1]++} END{printf "%d %d %d", c["A"]+0, c["D"]+0, c["M"]+0}')"
  flags=""; note=""
  [ "${del:-0}" -gt 500 ] && { flags="${flags}TREE-BROKEN "; note="MERGE-FORBIDDEN plumbing-tree"; }
  [ "$cn" != "PLANETA9091" ] && { flags="${flags}IDENTITY "; note="committer=$cn <$ce>"; }
  [ "$sha" = "$(git rev-parse --short "$MASTER")" ] && { flags="${flags}AT-MASTER "; note="no unique push yet"; }
  [ "$((add+mod))" -eq 0 ] && [ -z "$flags" ] && { flags="EMPTY "; note="zero payload vs master"; }
  [ -n "$flags" ] && case "$flags" in *TREE-BROKEN*|*IDENTITY*) red_total=$((red_total+1));; esac
  printf '%-24s %-10s %-22s %-7s %6s %6s %6s  %s\n' "${ref#origin/}" "$sha" "$cn <$ce>" "${flags:--}" "$add" "$del" "$mod" "$note"
done
echo "RED_TOTAL=$red_total"
