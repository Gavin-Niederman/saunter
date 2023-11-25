mod tick;

use std::{
    sync::{Arc, RwLock},
    thread,
    time::Instant,
};

use saunter::{
    event::Event,
    tick::{Snapshot, Snapshots},
    tickloop::Loop,
};
use tick::NoWindowTick;

const TPS: f32 = 1.0;

fn main() {
    simplelog::TermLogger::init(
        log::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap_or(println!("Failed to initialize logger"));

    let mut val = 1.0;

    let (mut tick_loop, event_sender, ticks): (
        Loop<_, ()>,
        _,
        &'static mut Arc<RwLock<Snapshots<_>>>,
    ) = Loop::init(
        move |
            _dt,
            _events: Vec<saunter::event::Event<()>>,
            time,
        | {
            val = 1.0 - val;
            log::info!("ticked {}", val);
    
            Ok(NoWindowTick {
                val,
                time,
            })
        },
        NoWindowTick::new(Instant::now(), 0.0),
        TPS,
    );

    let tick_loop_tics = ticks.clone();
    thread::spawn(move || tick_loop.start(tick_loop_tics));

    loop {
        event_sender
            .send(Event::Other(()))
            .unwrap_or_else(|err| log::error!("{:?}", err));

        let read_ticks = ticks.read().unwrap();

        if let Some(last) = &read_ticks.last_tick {
            let mapped_t = ((last.get_time().elapsed().as_secs_f32() * TPS as f32) - 1.0).max(0.0); //subtract 1 to get the previous tick
            if let Ok(lerped) = read_ticks.lerp(mapped_t) {
                log::info!("{}", lerped.val);
            }
        }
    }
}
