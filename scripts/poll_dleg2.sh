#!/usr/bin/env bash
# poll_dleg2.sh — poll GH run 35652772569 until completed
TOK=$(git -C /home/z/c-crussty remote get-url origin | sed -n 's|^https://[^:]*:\([^@]*\)@github\.com/.*|\1|p')
RUN=35652772569
for i in $(seq 1 30); do
  sleep 120
  R=$(curl -s -H "Authorization: Bearer $TOK" -H "Accept: application/vnd.github+json" \
    "https://api.github.com/repos/PLANETA9091/c-crussty/actions/runs/$RUN")
  ST=$(echo "$R" | python3 -c "import sys,json;d=json.load(sys.stdin);print(d.get('status'),d.get('conclusion'))" 2>/dev/null)
  echo "$(date -u +%H:%M:%S) poll#$i: $ST" >> /tmp/poll_dleg2.txt
  case "$ST" in
    *completed*) echo "FINAL: $ST" >> /tmp/poll_dleg2.txt; exit 0;;
  esac
done
echo "FINAL: TIMEOUT-30-polls" >> /tmp/poll_dleg2.txt
