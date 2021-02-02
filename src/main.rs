mod math;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Example", 800, 600).build().unwrap();
    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> =
        window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(100, 100, 100, 100)).unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.present();
    }
}
