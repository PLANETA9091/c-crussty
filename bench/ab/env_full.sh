# bench/ab/env_full.sh — arm F: c-crussty FULL surface (every env-gated lever ON).
# Sourced by run_paper_ab.sh before the arm-F JVM launch. This is the owner-
# mandated "paper + full c-crussty" posture: all deployable (env-gated) levers
# armed simultaneously, factory default-ON bridges left on, lever_flag STRICT
# A/B lanes left dormant (they are mutually exclusive by design — one flag).
# NOTE: a few of these (zero_alloc, flat_traversal, zero_cursor v1) are
# FAIL-zone documented in docs/KERNEL_POLICY.md; they are armed here because
# the full-surface posture explicitly includes them — per-lever attribution
# happens in the report.
export CRUSSTY_NATIVE_IMPROVED_NOISE=1
export CRUSSTY_NATIVE_NOISE_FILL=1
export CRUSSTY_NATIVE_PERLIN_NOISE=1
export CRUSSTY_FLUID_PUSH_GUARD=1
export CRUSSTY_PALETTED_DEMUX=1
export CRUSSTY_ALLOC_DIET=1
export CRUSSTY_SNAPREG=1
export CRUSSTY_INSIDE_CACHE=1
export CRUSSTY_INSIDE_EPOCH_GATE=1
export CRUSSTY_INSIDE_BATCH=1
export CRUSSTY_INSIDE_BITMASK=1
export CRUSSTY_FLUSH_DIET=1
export CRUSSTY_ZERO_CURSOR=1
export CRUSSTY_ZERO_ALLOC=1
export CRUSSTY_FLAT_TRAVERSAL=1
export CRUSSTY_FLUID_DIRTY=1
export CRUSSTY_FLUID_DIRTY_LEDGER=1
export CRUSSTY_FLUID_BITMASK=1
export CRUSSTY_FLUID_FREE=1
export CRUSSTY_REGION_THREADS=2
export CRUSSTY_BATCH_COLLECTOR=1
export CRUSSTY_BU_DEFER=1
export CRUSSTY_TRAVEL_DIET=1
export CRUSSTY_SKIP_STORE_BB=1
export CRUSSTY_SBLK_R1=1
export CRUSSTY_SELECTOR_BULK=1
export CRUSSTY_MC312A_GUARD=1
export CRUSSTY_BATCH=on
