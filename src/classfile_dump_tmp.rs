//! TEMP dump harness for TASK-400-D verification repro (deleted before commit).
use crate::classfile::{
    retarget_ldcw_virtual_to_static, RetargetOutcome, WAKEUP_ANCHORS, WAKEUP_FROM_RUNNING,
    WAKEUP_FROM_TICK, WAKEUP_METHOD, WAKEUP_METHOD_DESC, WAKEUP_TO_RUNNING, WAKEUP_TO_TICK,
};

#[test]
fn dump_retargeted_mob_tmp() {
    let bytes = include_bytes!("../tests/fixtures/Mob.class");
    let mut cur: Vec<u8> = bytes.to_vec();
    for (from, to) in [
        (WAKEUP_FROM_TICK, WAKEUP_TO_TICK),
        (WAKEUP_FROM_TICK, WAKEUP_TO_TICK),
        (WAKEUP_FROM_RUNNING, WAKEUP_TO_RUNNING),
        (WAKEUP_FROM_RUNNING, WAKEUP_TO_RUNNING),
    ] {
        let (b, outcome) = retarget_ldcw_virtual_to_static(
            &cur,
            WAKEUP_METHOD,
            WAKEUP_METHOD_DESC,
            &WAKEUP_ANCHORS,
            from,
            to,
        )
        .unwrap();
        match outcome {
            RetargetOutcome::Retargeted { .. } => cur = b,
            _ => panic!("unexpected {:?}", outcome),
        }
    }
    std::fs::write("/tmp/Mob_patched.class", &cur).unwrap();
    std::fs::write("/tmp/Mob_orig.class", bytes).unwrap();
}
