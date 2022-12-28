use std::{sync::mpsc, thread};
use saunter::listener::Listener;

fn main() {
    let (t, x) = mpsc::channel::<winit::event::Event<'_, ()>>();
    let mut loop_ = saunter::tickloop::Loop::new(Box::new(PrinterListener {}), 60, x);
    thread::spawn(move || loop_.start());

    let event_loop = winit::event_loop::EventLoop::new();
    event_loop.run( move |event, _, control_flow| {
        control_flow.set_poll();
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::CloseRequested, .. } = event {
            
        }
        //TODO: fix this t.send(event.to_owned()).unwrap();
    })
}

struct PrinterListener {}
impl Listener for PrinterListener {
    fn tick(&mut self, _dt: f32, events: &mut Vec<winit::event::Event<'_, ()>>) {
        for event in events {
            println!("{:?}", event);
        }
    }
}