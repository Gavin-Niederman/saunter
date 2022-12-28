use saunter::listener::Listener;
use std::{sync::mpsc, thread};

fn main() {
    let (t, x) = mpsc::channel::<winit::event::Event<'_, ()>>();
    let mut loop_ = saunter::tickloop::Loop::new(Box::new(PrinterListener {}), 2, x);
    thread::spawn(move || loop_.start());

    let event_loop = winit::event_loop::EventLoop::new();
    let _window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        if let winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::CloseRequested,
            ..
        } = event {
            control_flow.set_exit();
        }

        t.send(event.to_static().unwrap_or(winit::event::Event::NewEvents(winit::event::StartCause::Poll)))
            .unwrap_or_else(|err| eprintln!("{:?}", err));
    })
}

struct PrinterListener {}
impl Listener for PrinterListener {
    fn tick(&mut self, _dt: f32, events: &mut Vec<winit::event::Event<'_, ()>>) {
        for event in events {
            println!("Tick {:?}", event);
        }
    }
}
