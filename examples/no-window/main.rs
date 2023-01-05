mod listener;
mod tick;

use std::{sync::{Arc, RwLock}, time::Instant, thread};

use listener::NoWindowListener;
use saunter::{tickloop::Loop, tick::{Ticks, Tick}, event::Event, math};
use tick::NoWindowTick;

const TPS: f32 = 66.0;

fn main() {
    simplelog::TermLogger::init(
        log::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap_or(println!("Failed to initialize logger"));
    
    let (mut tick_loop, event_sender, ticks): (
        Loop<_, ()>,
        _,
        &'static mut Arc<RwLock<Ticks<_>>>,
    ) = Loop::init(
        Box::new(NoWindowListener { val: 1.0 }),
        NoWindowTick::new(Instant::now(), 0.0),
        TPS,
    );
    
    let tick_loop_tics = ticks.clone();
    thread::spawn(move || tick_loop.start(tick_loop_tics));

    loop {
        event_sender.send(Event::Other(())).unwrap_or_else(|err| log::error!("{:?}", err));

        let read_ticks = ticks.read().unwrap();

        if let Some(last) = &read_ticks.last_tick {
            let mapped_t = math::max(
                (last.get_time().elapsed().as_secs_f32() * TPS as f32) - 1.0, //subtract 1 to get the previous tick
                0.0,
            );
            if let Ok(lerped) = read_ticks.lerp(mapped_t) {
                log::info!("{}", lerped.val);
            }
        }
    }
}