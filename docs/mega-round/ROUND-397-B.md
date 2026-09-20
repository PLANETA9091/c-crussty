# ROUND-397-B — ITEMS-STAGGER v2 (lever_flag="items_stagger2", CRUSSTY_LEVER_ARG=1)

Re-arm of the ROUND-396-B vector B after the fail-closed dormant verdict.
Same mechanism: phase-scheduled heavy checks of ItemEntity.tick —
`due(e, unit) = ((e.tickCount/unit) + e.getId()) % N == 0`, unit = vanilla
grid 40 (merge) / 4 (settled move) / 1 (noCollision). ARG=1 (round-397
dispatch convention, variant selector) -> designed **N=4** schedule; ARG>1
selects that N directly (bridge static-init). Dormant unless
CRUSSTY_LEVER_FLAG == "items_stagger2" (exact) — vanilla by construction.

## Root cause of the round-1 dormant (real kernel census, purpur-1.21.10)

javap of the run-s7204 patched-kernel `ItemEntity.tick()V`:

| site | bc | opcode | Methodref owner | round-1 pattern |
|---|---|---|---|---|
| 174 | `Level.noCollision(Entity;AABB)Z` | invokevirtual | Level | ✓ matched |
| 269 | `move(MoverType;Vec3)V` | invokevirtual | **ItemEntity** | ✗ expected Entity |
| 471 | `mergeWithNeighbours()V` (private) | invokevirtual (JEP 181) | ItemEntity | ✓ matched |

The move site is an INHERITED `this.move(...)`: javac emits the receiver's
static type as the Methodref owner (`ItemEntity.move`), not the declarator.
Round-1's `retarget_virtual_to_static` scanned invokevirtual-only and
matched the declarator triple only → gate-2 NotFound → strict compose
rejected → fail-closed dormant. (The "invokespecial super-call" wording of
the round-1 verdict was imprecise about the opcode; the fix is the same.)

## Fix (both edits per task)

1. **`classfile.rs`: new `retarget_invoke_to_static`** — scans
   invokevirtual (0xb6) AND invokespecial (0xb7), matches ANY of the given
   from-triples (owner tolerance: ItemEntity or Entity for move), validates
   the receiver-prepended static shape for any receiver class
   (`desc_after_first_param`), same-length rewrite to invokestatic.
   The pre-existing `retarget_virtual_to_static` is untouched (other levers
   keep byte-identical behavior).
2. **Gate-2 kept**: with the corrected census the pattern is proven safe on
   all 3 sites (sites=1 each, receiver-on-stack forms, ItemEntity declares
   no move override so the bridge's `self.move(...)` virtual dispatch lands
   exactly where vanilla's call resolved) → ARM all 3 gates, no partial arm.
   Flag-gate + parity-by-construction unchanged.

Parity deviations (DOC-DEV, same as round-1): merge latency ≤ 40 ticks ×
phase spread (final merge set identical — vanilla mergeWithNeighbours body
runs on due ticks via privateLookup MethodHandle); settled items skip the
(4→4·N)-th micro-move (position already grounded; xo/yo/zo set before the
site); squeezed-items re-check noCollision within ≤ N ticks (safe default
false = normal physics). Merge/despawn/pickup/baseTick untouched.

## Validation

- javac --release 21 bridge vs kernel-signature stubs (members verified
  against the kernel javap census; javac reproduces the ItemEntity-owner
  methodref behavior on inherited calls).
- rust: `cargo test --lib` 202 pass incl. new
  `items_stagger2_move_sites_virtual_and_special_owner_tolerant`
  (synthetic class with virtual-ItemEntity + special-Entity move sites:
  both rewritten, idempotent AlreadyPatched, foreign owner NotFound).
- Strict compose sites=1 ×3 on the real census shapes; fail-dominant on
  any violation; probe-before-retransform; boot-quiet activation.
