use saunter::derive::Interpolate;
use saunter::snapshot::{Snapshot, Snapshots};
use std::time::Instant;

#[derive(Copy, Clone, Debug, Interpolate)]
struct TestTick {
    tick: u8,
}

impl Snapshot for TestTick {
    fn get_time(&self) -> &Instant {
        unimplemented!()
    }
}

#[test]
fn test_ticks_update() {
    let mut ticks = Snapshots {
        last_snapshot: None,
        new_snapshot: TestTick { tick: 0 },
    };

    assert!(ticks.last_snapshot.is_none());
    assert_eq!(ticks.new_snapshot.tick, 0);

    ticks.update(TestTick { tick: 1 });
    assert!(ticks.last_snapshot.is_some());
    assert_eq!(ticks.last_snapshot.unwrap().tick, 0);
    assert_eq!(ticks.new_snapshot.tick, 1);

    ticks.update(TestTick { tick: 2 });
    assert_eq!(ticks.last_snapshot.unwrap().tick, 1);
    assert_eq!(ticks.new_snapshot.tick, 2);

    ticks.update(TestTick { tick: 3 });
    assert_eq!(ticks.last_snapshot.unwrap().tick, 2);
    assert_eq!(ticks.new_snapshot.tick, 3);
}
