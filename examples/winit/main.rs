mod listener;
mod tick;
use listener::WinitListener;
use saunter::event::Event;
use tick::WinitTick;

use saunter::tick::{Snapshot, Snapshots};
use saunter::tickloop::Loop;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Instant;

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
        Loop<_, winit::event::Event<'_, ()>>,
        _,
        &'static mut Arc<RwLock<Snapshots<_>>>,
    ) = Loop::init(
        Box::new(WinitListener { val: 1.0 }),
        WinitTick::new(Instant::now(), 0.0),
        TPS,
    );

    let tick_loop_tics = ticks.clone();
    thread::spawn(move || tick_loop.start(tick_loop_tics));

    let event_loop = winit::event_loop::EventLoop::new();
    let _window = winit::window::WindowBuilder::new()
        .with_min_inner_size(winit::dpi::LogicalSize::new(10, 10))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        if let winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::CloseRequested,
            ..
        } = event
        {
            control_flow.set_exit();
        }

        event_sender
            .send(Event::Other(event.to_static().unwrap_or(
                winit::event::Event::NewEvents(winit::event::StartCause::Poll),
            )))
            .unwrap_or_else(|err| log::error!("{:?}", err));

        let read_ticks = ticks.read().unwrap();

        if let Some(last) = &read_ticks.last_tick {
            let mapped_t = ((last.get_time().elapsed().as_secs_f32() * TPS as f32) - 1.0).max(0.0); //subtract 1 to get the previous tick
            
            if let Ok(lerped) = read_ticks.lerp(mapped_t) {
                let _lerped = lerped;
            }
        }
    })
}
