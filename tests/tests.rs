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
        last_tick: None,
        new_tick: TestTick { tick: 0 },
    };

    assert!(ticks.last_tick.is_none());
    assert_eq!(ticks.new_tick.tick, 0);

    ticks.update(TestTick { tick: 1 });
    assert!(ticks.last_tick.is_some());
    assert_eq!(ticks.last_tick.unwrap().tick, 0);
    assert_eq!(ticks.new_tick.tick, 1);

    ticks.update(TestTick { tick: 2 });
    assert_eq!(ticks.last_tick.unwrap().tick, 1);
    assert_eq!(ticks.new_tick.tick, 2);

    ticks.update(TestTick { tick: 3 });
    assert_eq!(ticks.last_tick.unwrap().tick, 2);
    assert_eq!(ticks.new_tick.tick, 3);
}
