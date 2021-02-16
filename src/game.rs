use femtovg;
use std::time::Duration;
use winit::event::{Event};

pub struct Renderer {}

pub struct Simulation {}

impl Simulation {
    pub fn new() -> Self {
        Simulation {}
    }
    pub fn handle_input(&mut self, event: &Event<()>) {}
    pub fn simulate(&mut self, dt: Duration) {}
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }
    pub fn render<T: femtovg::Renderer>(&self, canvas: &mut femtovg::Canvas<T>) {}
}

pub struct Game {
    pub renderer: Renderer,
    pub simulation: Simulation,
}

impl Game {
    pub fn new() -> Self {
        return Game {
            renderer: Renderer::new(),
            simulation: Simulation::new(),
        };
    }
}
