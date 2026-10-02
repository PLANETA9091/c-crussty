package areamapsmoke;

import ca.spottedleaf.moonrise.common.misc.SingleUserAreaMap;

import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.List;

/**
 * Concrete {@link SingleUserAreaMap} that records every add/remove callback
 * the patched {@code update()} body makes through
 * {@code SingleUserAreaMapOps.run()}. One instance per check/rect; never
 * shared between threads in the harness.
 */
public final class RecordingMap extends SingleUserAreaMap<Object> {

    /** One applied callback: add (op 0) or remove (op != 0) at a chunk cell. */
    public record Callback(boolean add, int x, int z) {}

    public final List<Callback> callbacks = new ArrayList<>();
    public final Object token;

    public RecordingMap(Object token) {
        super(token);
        this.token = token;
    }

    @Override
    protected void addCallback(Object param, int chunkX, int chunkZ) {
        assertParam(param, "addCallback");
        callbacks.add(new Callback(true, chunkX, chunkZ));
    }

    @Override
    protected void removeCallback(Object param, int chunkX, int chunkZ) {
        assertParam(param, "removeCallback");
        callbacks.add(new Callback(false, chunkX, chunkZ));
    }

    private void assertParam(Object param, String who) {
        if (param != token) {
            throw new AssertionError(who + ": param identity != this.parameter (patched body must pass the map's own parameter field)");
        }
    }

    /**
     * Seeds the private position fields so the patched update() behaves as if
     * the map had already been tracking (fx, fz) at distance od (the real map
     * reaches this state after its first successful tracking cycle).
     */
    public void initState(int fx, int fz, int od) {
        try {
            setInt("lastChunkX", fx);
            setInt("lastChunkZ", fz);
            setInt("distance", od);
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException("initState failed", e);
        }
    }

    private void setInt(String name, int v) throws ReflectiveOperationException {
        Field f = SingleUserAreaMap.class.getDeclaredField(name);
        f.setAccessible(true);
        f.setInt(this, v);
    }
}
