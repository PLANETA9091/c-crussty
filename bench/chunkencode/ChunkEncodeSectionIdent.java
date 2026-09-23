import io.netty.buffer.Unpooled;
import net.minecraft.SharedConstants;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Holder;
import net.minecraft.core.IdMap;
import net.minecraft.core.LayeredRegistryAccess;
import net.minecraft.core.Registry;
import net.minecraft.core.RegistryAccess;
import net.minecraft.core.registries.Registries;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode;
import net.minecraft.server.RegistryLayer;
import net.minecraft.world.level.biome.Biome;
import net.minecraft.world.level.biome.Biomes;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.PalettedContainer;

import net.minecraft.world.level.chunk.PalettedContainerRO;
import net.minecraft.world.level.chunk.Strategy;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.Random;

/**
 * TASK-149 phase-1 — nativeEncodeSectionData/Sized ABI identification (n=1 rows, black-box).
 *
 * Reference = REAL vanilla wire bytes: LevelChunkSection.write semantics re-produced
 * section-by-section with REAL PalettedContainer.write(FriendlyByteBuf) calls:
 *   [short nonEmpty][byte bits][palette: VarInt(0)+id | VarInt(n)+ids | nothing][VarInt len + longs]
 * Native input hypothesis v1 (structural match to the *Sized variant's grouping):
 *   a0  short[]  blockCounts per section
 *   a1  byte[]   block bits per section
 *   a2  int[]    block palette global ids, flattened in section order
 *   a3  byte[]   block palette sizes per section (0 = global, k = explicit)
 *   a4  int[]    block palette offsets per section (into a2)
 *   a5  long[]   block packed data, flattened (empty slice for bits==0)
 *   a6  byte[]   biome bits per section
 *   a7  int[]    biome palette global ids, flattened
 *   a8  byte[]   biome palette sizes per section
 *   a9  int[]    biome palette offsets per section
 *   a10 long[]   biome packed data, flattened
 *   a11 byte[]   dst
 * return: written-length hypothesis (printed, not assumed).
 *
 * Output lines (stdout, TSV):
 *   IDENT\tcase\t<N>\trefLen=\tjavaRc=\tmismatchAt=\tverdict=MATCH|MISMATCH
 *   HEXDIFF\tcase\toff=\tref:...\tnat:...
 *   SECTION\tcase\tidx\tbits=\tpal=\tdataLongs=
 */
public final class ChunkEncodeSectionIdent {

    private static final int N = 24; // 1.21.10 block sections: (-64..320)/16
    private static final int SECTION_VOLUME = 16 * 16 * 16;

    record SecData(short nonEmpty, PalettedContainer<BlockState> states, PalettedContainer<Holder<Biome>> biomes) {}

    public static void main(String[] args) throws Exception {
        String libs = System.getProperty("chunkencode.libs", "");
        if (libs.isEmpty()) { System.err.println("FATAL: -Dchunkencode.libs not set"); System.exit(2); }
        for (String p : libs.split(java.io.File.pathSeparator)) {
            if (!p.isBlank()) System.load(p);
        }
        System.err.println("chunk-encode .so loaded (SectionData identification)");
        try { SharedConstants.tryDetectVersion(); } catch (Throwable ignored) { }
        try { net.minecraft.server.Bootstrap.bootStrap(); } catch (Throwable ignored) { }

        IdMap<BlockState> blockReg = Block.BLOCK_STATE_REGISTRY;
        Strategy<BlockState> blockStrategy = Strategy.createForBlockStates(blockReg);
        // Synthetic biome id-space: the biome registry is datapack-loaded and absent from
        // static RegistryAccess layers. Wire identification needs only a SELF-CONSISTENT
        // IdMap<Holder<Biome>> — the kernel receives global ids as plain ints; the vanilla
        // reference emits ids from this same map. Value objects are never dereferenced.
        List<Holder<Biome>> biomeChoices = new ArrayList<>();
        for (int b = 0; b < 16; b++) biomeChoices.add(fakeBiomeHolder(b));
        IdMap<Holder<Biome>> biomeReg = new IdMap<>() {
            public int getId(Holder<Biome> h) {
                for (int i = 0; i < biomeChoices.size(); i++) if (biomeChoices.get(i) == h) return i;
                return -1;
            }
            public Holder<Biome> byId(int i) { return (i >= 0 && i < biomeChoices.size()) ? biomeChoices.get(i) : null; }
            public int size() { return biomeChoices.size(); }
            public java.util.Iterator<Holder<Biome>> iterator() { return biomeChoices.iterator(); }
        };
        Strategy<Holder<Biome>> biomeStrategy = Strategy.createForBiomes(biomeReg);

        int totalBlocks = 0, totalBiomes = 0;
        int maxBlockId = blockReg.size() - 1;
        Random r = new Random(0x51EC7102L);

        for (int n = 20; n <= 26; n++) {          // probe N in {20..26} — pins the native's N source
            SecData[] secs = buildSections(n, maxBlockId, biomeChoices, blockReg, blockStrategy, biomeStrategy, r);
            // reference
            FriendlyByteBuf ref = new FriendlyByteBuf(Unpooled.buffer());
            for (int i = 0; i < n; i++) {
                SecData s = secs[i];
                ref.writeShort(s.nonEmpty);
                s.states.write(ref);
                s.biomes.write(ref);
            }
            byte[] refBytes = new byte[ref.writerIndex()];
            ref.getBytes(0, refBytes);

            // native inputs per hypothesis v1
            short[] counts = new short[n];
            byte[] bitsB = new byte[n];
            List<Integer> pal = new ArrayList<>();
            byte[] palSizes = new byte[n];
            int[] palOffs = new int[n];
            List<Long> data = new ArrayList<>();
            byte[] bitsBiome = new byte[n];
            List<Integer> palBiome = new ArrayList<>();
            byte[] palSizesBiome = new byte[n];
            int[] palOffsBiome = new int[n];
            List<Long> dataBiome = new ArrayList<>();

            for (int i = 0; i < n; i++) {
                SecData s = secs[i];
                counts[i] = s.nonEmpty;
                PalettedContainerRO.PackedData<BlockState> pb = s.states.pack(blockStrategy);
                int bits = s.states.bitsPerEntry();
                bitsB[i] = (byte) bits;
                List<BlockState> entries = pb.paletteEntries();
                if (bits == 0) {
                    palSizes[i] = 1;                      // single-value: wire = VarInt(0)+id
                    palOffs[i] = pal.size();
                    pal.add(blockReg.getId(entries.get(0)));
                    // no storage longs on the wire for bits==0 (VarInt(0) length) — empty data slice
                } else {
                    boolean global = entries.isEmpty();
                    palSizes[i] = (byte) entries.size();  // global => 0
                    palOffs[i] = pal.size();
                    for (BlockState st : entries) pal.add(blockReg.getId(st));
                    long[] raw = pb.storage().map(java.util.stream.LongStream::toArray).orElse(new long[0]);
                    for (long v : raw) data.add(v);
                }
                PalettedContainerRO.PackedData<Holder<Biome>> pbi = s.biomes.pack(biomeStrategy);
                int bitsi = s.biomes.bitsPerEntry();
                bitsBiome[i] = (byte) bitsi;
                List<Holder<Biome>> entriesi = pbi.paletteEntries();
                if (bitsi == 0) {
                    palSizesBiome[i] = 1;
                    palOffsBiome[i] = palBiome.size();
                    palBiome.add(biomeReg.getId(entriesi.get(0)));
                } else {
                    palSizesBiome[i] = (byte) entriesi.size();
                    palOffsBiome[i] = palBiome.size();
                    for (Holder<Biome> b : entriesi) palBiome.add(biomeReg.getId(b));
                    long[] rawi = pbi.storage().map(java.util.stream.LongStream::toArray).orElse(new long[0]);
                    for (long v : rawi) dataBiome.add(v);
                }
            }

            short[] countsF = counts; byte[] bitsF = bitsB;
            int[] palF = pal.stream().mapToInt(Integer::intValue).toArray();
            long[] dataF = data.stream().mapToLong(Long::longValue).toArray();
            byte[] bitsBiomeF = bitsBiome;
            int[] palBiomeF = palBiome.stream().mapToInt(Integer::intValue).toArray();
            long[] dataBiomeF = dataBiome.stream().mapToLong(Long::longValue).toArray();
            byte[] dst = new byte[1 << 20];

            int rc = PaperNativeChunkPacketEncode.nativeEncodeSectionData(
                    countsF, bitsF, palF, palSizes, palOffs, dataF,
                    bitsBiomeF, palBiomeF, palSizesBiome, palOffsBiome, dataBiomeF, dst);

            // compare
            int mism = -1;
            int lim = Math.min(refBytes.length, dst.length);
            for (int i = 0; i < lim; i++) {
                if (dst[i] != refBytes[i]) { mism = i; break; }
            }
            String verdict = (mism == -1 && refBytes.length <= lim) ? "MATCH" : "MISMATCH";
            System.out.println("IDENT\tcaseN" + n + "\trefLen=" + refBytes.length
                    + "\trc=" + rc + "\tpalLen=" + palF.length + "\tdataLen=" + dataF.length
                    + "\tmismatchAt=" + mism + "\tverdict=" + verdict);
            if ("MATCH".equals(verdict)) continue;
            // hexdump around divergence (or start)
            int at = mism == -1 ? Math.min(refBytes.length, lim) : mism;
            int from = Math.max(0, at - 16);
            StringBuilder sb = new StringBuilder("HEXDIFF\tcaseN" + n + "\toff=" + at + "\tref:");
            for (int i = from; i < Math.min(refBytes.length, at + 16); i++) sb.append(String.format("%02x", refBytes[i]));
            sb.append("\tnat:");
            for (int i = from; i < Math.min(dst.length, at + 16); i++) sb.append(String.format("%02x", dst[i]));
            System.out.println(sb);
            // per-section summary of the first 3 sections to localize
            for (int i = 0; i < Math.min(3, n); i++) {
                SecData s = secs[i];
                System.out.println("SECTION\tcaseN" + n + "\tidx=" + i + "\tbits=" + s.states.bitsPerEntry()
                        + "\tnonEmpty=" + s.nonEmpty + "\tbiomeBits=" + s.biomes.bitsPerEntry());
            }
        }
        System.out.println("SINK\t" + (totalBlocks + totalBiomes));
    }

    /** Minimal identity-only Holder stand-in (value never dereferenced by container/palette paths). */
    private static Holder<Biome> fakeBiomeHolder(final int id) {
        return new Holder<>() {
            public Biome value() { return null; }
            public boolean isBound() { return true; }
            public boolean is(net.minecraft.resources.ResourceLocation rl) { return false; }
            public boolean is(net.minecraft.resources.ResourceKey<Biome> k) { return false; }
            public boolean is(java.util.function.Predicate<net.minecraft.resources.ResourceKey<Biome>> p) { return false; }
            public boolean is(net.minecraft.tags.TagKey<Biome> t) { return false; }
            public boolean is(Holder<Biome> h) { return h == this; }
            public java.util.stream.Stream<net.minecraft.tags.TagKey<Biome>> tags() { return java.util.stream.Stream.empty(); }
            public com.mojang.datafixers.util.Either<net.minecraft.resources.ResourceKey<Biome>, Biome> unwrap() {
                return com.mojang.datafixers.util.Either.right(null);
            }
            public Optional<net.minecraft.resources.ResourceKey<Biome>> unwrapKey() { return Optional.empty(); }
            public Holder.Kind kind() { return Holder.Kind.DIRECT; }
            public boolean canSerializeIn(net.minecraft.core.HolderOwner<Biome> o) { return true; }
            public String getRegisteredName() { return "synthetic_" + id; }
        };
    }

    private static SecData[] buildSections(int n, int maxBlockId, List<Holder<Biome>> biomeChoices,
                                           IdMap<BlockState> blockReg, Strategy<BlockState> blockStrategy,
                                           Strategy<Holder<Biome>> biomeStrategy, Random r) {
        SecData[] out = new SecData[n];
        BlockState air = Blocks.AIR.defaultBlockState();
        for (int i = 0; i < n; i++) {
            PalettedContainer<BlockState> st = new PalettedContainer<>(air, blockStrategy);
            int distinct = switch (i % 6) {
                case 0 -> 0;        // all air (single)
                case 1 -> 1;        // full single non-air
                case 2 -> 2;        // 1-bit linear
                case 3 -> 16;       // 4-bit linear
                case 4 -> 64;       // 6-bit hashmap
                default -> 300;     // global
            };
            int nonEmpty = 0;
            if (distinct == 1) {
                BlockState stone = Blocks.STONE.defaultBlockState();
                for (int v = 0; v < SECTION_VOLUME; v++) {
                    st.set(v % 16, (v / 16) % 16, v / 256, stone);
                }
                nonEmpty = SECTION_VOLUME;
            } else if (distinct >= 2) {
                BlockState[] states = new BlockState[distinct];
                for (int k = 0; k < distinct; k++) {
                    int id = (i * 37 + k * 97) % Math.max(1, maxBlockId);
                    states[k] = blockReg.byId(id);
                    if (states[k] == null) states[k] = Blocks.STONE.defaultBlockState();
                }
                for (int v = 0; v < SECTION_VOLUME; v++) {
                    BlockState pick = states[r.nextInt(distinct)];
                    if (pick.isAir()) pick = Blocks.STONE.defaultBlockState();
                    st.set(v % 16, (v / 16) % 16, v / 256, pick);
                    nonEmpty++;
                }
            }
            PalettedContainer<Holder<Biome>> bi = new PalettedContainer<>(biomeChoices.get(0), biomeStrategy);
            int biomeDistinct = switch (i % 4) {
                case 0 -> 1;
                case 1 -> 3;
                case 2 -> 8;
                default -> 12;
            };
            if (biomeDistinct > 1) {
                for (int v = 0; v < 64; v++) {
                    Holder<Biome> pick = biomeChoices.get(r.nextInt(biomeDistinct));
                    bi.set(v % 4, (v / 4) % 4, v / 16, pick);
                }
            }
            out[i] = new SecData((short) nonEmpty, st, bi);
        }
        return out;
    }
}
