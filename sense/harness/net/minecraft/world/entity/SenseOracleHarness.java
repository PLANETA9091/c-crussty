package net.minecraft.world.entity;

import java.util.Random;

/**
 * SENSE-PLANE локальный lockstep-оракул (TASK-438-A2): exhaustive + seeded
 * fuzz corePick-vs-naivePick на синтетике — тот же контракт, что selfTest()
 * внутри блоба, плюс крупный стохастический прогон с частыми связками.
 * НЕ компилируется build_430b_blobs.sh (dev-only), запуск:
 *   javac --release 21 -cp sense/build -d sense/harness sense/harness/net/minecraft/world/entity/SenseOracleHarness.java
 *   java -cp sense/build:sense/harness net.minecraft.world.entity.SenseOracleHarness
 */
public class SenseOracleHarness {

    public static void main(String[] args) {
        if (!SenseOps.decisionCoreOracle()) {
            System.out.println("ORACLE FAIL: exhaustive corePick != naivePick");
            System.exit(1);
        }
        Random rnd = new Random(424242);
        long ties = 0;
        for (int iter = 0; iter < 200_000; iter++) {
            int n = 1 + rnd.nextInt(64);
            double[] d = new double[n];
            boolean[] pass = new boolean[n];
            for (int i = 0; i < n; i++) {
                // 0..6 — частые связки (7 значений на 64 кандидатов), 7 -> 1.0
                double base = rnd.nextInt(8);
                d[i] = base == 7 ? 1.0 : base;
                pass[i] = rnd.nextBoolean();
            }
            int idxR = rnd.nextInt(n + 1) - 1; // -1 = R отсутствует
            boolean rPass = idxR >= 0 && pass[idxR];
            int acc = SenseOps.corePick(d, pass, idxR, rPass);
            int naive = SenseOps.naivePick(d, pass);
            if (acc != naive) {
                System.out.println("FUZZ MISMATCH iter=" + iter + " n=" + n + " idxR=" + idxR);
                System.exit(1);
            }
            // tie-покрытие: считать связки, попавшие в победную дистанцию
            if (acc >= 0) {
                for (int i = 0; i < n; i++) {
                    if (i != acc && pass[i] && d[i] == d[acc]) {
                        ties++;
                        break;
                    }
                }
            }
        }
        System.out.println("ORACLE OK exhaustive + 200k fuzz seed 424242, tie-hit cases=" + ties);
    }
}
