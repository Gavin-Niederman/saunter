use saunter::math;
use saunter::tick::Tick;
use saunter::tick::Ticks;
use saunter::tickloop::Loop;
use std::sync::Arc;
use std::time::Instant;
use std::{
    sync::{mpsc, RwLock},
    thread,
};

mod listener;
mod tick;
use listener::PrinterListener;
use tick::PrintTick;

const TPS: u32 = 2;

fn main() {
    simplelog::TermLogger::init(
        log::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    ).unwrap_or(println!("Failed to initialize logger"));

    log::info!("Starting print-inputs example...");

    // Create a channel to send events to the tickloop
    let (event_sender, event_reciever) = mpsc::channel::<winit::event::Event<'_, ()>>();

    // Create a static(hence the leak) WrLock<Ticks> to store the last and new ticks for rendering.
    let ticks = Box::leak(Box::new(Arc::new(RwLock::new(Ticks::new(PrintTick::new(
        Instant::now(),
        0.0,
    ))))));

    let mut tick_loop: Loop<'_, PrintTick> =
        saunter::tickloop::Loop::new(Box::new(PrinterListener { val: 1.0 }), TPS, event_reciever);

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
            .send(event.to_static().unwrap_or(winit::event::Event::NewEvents(
                winit::event::StartCause::Poll,
            )))
            .unwrap_or_else(|err| log::error!("{:?}", err));

        let read_ticks = ticks.read().unwrap();

        if let Some(last) = &read_ticks.last_tick {
            let mapped_t = math::max(
                (last.get_time().elapsed().as_secs_f32() * TPS as f32) - 1.0,
                0.0,
            );
            if let Ok(lerped) = read_ticks.lerp(mapped_t) {
                let _lerped = lerped;
            }
        }
    })
}
