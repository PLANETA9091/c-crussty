package ca.spottedleaf.moonrise.common.misc;

/** Recording SingleUserAreaMap: collects (op,x,z) triples in callback order. */
final class RecAreaMap extends SingleUserAreaMap<String> {
    final java.util.List<String> log = new java.util.ArrayList<String>();
    @Override protected void addCallback(String param, int chunkX, int chunkZ) {
        log.add("A:" + chunkX + ":" + chunkZ);
    }
    @Override protected void removeCallback(String param, int chunkX, int chunkZ) {
        log.add("R:" + chunkX + ":" + chunkZ);
    }
}
