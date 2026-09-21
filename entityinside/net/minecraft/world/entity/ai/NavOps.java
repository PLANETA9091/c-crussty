package net.minecraft.world.entity.ai;

import net.minecraft.core.BlockPos;
import net.minecraft.tags.BlockTags;
import net.minecraft.tags.FluidTags;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.level.block.BaseRailBlock;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.DoorBlock;
import net.minecraft.world.level.block.FenceGateBlock;
import net.minecraft.world.level.block.LeavesBlock;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.material.FluidState;
import net.minecraft.world.level.pathfinder.PathComputationType;
import net.minecraft.world.level.pathfinder.PathType;
import net.minecraft.world.level.pathfinder.WalkNodeEvaluator;

/**
 * TASK-401-C nav-subsystem bridge (cmp401_navsys) — canonical BlockState →
 * PathType memo with the table living on the rust side (nav_path_type.rs,
 * flat dense array indexed by the BLOCK_STATE_REGISTRY id).
 *
 * PARITY: WalkNodeEvaluator.getPathTypeFromState(BlockGetter, BlockPos) is a
 * PURE function of the canonical (immutable, interned) BlockState — its whole
 * vanilla body is a tag/instanceof/property chain (Lithium ai.pathing upstream
 * proof). Keying the memo by Block.getId(state) (dense registry id of the
 * canonical state) makes staleness IMPOSSIBLE: identical state ⇒ identical
 * result. computeVanillaPathType below is a 1:1 port of the kernel bytecode
 * chain (javap roundtrip, incl. the fence-gate fall-through and the
 * WITHER_ROSE/POINTED_DRIPSTONE DAMAGE_CAUTIOUS branch order).
 *
 * FAIL-CLOSED: natives unbound ⇒ NATIVES_OK=false ⇒ direct compute (vanilla
 * semantics, zero cache); rust table rejects an id ⇒ recompute next call.
 * Any Throwable ⇒ vanilla-semantics compute. No allocation on the hot path.
 */
public final class NavOps {
    private static final PathType[] TYPES = PathType.values();

    private static final int NAV_PROBE_MAGIC = 0x401C;
    private static final boolean NATIVES_OK;

    static {
        boolean ok;
        try {
            ok = navProbe() == NAV_PROBE_MAGIC;
        } catch (Throwable t) {
            ok = false;
        }
        NATIVES_OK = ok;
    }

    private NavOps() {
    }

    public static native int navProbe();          // ()I magic probe
    public static native int navTypeGet(int id);  // (I)I -> ordinal or -1 (miss)
    public static native int navTypePut(int id, int ordinal); // (II)I -> 0 ok / -1 reject

    /**
     * Redirect target for
     * WalkNodeEvaluator.getPathTypeFromState(BlockGetter, BlockPos) — same
     * static descriptor, vanilla semantics, memoized per canonical state.
     */
    public static PathType getPathTypeFromState(BlockGetter level, BlockPos pos) {
        BlockState state = level.getBlockStateIfLoaded(pos);
        if (state == null) {
            return PathType.BLOCKED;
        }
        if (NATIVES_OK) {
            int id = Block.getId(state);
            int cached = navTypeGet(id);
            if (cached >= 0 && cached < TYPES.length) {
                return TYPES[cached];
            }
            PathType computed = computeVanillaPathType(state);
            navTypePut(id, computed.ordinal());
            return computed;
        }
        return computeVanillaPathType(state);
    }

    /**
     * 1:1 port of the vanilla WalkNodeEvaluator.getPathTypeFromState chain
     * (kernel bytecode, javap-verified). Must not throw for any state.
     */
    private static PathType computeVanillaPathType(BlockState state) {
        Block block = state.getBlock();
        if (state.isAir()) {
            return PathType.OPEN;
        }
        if (state.is(BlockTags.TRAPDOORS) || state.is(Blocks.LILY_PAD) || state.is(Blocks.BIG_DRIPLEAF)) {
            return PathType.TRAPDOOR;
        }
        if (state.is(Blocks.POWDER_SNOW)) {
            return PathType.POWDER_SNOW;
        }
        if (state.is(Blocks.CACTUS) || state.is(Blocks.SWEET_BERRY_BUSH) || state.is(Blocks.STONECUTTER)) {
            return PathType.DAMAGE_OTHER;
        }
        if (state.is(Blocks.HONEY_BLOCK)) {
            return PathType.STICKY_HONEY;
        }
        if (state.is(Blocks.COCOA)) {
            return PathType.COCOA;
        }
        if (state.is(Blocks.WITHER_ROSE) || state.is(Blocks.POINTED_DRIPSTONE)) {
            return PathType.DAMAGE_CAUTIOUS;
        }
        FluidState fluid = state.getFluidState();
        if (fluid.is(FluidTags.LAVA)) {
            return PathType.LAVA;
        }
        if (WalkNodeEvaluator.isBurningBlock(state)) {
            return PathType.DAMAGE_FIRE;
        }
        if (block instanceof DoorBlock door) {
            if (state.getValue(DoorBlock.OPEN)) {
                return PathType.DOOR_OPEN;
            }
            return door.type().canOpenByHand() ? PathType.DOOR_WOOD_CLOSED : PathType.DOOR_IRON_CLOSED;
        }
        if (block instanceof BaseRailBlock) {
            return PathType.RAIL;
        }
        if (block instanceof LeavesBlock) {
            return PathType.LEAVES;
        }
        if (state.is(BlockTags.FENCES) || state.is(BlockTags.WALLS)
                || (block instanceof FenceGateBlock && !state.getValue(FenceGateBlock.OPEN))) {
            return PathType.FENCE;
        }
        if (!state.isPathfindable(PathComputationType.LAND)) {
            return PathType.BLOCKED;
        }
        if (fluid.is(FluidTags.WATER)) {
            return PathType.WATER;
        }
        return PathType.OPEN;
    }
}
