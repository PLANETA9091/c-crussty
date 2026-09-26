/** STW-fixture: генерирует ParallelGC-запись с управляемым числом Full/Young
 * пауз для офлайн-валидации StwCensusJfr vs gc.log (TASK-463-49 dry-run). */
public class GenStwFixture {
    public static void main(String[] args) {
        int fullN = Integer.parseInt(System.getProperty("full.n", "10"));
        long churnMs = Long.parseLong(System.getProperty("churn.ms", "4000"));
        Object[] sink = new Object[64];
        long end = System.nanoTime() + churnMs * 1_000_000L;
        int i = 0;
        while (System.nanoTime() < end) {
            sink[i++ & 63] = new byte[1 << 20];
        }
        for (int g = 0; g < fullN; g++) {
            byte[][] hold = new byte[300_000][];
            for (int k = 0; k < hold.length; k++) hold[k] = new byte[1024];
            sink[0] = hold;
            System.gc();
        }
        end = System.nanoTime() + churnMs * 1_000_000L;
        while (System.nanoTime() < end) {
            sink[i++ & 63] = new byte[1 << 20];
        }
        System.out.println("fixture done");
    }
}
