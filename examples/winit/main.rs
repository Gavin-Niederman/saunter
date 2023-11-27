mod tick;
use tick::WinitTick;

use saunter::snapshot::Snapshot;
use saunter::tickloop::TickLoop;
use std::thread;
use std::time::Instant;
use winit::event::Event;

const TPS: f32 = 66.0;

fn main() {
    simplelog::TermLogger::init(
        log::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap_or(println!("Failed to initialize logger"));

    let mut val = 1.0;

    let (mut tick_loop, _, mut ctrl, snapshots) = TickLoop::init(
        move |_dt, events: Vec<Event<()>>, _, time| {
            val = 1.0 - val;

            for event in events {
                if let winit::event::Event::WindowEvent { event, .. } = event {
                    log::info!("Tick {:?}", event);
                }
            }

            Ok(WinitTick::new(time, val))
        },
        WinitTick::new(Instant::now(), 0.0),
        TPS,
    );

    thread::spawn(move || tick_loop.start());

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let _window = winit::window::WindowBuilder::new()
        .with_min_inner_size(winit::dpi::LogicalSize::new(10, 10))
        .build(&event_loop)
        .unwrap();

    event_loop
        .run(move |event, elwt| {
            match event {
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => elwt.exit(),
                winit::event::Event::AboutToWait => {
                    elwt.set_control_flow(winit::event_loop::ControlFlow::Poll)
                }
                _ => {}
            }

            ctrl.stop();

            let read_ticks = snapshots.read().unwrap();

            if let Some(last) = &read_ticks.last_snapshot {
                let mapped_t =
                    ((last.get_time().elapsed().as_secs_f32() * TPS as f32) - 1.0).max(0.0); //subtract 1 to get the previous tick

                if let Ok(lerped) =
                    read_ticks.interpolate_ticks(mapped_t, saunter::interpolate::linear)
                {
                    let _lerped = lerped;
                }
            }
        })
        .unwrap();
}
