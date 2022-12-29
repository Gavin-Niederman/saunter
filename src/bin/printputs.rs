use saunter::error::SaunterError;
use saunter::math::MathError;
use saunter::tickloop::Loop;
use saunter::{listener::Listener, tick::Ticks};
use std::sync::Arc;
use std::{
    sync::{mpsc, RwLock},
    thread,
};

fn main() {
    // Create a channel to send events to the tickloop
    let (event_sender, event_reciever) = mpsc::channel::<winit::event::Event<'_, ()>>();
    let ticks = Box::leak(Box::new(Arc::new(RwLock::new(Ticks::new(PrintTick {})))));

    // Create a WrLock<Ticks> to store the last and new ticks for rendering.

    let mut tick_loop: Loop<'_, PrintTick> =
        saunter::tickloop::Loop::new(Box::new(PrinterListener {}), 2, event_reciever);

    let tick_loop_tics = ticks.clone();
    thread::spawn(move || tick_loop.start(tick_loop_tics));

    let event_loop = winit::event_loop::EventLoop::new();
    let _window = winit::window::WindowBuilder::new()
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
            .unwrap_or_else(|err| eprintln!("{:?}", err));
            
        let read_ticks = ticks.read().unwrap();
        let _lerped_tick = read_ticks.lerp(0.5).unwrap_or_default();
    })
}

struct PrinterListener {}
impl Listener for PrinterListener {
    fn tick(&mut self, _dt: f32, events: &mut Vec<winit::event::Event<'_, ()>>) -> Result<PrintTick, SaunterError> {
        for event in events {
            if let winit::event::Event::WindowEvent { event, .. } = event {
                println!("Tick {:?}", event);
            }
        }
        
        Ok(PrintTick {})
    }

    type TickType = PrintTick;
}

#[derive(Clone, Default)]
pub struct PrintTick {}
impl saunter::tick::Tick for PrintTick {
    fn lerp(&self, b: &Self, t: f32) -> Result<Self, MathError> {
        Ok(PrintTick {})
    }
}
