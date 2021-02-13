mod game;
mod math;
mod stopwatch;

use femtovg::Color;
use femtovg::{renderer::OpenGl, Canvas};
use glutin::ContextBuilder;
use resource::resource;
use std::time::{Duration, Instant};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let (renderer, windowed_context) = {
        let wb = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(1000, 600))
            .with_title("femtovg demo");

        let windowed_context = ContextBuilder::new()
            .with_vsync(false)
            .build_windowed(wb, &event_loop)
            .unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let renderer = OpenGl::new(|s| windowed_context.get_proc_address(s) as *const _)
            .expect("Cannot create renderer");

        (renderer, windowed_context)
    };
    let target_frame_time = Duration::from_micros(16_667);
    let mut t = Duration::from_millis(0);
    let dt = Duration::from_micros(16_667);
    let mut accumulator = Duration::from_millis(0);
    let mut current_time = Instant::now();
    let mut render_time = Instant::now();

    let mut g = game::init_game();
    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
    let fonts = game::Fonts {
        regular: canvas
            .add_font_mem(&resource!("assets/Roboto-Regular.ttf"))
            .expect("Cannot add font"),
    };
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        let window = windowed_context.window();
        g.handle_input(&event);
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(*physical_size);
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::LoopDestroyed => return,
            Event::MainEventsCleared => {
                let new_time = Instant::now();
                let frame_time = new_time - current_time;
                current_time = new_time;
                accumulator += frame_time;
                while accumulator >= dt {
                    g.simulate(dt);
                    t += dt;
                    accumulator -= dt;
                }
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let since_last_render = Instant::now() - render_time;
                if since_last_render < target_frame_time {
                    std::thread::sleep(target_frame_time - since_last_render);
                }
                render_time = Instant::now();
                let size = window.inner_size();
                let dpi_factor = window.scale_factor();
                canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
                canvas.clear_rect(
                    0,
                    0,
                    size.width as u32,
                    size.height as u32,
                    Color::rgbf(255.0, 255.0, 255.0),
                );
                canvas.save_with(|canvas| {
                    canvas.reset();
                    g.render(canvas, &fonts);
                });
                canvas.flush();
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
