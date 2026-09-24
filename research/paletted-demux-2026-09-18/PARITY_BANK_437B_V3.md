# PALETTED-DEMUX parity bank — v3 WRITE-PATH GUARD re-run (TASK-437-B, cmp436_pdemux)

Run: `parity_harness_output_437b_v3.txt` (this dir) — tick-437, branch round-437-b-pdemux.

## Bank contract vs the S7-131/S7-145 v2 bank (`parity_harness_output.txt`)

| leg | v2 bank (S7-131/S7-145) | v3 run (437-B) | verdict |
|---|---|---|---|
| vanilla fixture fingerprint | len=30967, major=65 | len=30967, major=65 (byte-identical vs round-396-a kernel entry) | MATCH |
| 20000-op lockstep | PASS 4883/4883 | PASS 4883/4883 (same seed 424242, same op stream) | MATCH |
| 1000 fast-path reads parity | PASS | PASS | MATCH |
| write unmarks snap + refcount release | PASS (1 -> 0) | PASS (1 -> 0) | MATCH |
| re-materialize + parity | PASS | PASS | MATCH |
| concurrency smoke (3R+1W, 500ms) | PASS | PASS | MATCH |
| patched image len | 31340 | 31521 (+181 = gate machinery) | EXPECTED DELTA |
| snapGen trace at force | 30009 (gen 30008 = 15004 x 2 bumps) | 883 (gen 882 = 441 x 2 bumps + ~14.5k SKIPPED pairs) | DESIGNED DELTA |

## Why snapGen 30009 -> 883 is the guard's fingerprint, not a drift

v2 paid the 2x volatile gen++ pair on EVERY mutation, so gen = 2 x mutations
for the whole stream (~15004 mutations -> gen 30008). v3's gate
(crusstyWriteGate/crusstyEpilogue) skips the volatile pair when
snapGen==0 && crusstyEpoch>=2 (release-blacklisted, write-heavy). In the
20k-op stream the container hits the 2-build blacklist at ~op 600; from that
point ~97% of mutations stop paying the pair — gen freezes at 882 and the
forced materialize publishes snapGen=883. All SEMANTIC assertions (read
parity, old-value parity, refcount, concurrency) hold bit-for-bit: the skip
state is read-invisible (no snapshot live, none publishable — the
tryMaterialize epoch-transition guard closes the publish-into-skip race).

Read-path (get fast path + Ops.get) is UNTOUCHED by v3 — the fast-lane win
(paletted 7.06% -> 5.76%, x435) is preserved; only the write-hot tax is
removed.

## Environment note (437-B reproduction)

/tmp/pdec/matsrv (purpur jar + paperclip libraries) was purged; the run used
KERNEL_JAR=research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar (its
PalettedContainer.class entry is byte-identical to the pinned 30967B fixture
— verified with cmp) and a mirror libraries dir (/tmp/pdec-mirror-libs:
fastutil, DFU 8.0.16, guava 32.1.2, netty 4.1.97, slf4j/log4j 2.22.1,
concurrentutil 0.0.2, commons-lang3 3.14.0, paper-api, adventure, joml).
ASM: cplug-sdk/asm-lib/asm-9.7.1.jar; ECJ: randomtick/ecj.jar.
