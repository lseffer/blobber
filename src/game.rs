use femtovg::{Canvas, Color, Paint, Path, Renderer};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct Camera {
    pos: f32,
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Player {
    pub pos: Point,
}

impl Player {
    pub fn control_left(&mut self) {
        self.pos.x -= 10.0;
    }
    pub fn control_right(&mut self) {
        self.pos.x += 10.0;
    }
    pub fn control_up(&mut self) {
        self.pos.y -= 10.0;
    }
    pub fn control_down(&mut self) {
        self.pos.y += 10.0;
    }
}

pub struct GameState {
    camera: Camera,
    pub player: Player,
}

impl GameState {
    pub fn handle_input(&mut self, input: &KeyboardInput) {
        match input {
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
        }
    }

    pub fn render<T: Renderer>(&self, canvas: &mut Canvas<T>) {
        let mut path = Path::new();
        path.rect(self.player.pos.x, self.player.pos.y, 100.0, 100.0);
        canvas.fill_path(&mut path, Paint::color(Color::rgba(0, 0, 0, 128)));
    }
}

pub fn init_game() -> GameState {
    let c = Camera { pos: 0.0 };
    let g = GameState {
        camera: c,
        player: Player {
            pos: Point { x: 0.0, y: 1.0 },
        },
    };
    return g;
}
