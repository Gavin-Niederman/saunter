mod snapshot;

use std::{thread, time::Instant};

use saunter::{snapshot::Snapshot, tickloop::TickLoop};
use snapshot::NoWindowSnapshot;

const TPS: f32 = 30.0;

fn main() {
    simplelog::TermLogger::init(
        log::LevelFilter::Debug,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap_or(println!("Failed to initialize logger"));

    let mut val = 1.0;

    let (mut tick_loop, event_sender, _, ticks) = TickLoop::init(
        move |_dt, _events, _ctrl, time| {
            val = 1.0 - val;
            log::info!("ticked {}", val);

            Ok(NoWindowSnapshot { val, time })
        },
        TPS,
    );

    thread::spawn(move || tick_loop.start());

    loop {
        event_sender
            .send(())
            .unwrap_or_else(|err| log::error!("{:?}", err));

        let read_ticks = ticks.read().unwrap();

        if let Some(last) = &read_ticks.last_snapshot {
            let mapped_t = ((last.get_time().elapsed().as_secs_f32() * TPS as f32) - 1.0).max(0.0); //subtract 1 to get the previous tick
            if let Ok(lerped) = read_ticks.interpolate_ticks(mapped_t, saunter::interpolate::linear)
            {
                log::info!("{}", lerped.val);
            }
        }
    }
}
