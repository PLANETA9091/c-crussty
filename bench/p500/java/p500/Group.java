package p500;

/** P500 bench group contract (generated set). */
public interface Group {
    String fqcn();
    String sig();
    String[] methods();
    void setup(int strategy);
    long call(int idx);
}
