use crate::math::rect::Rect;
use crate::simulation::{blob::BlobId, InputEvent, Simulation};
use femtovg;
use std::collections::{HashMap, HashSet};

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn render<T: femtovg::Renderer>(
        &self,
        simulation: &Simulation,
        timepoint: f32,
        canvas: &mut femtovg::Canvas<T>,
    ) {
        let objects = simulation.objects(Rect::new_empty());
        for object in objects {
            let mut path = femtovg::Path::new();
            path.circle(
                object.circle.pos.x,
                object.circle.pos.y,
                object.circle.radius,
            );
            canvas.fill_path(
                &mut path,
                femtovg::Paint::color(femtovg::Color::rgba(0, 0, 0, 128)),
            );
            path = femtovg::Path::new();
            path.move_to(object.circle.pos.x, object.circle.pos.y);
            path.line_to(
                object.circle.pos.x + object.rotation.cos() * 12.0,
                object.circle.pos.y + object.rotation.sin() * 12.0,
            );
            let mut paint = femtovg::Paint::color(femtovg::Color::rgba(255, 0, 0, 128));
            paint.set_line_width(5.0);
            canvas.stroke_path(&mut path, paint);
        }
    }
}

pub struct Game {
    pub simulation: Simulation,
    pub renderer: Renderer,
    keymap: HashMap<winit::event::VirtualKeyCode, (BlobId, InputEvent)>,
}

impl Game {
    fn add_keys(
        keymap: &mut HashMap<winit::event::VirtualKeyCode, (BlobId, InputEvent)>,
        blob_id: BlobId,
        forward: winit::event::VirtualKeyCode,
        backward: winit::event::VirtualKeyCode,
        left: winit::event::VirtualKeyCode,
        right: winit::event::VirtualKeyCode,
    ) {
        keymap.insert(forward, (blob_id, InputEvent::Forward));
        keymap.insert(backward, (blob_id, InputEvent::Backward));
        keymap.insert(left, (blob_id, InputEvent::TurnLeft));
        keymap.insert(right, (blob_id, InputEvent::TurnRight));
    }

    pub fn new() -> Self {
        use winit::event::VirtualKeyCode::*;
        let mut keymap = HashMap::<winit::event::VirtualKeyCode, (BlobId, InputEvent)>::new();

        Game::add_keys(&mut keymap, 0, Up, Down, Left, Right);
        Game::add_keys(&mut keymap, 1, W, S, A, D);

        return Game {
            simulation: Simulation::new(),
            renderer: Renderer::new(),
            keymap,
        };
    }

    pub fn handle_inputs(
        &self,
        keys_down: &HashSet<winit::event::VirtualKeyCode>,
    ) -> HashMap<BlobId, Vec<InputEvent>> {
        // TODO This function is not really needed, instead an 'queue_input' or something
        // can be added, that should be called after each input.
        let mut input_events = HashMap::<BlobId, Vec<InputEvent>>::new();

        // This only works if each input key maps to one InputEvent, this might
        // not be the case (e.g. Forward + Ctrl should only lead to
        // InputEvent::FullThrottleForward and not one InputEvent::Forward and
        // InputEvent::FullThrottle/InputEvent::FullThrottleForward).
        // Then we first have to check all combined keys, remove them from vector, or something.
        for key in keys_down {
            match self.keymap.get(&key) {
                Some((blob_id, mut event)) => input_events
                    .entry(*blob_id)
                    .or_insert(Vec::new())
                    .push(event),
                None => (),
            }
        }

        // We remove duplicates.
        // NOTE Could be done here, but maybe instead when the simulation
        // receives the events?
        for (_, events) in &mut input_events {
            events.sort();
            events.dedup();
        }

        input_events
    }

    pub fn render<T: femtovg::Renderer>(&self, timepoint: f32, canvas: &mut femtovg::Canvas<T>) {
        self.renderer.render(&self.simulation, timepoint, canvas);
    }
}
