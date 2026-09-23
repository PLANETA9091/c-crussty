# RECON-14b — декомпозиция cursor/travel-лейна (s7173, run 35444005075 @ 27ef945)


## ALLOC: family 2,077/9,047 = 22.96%
- inside-blocks: 1,911 = 92.0% of family | leaves: net.minecraft.world.phys.AABB_[i]=744; net.minecraft.world.phys.Vec3_[i]=494; long[]_[i]=400; net.minecraft.core.BlockPos$=184
  - callers: Entity.checkInsideBlocks=1893; InsideBlockOps.mirror=18
- travel-physics: 166 = 8.0% of family | leaves: net.minecraft.world.phys.Vec3_[i]=127; net.minecraft.core.Vec3i_[i]=35; net.minecraft.world.phys.AABB_[i]=2; java.util.Optional_[i]=1
  - callers: Entity.checkInsideBlocks=166

## CPU: family 9,203/127,109 = 7.24%
- inside-blocks: 8,659 = 94.1% of family | leaves: LongOpenHashSet.add=751; ZeroCursorIter.step=688; ZeroCursorOps.lambda8=546; BatchCollector.flushStep=438
  - callers: Entity.checkInsideBlocks=8612; InsideBlockOps.mirror=47
- travel-physics: 544 = 5.9% of family | leaves: BlockGetter.addCollisionsAlongTravel=151; BlockGetter.getFurthestCorner=105; Mth.floor=76; Mth.frac=53
  - callers: Entity.checkInsideBlocks=544
