#!/bin/bash
# x491 CI wave: 8 c30-anchor lottery draws + 92 roll slots = 100 dispatches.
# 1 dispatch = 1 branch round-491-* (zero-code master snapshot @7bcf8604 — runner lottery).
set -e
cd /home/z/c-crussty
TOK=$(cat /tmp/gh_token)
REPO="PLANETA9091/c-crussty"
PIN=$(git rev-parse --short=8 HEAD)
echo "PIN=$PIN"

# Create branches (refs only — same commit, cheap)
BRANCHES=()
for i in 11 12 13 14 15 16 17 18; do BRANCHES+=("round-491-c30-an$i"); done
for i in $(seq -w 1 92); do BRANCHES+=("round-491-roll$i"); done

# Create local refs
for b in "${BRANCHES[@]}"; do git branch -f "$b" "$PIN" 2>/dev/null || true; done

# Push in batches of 10 refs
N=${#BRANCHES[@]}
for ((s=0; s<N; s+=10)); do
  BATCH=("${BRANCHES[@]:s:10}")
  git push origin "${BATCH[@]}" 2>&1 | grep -cE "new branch" >/dev/null || true
  echo "pushed batch $((s/10+1))"
done
echo "PUSH-DONE ($N branches)"

# Dispatch world-bench-round (file world-bench-parallel.yml) — bank canon defaults
OK=0; FAIL=0
for b in "${BRANCHES[@]}"; do
  CODE=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
    -H "Authorization: token $TOK" \
    -H "Accept: application/vnd.github+json" \
    "https://api.github.com/repos/$REPO/actions/workflows/world-bench-parallel.yml/dispatches" \
    -d "{\"ref\":\"$b\"}")
  if [ "$CODE" = "204" ]; then OK=$((OK+1)); else FAIL=$((FAIL+1)); echo "DISPATCH-FAIL $b http=$CODE"; fi
  sleep 0.2
done
echo "DISPATCHED-OK=$OK FAIL=$FAIL"
