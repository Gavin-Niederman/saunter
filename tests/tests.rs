use std::time::Instant;
use saunter::{math::MathError, tick::{Ticks, Tick}};

#[derive(Copy, Clone, Debug)]
struct TestTick {
    tick: u8,
}

impl Tick for TestTick {
    fn lerp(_a: &Self, _b: &Self, _t: f32) -> Result<Self, MathError> {
        unimplemented!()
    }

    fn get_time(&self) -> &Instant {
        unimplemented!()
    }
}

#[test]
fn test_ticks_update() {
    let mut ticks = Ticks {
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
