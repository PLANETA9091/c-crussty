import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;
import java.util.zip.ZipEntry;
import java.util.zip.ZipOutputStream;

import com.sun.tools.attach.VirtualMachine;

import javax.tools.JavaCompiler;
import javax.tools.StandardJavaFileManager;
import javax.tools.ToolProvider;

/**
 * TASK-149 phase-2a attach runner — launched in SOURCE MODE (java Task149Attach.java ...)
 * so no javac binary is needed; the launcher's jdk.compiler compiles this file in-memory,
 * then ToolProvider compiles the shadow agent to classes, we zip an agent jar, attach to
 * the live server and loadAgent it. Bench-lane tooling only — the server boot itself
 * stays pure-inject (stock JDK + ONLY -agentpath).
 *
 * args: <pid> <ChunkEncodeSectionShadow.java path> <out tsv path>
 * prints the resulting TSV to stdout after a successful loadAgent.
 */
public final class Task149Attach {

    public static void main(String[] args) throws Exception {
        if (args.length < 3) {
            System.err.println("usage: Task149Attach <pid> <shadow .java> <out tsv>");
            System.exit(2);
        }
        String pid = args[0];
        Path src = Paths.get(args[1]);
        Path tsv = Paths.get(args[2]).toAbsolutePath();
        Path work = Files.createTempDirectory("task149attach");

        // 1. compile shadow agent -> work/classes
        JavaCompiler jc = ToolProvider.getSystemJavaCompiler();
        if (jc == null) { System.err.println("FATAL: no system java compiler (jdk.compiler absent)"); System.exit(3); }
        Path classes = work.resolve("classes");
        Files.createDirectories(classes);
        try (StandardJavaFileManager fm = jc.getStandardFileManager(null, null, StandardCharsets.UTF_8)) {
            var unit = fm.getJavaFileObjects(src);
            List<String> opts = Arrays.asList("-d", classes.toString(), "-nowarn");
            JavaCompiler.CompilationTask task = jc.getTask(null, fm, null, opts, null, unit);
            if (!task.call()) { System.err.println("FATAL: shadow compile failed"); System.exit(3); }
        }
        Path cls = classes.resolve("ChunkEncodeSectionShadow.class");
        if (!Files.isReadable(cls)) { System.err.println("FATAL: class not produced"); System.exit(3); }

        // 2. agent jar (manifest first entry, as required)
        Path jar = work.resolve("task149-shadow-agent.jar");
        String manifest = "Manifest-Version: 1.0\r\n"
                + "Agent-Class: ChunkEncodeSectionShadow\r\n"
                + "Can-Redefine-Classes: false\r\n"
                + "Can-Retransform-Classes: false\r\n"
                + "\r\n";
        try (ZipOutputStream z = new ZipOutputStream(Files.newOutputStream(jar))) {
            z.putNextEntry(new ZipEntry("META-INF/MANIFEST.MF"));
            z.write(manifest.getBytes(StandardCharsets.UTF_8));
            z.closeEntry();
            z.putNextEntry(new ZipEntry("ChunkEncodeSectionShadow.class"));
            z.write(Files.readAllBytes(cls));
            z.closeEntry();
        }

        // 3. attach + loadAgent (blocks until agentmain returns)
        Files.deleteIfExists(tsv);
        VirtualMachine vm = VirtualMachine.attach(pid);
        try {
            vm.loadAgent(jar.toString(), "out=" + tsv);
        } finally {
            vm.detach();
        }

        // 4. print TSV (rig greps SHADOW rows)
        if (!Files.isReadable(tsv)) { System.err.println("FATAL: agent produced no TSV at " + tsv); System.exit(4); }
        for (String line : Files.readAllLines(tsv, StandardCharsets.UTF_8)) System.out.println(line);
        System.out.println("ATTACH_OK\ttsv=" + tsv);
    }
}
