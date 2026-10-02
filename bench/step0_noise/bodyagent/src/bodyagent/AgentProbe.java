package bodyagent;

/** Dormancy canary for the java arm: the class exists on the classpath in
 *  BOTH runs (it rides in the same jar), so presence of the CLASS proves
 *  nothing — only a live agent attach would have run premain. This probe
 *  reports a system property the premain sets, instead of class presence. */
public final class AgentProbe {
    private AgentProbe() {}
    public static boolean agentLoaded() {
        return "1".equals(System.getProperty("crussty.bodyagent.armed", "0"));
    }
}
