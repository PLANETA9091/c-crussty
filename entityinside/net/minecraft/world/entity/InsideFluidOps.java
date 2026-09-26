package net.minecraft.world.entity;

import net.minecraft.core.BlockPos;
import net.minecraft.world.level.material.FluidState;
import net.minecraft.world.phys.Vec3;

/**
 * InsideFluidOps (ROUND-468-S18, R468-S18 fluid-empty fastpath on the
 * checkInsideBlocks visit lambda).
 *
 * Scope: the checkInsideBlocks decomposition (S18 RESEARCH) pinned the
 * visit-lambda sub-call census (RECON-4, leg#5 cpu-collapsed): collidedWithFluid
 * = 1671 samples = 32.6% of the visit-lambda body (5126) = 1.31% CPU — the TOP
 * un-owned vanilla sub-lane of the inside-blocks lane on the carrier. The
 * lambda calls collidedWithFluid(state.getFluidState(), pos, from, to)
 * for EVERY non-air visited block; on the flat-plane bench the overwhelming
 * majority of those fluid states are EMPTY.
 *
 * Vanilla bit-exactness (javap contract, patched-kernel.jar):
 *   Entity.collidedWithFluid: FluidState.getAABB(level,pos) -> ifnull -> false;
 *   Fluid.getAABB:            isEmpty() ? aconst_null : ...  (offset 1..8).
 * So an EMPTY fluid state ALWAYS yields false after a dead null-path walk.
 * The gate below short-circuits exactly that dead path:
 *   !fluid.isEmpty() && e.collidedWithFluid(fluid, pos, from, to)
 * — same observable boolean for every input, zero state, zero cache, fail-open
 * on the non-empty path (the vanilla body runs verbatim; its body is NOT
 * retargeted — the single retargeted call site lives in
 * Entity.lambda$checkInsideBlocks$2, and the vanilla method is called back
 * from here plain, so there is no recursion).
 *
 * Deliver: entity_compose stage (S7-162 single compose-chain) retargets the
 * ONE collidedWithFluid invokevirtual inside lambda$checkInsideBlocks$2 to
 * gate(Entity,FluidState,BlockPos,Vec3,Vec3)Z (receiver-first, 3B->3B,
 * length-preserving). NCDFE canon: this class is defined into the kernel
 * loader BEFORE the composed Entity bytes are served (inside_fluid.rs
 * ARM-AFTER-DEFINE, fa9054d9 pattern); a missed window leaves the stage out
 * of the chain and the site vanilla (fail-dominant).
 */
public final class InsideFluidOps {

    private InsideFluidOps() {
    }

    /**
     * Receiver-first static gate replacing the invokevirtual call site.
     * Bit-exact: empty fluid -> false (vanilla dead-path short-circuit),
     * otherwise the vanilla method decides (AABB clip semantics untouched).
     */
    public static boolean gate(Entity e, FluidState fluid, BlockPos pos, Vec3 from, Vec3 to) {
        return !fluid.isEmpty() && e.collidedWithFluid(fluid, pos, from, to);
    }
}
