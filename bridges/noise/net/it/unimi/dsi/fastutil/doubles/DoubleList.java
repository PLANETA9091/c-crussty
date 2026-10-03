// Compile-time stub for the fastutil DoubleList shape used by
// PerlinNoiseNativeOps (amplitudes extraction). NOT shipped: the kernel
// provides the real it.unimi.dsi.fastutil.doubles.DoubleList at runtime.
// Only the two members the bridge calls are declared; invokeinterface
// resolves them against the real (inherited) implementation at runtime.
package it.unimi.dsi.fastutil.doubles;

public interface DoubleList {
    double getDouble(int index);
    double[] toDoubleArray();
}
