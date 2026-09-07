package areamapsmoke;

import java.util.HashSet;
import java.util.Set;

/**
 * Java mirror of {@code src/area_map.rs::naive_set_difference} -- the
 * reference semantics every produced (op, x, z) triple is compared against:
 * adds = new square \ old square, removes = old square \ new square.
 * Cells are compared as packed bridge keys (x low 32 bits, z high 32 bits).
 */
final class NaiveDiff {
    private NaiveDiff() {}

    static long key(int x, int z) {
        return ((long) z << 32) | (x & 0xFFFFFFFFL);
    }

    static void diff(int fx, int fz, int od, int tx, int tz, int nd,
                     Set<Long> adds, Set<Long> removes) {
        adds.clear();
        removes.clear();
        for (int x = tx - nd; x <= tx + nd; x++) {
            for (int z = tz - nd; z <= tz + nd; z++) {
                if (x < fx - od || x > fx + od || z < fz - od || z > fz + od) {
                    adds.add(key(x, z));
                }
            }
        }
        for (int x = fx - od; x <= fx + od; x++) {
            for (int z = fz - od; z <= fz + od; z++) {
                if (x < tx - nd || x > tx + nd || z < tz - nd || z > tz + nd) {
                    removes.add(key(x, z));
                }
            }
        }
    }
}
