#!/usr/bin/env bash
# safe_ledger_push.sh — x466 14f/C77: изолированный worktree-флоу LEDGER-push.
#
# ЗАЧЕМ (уроки x466):
#   ×466-урок-2 (C12 раскритие): LEDGER-аппенд из ОБЩЕГО worktree /home/z/c-crussty
#     сделал `push HEAD:master`, который протащил на master ЧУЖИЕ коммиты
#     (C02-фиксы 90fd259f+66a57c02 + mega-drift 24264d7b) — канон нарушен.
#   ×466-урок-1: общие worktree = lost-update ×2; канон — изолированный worktree.
#
# ЧТО ДЕЛАЕТ (флоу, каждый шаг верифицируется):
#   1. git fetch origin master                     (свежий снапшот)
#   2. git worktree add --no-checkout --detach WT origin/master + SPARSE-Checkout
#      только docs/ (disk-канон: box 100% full ×9.9G — полный checkout worktree
#      ≈165M+ на аппенд в 1 файл недопустим; sparse = KB-масштаб)
#      + инвариант: WT.HEAD == origin/master и ledger материализован
#   3. append секции в docs/LAB_LEDGER.md          (только этот файл!)
#      + инвариант: git status --porcelain показывает ТОЛЬКО ledger
#   4. commit (subject из --msg)
#   5. push origin HEAD:master — гонка аппендов двух агентов = rejected →
#      авто-retry (≤ MAX_RETRY): re-fetch → reset --hard → re-append → re-commit
#   6. пост-верификация: origin/master == запушенный sha
#   7. cleanup: worktree remove --force (trap — даже при FAIL)
#
# Никаких веток не создаёт (detached),cwd не трогает, чужие коммиты физически
# не могут попасть в push: WT рождён от origin/master и содержит ровно 1
# новый коммит.
#
# Usage:
#   scripts/safe_ledger_push.sh -F секция.md [-m "commit subject"]
#   scripts/safe_ledger_push.sh -m "однострочный аппенд" [-m "subject"]
#   cat секция.md | scripts/safe_ledger_push.sh [-m "commit subject"]
# Env: DRY_RUN=1 (всё кроме commit+push; печать хвоста+diff), MAX_RETRY=3,
#      LEDGER_FILE=docs/LAB_LEDGER.md.
# Exit: 0 = PUSHED <sha>; 1 = FAIL (cleanup гарантирован).

set -uo pipefail

REPO="$(cd "$(dirname "$0")/.." && pwd)"
LEDGER_FILE="${LEDGER_FILE:-docs/LAB_LEDGER.md}"
MAX_RETRY="${MAX_RETRY:-3}"
SUBJECT=""
SECTION_FILE=""
SECTION_TEXT=""

while [ $# -gt 0 ]; do
  case "$1" in
    -m) if [ -z "$SUBJECT" ]; then SUBJECT="$2"; else SECTION_TEXT+="$2"$'\n'; fi; shift 2 ;;
    -F) SECTION_FILE="$2"; shift 2 ;;
    -h|--help) sed -n '2,34p' "$0"; exit 0 ;;
    *) echo "safe_ledger_push: usage-FAIL '$1' (см. --help)" >&2; exit 2 ;;
  esac
done

if [ -n "$SECTION_FILE" ]; then
  [ -f "$SECTION_FILE" ] || { echo "safe_ledger_push: FAIL -F '$SECTION_FILE' не найден" >&2; exit 1; }
  SECTION_TEXT+="$(cat "$SECTION_FILE")"$'\n'
fi
if [ -z "$SECTION_FILE" ] && [ -z "$SECTION_TEXT" ] && [ ! -t 0 ]; then
  SECTION_TEXT+="$(cat)"$'\n'
fi
SECTION_TEXT="${SECTION_TEXT//$'\r'/}"
[ -n "${SECTION_TEXT//[[:space:]]/}" ] || { echo "safe_ledger_push: FAIL пустой аппенд" >&2; exit 2; }
[ -f "$REPO/$LEDGER_FILE" ] || { echo "safe_ledger_push: FAIL $REPO/$LEDGER_FILE отсутствует" >&2; exit 1; }

DRY="${DRY_RUN:-0}"
WT=""
cleanup() {
  if [ -n "$WT" ] && [ -d "$WT" ]; then
    git -C "$REPO" worktree remove --force "$WT" >/dev/null 2>&1 || rm -rf "$WT"
    git -C "$REPO" worktree prune >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

step() { printf '[safe_ledger] %s\n' "$1"; }

step "repo=$REPO ledger=$LEDGER_FILE retry=$MAX_RETRY dry=$DRY"

# --- 1. fetch ---
step "fetch origin master"
git -C "$REPO" fetch origin master >/dev/null 2>&1 || { echo "safe_ledger_push: FAIL fetch" >&2; exit 1; }
BASE="$(git -C "$REPO" rev-parse origin/master)" || exit 1
step "base origin/master=$BASE"

# --- 2. изолированный worktree (SPARSE: только каталог ledger — disk-канон) ---
WT="$(mktemp -d "${TMPDIR:-/tmp}/safe_ledger.XXXXXX")"
git -C "$REPO" worktree add --no-checkout --detach "$WT" "$BASE" >/dev/null 2>&1 || { echo "safe_ledger_push: FAIL worktree add" >&2; exit 1; }
LEDGER_DIR="$(dirname "$LEDGER_FILE")"
if [ "$LEDGER_DIR" != "." ]; then
  git -C "$WT" sparse-checkout init --cone >/dev/null 2>&1 \
    && git -C "$WT" sparse-checkout set "$LEDGER_DIR" >/dev/null 2>&1 \
    || { echo "safe_ledger_push: FAIL sparse-checkout init" >&2; exit 1; }
fi
git -C "$WT" checkout -f "$BASE" --quiet >/dev/null 2>&1 || { echo "safe_ledger_push: FAIL sparse checkout" >&2; exit 1; }
WT_HEAD="$(git -C "$WT" rev-parse HEAD)"
[ "$WT_HEAD" = "$BASE" ] || { echo "safe_ledger_push: FAIL инвариант WT.HEAD($WT_HEAD) != base($BASE)" >&2; exit 1; }
[ -f "$WT/$LEDGER_FILE" ] || { echo "safe_ledger_push: FAIL $LEDGER_FILE не материализован в sparse-WT" >&2; exit 1; }
step "worktree=$WT (detached @$BASE, sparse=$LEDGER_DIR, изолирован — ×466-урок-1)"

append_section() {
  local f="$WT/$LEDGER_FILE"
  [ -s "$f" ] || { echo "safe_ledger_push: FAIL ledger пуст/нет в WT" >&2; return 1; }
  # хвостовой '\n' канон: аппенд не склеивается с последней строкой
  [ -n "$(tail -c1 "$f")" ] && printf '\n' >> "$f"
  printf '%s\n' "$SECTION_TEXT" >> "$f"
  return 0
}

PUSHED=""
attempt=1
while :; do
  # --- 3. append + инвариант только-ledger ---
  git -C "$WT" reset --hard "$BASE" --quiet
  append_section || exit 1
  DIRTY="$(git -C "$WT" status --porcelain)"
  BAD="$(printf '%s\n' "$DIRTY" | grep -v "^ M $LEDGER_FILE\$" || true)"
  [ -z "$BAD" ] || { echo "safe_ledger_push: FAIL тронуты чужие пути (×466-урок-2 инвариант):" >&2; printf '%s\n' "$BAD" >&2; exit 1; }
  [ -n "$DIRTY" ] || { echo "safe_ledger_push: FAIL аппенд не изменил ledger" >&2; exit 1; }

  if [ "$DRY" = "1" ]; then
    step "DRY_RUN: diff-хвост аппенда (5 строк):"
    tail -5 "$WT/$LEDGER_FILE" | sed 's/^/    | /'
    step "DRY_RUN: push НЕ выполнен, cleanup"
    exit 0
  fi

  # --- 4. commit ---
  SUBJECT="${SUBJECT:-LEDGER append via safe_ledger_push (x466 14f): isolated-worktree flow}"
  if ! git -C "$WT" commit -q -m "$SUBJECT" -- "$LEDGER_FILE"; then
    echo "safe_ledger_push: FAIL commit" >&2; exit 1
  fi
  SHA="$(git -C "$WT" rev-parse HEAD)"
  step "commit=$SHA (только $LEDGER_FILE поверх $BASE)"

  # --- 5. push ---
  if PUSH_OUT="$(git -C "$WT" push origin HEAD:master 2>&1)"; then
    PUSHED="$SHA"
    step "push OK: HEAD:master"
    break
  fi
  if printf '%s\n' "$PUSH_OUT" | grep -qiE "rejected|non-fast-forward|fetch first"; then
    if [ "$attempt" -ge "$MAX_RETRY" ]; then
      echo "safe_ledger_push: FAIL push после $MAX_RETRY попыток (гонка аппендов)" >&2
      exit 1
    fi
    attempt=$((attempt + 1))
    step "push rejected (гонка) — retry $attempt/$MAX_RETRY: re-fetch → re-append"
    git -C "$REPO" fetch origin master >/dev/null 2>&1 || { echo "safe_ledger_push: FAIL re-fetch" >&2; exit 1; }
    BASE="$(git -C "$REPO" rev-parse origin/master)"
    continue
  fi
  echo "safe_ledger_push: FAIL push: $PUSH_OUT" >&2
  exit 1
done

# --- 6. пост-верификация ---
git -C "$REPO" fetch origin master >/dev/null 2>&1
REMOTE_NOW="$(git -C "$REPO" rev-parse origin/master)"
if [ "$REMOTE_NOW" != "$PUSHED" ]; then
  echo "safe_ledger_push: FAIL пост-верификация: origin/master($REMOTE_NOW) != pushed($PUSHED)" >&2
  exit 1
fi
step "пост-верификация: origin/master == $PUSHED"
echo "PUSHED $PUSHED master (LEDGER $LEDGER_FILE, +$((attempt-1)) retry)"
