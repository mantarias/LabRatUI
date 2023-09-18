use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Button Example")
        .build(&event_loop)
        .unwrap();

    let gl_context = match window.new_gl_context() {
        Ok(ctx) => ctx,
        Err(err) => panic!("Failed to create GL context: {}", err),
    };
    gl_context.make_current().unwrap();

    let gl = match glow::Context::from_loader_function(|s| gl_context.get_proc_address(s)) {
        Ok(gl) => gl,
        Err(err) => panic!("Failed to load GL: {}", err),
    };

    let mut button_pressed = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::MouseInput {
                    state,
                    button,
                    ..
                } => {
                    if button == MouseButton::Left {
                        button_pressed = state == ElementState::Pressed;

                        if button_pressed {
                            unsafe {
                                gl.clear_color(0.0, 1.0, 0.0, 1.0);
                            }
                        } else {
                            unsafe {
                                gl.clear_color(1.0, 0.0, 0.0, 1.0);
                            }
                        }

                        window.request_redraw();
                    }
                }
                WindowEvent::RedrawRequested => {
                    unsafe {
                        gl.clear(glow::COLOR_BUFFER_BIT);
                    }
                    gl_context.swap_buffers().unwrap();
                }
                _ => {}
            },
            _ => {}
        }
    });
}
