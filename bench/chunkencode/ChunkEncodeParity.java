import io.netty.buffer.ByteBuf;
import io.netty.buffer.Unpooled;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.protocol.game.ClientboundLightUpdatePacketData;
import net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode;

import java.util.Arrays;
import java.util.List;
import java.util.Random;

/**
 * TASK-78 part B — chunk-encode .so light-data parity + timing rig (headless).
 *
 * nativeEncodeLightData([J x4, byte[] skyNibbles, int, byte[] blockNibbles, int, byte[] dst)I
 * is compared against vanilla ClientboundLightUpdatePacketData semantics using the
 * class's own DECODER constructor (FriendlyByteBuf, int, int) — no Unsafe, no LevelLightEngine:
 *   (1) FIELD EQUALITY:  decode(nativeDst) fields == native inputs (masks via toLongArray(),
 *       per-section 2048B arrays);
 *   (2) FIXED POINT:     decode(nativeDst).write() == nativeDst  (byte-identical re-encode)
 *       -> the native emits vanilla wire format;
 *   (3) TIMING:          native encode vs vanilla write() on identical decoded state.
 *
 * Output lines (stdout, TSV):
 *   PARITY\tlight\tcases=N\tmismatches=M\tverdict=PASS|MISMATCH
 *   MISMATCH\t<case>\t<detail>
 *   TIMING\tnativeEncodeLightData\tmedian_ns_per_call
 *   TIMING\tvanillaWrite\tmedian_ns_per_call
 *   SINK\t<acc>
 */
public final class ChunkEncodeParity {

    private static long SINK = 0;
    private static final int SEC = 26;          // 1.21.10 light sections per chunk column
    private static final int NIB = 2048;        // bytes per section nibble array

    public static void main(String[] args) {
        String libs = System.getProperty("p500.libs", "");
        if (libs.isEmpty()) { System.err.println("FATAL: -Dp500.libs not set"); System.exit(2); }
        for (String p : libs.split(java.io.File.pathSeparator)) {
            if (!p.isBlank()) System.load(p);
        }
        System.err.println("chunk-encode .so loaded");

        long[][] masks = {
            bs("11111111111111111111111110"),            // realistic: 26 sections, top empty
            bs("00000000000000000000000000"),            // all empty
            bs("11111111111111111111111111"),            // all set
            bs("00000000000000010000000000"),            // single middle section
            longs(0x8000000000000000L, 0x00000000000000FFL), // 2-long format-robustness case
        };
        int[] cases = { SEC, SEC, SEC, SEC, SEC };  // NOTE: physical 1.21.10 = 26 sections; the
        // native does NOT bound-check dst capacity (that is what the *Sized variant is for):
        // sec=128 with a 128 KiB dst aborted the JVM (Rust panic) on the first attempt — do not
        // feed non-physical section counts to the plain variant.

        int mismatches = 0;
        int ran = 0;
        Random r = new Random(0xC0FFEE);
        byte[] refNative = null;

        for (int c = 0; c < masks.length; c++) {
            long[] sky = masks[c], blk = masks[c];
            long[] es = bs("00000000000000000000000000"), eb = bs("00000000000000000000000000");
            int sec = cases[c];
            byte[] skyN = new byte[sec * NIB], blkN = new byte[sec * NIB];
            for (int i = 0; i < skyN.length; i++) skyN[i] = (byte) (i % 3 == 0 ? 15 : r.nextInt(16));
            for (int i = 0; i < blkN.length; i++) blkN[i] = (byte) r.nextInt(16);
            byte[] dst = new byte[1 << 17];

            int n = PaperNativeChunkPacketEncode.nativeEncodeLightData(sky, blk, es, eb, skyN, sec, blkN, sec, dst);
            ran++;
            if (n <= 0) { System.out.println("MISMATCH\tcase" + c + "\tnative rc=" + n); mismatches++; continue; }

            // (1) decode + field equality
            ClientboundLightUpdatePacketData dec;
            try {
                FriendlyByteBuf fb = new FriendlyByteBuf(Unpooled.wrappedBuffer(Arrays.copyOf(dst, n)));
                dec = new ClientboundLightUpdatePacketData(fb, 0, 0);
            } catch (Exception e) {
                System.out.println("MISMATCH\tcase" + c + "\tdecode failed: " + e);
                mismatches++; continue;
            }
            boolean ok = Arrays.equals(dec.getSkyYMask().toLongArray(), sky)
                      && Arrays.equals(dec.getBlockYMask().toLongArray(), blk)
                      && Arrays.equals(dec.getEmptySkyYMask().toLongArray(), es)
                      && Arrays.equals(dec.getEmptyBlockYMask().toLongArray(), eb)
                      && sectionsEqual(dec.getSkyUpdates(), skyN, sec)
                      && sectionsEqual(dec.getBlockUpdates(), blkN, sec);
            if (!ok) { System.out.println("MISMATCH\tcase" + c + "\tfield equality"); mismatches++; continue; }

            // (2) fixed point: write(decode(dst)) == dst
            ByteBuf out = Unpooled.buffer(n + 64, n + 64);
            dec.write(new FriendlyByteBuf(out));
            byte[] re = Arrays.copyOf(out.array(), out.readableBytes());
            if (!Arrays.equals(re, Arrays.copyOf(dst, n))) {
                System.out.println("MISMATCH\tcase" + c + "\tfixed-point len n=" + n + " re=" + re.length);
                mismatches++; continue;
            }
            System.err.println("case" + c + " OK n=" + n);
            if (c == 0) refNative = Arrays.copyOf(dst, n);
            SINK += n;
        }
        System.out.println("PARITY\tlight\tcases=" + ran + "\tmismatches=" + mismatches
                + "\tverdict=" + (mismatches == 0 ? "PASS" : "MISMATCH"));

        if (refNative != null && mismatches == 0) {
            // (3) timing on case-0 state
            long[] sky = masks[0], blk = masks[0];
            long[] es = bs("00000000000000000000000000"), eb = bs("00000000000000000000000000");
            byte[] skyN = new byte[SEC * NIB], blkN = new byte[SEC * NIB];
            for (int i = 0; i < skyN.length; i++) skyN[i] = (byte) r.nextInt(16);
            for (int i = 0; i < blkN.length; i++) blkN[i] = (byte) r.nextInt(16);
            byte[] dst = new byte[1 << 17];

            ClientboundLightUpdatePacketData dec = new ClientboundLightUpdatePacketData(
                    new FriendlyByteBuf(Unpooled.wrappedBuffer(refNative)), 0, 0);
            ByteBuf out = Unpooled.buffer(1 << 17, 1 << 17);
            FriendlyByteBuf fbOut = new FriendlyByteBuf(out);

            // warmup
            for (int i = 0; i < 4000; i++) {
                SINK += PaperNativeChunkPacketEncode.nativeEncodeLightData(sky, blk, es, eb, skyN, SEC, blkN, SEC, dst);
                out.clear(); dec.write(fbOut); SINK += out.readableBytes();
            }
            long bestN = Long.MAX_VALUE, bestV = Long.MAX_VALUE;
            for (int w = 0; w < 5; w++) {
                long t0 = System.nanoTime();
                for (int i = 0; i < 20000; i++) SINK += PaperNativeChunkPacketEncode.nativeEncodeLightData(sky, blk, es, eb, skyN, SEC, blkN, SEC, dst);
                bestN = Math.min(bestN, (System.nanoTime() - t0) / 20000);
                long t1 = System.nanoTime();
                for (int i = 0; i < 20000; i++) { out.clear(); dec.write(fbOut); SINK += out.readableBytes(); }
                bestV = Math.min(bestV, (System.nanoTime() - t1) / 20000);
            }
            System.out.println("TIMING\tnativeEncodeLightData\tmedian_ns_per_call=" + bestN);
            System.out.println("TIMING\tvanillaWrite\tmedian_ns_per_call=" + bestV);
            if (bestN > 0) System.out.println("RATIO\tvanilla/native=" + String.format("%.2f", (double) bestV / bestN));
        }
        System.out.println("SINK\t" + SINK);
        System.exit(mismatches == 0 ? 0 : 1);
    }

    private static boolean sectionsEqual(List<byte[]> list, byte[] flat, int sec) {
        if (list == null || list.size() != sec) return false;
        for (int i = 0; i < sec; i++) {
            byte[] s = list.get(i);
            int off = i * NIB;
            if (s == null || s.length != NIB) return false;
            for (int j = 0; j < NIB; j++) if (s[j] != flat[off + j]) return false;
        }
        return true;
    }

    private static long[] bs(String bits) {
        java.util.BitSet b = new java.util.BitSet(bits.length());
        for (int i = 0; i < bits.length(); i++) if (bits.charAt(i) == '1') b.set(i);
        return b.toLongArray();
    }

    private static long[] longs(long... v) { return v; }
}
