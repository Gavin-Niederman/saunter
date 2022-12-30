use saunter::error::SaunterError;
use saunter::math::{MathError, self};
use saunter::tick::Tick;
use saunter::tickloop::Loop;
use saunter::{listener::Listener, tick::Ticks};
use std::sync::Arc;
use std::time::Instant;
use std::{
    sync::{mpsc, RwLock},
    thread,
};

const TPS: u32 = 2;

fn main() {
    // Create a channel to send events to the tickloop
    let (event_sender, event_reciever) = mpsc::channel::<winit::event::Event<'_, ()>>();

    // Create a static(hence the leak) WrLock<Ticks> to store the last and new ticks for rendering.
    let ticks = Box::leak(Box::new(Arc::new(RwLock::new(Ticks::new(PrintTick::new(
        Instant::now(), 0.0,
    ))))));

    let mut tick_loop: Loop<'_, PrintTick> =
        saunter::tickloop::Loop::new(Box::new(PrinterListener {val: 1.0}), TPS, event_reciever);

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

        //FIXME: mapped_t is 1 for many frames before the next tick is received.
        if let Some(last) = &read_ticks.last_tick {
            let mapped_t = math::clamp((last.get_time().elapsed().as_secs_f32() * TPS as f32) - 1.0, 0.0, 1.0);
            println!("Mapped t: {}", mapped_t);
            if let Ok(lerped) = read_ticks.lerp(mapped_t) {
                let lerped = lerped;
                println!("lerped val: {}", lerped.val);
            }
        }

    })
}

struct PrinterListener {
    val: f32,
}
impl Listener for PrinterListener {
    fn tick(
        &mut self,
        _dt: f32,
        events: &mut Vec<winit::event::Event<'_, ()>>,
    ) -> Result<PrintTick, SaunterError> {
        self.val = 1.0 - self.val;

        for event in events {
            if let winit::event::Event::WindowEvent { event, .. } = event {
                // println!("Tick {:?}", event);
            }
        }

        Ok(PrintTick::new(Instant::now(), self.val))
    }

    type TickType = PrintTick;
}

#[derive(Clone, Debug)]
pub struct PrintTick {
    time: Instant,
    pub val: f32,
}
impl saunter::tick::Tick for PrintTick {
    fn lerp(&self, b: &Self, t: f32) -> Result<Self, MathError> {
        Ok(PrintTick {
            time: math::lerp_instant(&self.time, &b.time, t)?,
            val: math::lerp(self.val, b.val, t)?,
        })
    }

    fn get_time(&self) -> &Instant {
        &self.time
    }
}

impl PrintTick {
    pub fn new(time: Instant, val: f32) -> Self {
        PrintTick { time, val }
    }
}
