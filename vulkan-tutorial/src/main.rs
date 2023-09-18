


#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello Diego")
        .with_inner_size(LogicalSize::new(1024, 1024))
        .build(&event_loop)?;

    // App

    let mut app = unsafe { App::create(&window)? };
    let mut destroying = false;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            // Render a frame if our Vulkan app is not being destroyed.
            Event::MainEventsCleared if !destroying =>
                unsafe { app.render(&window) }.unwrap(),
            // Destroy our Vulkan app.
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                unsafe { app.destroy(); }
            }
            _ => {}
        }
    });
}

/// Our Vulkan app.
#[derive(Clone, Debug)]
struct App {
}

impl App {
    /// Creates our Vulkan app.
    unsafe fn create(window: &Window) -> Result<Self> {
        }

    /// Renders a frame for our Vulkan app.
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        self.draw_triangle()?;
        Ok(())
    }

    /// Destroys our Vulkan app.
    unsafe fn destroy(&mut self) {}
    
    /// Draw a triangle.
    unsafe fn draw_triangle(&self) -> Result<()> {
        // Step 1: Initialize Vulkan (Done in your `create` method)

        // Step 2: Create Buffers
        // Create a buffer for the three vertices making up the triangle.

        

        // Step 3: Define Shaders
        // Define a simple vertex shader to just pass through the vertex coordinates.
        // Define a simple fragment shader to set all pixels to a single color.

        // Step 4: Pipeline Creation
        // Create a simple pipeline that uses your simple shaders.

        // Step 5: Rendering
        // Record commands into a command buffer to draw the triangle.

        // Step 6: Presentation
        // Submit the command buffer to a queue to be rendered.

        Ok(())
    }
}



/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {}
