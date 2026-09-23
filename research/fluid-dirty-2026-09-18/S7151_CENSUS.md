# S7-151 CENSUS v2 — вызыватели scan/section-write (kernel e2992d63)

## Вызыватели Entity.updateFluidHeightAndDoFluidPushing (invoke): 1 классов

- `net/minecraft/world/entity/Entity` — 2 вызовов:
    41: invokevirtual #3034               // Method updateFluidHeightAndDoFluidPushing:(Lnet/minecraft/tags/TagKey;D)Z
    39: invokevirtual #3034               // Method updateFluidHeightAndDoFluidPushing:(Lnet/minecraft/tags/TagKey;D)Z

## LevelChunk: вызовы LevelChunkSection.setBlockState:

- in `None`
    73: invokevirtual #651                // Method net/minecraft/world/level/chunk/LevelChunkSection.setBlockState:(IIILnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/state/BlockState;
