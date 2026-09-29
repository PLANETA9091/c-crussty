#!/bin/bash
# x491 RE-GRAYN wave-2: canary x1 + dp-chord x5 + p208r3 + 165l2/l3 + 205r3 + STZ-42-900s + GLOB x9 = 20 dispatches
set -e
cd /home/z/c-crussty
TOK=$(cat /tmp/gh_token)
REPO="PLANETA9091/c-crussty"
WF="world-bench-parallel.yml"
MERGE=$(git rev-parse --short=8 HEAD)
LIMBOFIX="round-491-c97-limbofix"

# 0) Push limbofix branch (created by C97 commander)
git push origin "$LIMBOFIX" 2>&1 | tail -1

# 1) Create wave-2 branches from merge SHA
declare -a NAMES=(round-491-parity-canary round-491-c47-dp50k3a round-491-c47-dp50k3b round-491-c47-dp50k3c round-491-c47-dp50k3d round-491-c47-dp50k3e round-491-c14-p208r3 round-491-c48-165l2 round-491-c48-165l3 round-491-c77-p205r3 round-491-roll93 round-491-roll94 round-491-roll95 round-491-roll96 round-491-roll97 round-491-roll98 round-491-roll99 round-491-roll100 round-491-roll101 round-491-stz42-900s)
for b in "${NAMES[@]}"; do git branch -f "$b" "$MERGE" 2>/dev/null || true; done
for b in "${NAMES[@]}"; do git push origin "$b" 2>&1 | grep -qE "new branch|up to date" || true; done
echo "BRANCHES-PUSHED"

# 2) Dispatch helper
disp() { # name ref json-args
  CODE=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
    -H "Authorization: token $TOK" -H "Accept: application/vnd.github+json" \
    "https://api.github.com/repos/$REPO/actions/workflows/$WF/dispatches" \
    -d "{\"ref\":\"$2\"$3}")
  echo "$1 -> $CODE"
  sleep 0.2
}
D=",\"inputs\":{"
for b in round-491-c47-dp50k3a round-491-c47-dp50k3b round-491-c47-dp50k3c round-491-c47-dp50k3d round-491-c47-dp50k3e; do
  SEED=$((40 + RANDOM % 20))
  disp "dp-chord $b" "$b" "$D\"population_target\":\"50000\",\"population_seed\":\"$SEED\"}"
done
disp "p208r3" round-491-c14-p208r3 "$D\"population_target\":\"208000\",\"gc_tune\":\"6\",\"server_xmx\":\"12G\"}"
disp "165l2" round-491-c48-165l2 "$D\"population_target\":\"165000\",\"gc_tune\":\"6\",\"server_xmx\":\"12G\",\"population_seed\":\"43\"}"
disp "165l3" round-491-c48-165l3 "$D\"population_target\":\"165000\",\"gc_tune\":\"6\",\"server_xmx\":\"12G\",\"population_seed\":\"44\"}"
disp "205r3" round-491-c77-p205r3 "$D\"population_target\":\"205000\",\"gc_tune\":\"6\",\"server_xmx\":\"12G\"}"
disp "stz42-900s" round-491-stz42-900s "$D\"radius\":\"480\",\"seconds\":\"900\",\"gc_tune\":\"6\"}"
disp "parity-canary" round-491-parity-canary ""
for b in round-491-roll93 round-491-roll94 round-491-roll95 round-491-roll96 round-491-roll97 round-491-roll98 round-491-roll99 round-491-roll100 round-491-roll101; do
  disp "GLOB $b" "$b" ""
done
echo "WAVE-2-DONE"
