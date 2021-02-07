mod game;

use femtovg::Color;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use femtovg::{
    //CompositeOperation,
    renderer::OpenGl,
    Canvas,
};

use glutin::ContextBuilder;

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

    let mut g = game::init_game();
    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        let window = windowed_context.window();

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(*physical_size);
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { ref input, .. } => g.handle_input(input),
                _ => (),
            },
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(_) => {
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
                    g.render(canvas);
                });
                canvas.flush();
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
