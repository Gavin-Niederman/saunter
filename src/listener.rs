use winit::event::Event;

pub trait Listener: Send + Sync {
    fn tick(&mut self, dt: f32, events: &mut Vec<Event<'_, ()>>);
}
