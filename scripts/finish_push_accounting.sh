#!/usr/bin/env bash
# finish_push_accounting.sh — ОДНА КОМАНДА на первый тик с токеном:
# (1) пуш накопленных коммитов c-crussty; (2) backfill CLAIMS 365-369 в logs-репо;
# (3) re-pull обоих. После него — absorb 35488526730 (API-скача) → развилка I.
set -euo pipefail
CC=/home/z/c-crussty
LOGS=/home/z/crussty-dev-logs
TOKFILE=/tmp/gh_token
say() { echo "[finish] $*"; }

[ -s "$TOKFILE" ] || { say "BLOCKER: нет $TOKFILE"; exit 2; }
TOK=$(cat "$TOKFILE")

# (1) пуш ядра
cd "$CC"
AHEAD=$(git rev-list --count origin/master..master)
say "c-crussty: ahead=$AHEAD — пушу"
git push origin master
git pull --rebase origin master >/dev/null 2>&1

# (2) logs-репо: клон/пул + backfill CLAIMS
if [ ! -d "$LOGS/.git" ]; then
  git clone -q -b main "https://x-access-token:${TOK}@github.com/PLANETA9091/crussty-dev-logs.git" "$LOGS"
fi
cd "$LOGS" && git pull --rebase origin main >/dev/null 2>&1
BACKFILL="$CC/scripts/claims_backfill_365-369.md"
if [ -f "$BACKFILL" ]; then
  if ! rg -q "TASK-369" CLAIMS.md 2>/dev/null; then
    cat "$BACKFILL" >> CLAIMS.md
    git add CLAIMS.md
    git commit -q -m "CLAIMS backfill TASK-365..369: блокер-период (сбросы песочницы, потеря токена) - персистентный worktree, absorb v2.1, digest-анкер"
    git push origin main
    say "CLAIMS backfill 365-369 запушен"
  else
    say "CLAIMS 369 уже есть — backfill пропущен"
  fi
fi
git pull --rebase origin main >/dev/null 2>&1
say "готово. СЛЕДУЮЩЕЕ ДЕЙСТВИЕ: python3 scripts/bench4_recon/absorb_s7196.py 35488526730"
