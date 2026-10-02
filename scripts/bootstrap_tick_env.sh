#!/usr/bin/env bash
# bootstrap_tick_env.sh — восстановление окружения тика после сброса песочницы.
# С 14:43 +08 worktree живёт в /home/z/my-project/c-crussty (персистентно),
# /home/z/c-crussty = симлинк. Скрипт чинит симлинк и проверяет токен.
set -uo pipefail
PERSIST=/home/z/my-project/c-crussty
LINK=/home/z/c-crussty
TOKFILE=/tmp/gh_token
say() { echo "[bootstrap] $*"; }
if [ -d "$PERSIST/.git" ]; then
  [ -e "$LINK" ] || { ln -sfn "$PERSIST" "$LINK" && say "симлинк $LINK восстановлен"; }
  git -C "$PERSIST" pull --rebase origin master >/dev/null 2>&1 && say "re-pull ok ($(git -C "$PERSIST" rev-parse --short HEAD)), ahead=$(git -C "$PERSIST" rev-list --count origin/master..master)"
else
  say "FAIL: персистентный клон $PERSIST отсутствует — клонируй заново"
  exit 1
fi
if [ -s "$TOKFILE" ]; then say "токен на месте"; else say "BLOCKER: $TOKFILE пуст — push/absorb/диспатч невозможны"; exit 2; fi
say "готово"
