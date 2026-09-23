#!/usr/bin/env bash
# S7-84 a22 OFFLINE COMPILE-VERIFY (pre-registered: 0 boots if fail)
# Extracts the inline Rust cdylib + Java agent from crac_p6b_realserver.sh and compiles both.
set -u
RIG=/home/z/ccrussty/c-crussty/bench/boot/crac_p6b_realserver.sh
V=/tmp/crac_v12_verify; rm -rf "$V"; mkdir -p "$V"; cd "$V"
CRACJAR=/tmp/crac_p5/crac.jar
[ -f "$CRACJAR" ] || { echo "VERIFY-ABORT no $CRACJAR"; exit 9; }
# 1. Rust extract + compile
awk '/^cat > fd_surgery.rs << .REOF.$/{f=1;next} /^REOF$/{f=0} f' "$RIG" > fd_surgery.rs
wc -l fd_surgery.rs
export PATH="$HOME/.cargo/bin:$PATH"
if rustc --edition 2021 -O --crate-type cdylib fd_surgery.rs -o libfdsurgery.so 2> rustc.err; then
  echo "RUSTC-VERIFY-OK ($(nm -D libfdsurgery.so | grep -c Java_CrusstyCracHookV2) natives)"
else echo "RUSTC-VERIFY-FAIL"; head -20 rustc.err; exit 11; fi
# 2. Java extract + compile (same javac line as rig)
awk '/^cat > CrusstyCracHookV2.java << .JEOF.$/{f=1;next} /^JEOF$/{f=0} f' "$RIG" > CrusstyCracHookV2.java
wc -l CrusstyCracHookV2.java
if /home/z/jdk21/bin/javac -cp "$CRACJAR" CrusstyCracHookV2.java 2> javac.err; then
  echo "JAVAC-VERIFY-OK ($(ls CrusstyCracHookV2*.class | wc -l) classes)"
else echo "JAVAC-VERIFY-FAIL"; cat javac.err; exit 12; fi
# 3. bash syntax of the rig itself
bash -n "$RIG" && echo "BASH-SYNTAX-OK" || { echo "BASH-SYNTAX-FAIL"; exit 13; }
echo "COMPILE-VERIFY-ALL-OK"
