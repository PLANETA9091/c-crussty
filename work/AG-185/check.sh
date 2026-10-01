#!/bin/bash
TOK=$(cat /tmp/gh_token)
REPO="PLANETA9091/c-crussty"
for id in "$@"; do
  curl -s -H "Authorization: token $TOK" "https://api.github.com/repos/$REPO/actions/runs/$id" \
   | python3 -c "
import json,sys
try:
  d=json.load(sys.stdin)
  print(f\"{d.get('id','?')}\t{str(d.get('name','?'))[:32]}\t{str(d.get('head_branch','?'))[:34]}\t{d.get('status','?')}\t{d.get('conclusion','?')}\t{d.get('created_at','?')}\")
except Exception as e:
  print('$id\tERR', e)
"
done
