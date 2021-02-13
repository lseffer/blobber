use crate::math::point::PointF32;
use crate::stopwatch;
use femtovg::{Canvas, Color, FontId, Paint, Path, Renderer};
use std::time::Duration;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct Fonts {
    pub regular: FontId,
}

pub struct Player {
    pub pos: PointF32,
    pub vel: PointF32,
    pub acc: PointF32,
    pub force: PointF32,
    pub mass: f32,
}

impl Player {
    pub fn control_left(&mut self) {
        self.force.x -= 20.0;
    }
    pub fn control_right(&mut self) {
        self.force.x += 20.0;
    }
    pub fn control_up(&mut self) {
        self.force.y -= 20.0;
    }
    pub fn control_down(&mut self) {
        self.force.y += 20.0;
    }

    pub fn simulate(&mut self, dt: Duration) {
        self.force.x *= 0.95;
        self.force.y *= 0.95;
        self.acc.x = self.force.x;
        self.acc.y = self.force.y;
        self.vel.x += self.acc.x * dt.as_secs_f32();
        self.vel.y += self.acc.y * dt.as_secs_f32();
        self.pos.x += self.vel.x * dt.as_secs_f32();
        self.pos.y += self.vel.y * dt.as_secs_f32();
    }

    pub fn render<T: Renderer>(&self, canvas: &mut Canvas<T>) {
        let mut path = Path::new();
        path.circle(self.pos.x, self.pos.y, 40.0);
        canvas.fill_path(&mut path, Paint::color(Color::rgba(0, 0, 0, 128)));
    }
}

pub struct GameFlags {
    show_fps: bool,
}

pub struct GameState {
    pub game_flags: GameFlags,
    pub stopwatch: stopwatch::StopWatch,
    pub player: Player,
}

impl GameState {
    pub fn handle_input(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::KeyboardInput { ref input, .. } => match input {
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        state: ElementState::Pressed,
                        ..
                    } => self.player.control_up(),
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        state: ElementState::Pressed,
                        ..
                    } => self.player.control_down(),
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        state: ElementState::Pressed,
                        ..
                    } => self.player.control_left(),
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Right),
                        state: ElementState::Pressed,
                        ..
                    } => self.player.control_right(),
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }

    pub fn render<T: Renderer>(&mut self, canvas: &mut Canvas<T>, fonts: &Fonts) {
        self.player.render(canvas);
        if self.game_flags.show_fps {
            self.stopwatch.render(canvas, fonts);
        }
    }

    pub fn simulate(&mut self, dt: Duration) {
        self.player.simulate(dt);
        self.stopwatch.simulate();
    }
}

pub fn init_game() -> GameState {
    let g = GameState {
        stopwatch: stopwatch::StopWatch::new(100),
        game_flags: GameFlags { show_fps: true },
        player: Player {
            pos: PointF32 { x: 50.0, y: 50.0 },
            force: PointF32 { x: 20.0, y: 20.0 },
            acc: PointF32 { x: 0.0, y: 0.0 },
            vel: PointF32 { x: 0.0, y: 0.0 },
            mass: 1.0,
        },
    };
    return g;
}
