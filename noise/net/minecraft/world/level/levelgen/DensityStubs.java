// Compile-time stubs for the TASK-108 bridge (NormalNoiseBatchOps lives in
// this package, so package-private stub visibility is enough here).
// Shapes mirror the 1.21.10 mojang-mapped kernel classes (javap-pinned
// 2026-09-09: DensityFunctions$Noise record + ShiftNoise interface with
// default fillArray + NoiseHolder delegation). NOT shipped: discarded by
// scripts/build_noise.sh; the real classes resolve in the kernel loader.
package net.minecraft.world.level.levelgen;

import net.minecraft.core.Holder;
import net.minecraft.util.KeyDispatchDataCodec;
import net.minecraft.world.level.levelgen.synth.NormalNoise;

interface DensityFunction {
    double compute(FunctionContext f);
    void fillArray(double[] o, ContextProvider c);
    DensityFunction mapAll(Visitor v);
    double minValue();
    double maxValue();
    KeyDispatchDataCodec<DensityFunction> codec();

    interface FunctionContext {
        int blockX();
        int blockY();
        int blockZ();
    }

    interface ContextProvider {
        FunctionContext forIndex(int i);
        void fillAllDirectly(double[] o, DensityFunction f);
    }

    interface Visitor {
    }

    class NoiseHolder {
        public NoiseHolder(Holder h) {
        }
        public NoiseHolder(Holder h, NormalNoise n) {
        }
        public NormalNoise noise() {
            return null;
        }
    }
}

// TASK-108 v3 compile-time shapes (javap-pinned 1.21.10): the interpreter
// instanceof-checks these types; real classes resolve in the kernel loader.

class NoiseChunk {
    boolean fillingCell;

    static final class NoiseInterpolator implements DensityFunction {
        public DensityFunction wrapped() {
            return null;
        }
        public double compute(DensityFunction.FunctionContext f) {
            return 0.0D;
        }
        public void fillArray(double[] o, DensityFunction.ContextProvider c) {
        }
        public DensityFunction mapAll(DensityFunction.Visitor v) {
            return null;
        }
        public double minValue() {
            return 0.0D;
        }
        public double maxValue() {
            return 0.0D;
        }
        public KeyDispatchDataCodec<DensityFunction> codec() {
            return null;
        }
    }
}

final class DensityFunctions {
    private DensityFunctions() {
    }

    static final class Noise implements DensityFunction {
        Noise(NoiseHolder noise, double xzScale, double yScale) {
        }
        public NoiseHolder noise() {
            return null;
        }
        public double xzScale() {
            return 0.0D;
        }
        public double yScale() {
            return 0.0D;
        }
        public double compute(FunctionContext f) {
            return 0.0D;
        }
        public void fillArray(double[] o, ContextProvider c) {
        }
        public DensityFunction mapAll(Visitor v) {
            return null;
        }
        public double minValue() {
            return 0.0D;
        }
        public double maxValue() {
            return 0.0D;
        }
        public KeyDispatchDataCodec<DensityFunction> codec() {
            return null;
        }
    }

    interface ShiftNoise extends DensityFunction {
        NoiseHolder offsetNoise();
    }

    static final class ShiftA implements ShiftNoise {
        ShiftA(NoiseHolder offsetNoise) {
        }
        public NoiseHolder offsetNoise() {
            return null;
        }
        public double compute(FunctionContext f) {
            return 0.0D;
        }
        public void fillArray(double[] o, ContextProvider c) {
        }
        public DensityFunction mapAll(Visitor v) {
            return null;
        }
        public double minValue() {
            return 0.0D;
        }
        public double maxValue() {
            return 0.0D;
        }
        public KeyDispatchDataCodec<DensityFunction> codec() {
            return null;
        }
    }

    static final class ShiftB implements ShiftNoise {
        ShiftB(NoiseHolder offsetNoise) {
        }
        public NoiseHolder offsetNoise() {
            return null;
        }
        public double compute(FunctionContext f) {
            return 0.0D;
        }
        public void fillArray(double[] o, ContextProvider c) {
        }
        public DensityFunction mapAll(Visitor v) {
            return null;
        }
        public double minValue() {
            return 0.0D;
        }
        public double maxValue() {
            return 0.0D;
        }
        public KeyDispatchDataCodec<DensityFunction> codec() {
            return null;
        }
    }

    // ---- v3 interpreter shapes (javap-pinned 1.21.10) ----

    interface PureTransformer extends DensityFunction {
        DensityFunction input();
        double transform(double v);
    }

    static final class Constant implements DensityFunction {
        public double value() {
            return 0.0D;
        }
        public double compute(FunctionContext f) {
            return 0.0D;
        }
        public void fillArray(double[] o, ContextProvider c) {
        }
        public DensityFunction mapAll(Visitor v) {
            return null;
        }
        public double minValue() {
            return 0.0D;
        }
        public double maxValue() {
            return 0.0D;
        }
        public KeyDispatchDataCodec<DensityFunction> codec() {
            return null;
        }
    }

    static final class YClampedGradient implements DensityFunction {
        public int fromY() {
            return 0;
        }
        public int toY() {
            return 0;
        }
        public double fromValue() {
            return 0.0D;
        }
        public double toValue() {
            return 0.0D;
        }
        public double compute(FunctionContext f) {
            return 0.0D;
        }
        public void fillArray(double[] o, ContextProvider c) {
        }
        public DensityFunction mapAll(Visitor v) {
            return null;
        }
        public double minValue() {
            return 0.0D;
        }
        public double maxValue() {
            return 0.0D;
        }
        public KeyDispatchDataCodec<DensityFunction> codec() {
            return null;
        }
    }

    interface TwoArgumentSimpleFunction extends DensityFunction {
        enum Type {
            ADD, MUL, MIN, MAX
        }

        Type type();

        DensityFunction argument1();

        DensityFunction argument2();
    }

    static final class Ap2 implements TwoArgumentSimpleFunction {
        public Type type() {
            return Type.ADD;
        }
        public DensityFunction argument1() {
            return null;
        }
        public DensityFunction argument2() {
            return null;
        }
        public double compute(FunctionContext f) {
            return 0.0D;
        }
        public void fillArray(double[] o, ContextProvider c) {
        }
        public DensityFunction mapAll(Visitor v) {
            return null;
        }
        public double minValue() {
            return 0.0D;
        }
        public double maxValue() {
            return 0.0D;
        }
        public KeyDispatchDataCodec<DensityFunction> codec() {
            return null;
        }
    }

    static final class HolderHolder implements DensityFunction {
        public Holder<DensityFunction> function() {
            return null;
        }
        public double compute(FunctionContext f) {
            return 0.0D;
        }
        public void fillArray(double[] o, ContextProvider c) {
        }
        public DensityFunction mapAll(Visitor v) {
            return null;
        }
        public double minValue() {
            return 0.0D;
        }
        public double maxValue() {
            return 0.0D;
        }
        public KeyDispatchDataCodec<DensityFunction> codec() {
            return null;
        }
    }
}
