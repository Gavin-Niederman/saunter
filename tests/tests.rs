use saunter::tick::{Snapshot, Snapshots};
use std::time::Instant;

#[derive(Copy, Clone, Debug)]
struct TestTick {
    tick: u8,
}

impl Snapshot for TestTick {
    fn lerp(_a: &Self, _b: &Self, _t: f32) -> Self {
        unimplemented!()
    }

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
